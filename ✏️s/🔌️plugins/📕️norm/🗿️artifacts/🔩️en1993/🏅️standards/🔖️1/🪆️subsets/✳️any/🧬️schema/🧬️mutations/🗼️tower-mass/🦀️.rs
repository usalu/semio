//! 🗼️ EN 1993 tower leg mutation leaves: mass per length, strouhal, log decrement and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗼️change-tower-mass-per-length/🦀️.rs"]
pub mod change_tower_mass_per_length;
#[path = "../🗼️change-tower-strouhal/🦀️.rs"]
pub mod change_tower_strouhal;
#[path = "../🗼️change-tower-log-decrement/🦀️.rs"]
pub mod change_tower_log_decrement;
#[path = "../🗼️change-tower-air-density/🦀️.rs"]
pub mod change_tower_air_density;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_tower_mass_more::{change_tower_mean_wind_speed, change_tower_crossing_rate, change_tower_design_life, change_tower_k_factor};
