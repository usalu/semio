//! 🗂️ EN 1999 connection action mutation leaves: insert connection action, remove connection action.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-connection-action/🦀️.rs"]
pub mod insert_connection_action;
#[path = "../➖remove-connection-action/🦀️.rs"]
pub mod remove_connection_action;
//#endregion 🪢️Mounts
