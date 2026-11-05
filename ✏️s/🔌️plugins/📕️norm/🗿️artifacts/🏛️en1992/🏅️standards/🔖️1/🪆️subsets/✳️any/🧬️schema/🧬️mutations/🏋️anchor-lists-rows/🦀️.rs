//! 🏋️ EN 1992 anchor lists rows mutation leaves: anchor action.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️insert-anchor-action/🦀️.rs"]
pub mod insert_anchor_action;
#[path = "../🏋️remove-anchor-action/🦀️.rs"]
pub mod remove_anchor_action;
//#endregion 🪢️Mounts
