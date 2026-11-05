//! 🏛️ EN 1997 footing mutation leaves: length, base inclination deg, settlement limit.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️change-footing-length/🦀️.rs"]
pub mod change_footing_length;
#[path = "../🏛️change-footing-base-incl-deg/🦀️.rs"]
pub mod change_footing_base_incl_deg;
#[path = "../🏛️change-footing-settlement-limit/🦀️.rs"]
pub mod change_footing_settlement_limit;
//#endregion 🪢️Mounts
