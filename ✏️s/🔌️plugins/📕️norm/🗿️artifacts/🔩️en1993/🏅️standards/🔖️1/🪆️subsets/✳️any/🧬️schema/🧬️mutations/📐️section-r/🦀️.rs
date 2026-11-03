//! 📐️ EN 1993 section mutation leaves: r, area, shear area y and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-section-r/🦀️.rs"]
pub mod change_section_r;
#[path = "../📐️change-section-area/🦀️.rs"]
pub mod change_section_area;
#[path = "../📐️change-section-shear-area-y/🦀️.rs"]
pub mod change_section_shear_area_y;
#[path = "../📐️change-section-shear-area-z/🦀️.rs"]
pub mod change_section_shear_area_z;
#[path = "../📐️change-section-iy/🦀️.rs"]
pub mod change_section_iy;
#[path = "../📐️change-section-iz/🦀️.rs"]
pub mod change_section_iz;
#[path = "../📐️change-section-it/🦀️.rs"]
pub mod change_section_it;
//#endregion 🪢️Mounts
