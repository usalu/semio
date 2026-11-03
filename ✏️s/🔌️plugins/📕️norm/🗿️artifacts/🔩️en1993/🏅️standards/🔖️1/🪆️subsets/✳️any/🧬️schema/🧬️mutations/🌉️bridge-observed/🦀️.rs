//! 🌉️ EN 1993 bridge fatigue mutation leaves: observed traffic, design life, second lane traffic and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-observed-traffic/🦀️.rs"]
pub mod change_bridge_observed_traffic;
#[path = "../🌉️change-bridge-design-life/🦀️.rs"]
pub mod change_bridge_design_life;
#[path = "../🌉️change-bridge-second-lane-traffic/🦀️.rs"]
pub mod change_bridge_second_lane_traffic;
#[path = "../🌉️change-bridge-second-lane-weight/🦀️.rs"]
pub mod change_bridge_second_lane_weight;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_bridge_observed_more::{change_bridge_lambda_1, change_bridge_lambda_max, change_bridge_detail_id, change_bridge_plate_thickness};
