//! 📐️ VDI 3805 geometry mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📈️add-curve/🦀️.rs"]
pub mod add_curve;
#[path = "../🧊️add-geometry/🦀️.rs"]
pub mod add_geometry;
#[path = "../🔌️add-geometry-connection/🦀️.rs"]
pub mod add_geometry_connection;
#[path = "../📍️change-curve-points/🦀️.rs"]
pub mod change_curve_points;
#[path = "../🧮️change-geometry-parameters/🦀️.rs"]
pub mod change_geometry_parameters;
#[path = "../📉️remove-curve/🦀️.rs"]
pub mod remove_curve;
#[path = "../🚮️remove-geometry/🦀️.rs"]
pub mod remove_geometry;
#[path = "../✂️remove-geometry-connection/🦀️.rs"]
pub mod remove_geometry_connection;
#[path = "../📐️resize-geometry/🦀️.rs"]
pub mod resize_geometry;
//#endregion 🪢️Mounts
