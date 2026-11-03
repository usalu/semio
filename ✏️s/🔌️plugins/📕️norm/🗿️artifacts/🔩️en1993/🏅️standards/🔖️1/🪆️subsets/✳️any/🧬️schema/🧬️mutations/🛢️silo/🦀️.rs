//! 🛢️ EN 1993 silo shell mutation leaves: thickness, radius, depth and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️change-silo-thickness/🦀️.rs"]
pub mod change_silo_thickness;
#[path = "../🛢️change-silo-radius/🦀️.rs"]
pub mod change_silo_radius;
#[path = "../🛢️change-silo-depth/🦀️.rs"]
pub mod change_silo_depth;
#[path = "../🛢️change-silo-k/🦀️.rs"]
pub mod change_silo_k;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_silo_more::{change_silo_gamma, change_silo_fy, change_silo_length, change_silo_fabrication_class};
