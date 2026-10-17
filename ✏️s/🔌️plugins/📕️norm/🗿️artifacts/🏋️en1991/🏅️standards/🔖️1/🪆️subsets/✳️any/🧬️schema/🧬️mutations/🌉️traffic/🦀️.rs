//! 🌉️ EN 1991 traffic mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚶change-assumed-bridge-footway/🦀️.rs"]
pub mod change_assumed_bridge_footway;
#[path = "../🚛change-assumed-bridge-lm3/🦀️.rs"]
pub mod change_assumed_bridge_lm3;
#[path = "../🌾change-assumed-bridge-tandem/🦀️.rs"]
pub mod change_assumed_bridge_tandem;
#[path = "../🌉change-bridge-lane/🦀️.rs"]
pub mod change_bridge_lane;
#[path = "../📦change-bridge-load-group/🦀️.rs"]
pub mod change_bridge_load_group;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_traffic_2::{change_assumed_bridge_lm2, change_assumed_bridge_lm4, change_assumed_bridge_udl, change_bridge_lane_width, change_bridge_span};
