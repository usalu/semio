//! 🔩️ EN 1993 joint mutation leaves: extension to flange, extension edge, alpha factor and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-extension-to-flange/🦀️.rs"]
pub mod change_joint_extension_to_flange;
#[path = "../🔩️change-joint-extension-edge/🦀️.rs"]
pub mod change_joint_extension_edge;
#[path = "../🔩️change-joint-alpha-factor/🦀️.rs"]
pub mod change_joint_alpha_factor;
#[path = "../🔩️change-joint-bolt-stress-area/🦀️.rs"]
pub mod change_joint_bolt_stress_area;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_extension_more::{change_joint_bolt_grip_length, change_joint_column_flange_thk, change_joint_column_web_thickness, change_joint_column_web_depth, change_joint_column_web_shear_area};
