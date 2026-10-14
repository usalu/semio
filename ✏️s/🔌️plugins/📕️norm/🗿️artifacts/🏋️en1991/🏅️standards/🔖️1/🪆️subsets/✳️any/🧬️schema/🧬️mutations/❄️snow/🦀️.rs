//! ❄️ EN 1991 snow mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❄️change-altitude/🦀️.rs"]
pub mod change_altitude;
#[path = "../❄️change-en-sk/🦀️.rs"]
pub mod change_en_sk;
#[path = "../🏔️change-north-german-lowland-snow/🦀️.rs"]
pub mod change_north_german_lowland_snow;
#[path = "../🌨️change-roof-assumed-sk/🦀️.rs"]
pub mod change_roof_assumed_sk;
#[path = "../🗺️change-snow-zone/🦀️.rs"]
pub mod change_snow_zone;
#[path = "../➕️insert-roofs/🦀️.rs"]
pub mod insert_roofs;
#[path = "../➖️remove-roofs/🦀️.rs"]
pub mod remove_roofs;
//#endregion 🪢️Mounts
