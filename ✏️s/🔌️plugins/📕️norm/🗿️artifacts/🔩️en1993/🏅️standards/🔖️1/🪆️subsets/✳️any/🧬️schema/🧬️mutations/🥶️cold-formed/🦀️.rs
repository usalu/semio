//! 🥶️ EN 1993 cold-formed member mutation leaves: b bar, thickness, k sigma and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥶️change-cf-b-bar/🦀️.rs"]
pub mod change_cf_b_bar;
#[path = "../🥶️change-cf-thickness/🦀️.rs"]
pub mod change_cf_thickness;
#[path = "../🥶️change-cf-k-sigma/🦀️.rs"]
pub mod change_cf_k_sigma;
#[path = "../🥶️change-cf-psi/🦀️.rs"]
pub mod change_cf_psi;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_cold_formed_more::{change_cf_fy, change_cf_gross_resistance, change_cf_flange_width, change_cf_lip_length};
