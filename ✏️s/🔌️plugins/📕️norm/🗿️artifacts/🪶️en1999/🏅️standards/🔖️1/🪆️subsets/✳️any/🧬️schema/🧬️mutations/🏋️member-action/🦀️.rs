//! 🏋️ EN 1999 member action mutation leaves: kind, category, source and more.

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
#[path = "../🏋️change-action-gk-line/🦀️.rs"]
pub mod change_action_g_k_line;
#[path = "../🏋️change-action-qk-line/🦀️.rs"]
pub mod change_action_q_k_line;
#[path = "../🏋️change-action-vyk/🦀️.rs"]
pub mod change_action_v_y_k;
#[path = "../🏋️change-action-vzk/🦀️.rs"]
pub mod change_action_v_z_k;
//#endregion 🪢️Mounts
