//! ⚖️ EN 1995 actions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢️change-member-action-category/🦀️.rs"]
pub mod change_member_action_category;
#[path = "../⬇️change-member-action-f-point/🦀️.rs"]
pub mod change_member_action_f_point;
#[path = "../⚖️change-member-action-kind/🦀️.rs"]
pub mod change_member_action_kind;
#[path = "../⬇️change-member-action-q-line/🦀️.rs"]
pub mod change_member_action_q_line;
#[path = "../⏳️change-member-load-duration/🦀️.rs"]
pub mod change_member_load_duration;
#[path = "../➕️insert-member-action/🦀️.rs"]
pub mod insert_member_action;
#[path = "../➖️remove-member-action/🦀️.rs"]
pub mod remove_member_action;
//#endregion 🪢️Mounts
