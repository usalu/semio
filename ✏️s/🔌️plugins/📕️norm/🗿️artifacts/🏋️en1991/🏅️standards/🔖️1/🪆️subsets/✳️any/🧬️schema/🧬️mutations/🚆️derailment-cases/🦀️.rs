//! 🚆 EN 1991 derailment cases mutation leaves: derailment, derailment assumed force, derailment design force x and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚆️change-derailment-distance/🦀️.rs"]
pub mod change_derailment_distance;
#[path = "../🚆️change-derailment-design-force-x/🦀️.rs"]
pub mod change_derailment_design_force_x;
#[path = "../🚆️change-derailment-design-force-y/🦀️.rs"]
pub mod change_derailment_design_force_y;
#[path = "../🚆️change-derailment-x/🦀️.rs"]
pub mod change_derailment_x;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_derailment_cases_more::{change_derailment_y, change_derailment_assumed_force, insert_derailment, remove_derailment};
