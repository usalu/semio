//! 🧩️ EN 1993 row mutation leaves: joint-plate, foundation-block.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-joint-plate/🦀️.rs"]
pub mod insert_joint_plate;
#[path = "../➖️remove-joint-plate/🦀️.rs"]
pub mod remove_joint_plate;
#[path = "../➕️insert-foundation-block/🦀️.rs"]
pub mod insert_foundation_block;
#[path = "../➖️remove-foundation-block/🦀️.rs"]
pub mod remove_foundation_block;
#[path = "../➕️insert-frame-member-basis/🦀️.rs"]
pub mod insert_frame_member_basis;
#[path = "../➖️remove-frame-member-basis/🦀️.rs"]
pub mod remove_frame_member_basis;
#[path = "../➕️insert-frame-action/🦀️.rs"]
pub mod insert_frame_action;
#[path = "../➖️remove-frame-action/🦀️.rs"]
pub mod remove_frame_action;
//#endregion 🪢️Mounts
