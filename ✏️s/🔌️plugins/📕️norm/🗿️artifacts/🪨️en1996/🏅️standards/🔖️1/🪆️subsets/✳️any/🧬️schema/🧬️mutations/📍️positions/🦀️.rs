//! 📍️ EN 1996 opening and load positions and the distance a1 mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪟change-opening-position/🦀️.rs"]
pub mod change_opening_position;
#[path = "../📍️change-concentrated-position/🦀️.rs"]
pub mod change_concentrated_position;
#[path = "../📏change-concentrated-distance-a1/🦀️.rs"]
pub mod change_concentrated_distance_a1;
//#endregion 🪢️Mounts
