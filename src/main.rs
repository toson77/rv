fn main() {}

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

const MEMORY_SIZE: u32 = 1024 * 1024;

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE as usize],
        }
    }

    pub fn write(&mut self, addr: u32, word: u32) {
        let index = addr as usize;
        self.data[index] = (word & 0xff) as u8;
        self.data[index + 1] = (word >> 8 & 0xff) as u8;
        self.data[index + 2] = ((word >> 16) & 0xff) as u8;
        self.data[index + 3] = ((word >> 24) & 0xff) as u8;
    }

    pub fn read(&self, addr: u32) -> u32 {
        let index = addr as usize;
        self.data[index] as u32
            | (self.data[index + 1] as u32) << 8
            | (self.data[index + 2] as u32) << 16
            | (self.data[index + 3] as u32) << 24
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.data.splice(..data.len(), data);
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add { rd: u32, rs1: u32, rs2: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Or { rd: u32, rs1: u32, rs2: u32 },
    And { rd: u32, rs1: u32, rs2: u32 },
    Addi { rd: u32, rs1: u32, imm: u32 },
    Slli { rd: u32, rs1: u32, imm: u32 },
    Beq { rd: u32, rs1: u32, imm: u32 },
    Lw { rd: u32, rs1: u32, imm: u32 },
    Sw { rd: u32, rs1: u32, imm: u32 },
}

impl Instruction {
    pub fn decode(inst: u32) -> Result<Instruction, Error> {
        let opcode = inst & 0x0000007f;
        let rd = (inst & 0x00000f80) >> 7;
        let funct3 = (inst & 0x00007000) >> 12;
        let rs1 = (inst & 0x000f800) >> 15;
        let rs2(inst & 0xfe000000) >> 25;

        match opcode {
            0b0010011 => {
                let imm = (inst & 0xfff00000) >> 20;
                match funct3 {
                    0x0 => Ok(Addi {rd, rs1, imm}),
                    0x1 => Ok(Slli {rd, rs1, imm}),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
        },

    }
}
