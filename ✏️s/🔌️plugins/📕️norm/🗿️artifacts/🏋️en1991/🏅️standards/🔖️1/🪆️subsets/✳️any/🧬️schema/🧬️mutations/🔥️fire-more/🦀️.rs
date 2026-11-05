//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⏱️change-fire-duration/🦀️.rs"]
pub mod change_fire_duration;
#[path = "../⛽change-fire-load-density-qf/🦀️.rs"]
pub mod change_fire_load_density_qf;
#[path = "../🔥change-fire-mode/🦀️.rs"]
pub mod change_fire_mode;
#[path = "../🏢change-fire-occupancy/🦀️.rs"]
pub mod change_fire_occupancy;
#[path = "../🧯change-fire-load-basis/🦀️.rs"]
pub mod change_fire_load_basis;
//#endregion 🪢️Mounts
