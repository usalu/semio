//! 🔆️ DIN EN 16798 indoor mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🫧change-zone-co2/🦀️.rs"]
pub mod change_zone_co2;
#[path = "../💡change-zone-illuminance/🦀️.rs"]
pub mod change_zone_illuminance;
#[path = "../🔊️change-zone-noise/🦀️.rs"]
pub mod change_zone_noise;
#[path = "../💨️change-zone-outdoor-air/🦀️.rs"]
pub mod change_zone_outdoor_air;
//#endregion 🪢️Mounts
