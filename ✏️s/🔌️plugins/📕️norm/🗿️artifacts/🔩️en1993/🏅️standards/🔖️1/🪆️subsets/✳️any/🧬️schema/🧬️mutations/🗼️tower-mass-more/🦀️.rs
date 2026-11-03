//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗼️change-tower-mean-wind-speed/🦀️.rs"]
pub mod change_tower_mean_wind_speed;
#[path = "../🗼️change-tower-crossing-rate/🦀️.rs"]
pub mod change_tower_crossing_rate;
#[path = "../🗼️change-tower-design-life/🦀️.rs"]
pub mod change_tower_design_life;
#[path = "../🗼️change-tower-k-factor/🦀️.rs"]
pub mod change_tower_k_factor;
//#endregion 🪢️Mounts
