//! 🧵️ EN 1993 row mutation leaves: weld-group, parametric-section, hollow-joint, joint-row.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-weld-group/🦀️.rs"]
pub mod insert_weld_group;
#[path = "../➖️remove-weld-group/🦀️.rs"]
pub mod remove_weld_group;
#[path = "../➕️insert-parametric-section/🦀️.rs"]
pub mod insert_parametric_section;
#[path = "../➖️remove-parametric-section/🦀️.rs"]
pub mod remove_parametric_section;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_rows_more::{insert_hollow_joint, remove_hollow_joint, insert_joint_row, remove_joint_row};
