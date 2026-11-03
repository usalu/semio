//! ⚓️ EN 1990 actions fields mutation leaves: accidental ad, permanent gk, permanent kind and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚓️change-permanent-kind/🦀️.rs"]
pub mod change_permanent_kind;
#[path = "../⚓️change-permanent-gk/🦀️.rs"]
pub mod change_permanent_gk;
#[path = "../🏋️change-variable-category/🦀️.rs"]
pub mod change_variable_category;
#[path = "../🏋️change-variable-qk/🦀️.rs"]
pub mod change_variable_qk;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1990_mut_actions_fields_more::{change_variable_group, change_accidental_ad, change_seismic_a_ek, change_seismic_importance_class};
