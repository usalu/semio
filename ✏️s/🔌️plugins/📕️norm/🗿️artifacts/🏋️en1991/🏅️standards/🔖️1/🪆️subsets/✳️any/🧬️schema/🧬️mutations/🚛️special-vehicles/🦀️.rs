//! 🚛 EN 1991 special vehicles mutation leaves: special vehicle, special vehicle designation, special vehicle x and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚛️change-special-vehicle-designation/🦀️.rs"]
pub mod change_special_vehicle_designation;
#[path = "../🚛️change-special-vehicle-x/🦀️.rs"]
pub mod change_special_vehicle_x;
#[path = "../🚛️change-special-vehicle-y/🦀️.rs"]
pub mod change_special_vehicle_y;
#[path = "../🚛️insert-special-vehicle/🦀️.rs"]
pub mod insert_special_vehicle;
#[path = "../🚛️remove-special-vehicle/🦀️.rs"]
pub mod remove_special_vehicle;
//#endregion 🪢️Mounts
