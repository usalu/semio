//! 🌡️ DIN EN 16798 thermal mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💨change-zone-air-speed/🦀️.rs"]
pub mod change_zone_air_speed;
#[path = "../👔change-zone-clothing/🦀️.rs"]
pub mod change_zone_clothing;
#[path = "../🧭️change-zone-comfort-model/🦀️.rs"]
pub mod change_zone_comfort_model;
#[path = "../🏃️change-zone-metabolic-rate/🦀️.rs"]
pub mod change_zone_metabolic_rate;
#[path = "../💧️change-zone-rh/🦀️.rs"]
pub mod change_zone_rh;
#[path = "../☀️change-zone-t-op-summer/🦀️.rs"]
pub mod change_zone_t_op_summer;
#[path = "../❄️change-zone-t-op-winter/🦀️.rs"]
pub mod change_zone_t_op_winter;
#[path = "../💨change-zone-turbulence/🦀️.rs"]
pub mod change_zone_turbulence;
//#endregion 🪢️Mounts
