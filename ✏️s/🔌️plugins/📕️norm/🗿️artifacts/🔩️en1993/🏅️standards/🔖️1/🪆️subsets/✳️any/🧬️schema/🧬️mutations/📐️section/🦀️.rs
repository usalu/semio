//! 📐️ EN 1993 section mutation leaves: designation, kind, h and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-section-designation/🦀️.rs"]
pub mod change_section_designation;
#[path = "../📐️change-section-kind/🦀️.rs"]
pub mod change_section_kind;
#[path = "../📐️change-section-fabrication/🦀️.rs"]
pub mod change_section_fabrication;
#[path = "../📐️change-section-h/🦀️.rs"]
pub mod change_section_h;
#[path = "../📐️change-section-b/🦀️.rs"]
pub mod change_section_b;
#[path = "../📐️change-section-tw/🦀️.rs"]
pub mod change_section_tw;
#[path = "../📐️change-section-tf/🦀️.rs"]
pub mod change_section_tf;
//#endregion 🪢️Mounts
