//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-anchor-length/🦀️.rs"]
pub mod change_joint_anchor_length;
#[path = "../🔩️change-joint-base-normal-force/🦀️.rs"]
pub mod change_joint_base_normal_force;
#[path = "../🔩️change-joint-base-moment/🦀️.rs"]
pub mod change_joint_base_moment;
#[path = "../🔩️change-joint-base-lever-left/🦀️.rs"]
pub mod change_joint_base_lever_left;
#[path = "../🔩️change-joint-base-lever-right/🦀️.rs"]
pub mod change_joint_base_lever_right;
//#endregion 🪢️Mounts
