//! 🛤️ EN 1996 lintel mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛤️change-lintel-position/🦀️.rs"]
pub mod change_lintel_position;
#[path = "../🛤️change-lintel-span/🦀️.rs"]
pub mod change_lintel_span;
#[path = "../🛤️change-lintel-depth/🦀️.rs"]
pub mod change_lintel_depth;
#[path = "../🛤️change-lintel-bearing-length/🦀️.rs"]
pub mod change_lintel_bearing_length;
#[path = "../➕️insert-lintel/🦀️.rs"]
pub mod insert_lintel;
#[path = "../➖️remove-lintel/🦀️.rs"]
pub mod remove_lintel;
//#endregion 🪢️Mounts
