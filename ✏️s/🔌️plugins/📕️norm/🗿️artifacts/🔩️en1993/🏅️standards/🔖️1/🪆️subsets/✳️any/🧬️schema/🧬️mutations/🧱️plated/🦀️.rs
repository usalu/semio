//! 🧱️ EN 1993 plated panel mutation leaves: a, b, thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-a/🦀️.rs"]
pub mod change_plated_a;
#[path = "../🧱️change-plated-b/🦀️.rs"]
pub mod change_plated_b;
#[path = "../🧱️change-plated-thickness/🦀️.rs"]
pub mod change_plated_thickness;
#[path = "../🧱️change-plated-fy/🦀️.rs"]
pub mod change_plated_fy;
#[path = "../🧱️change-plated-k-sigma/🦀️.rs"]
pub mod change_plated_k_sigma;
//#endregion 🪢️Mounts
