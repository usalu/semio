//! 🧱️ EN 1995 panel-layers mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-panel-layer/🦀️.rs"]
pub mod insert_panel_layer;
#[path = "../➖️remove-panel-layer/🦀️.rs"]
pub mod remove_panel_layer;
#[path = "../🧱️change-panel-layer-thickness/🦀️.rs"]
pub mod change_panel_layer_thickness;
#[path = "../🧱️change-panel-layer-orientation/🦀️.rs"]
pub mod change_panel_layer_orientation;
#[path = "../🧱️change-panel-layer-grade/🦀️.rs"]
pub mod change_panel_layer_grade;
//#endregion 🪢️Mounts
