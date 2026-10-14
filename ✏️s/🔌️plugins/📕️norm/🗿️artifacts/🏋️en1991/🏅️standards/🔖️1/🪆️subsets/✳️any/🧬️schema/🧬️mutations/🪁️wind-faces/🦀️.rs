//! 🪁️ EN 1991 wind faces mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛡️change-wind-face-assumed-wp/🦀️.rs"]
pub mod change_wind_face_assumed_wp;
#[path = "../➕️insert-wind-faces/🦀️.rs"]
pub mod insert_wind_faces;
#[path = "../➖️remove-wind-faces/🦀️.rs"]
pub mod remove_wind_faces;
//#endregion 🪢️Mounts
