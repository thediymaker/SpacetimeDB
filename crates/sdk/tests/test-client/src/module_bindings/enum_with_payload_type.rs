// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.1.1 (commit 5611e402a8a162f0bcd903cc5db9cf24a00476f8).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::simple_enum_type::SimpleEnum;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub enum EnumWithPayload {
    U8(u8),

    U16(u16),

    U32(u32),

    U64(u64),

    U128(u128),

    U256(__sats::u256),

    I8(i8),

    I16(i16),

    I32(i32),

    I64(i64),

    I128(i128),

    I256(__sats::i256),

    Bool(bool),

    F32(f32),

    F64(f64),

    Str(String),

    Identity(__sdk::Identity),

    ConnectionId(__sdk::ConnectionId),

    Timestamp(__sdk::Timestamp),

    Bytes(Vec<u8>),

    Ints(Vec<i32>),

    Strings(Vec<String>),

    SimpleEnums(Vec<SimpleEnum>),
}

impl __sdk::InModule for EnumWithPayload {
    type Module = super::RemoteModule;
}
