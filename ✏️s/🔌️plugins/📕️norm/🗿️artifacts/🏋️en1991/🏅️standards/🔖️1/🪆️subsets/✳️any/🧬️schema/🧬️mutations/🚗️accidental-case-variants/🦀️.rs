//! 🚗 EN 1991 accidental case variants mutation leaves: accidental assumed pressure, accidental explosion mass, accidental standoff and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚗️change-accidental-vehicle-mass/🦀️.rs"]
pub mod change_accidental_vehicle_mass;
#[path = "../🚗️change-accidental-vehicle-speed/🦀️.rs"]
pub mod change_accidental_vehicle_speed;
#[path = "../🚗️change-accidental-explosion-mass/🦀️.rs"]
pub mod change_accidental_explosion_mass;
#[path = "../🚗️change-accidental-standoff/🦀️.rs"]
pub mod change_accidental_standoff;
#[path = "../🚗️change-accidental-assumed-pressure/🦀️.rs"]
pub mod change_accidental_assumed_pressure;
//#endregion 🪢️Mounts
