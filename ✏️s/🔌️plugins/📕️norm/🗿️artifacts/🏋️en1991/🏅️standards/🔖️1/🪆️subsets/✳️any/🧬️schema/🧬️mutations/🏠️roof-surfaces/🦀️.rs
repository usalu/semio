//! 🏠 EN 1991 roof surfaces mutation leaves: roof surface extent, roof surface face, roof surface pitch deg and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏠️change-roof-surface-face/🦀️.rs"]
pub mod change_roof_surface_face;
#[path = "../🏠️change-roof-surface-shape/🦀️.rs"]
pub mod change_roof_surface_shape;
#[path = "../🏠️change-roof-surface-pitch-deg/🦀️.rs"]
pub mod change_roof_surface_pitch_deg;
#[path = "../🏠️change-roof-surface-extent/🦀️.rs"]
pub mod change_roof_surface_extent;
#[path = "../🏠️insert-roof-surface/🦀️.rs"]
pub mod insert_roof_surface;
#[path = "../🏠️remove-roof-surface/🦀️.rs"]
pub mod remove_roof_surface;
//#endregion 🪢️Mounts
