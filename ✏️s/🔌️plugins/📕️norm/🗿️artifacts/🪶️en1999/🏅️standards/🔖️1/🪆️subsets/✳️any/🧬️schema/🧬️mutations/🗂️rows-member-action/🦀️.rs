//! 🗂️ EN 1999 member action mutation leaves: insert member action, remove member action.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-member-action/🦀️.rs"]
pub mod insert_member_action;
#[path = "../➖remove-member-action/🦀️.rs"]
pub mod remove_member_action;
//#endregion 🪢️Mounts
