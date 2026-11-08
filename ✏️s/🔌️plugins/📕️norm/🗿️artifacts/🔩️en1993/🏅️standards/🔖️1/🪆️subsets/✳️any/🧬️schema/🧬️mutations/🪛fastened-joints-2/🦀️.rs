//! (part 2) 🪛 FastenedJoint mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
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
//#endregion 🪢️Mounts
