//! 🪛 FastenedJoint mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-fastened-joint-kind/🦀️.rs"]
pub mod change_fastened_joint_kind;
#[path = "../🧱️change-fastened-joint-per-row/🦀️.rs"]
pub mod change_fastened_joint_per_row;
#[path = "../🧱️change-fastened-joint-count/🦀️.rs"]
pub mod change_fastened_joint_count;
#[path = "../➕️insert-fastened-joint/🦀️.rs"]
pub mod insert_fastened_joint;
#[path = "../➖️remove-fastened-joint/🦀️.rs"]
pub mod remove_fastened_joint;
#[path = "../🧱️change-fastened-joint-member-id/🦀️.rs"]
pub mod change_fastened_joint_member_id;
#[path = "../🧱️change-panel-steel-grade/🦀️.rs"]
pub mod change_panel_steel_grade;
#[path = "../🧱️change-panel-steel-subgrade/🦀️.rs"]
pub mod change_panel_steel_subgrade;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_fastened_joints_2::{change_fastened_joint_shear_resistance, change_fastened_joint_tension_force, change_fastened_joint_shear_force, change_fastened_joint_sheet_width, change_fastened_joint_gauge, change_fastened_joint_pitch, change_fastened_joint_edge_distance, change_fastened_joint_end_distance};
pub use semio_s_artifact_norm_en1993_mut_fastened_joints_3::{change_fastened_joint_base_strength, change_fastened_joint_head_strength, change_fastened_joint_base_thickness, change_fastened_joint_head_thickness, change_fastened_joint_washer_diameter, change_fastened_joint_diameter};
