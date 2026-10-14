//! 🪚️ EN 1995 plates mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../↔️change-connection-plate-thickness/🦀️.rs"]
pub mod change_connection_plate_thickness;
#[path = "../🔢️change-connection-shear-planes/🦀️.rs"]
pub mod change_connection_shear_planes;
#[path = "../🔩️change-connection-steel-plate/🦀️.rs"]
pub mod change_connection_steel_plate;
#[path = "../↔️change-connection-t1/🦀️.rs"]
pub mod change_connection_t1;
#[path = "../↔️change-connection-t2/🦀️.rs"]
pub mod change_connection_t2;
//#endregion 🪢️Mounts
