//! 🗂️ EN 1999 transverse stiffener mutation leaves: insert transverse stiffener, remove transverse stiffener.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-transverse-stiffener/🦀️.rs"]
pub mod insert_transverse_stiffener;
#[path = "../➖remove-transverse-stiffener/🦀️.rs"]
pub mod remove_transverse_stiffener;
//#endregion 🪢️Mounts
