//! 🟫️ EN 1996 units mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-density/🦀️.rs"]
pub mod change_density;
#[path = "../🏭️change-masonry-class/🦀️.rs"]
pub mod change_masonry_class;
#[path = "../🧱change-unit-fb/🦀️.rs"]
pub mod change_unit_fb;
#[path = "../🧱change-unit-group/🦀️.rs"]
pub mod change_unit_group;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1996_mut_units_more::{change_unit_height, change_unit_length, change_unit_material, change_unit_width};
