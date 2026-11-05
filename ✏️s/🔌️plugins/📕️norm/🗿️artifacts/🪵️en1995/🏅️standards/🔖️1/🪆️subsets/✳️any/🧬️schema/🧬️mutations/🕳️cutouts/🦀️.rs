//! 🕳️ EN 1995 cutouts mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-cutout-member/🦀️.rs"]
pub mod change_cutout_member;
#[path = "../🕳️change-cutout-kind/🦀️.rs"]
pub mod change_cutout_kind;
#[path = "../🕳️change-cutout-face/🦀️.rs"]
pub mod change_cutout_face;
#[path = "../🕳️change-cutout-distance/🦀️.rs"]
pub mod change_cutout_distance;
#[path = "../🕳️change-cutout-length/🦀️.rs"]
pub mod change_cutout_length;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_cutouts_more::{change_cutout_depth, change_cutout_centre_height, change_cutout_slope, change_cutout_rods, change_cutout_rod_resistance};
