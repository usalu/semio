//! 🏬️ EN 1991 gravity floor mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢change-floor-assumed-qk/🦀️.rs"]
pub mod change_floor_assumed_qk;
#[path = "../➕️insert-floor/🦀️.rs"]
pub mod insert_floor;
#[path = "../➖️remove-floor/🦀️.rs"]
pub mod remove_floor;
#[path = "../🗺️change-floor-area/🦀️.rs"]
pub mod change_floor_area;
//#endregion 🪢️Mounts
