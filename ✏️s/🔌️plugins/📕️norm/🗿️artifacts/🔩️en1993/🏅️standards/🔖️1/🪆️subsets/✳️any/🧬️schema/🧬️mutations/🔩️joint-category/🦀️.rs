//! 🔩️ EN 1993 joint mutation leaves: category, friction mu, preload force and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-category/🦀️.rs"]
pub mod change_joint_category;
#[path = "../🔩️change-joint-friction-mu/🦀️.rs"]
pub mod change_joint_friction_mu;
#[path = "../🔩️change-joint-preload-force/🦀️.rs"]
pub mod change_joint_preload_force;
#[path = "../🔩️change-joint-slip-factor-ks/🦀️.rs"]
pub mod change_joint_slip_factor_ks;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_category_more::{change_joint_friction_surfaces, change_joint_plate_fy, change_joint_plate_width, change_joint_bolt_to_web};
