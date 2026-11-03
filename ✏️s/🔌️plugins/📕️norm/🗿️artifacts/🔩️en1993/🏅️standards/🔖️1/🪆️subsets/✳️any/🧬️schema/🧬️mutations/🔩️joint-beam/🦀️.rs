//! 🔩️ EN 1993 joint mutation leaves: beam inertia, beam span, braced frame and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-beam-inertia/🦀️.rs"]
pub mod change_joint_beam_inertia;
#[path = "../🔩️change-joint-beam-span/🦀️.rs"]
pub mod change_joint_beam_span;
#[path = "../🔩️change-joint-braced-frame/🦀️.rs"]
pub mod change_joint_braced_frame;
#[path = "../🔩️change-joint-stiffness-ratio/🦀️.rs"]
pub mod change_joint_stiffness_ratio;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_beam_more::{change_joint_xform_parameter, change_joint_assumed_class, change_joint_base_plate_length, change_joint_concrete_fcd, change_joint_concrete_modulus};
