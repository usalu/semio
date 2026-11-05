//! 🏠️ EN 1991 snow roof mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-roof/🦀️.rs"]
pub mod insert_roof;
#[path = "../➖️remove-roof/🦀️.rs"]
pub mod remove_roof;
#[path = "../📐change-roof-pitch-deg/🦀️.rs"]
pub mod change_roof_pitch_deg;
#[path = "../📏change-roof-parapet-height/🦀️.rs"]
pub mod change_roof_parapet_height;
#[path = "../🧱change-roof-drift-obstruction-height/🦀️.rs"]
pub mod change_roof_drift_obstruction_height;
//#endregion 🪢️Mounts
