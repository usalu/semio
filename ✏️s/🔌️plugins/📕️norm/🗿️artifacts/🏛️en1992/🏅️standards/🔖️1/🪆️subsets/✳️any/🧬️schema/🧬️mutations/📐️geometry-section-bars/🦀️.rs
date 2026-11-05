//! 📐️ EN 1992 geometry section bars mutation leaves: bar point diameter, bar point layer id, bar point y and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-section-point-y/🦀️.rs"]
pub mod change_section_point_y;
#[path = "../📐️change-section-point-z/🦀️.rs"]
pub mod change_section_point_z;
#[path = "../📍️change-bar-point-layer-id/🦀️.rs"]
pub mod change_bar_point_layer_id;
#[path = "../📍️change-bar-point-y/🦀️.rs"]
pub mod change_bar_point_y;
#[path = "../📍️change-bar-point-z/🦀️.rs"]
pub mod change_bar_point_z;
#[path = "../📍️change-bar-point-diameter/🦀️.rs"]
pub mod change_bar_point_diameter;
//#endregion 🪢️Mounts
