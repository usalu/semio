//! (part 3) 🪛 FastenedJoint mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
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
