//! 🔩️ EN 1993 joint mutation leaves: kind, member id, bolt class and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-kind/🦀️.rs"]
pub mod change_joint_kind;
#[path = "../🔩️change-joint-member-id/🦀️.rs"]
pub mod change_joint_member_id;
#[path = "../🔩️change-joint-bolt-class/🦀️.rs"]
pub mod change_joint_bolt_class;
#[path = "../🔩️change-joint-bolt-diameter/🦀️.rs"]
pub mod change_joint_bolt_diameter;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_more::{change_joint_bolt_rows, change_joint_bolts_per_row, change_joint_pitch, change_joint_gauge, change_joint_end_distance};
