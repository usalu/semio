//! 🛢️ EN 1993 silo shell mutation leaves: plan area, plan perimeter, liquid density and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️change-silo-plan-area/🦀️.rs"]
pub mod change_silo_plan_area;
#[path = "../🛢️change-silo-plan-perimeter/🦀️.rs"]
pub mod change_silo_plan_perimeter;
#[path = "../🛢️change-silo-liquid-density/🦀️.rs"]
pub mod change_silo_liquid_density;
#[path = "../🛢️change-silo-liquid-height/🦀️.rs"]
pub mod change_silo_liquid_height;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_silo_plan_more::{change_silo_course_elevation, change_silo_internal_pressure, change_silo_external_pressure, change_silo_temperature_change};
