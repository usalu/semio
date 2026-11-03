//! 🏛️ EN 1994 column action rows mutation leaves: column action.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️insert-column-action/🦀️.rs"]
pub mod insert_column_action;
#[path = "../🏛️remove-column-action/🦀️.rs"]
pub mod remove_column_action;
//#endregion 🪢️Mounts
