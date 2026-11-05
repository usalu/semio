//! 📐️ EN 1999 section mutation leaves: kind, height, width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-section-kind/🦀️.rs"]
pub mod change_section_kind;
#[path = "../📐️change-section-height/🦀️.rs"]
pub mod change_section_height;
#[path = "../📐️change-section-width/🦀️.rs"]
pub mod change_section_width;
#[path = "../📐️change-section-flange-thickness/🦀️.rs"]
pub mod change_section_flange_thickness;
#[path = "../📐️change-section-web-thickness/🦀️.rs"]
pub mod change_section_web_thickness;
#[path = "../📐️change-section-outer-diameter/🦀️.rs"]
pub mod change_section_outer_diameter;
//#endregion 🪢️Mounts
