//! 📐️ EN 1995 tapers mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-taper-member/🦀️.rs"]
pub mod change_taper_member;
#[path = "../📐️change-taper-kind/🦀️.rs"]
pub mod change_taper_kind;
#[path = "../📐️change-taper-taper-angle/🦀️.rs"]
pub mod change_taper_taper_angle;
#[path = "../📐️change-taper-apex-depth/🦀️.rs"]
pub mod change_taper_apex_depth;
#[path = "../📐️change-taper-inner-radius/🦀️.rs"]
pub mod change_taper_inner_radius;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_tapers_more::{change_taper_lamella, change_taper_apex_angle, change_taper_side, change_taper_rods, change_taper_rod_resistance};
