//! 🧬 EN 1992 geometry tendons studs mutation leaves: stud ring count, stud ring diameter, stud ring radius and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧬️change-tendon-point-x/🦀️.rs"]
pub mod change_tendon_point_x;
#[path = "../🧬️change-tendon-point-y/🦀️.rs"]
pub mod change_tendon_point_y;
#[path = "../🧬️change-tendon-point-z/🦀️.rs"]
pub mod change_tendon_point_z;
#[path = "../🔩️change-stud-row-angle/🦀️.rs"]
pub mod change_stud_row_angle;
#[path = "../🔩️change-stud-row-count/🦀️.rs"]
pub mod change_stud_row_count;
#[path = "../🔩️change-stud-row-spacing-first/🦀️.rs"]
pub mod change_stud_row_spacing_first;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs_more::{change_stud_row_spacing_radial, change_stud_row_diameter, change_opening_x, change_opening_y, change_opening_width, change_opening_depth};
