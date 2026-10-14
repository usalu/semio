//! 🏭️ EN 1998 silos tanks mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🫙insert-silo/🦀️.rs"]
pub mod insert_silo;
#[path = "../🛢️insert-tank/🦀️.rs"]
pub mod insert_tank;
#[path = "../➖️remove-silo/🦀️.rs"]
pub mod remove_silo;
#[path = "../➖️remove-tank/🦀️.rs"]
pub mod remove_tank;
//#endregion 🪢️Mounts
