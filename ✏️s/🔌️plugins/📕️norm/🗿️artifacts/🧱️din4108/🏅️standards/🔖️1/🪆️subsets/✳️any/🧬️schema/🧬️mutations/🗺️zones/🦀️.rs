//! 🗺️ DIN 4108 zones mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-zone-floor-area/🦀️.rs"]
pub mod change_zone_floor_area;
#[path = "../🧱change-zone-heaviness/🦀️.rs"]
pub mod change_zone_heaviness;
#[path = "../🌙change-zone-night-ventilation/🦀️.rs"]
pub mod change_zone_night_ventilation;
#[path = "../➕️insert-zone/🦀️.rs"]
pub mod insert_zone;
#[path = "../➖️remove-zone/🦀️.rs"]
pub mod remove_zone;
//#endregion 🪢️Mounts
