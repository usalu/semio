//! 🔩️ EN 1993 joint mutation leaves: concrete area a0, concrete area a1, joint coefficient and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-concrete-area-a0/🦀️.rs"]
pub mod change_joint_concrete_area_a0;
#[path = "../🔩️change-joint-concrete-area-a1/🦀️.rs"]
pub mod change_joint_concrete_area_a1;
#[path = "../🔩️change-joint-joint-coefficient/🦀️.rs"]
pub mod change_joint_joint_coefficient;
#[path = "../🔩️change-joint-anchor-area/🦀️.rs"]
pub mod change_joint_anchor_area;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_concrete_more::{change_joint_anchor_length, change_joint_base_normal_force, change_joint_base_moment, change_joint_base_lever_left, change_joint_base_lever_right};
