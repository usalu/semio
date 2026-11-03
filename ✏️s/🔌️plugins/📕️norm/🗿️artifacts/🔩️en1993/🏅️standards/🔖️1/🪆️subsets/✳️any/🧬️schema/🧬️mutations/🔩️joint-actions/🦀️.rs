//! 🔩️ EN 1993 joint action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-joint-action/🦀️.rs"]
pub mod insert_joint_action;
#[path = "../➖️remove-joint-action/🦀️.rs"]
pub mod remove_joint_action;
#[path = "../🔩️change-joint-action-load-case-id/🦀️.rs"]
pub mod change_joint_action_load_case_id;
#[path = "../🔩️change-joint-action-shear/🦀️.rs"]
pub mod change_joint_action_shear;
#[path = "../🔩️change-joint-action-tension/🦀️.rs"]
pub mod change_joint_action_tension;
//#endregion 🪢️Mounts
