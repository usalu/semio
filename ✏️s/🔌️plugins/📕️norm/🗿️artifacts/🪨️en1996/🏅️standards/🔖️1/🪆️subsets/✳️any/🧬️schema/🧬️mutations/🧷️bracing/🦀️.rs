//! 🧷 EN 1996 bracing mutation leaves: bracing system, diaphragm stiffness.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧷️change-bracing-system/🦀️.rs"]
pub mod change_bracing_system;
#[path = "../🧷️change-diaphragm-stiffness/🦀️.rs"]
pub mod change_diaphragm_stiffness;
#[path = "../🧷️change-wall-web-spacing-m/🦀️.rs"]
pub mod change_wall_web_spacing_m;
//#endregion 🪢️Mounts
