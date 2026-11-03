//! 🏋️ EN 1992 member actions identity mutation leaves: action category, action kind, action source.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-action-kind/🦀️.rs"]
pub mod change_action_kind;
#[path = "../🏋️change-action-category/🦀️.rs"]
pub mod change_action_category;
#[path = "../🏋️change-action-source/🦀️.rs"]
pub mod change_action_source;
//#endregion 🪢️Mounts
