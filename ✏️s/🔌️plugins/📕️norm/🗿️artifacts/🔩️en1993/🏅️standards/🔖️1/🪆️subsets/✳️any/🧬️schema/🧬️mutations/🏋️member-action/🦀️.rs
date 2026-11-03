//! 🏋️ EN 1993 member action mutation leaves: member id, load case id, action n and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-action-member-id/🦀️.rs"]
pub mod change_action_member_id;
#[path = "../🏋️change-action-load-case-id/🦀️.rs"]
pub mod change_action_load_case_id;
#[path = "../🏋️change-action-n/🦀️.rs"]
pub mod change_action_n;
#[path = "../🏋️change-action-vy/🦀️.rs"]
pub mod change_action_vy;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_member_action_more::{change_action_vz, change_action_my, change_action_mz, change_action_t};
