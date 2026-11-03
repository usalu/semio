//! 🔩️ EN 1993 joint row mutation leaves: joint id, label, row type and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-row-joint-id/🦀️.rs"]
pub mod change_row_joint_id;
#[path = "../🔩️change-row-label/🦀️.rs"]
pub mod change_row_label;
#[path = "../🔩️change-row-row-type/🦀️.rs"]
pub mod change_row_row_type;
#[path = "../🔩️change-row-lever/🦀️.rs"]
pub mod change_row_lever;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_row_more::{change_row_bolt_to_web, change_row_edge_distance, change_row_extension_to_flange, change_row_extension_edge};
