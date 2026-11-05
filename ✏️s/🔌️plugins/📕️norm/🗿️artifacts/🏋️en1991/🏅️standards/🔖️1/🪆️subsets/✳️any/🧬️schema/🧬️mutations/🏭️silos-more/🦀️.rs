//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚖️change-silo-kind/🦀️.rs"]
pub mod change_silo_kind;
#[path = "../⭕change-silo-diameter/🦀️.rs"]
pub mod change_silo_diameter;
#[path = "../🎯change-silo-eccentricity/🦀️.rs"]
pub mod change_silo_eccentricity;
#[path = "../🔻change-silo-hopper-angle-deg/🦀️.rs"]
pub mod change_silo_hopper_angle_deg;
//#endregion 🪢️Mounts
