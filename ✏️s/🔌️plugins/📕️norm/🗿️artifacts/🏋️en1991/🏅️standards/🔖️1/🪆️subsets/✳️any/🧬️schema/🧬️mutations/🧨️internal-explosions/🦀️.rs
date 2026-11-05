//! 🧨 EN 1991 internal explosions mutation leaves: explosion assumed pressure, explosion vent area, explosion vent pressure and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧨️change-internal-explosion-volume/🦀️.rs"]
pub mod change_internal_explosion_volume;
#[path = "../🧨️change-explosion-vent-area/🦀️.rs"]
pub mod change_explosion_vent_area;
#[path = "../🧨️change-explosion-vent-pressure/🦀️.rs"]
pub mod change_explosion_vent_pressure;
#[path = "../🧨️change-explosion-assumed-pressure/🦀️.rs"]
pub mod change_explosion_assumed_pressure;
#[path = "../🧨️insert-internal-explosion/🦀️.rs"]
pub mod insert_internal_explosion;
#[path = "../🧨️remove-internal-explosion/🦀️.rs"]
pub mod remove_internal_explosion;
//#endregion 🪢️Mounts
