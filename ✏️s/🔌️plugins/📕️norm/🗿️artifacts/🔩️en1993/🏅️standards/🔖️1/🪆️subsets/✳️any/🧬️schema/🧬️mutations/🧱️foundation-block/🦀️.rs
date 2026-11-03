//! 🧱️ EN 1993 foundation block mutation leaves: joint id, label, block length and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-foundation-joint-id/🦀️.rs"]
pub mod change_foundation_joint_id;
#[path = "../🧱️change-foundation-label/🦀️.rs"]
pub mod change_foundation_label;
#[path = "../🧱️change-foundation-block-length/🦀️.rs"]
pub mod change_foundation_block_length;
#[path = "../🧱️change-foundation-block-width/🦀️.rs"]
pub mod change_foundation_block_width;
#[path = "../🧱️change-foundation-block-depth/🦀️.rs"]
pub mod change_foundation_block_depth;
#[path = "../🧱️change-foundation-concrete-fck/🦀️.rs"]
pub mod change_foundation_concrete_fck;
//#endregion 🪢️Mounts
