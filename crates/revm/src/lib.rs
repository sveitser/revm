#![allow(dead_code)]
//#![no_std]

pub mod db;
mod evm;
mod evm_impl;
mod inspector;
mod instructions;
mod machine;
mod models;
mod spec;
mod subroutine;
mod util;

pub use evm_impl::{EVMData, Host};

pub type DummyStateDB = InMemoryDB;

pub use db::{Database, DatabaseCommit, InMemoryDB};
pub use evm::{new, EVM};
pub use inspector::{Inspector, NoOpInspector, OverrideSpec};
pub use instructions::{
    opcode::{self, spec_opcode_gas, OpCode, OPCODE_JUMPMAP},
    Return,
};
pub use machine::{Gas, Machine};
pub use models::*;
pub use spec::*;
pub use subroutine::{Account, SubRoutine};

extern crate alloc;

pub(crate) const USE_GAS: bool = !cfg!(feature = "no_gas_measuring");

#[repr(C)]
#[derive(Debug)]
pub struct RetUint {
    pub n1: u64,
    pub n2: u64,
    pub n3: u64,
    pub n4: u64,
}

#[link(name = "intx")]
extern "C" {
    pub fn fast_div_rem(f: *const u64, s: *const u64) -> RetUint;
}

use primitive_types::{H256, U256};

pub fn test_it() {
    let f = U256::from_big_endian(H256::from_low_u64_be(20).as_ref());
    let s = U256::from_big_endian(H256::from_low_u64_be(10).as_ref());

    let t = unsafe { fast_div_rem(f.as_ref().as_ptr(), s.as_ref().as_ptr()) };
    println!("TEST_IT:{:?}", t);
}
