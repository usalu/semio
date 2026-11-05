//! ❄️ EN 1991 snow mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
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
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_snow_roofs::{insert_roof, remove_roof, change_roof_pitch_deg, change_roof_parapet_height, change_roof_drift_obstruction_height};
