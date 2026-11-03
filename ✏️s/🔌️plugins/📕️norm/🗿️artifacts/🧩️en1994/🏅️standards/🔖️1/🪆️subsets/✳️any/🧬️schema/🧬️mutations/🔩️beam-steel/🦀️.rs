//! 🔩️ EN 1994 beam steel section mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬛️change-beam-steel-area-m2/🦀️.rs"]
pub mod change_beam_steel_area_m2;
#[path = "../🔩️change-beam-steel-designation/🦀️.rs"]
pub mod change_beam_steel_designation;
#[path = "../🧱️change-beam-steel-tf-m/🦀️.rs"]
pub mod change_beam_steel_tf_m;
#[path = "../↕️change-beam-steel-height-m/🦀️.rs"]
pub mod change_beam_steel_height_m;
#[path = "../📐️change-beam-steel-w-pl-ym3/🦀️.rs"]
pub mod change_beam_steel_w_pl_y_m3;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1994_mut_beam_steel_more::{change_beam_steel_second_moment_m4, change_beam_steel_shear_area_m2, change_beam_steel_root_radius_m, change_beam_steel_web_thickness_m, change_beam_steel_width_m};
