//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️change-silo-course-elevation/🦀️.rs"]
pub mod change_silo_course_elevation;
#[path = "../🛢️change-silo-internal-pressure/🦀️.rs"]
pub mod change_silo_internal_pressure;
#[path = "../🛢️change-silo-external-pressure/🦀️.rs"]
pub mod change_silo_external_pressure;
#[path = "../🛢️change-silo-temperature-change/🦀️.rs"]
pub mod change_silo_temperature_change;
//#endregion 🪢️Mounts
