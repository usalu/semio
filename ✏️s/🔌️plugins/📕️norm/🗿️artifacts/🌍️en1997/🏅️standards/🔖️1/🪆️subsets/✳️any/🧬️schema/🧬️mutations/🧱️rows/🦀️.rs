//! 🧱 EN 1997 row mutation leaves (layers, footings, piles, retaining walls, slopes).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-retaining-wall/🦀️.rs"]
pub mod insert_retaining_wall;
#[path = "../➕insert-slope/🦀️.rs"]
pub mod insert_slope;
#[path = "../➖remove-retaining-wall/🦀️.rs"]
pub mod remove_retaining_wall;
#[path = "../➖remove-slope/🦀️.rs"]
pub mod remove_slope;
//#endregion 🪢️Mounts
