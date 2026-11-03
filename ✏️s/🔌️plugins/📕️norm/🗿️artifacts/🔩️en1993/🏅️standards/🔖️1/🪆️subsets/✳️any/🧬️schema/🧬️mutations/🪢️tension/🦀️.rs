//! 🪢️ EN 1993 tension component mutation leaves: f uk, f k, partial factor and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪢️change-tension-f-uk/🦀️.rs"]
pub mod change_tension_f_uk;
#[path = "../🪢️change-tension-fk/🦀️.rs"]
pub mod change_tension_f_k;
#[path = "../🪢️change-tension-partial-factor/🦀️.rs"]
pub mod change_tension_partial_factor;
#[path = "../🪢️change-tension-cable-length/🦀️.rs"]
pub mod change_tension_cable_length;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_tension_more::{change_tension_cable_weight, change_tension_cable_horiz_force, change_tension_cable_modulus, change_tension_cable_stress, change_tension_cable_unit_weight};
