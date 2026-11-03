//! 🏋️ EN 1992 anchor actions identity mutation leaves: anchor action category, anchor action kind, anchor action source.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-anchor-action-kind/🦀️.rs"]
pub mod change_anchor_action_kind;
#[path = "../🏋️change-anchor-action-category/🦀️.rs"]
pub mod change_anchor_action_category;
#[path = "../🏋️change-anchor-action-source/🦀️.rs"]
pub mod change_anchor_action_source;
//#endregion 🪢️Mounts
