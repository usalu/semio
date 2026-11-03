//! 🪛 FastenedJoint mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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
#[path = "../🧱️change-fastened-joint-shear-resistance/🦀️.rs"]
pub mod change_fastened_joint_shear_resistance;
#[path = "../🧱️change-fastened-joint-tension-force/🦀️.rs"]
pub mod change_fastened_joint_tension_force;
#[path = "../🧱️change-fastened-joint-shear-force/🦀️.rs"]
pub mod change_fastened_joint_shear_force;
#[path = "../🧱️change-fastened-joint-sheet-width/🦀️.rs"]
pub mod change_fastened_joint_sheet_width;
#[path = "../🧱️change-fastened-joint-gauge/🦀️.rs"]
pub mod change_fastened_joint_gauge;
#[path = "../🧱️change-fastened-joint-pitch/🦀️.rs"]
pub mod change_fastened_joint_pitch;
#[path = "../🧱️change-fastened-joint-edge-distance/🦀️.rs"]
pub mod change_fastened_joint_edge_distance;
#[path = "../🧱️change-fastened-joint-end-distance/🦀️.rs"]
pub mod change_fastened_joint_end_distance;
#[path = "../🧱️change-fastened-joint-base-strength/🦀️.rs"]
pub mod change_fastened_joint_base_strength;
#[path = "../🧱️change-fastened-joint-head-strength/🦀️.rs"]
pub mod change_fastened_joint_head_strength;
#[path = "../🧱️change-fastened-joint-base-thickness/🦀️.rs"]
pub mod change_fastened_joint_base_thickness;
#[path = "../🧱️change-fastened-joint-head-thickness/🦀️.rs"]
pub mod change_fastened_joint_head_thickness;
#[path = "../🧱️change-fastened-joint-washer-diameter/🦀️.rs"]
pub mod change_fastened_joint_washer_diameter;
#[path = "../🧱️change-fastened-joint-diameter/🦀️.rs"]
pub mod change_fastened_joint_diameter;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_member_warping_more::{change_frame_member_basis_member_id, change_frame_action_end_ratio, change_frame_action_start_ratio, change_frame_action_fz, change_frame_action_fx};
