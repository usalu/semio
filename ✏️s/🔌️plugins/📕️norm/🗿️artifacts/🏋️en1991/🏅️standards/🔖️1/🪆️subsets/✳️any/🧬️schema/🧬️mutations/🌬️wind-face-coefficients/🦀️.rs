//! 🌬 EN 1991 wind face coefficients mutation leaves: wind face c d, wind face c pe1, wind face c pe10 and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌬️change-wind-face-zone/🦀️.rs"]
pub mod change_wind_face_zone;
#[path = "../🌬️change-wind-face-c-pe10/🦀️.rs"]
pub mod change_wind_face_c_pe10;
#[path = "../🌬️change-wind-face-c-pe1/🦀️.rs"]
pub mod change_wind_face_c_pe1;
#[path = "../🌬️change-wind-face-c-pi/🦀️.rs"]
pub mod change_wind_face_c_pi;
#[path = "../🌬️change-wind-face-cs/🦀️.rs"]
pub mod change_wind_face_c_s;
#[path = "../🌬️change-wind-face-cd/🦀️.rs"]
pub mod change_wind_face_c_d;
//#endregion 🪢️Mounts
