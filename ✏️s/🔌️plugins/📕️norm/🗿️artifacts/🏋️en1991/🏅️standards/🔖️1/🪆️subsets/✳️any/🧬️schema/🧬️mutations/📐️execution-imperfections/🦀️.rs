//! 📐 EN 1991 execution imperfections mutation leaves: imperfection, imperfection assumed force, imperfection height and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-imperfection-height/🦀️.rs"]
pub mod change_imperfection_height;
#[path = "../📐️change-imperfection-tolerance/🦀️.rs"]
pub mod change_imperfection_tolerance;
#[path = "../📐️change-imperfection-vertical-load/🦀️.rs"]
pub mod change_imperfection_vertical_load;
#[path = "../📐️change-imperfection-x/🦀️.rs"]
pub mod change_imperfection_x;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_execution_imperfections_more::{change_imperfection_y, change_imperfection_assumed_force, insert_imperfection, remove_imperfection};
