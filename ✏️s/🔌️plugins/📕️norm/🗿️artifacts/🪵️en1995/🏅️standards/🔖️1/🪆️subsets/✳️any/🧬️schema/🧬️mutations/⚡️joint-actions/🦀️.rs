//! ⚡️ EN 1995 joint actions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-connection-action-fk/🦀️.rs"]
pub mod change_connection_action_fk;
#[path = "../⚖️change-connection-action-kind/🦀️.rs"]
pub mod change_connection_action_kind;
#[path = "../⏳️change-connection-load-duration/🦀️.rs"]
pub mod change_connection_load_duration;
#[path = "../➕️insert-connection-action/🦀️.rs"]
pub mod insert_connection_action;
#[path = "../➖️remove-connection-action/🦀️.rs"]
pub mod remove_connection_action;
//#endregion 🪢️Mounts
