//! 🪜️ EN 1993 built-up member mutation leaves: batten inertia, diagonal area, diagonal length and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪜️change-built-up-batten-inertia/🦀️.rs"]
pub mod change_built_up_batten_inertia;
#[path = "../🪜️change-built-up-diagonal-area/🦀️.rs"]
pub mod change_built_up_diagonal_area;
#[path = "../🪜️change-built-up-diagonal-length/🦀️.rs"]
pub mod change_built_up_diagonal_length;
#[path = "../🪜️change-built-up-design-axial/🦀️.rs"]
pub mod change_built_up_design_axial;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_built_up_batten_more::{change_built_up_design_moment, change_built_up_back_to_back, change_built_up_min_radius, change_built_up_chain_spacing};
