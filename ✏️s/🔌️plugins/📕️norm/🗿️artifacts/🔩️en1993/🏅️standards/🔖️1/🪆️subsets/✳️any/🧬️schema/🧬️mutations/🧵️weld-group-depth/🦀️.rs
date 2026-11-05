//! 🧵️ EN 1993 weld group mutation leaves: depth, shear force, eccentricity and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧵️change-weld-depth/🦀️.rs"]
pub mod change_weld_depth;
#[path = "../🧵️change-weld-shear-force/🦀️.rs"]
pub mod change_weld_shear_force;
#[path = "../🧵️change-weld-eccentricity/🦀️.rs"]
pub mod change_weld_eccentricity;
#[path = "../🧵️change-weld-steel-fu/🦀️.rs"]
pub mod change_weld_steel_fu;
#[path = "../🧵️change-weld-beta-w/🦀️.rs"]
pub mod change_weld_beta_w;
//#endregion 🪢️Mounts
