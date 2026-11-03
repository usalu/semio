//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-flange-thickness/🦀️.rs"]
pub mod change_plated_flange_thickness;
#[path = "../🧱️change-plated-flange-width/🦀️.rs"]
pub mod change_plated_flange_width;
#[path = "../🧱️change-plated-flange-fy/🦀️.rs"]
pub mod change_plated_flange_fy;
#[path = "../🧱️change-plated-longitudinal-offset/🦀️.rs"]
pub mod change_plated_longitudinal_offset;
#[path = "../🧱️change-plated-longitudinal-inertia/🦀️.rs"]
pub mod change_plated_longitudinal_inertia;
//#endregion 🪢️Mounts
