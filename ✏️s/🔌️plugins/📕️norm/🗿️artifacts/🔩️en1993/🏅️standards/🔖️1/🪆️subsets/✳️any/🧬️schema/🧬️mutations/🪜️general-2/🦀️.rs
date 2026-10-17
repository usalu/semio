//! 🪜️ EN 1993 general mutation leaves (part 2).

//#region 🪢️Mounts
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
#[path = "../➕️insert-material/🦀️.rs"]
pub mod insert_material;
#[path = "../➕️insert-section/🦀️.rs"]
pub mod insert_section;
#[path = "../➖️remove-material/🦀️.rs"]
pub mod remove_material;
#[path = "../➖️remove-section/🦀️.rs"]
pub mod remove_section;
#[path = "../✨️update-stainless-inputs/🦀️.rs"]
pub mod update_stainless_inputs;
#[path = "../↕️update-through-thickness-inputs/🦀️.rs"]
pub mod update_through_thickness_inputs;
//#endregion 🪢️Mounts
