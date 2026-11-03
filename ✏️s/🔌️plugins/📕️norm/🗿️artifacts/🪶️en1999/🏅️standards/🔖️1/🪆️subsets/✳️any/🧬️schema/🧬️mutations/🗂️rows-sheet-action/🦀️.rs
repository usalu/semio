//! 🗂️ EN 1999 cold-formed sheet action mutation leaves: insert sheet action, remove sheet action.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-sheet-action/🦀️.rs"]
pub mod insert_sheet_action;
#[path = "../➖remove-sheet-action/🦀️.rs"]
pub mod remove_sheet_action;
//#endregion 🪢️Mounts
