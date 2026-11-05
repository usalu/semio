//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-unit-height/🦀️.rs"]
pub mod change_unit_height;
#[path = "../🧱change-unit-length/🦀️.rs"]
pub mod change_unit_length;
#[path = "../🧱change-unit-material/🦀️.rs"]
pub mod change_unit_material;
#[path = "../🧱change-unit-width/🦀️.rs"]
pub mod change_unit_width;
//#endregion 🪢️Mounts
