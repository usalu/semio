//! 🌉 EN 1990 bridge sls rail mutation leaves: bridge damping ratio.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-damping-ratio/🦀️.rs"]
pub mod change_bridge_damping_ratio;
#[path = "../🌉️change-bridge-track-gauge/🦀️.rs"]
pub mod change_bridge_track_gauge;
#[path = "../🌉️change-bridge-line-speed/🦀️.rs"]
pub mod change_bridge_line_speed;
#[path = "../🌉️change-bridge-ballast-depth/🦀️.rs"]
pub mod change_bridge_ballast_depth;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1990_mut_bridge_sls_rail_more::{change_bridge_deck_mass, change_bridge_natural_frequency, change_bridge_end_rotation, change_bridge_basic_wind_speed};
