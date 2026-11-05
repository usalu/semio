//! 🌉️ EN 1993 bridge fatigue mutation leaves: deck thickness, trough spacing, trough width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-deck-thickness/🦀️.rs"]
pub mod change_bridge_deck_thickness;
#[path = "../🌉️change-bridge-trough-spacing/🦀️.rs"]
pub mod change_bridge_trough_spacing;
#[path = "../🌉️change-bridge-trough-width/🦀️.rs"]
pub mod change_bridge_trough_width;
#[path = "../🌉️change-bridge-trough-thickness/🦀️.rs"]
pub mod change_bridge_trough_thickness;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_bridge_deck_more::{change_bridge_crossbeam_spacing, change_bridge_surfacing_thickness, change_bridge_wheel_load, change_bridge_wheel_patch};
