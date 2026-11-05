//! 🗼️ EN 1998 bridges bearings mutation leaves: bridge id, pier id, device.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-bearing-bridge-id/🦀️.rs"]
pub mod change_bearing_bridge_id;
#[path = "../✏️change-bearing-pier-id/🦀️.rs"]
pub mod change_bearing_pier_id;
#[path = "../✏️change-bearing-device/🦀️.rs"]
pub mod change_bearing_device;
#[path = "../✏️change-bearing-xm/🦀️.rs"]
pub mod change_bearing_x_m;
#[path = "../✏️change-bearing-stiffness-n-per-m/🦀️.rs"]
pub mod change_bearing_stiffness_n_per_m;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_bridges_bearings_more::{change_bearing_d_rd_m, change_bearing_seat_length_m, change_bearing_min_support_m, insert_bearing, remove_bearing};
