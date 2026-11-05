//! 🏢️ EN 1991 gravity mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-assumed-construction-qk/🦀️.rs"]
pub mod change_assumed_construction_qk;
#[path = "../🔥change-construction-activity/🦀️.rs"]
pub mod change_construction_activity;
#[path = "../🚧change-self-weight-assumed-gk/🦀️.rs"]
pub mod change_self_weight_assumed_gk;
#[path = "../➕️insert-self-weight-element/🦀️.rs"]
pub mod insert_self_weight_element;
#[path = "../➖️remove-self-weight-element/🦀️.rs"]
pub mod remove_self_weight_element;
#[path = "../🥞change-self-weight-thickness/🦀️.rs"]
pub mod change_self_weight_thickness;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_gravity_floors::{change_floor_assumed_qk, insert_floor, remove_floor, change_floor_area};
