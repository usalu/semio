//! 🔗️ EN 1999 connection and shell collection-row mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-connection/🦀️.rs"]
pub mod insert_connection;
#[path = "../➖remove-connection/🦀️.rs"]
pub mod remove_connection;
#[path = "../➕insert-shell/🦀️.rs"]
pub mod insert_shell;
#[path = "../➖remove-shell/🦀️.rs"]
pub mod remove_shell;
//#endregion 🪢️Mounts
