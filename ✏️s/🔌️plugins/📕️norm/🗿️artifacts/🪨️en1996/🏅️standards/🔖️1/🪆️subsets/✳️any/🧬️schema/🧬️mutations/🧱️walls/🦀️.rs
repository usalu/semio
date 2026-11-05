//! 🧱️ EN 1996 walls mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏷️change-wall-label-de/🦀️.rs"]
pub mod change_wall_label_de;
#[path = "../🏷️change-wall-label-en/🦀️.rs"]
pub mod change_wall_label_en;
#[path = "../🧱change-wall-type/🦀️.rs"]
pub mod change_wall_type;
#[path = "../➕️insert-wall/🦀️.rs"]
pub mod insert_wall;
#[path = "../➖️remove-wall/🦀️.rs"]
pub mod remove_wall;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1996_mut_walls_geometry::{change_exposure, change_support_sides, change_wall_height, change_wall_length, change_wall_thickness};
