//! ⚗️ EN 1993 material mutation leaves: grade, fy, fu and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚗️change-material-grade/🦀️.rs"]
pub mod change_material_grade;
#[path = "../⚗️change-material-fy/🦀️.rs"]
pub mod change_material_fy;
#[path = "../⚗️change-material-fu/🦀️.rs"]
pub mod change_material_fu;
#[path = "../⚗️change-material-e-modulus/🦀️.rs"]
pub mod change_material_e_modulus;
#[path = "../⚗️change-material-g-modulus/🦀️.rs"]
pub mod change_material_g_modulus;
#[path = "../⚗️change-material-subgrade/🦀️.rs"]
pub mod change_material_subgrade;
#[path = "../⚗️change-material-kind/🦀️.rs"]
pub mod change_material_kind;
//#endregion 🪢️Mounts
