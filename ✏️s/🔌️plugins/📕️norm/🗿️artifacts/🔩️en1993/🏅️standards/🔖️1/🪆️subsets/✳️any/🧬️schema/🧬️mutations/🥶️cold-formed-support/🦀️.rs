//! 🥶️ EN 1993 cold-formed member mutation leaves: support reaction, support length, crippling category and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥶️change-cf-support-reaction/🦀️.rs"]
pub mod change_cf_support_reaction;
#[path = "../🥶️change-cf-support-length/🦀️.rs"]
pub mod change_cf_support_length;
#[path = "../🥶️change-cf-crippling-category/🦀️.rs"]
pub mod change_cf_crippling_category;
#[path = "../🥶️change-cf-stiffened-support/🦀️.rs"]
pub mod change_cf_stiffened_support;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_cold_formed_support_more::{change_cf_bending_moment, change_cf_moment_resistance, change_cf_sheeting_stiffness, change_cf_required_restraint_stiff};
