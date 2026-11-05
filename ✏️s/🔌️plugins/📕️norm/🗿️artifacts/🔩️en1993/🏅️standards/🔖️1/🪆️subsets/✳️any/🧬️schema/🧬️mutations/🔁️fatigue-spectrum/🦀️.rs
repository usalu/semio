//! 🔁️ EN 1993 fatigue band element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-fatigue-band/🦀️.rs"]
pub mod insert_fatigue_band;
#[path = "../➖️remove-fatigue-band/🦀️.rs"]
pub mod remove_fatigue_band;
#[path = "../🔁️change-fatigue-band-delta-sigma/🦀️.rs"]
pub mod change_fatigue_band_delta_sigma;
#[path = "../🔁️change-fatigue-band-cycles/🦀️.rs"]
pub mod change_fatigue_band_cycles;
//#endregion 🪢️Mounts
