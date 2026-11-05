//! 🧱️ EN 1993 plated panel mutation leaves: flange induced k, flange area, stress x and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-flange-induced-k/🦀️.rs"]
pub mod change_plated_flange_induced_k;
#[path = "../🧱️change-plated-flange-area/🦀️.rs"]
pub mod change_plated_flange_area;
#[path = "../🧱️change-plated-stress-x/🦀️.rs"]
pub mod change_plated_stress_x;
#[path = "../🧱️change-plated-stress-z/🦀️.rs"]
pub mod change_plated_stress_z;
#[path = "../🧱️change-plated-shear-stress/🦀️.rs"]
pub mod change_plated_shear_stress;
#[path = "../🧱️change-plated-alpha-cr/🦀️.rs"]
pub mod change_plated_alpha_cr;
//#endregion 🪢️Mounts
