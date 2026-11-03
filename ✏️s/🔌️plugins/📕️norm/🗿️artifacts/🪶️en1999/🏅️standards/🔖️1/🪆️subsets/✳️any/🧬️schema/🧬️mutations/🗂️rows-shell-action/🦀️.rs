//! 🗂️ EN 1999 shell action mutation leaves: insert shell action, remove shell action.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-shell-action/🦀️.rs"]
pub mod insert_shell_action;
#[path = "../➖remove-shell-action/🦀️.rs"]
pub mod remove_shell_action;
//#endregion 🪢️Mounts
