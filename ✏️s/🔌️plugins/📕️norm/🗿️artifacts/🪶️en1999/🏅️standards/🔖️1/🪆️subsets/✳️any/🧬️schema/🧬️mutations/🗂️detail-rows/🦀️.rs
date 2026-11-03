//! 🗂️ EN 1999 weld run mutation leaves: insert weld run, remove weld run, insert outline vertex and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-weld-run/🦀️.rs"]
pub mod insert_weld_run;
#[path = "../➖remove-weld-run/🦀️.rs"]
pub mod remove_weld_run;
#[path = "../➕insert-outline-vertex/🦀️.rs"]
pub mod insert_outline_vertex;
#[path = "../➖remove-outline-vertex/🦀️.rs"]
pub mod remove_outline_vertex;
#[path = "../➕insert-placement/🦀️.rs"]
pub mod insert_placement;
#[path = "../➖remove-placement/🦀️.rs"]
pub mod remove_placement;
//#endregion 🪢️Mounts
