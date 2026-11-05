//! ⛰️ EN 1997 slope mutation leaves: height, length, governing layer id and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⛰️change-slope-height/🦀️.rs"]
pub mod change_slope_height;
#[path = "../⛰️change-slope-length/🦀️.rs"]
pub mod change_slope_length;
#[path = "../⛰️change-slope-governing-layer-id/🦀️.rs"]
pub mod change_slope_governing_layer_id;
#[path = "../⛰️change-slope-circle-centre-x/🦀️.rs"]
pub mod change_slope_circle_centre_x;
#[path = "../⛰️change-slope-circle-centre-z/🦀️.rs"]
pub mod change_slope_circle_centre_z;
#[path = "../⛰️change-slope-circle-radius/🦀️.rs"]
pub mod change_slope_circle_radius;
#[path = "../⛰️change-slope-width/🦀️.rs"]
pub mod change_slope_width;
//#endregion 🪢️Mounts
