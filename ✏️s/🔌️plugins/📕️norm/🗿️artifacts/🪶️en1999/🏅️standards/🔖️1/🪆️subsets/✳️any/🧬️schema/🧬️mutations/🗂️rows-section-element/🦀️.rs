//! 🗂️ EN 1999 section element mutation leaves: insert section element, remove section element.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-section-element/🦀️.rs"]
pub mod insert_section_element;
#[path = "../➖remove-section-element/🦀️.rs"]
pub mod remove_section_element;
//#endregion 🪢️Mounts
