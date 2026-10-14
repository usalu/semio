//! 🌬️ EN 1991 wind mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢change-air-density/🦀️.rs"]
pub mod change_air_density;
#[path = "../🏝️change-coast-or-island/🦀️.rs"]
pub mod change_coast_or_island;
#[path = "../🌬️change-en-vb/🦀️.rs"]
pub mod change_en_vb;
#[path = "../📏change-mixed-terrain-distance/🦀️.rs"]
pub mod change_mixed_terrain_distance;
#[path = "../🧭change-mixed-terrain-upwind/🦀️.rs"]
pub mod change_mixed_terrain_upwind;
#[path = "../📐change-orography-factor/🦀️.rs"]
pub mod change_orography_factor;
#[path = "../🏞️change-terrain-category/🦀️.rs"]
pub mod change_terrain_category;
#[path = "../🪁change-wind-zone/🦀️.rs"]
pub mod change_wind_zone;
//#endregion 🪢️Mounts
