//! 🔲 EN 1996 flange walls mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔲change-flange-length/🦀️.rs"]
pub mod change_flange_length;
#[path = "../🔲change-flange-thickness/🦀️.rs"]
pub mod change_flange_thickness;
#[path = "../🔲change-flange-position/🦀️.rs"]
pub mod change_flange_position;
#[path = "../➕️insert-flange/🦀️.rs"]
pub mod insert_flange;
#[path = "../➖️remove-flange/🦀️.rs"]
pub mod remove_flange;
//#endregion 🪢️Mounts
