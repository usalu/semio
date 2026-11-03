//! 🏗️ EN 1999 member mutation leaves: bearing length, transverse weld and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-bearing-length/🦀️.rs"]
pub mod change_member_bearing_length;
#[path = "../🏗️change-member-transverse-weld/🦀️.rs"]
pub mod change_member_transverse_weld;
#[path = "../🏗️change-member-t-weld-offset/🦀️.rs"]
pub mod change_member_t_weld_offset;
//#endregion 🪢️Mounts
