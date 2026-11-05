//! 🧱️ EN 1993 plated panel mutation leaves: patch type, patch force, patch length and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-patch-type/🦀️.rs"]
pub mod change_plated_patch_type;
#[path = "../🧱️change-plated-patch-force/🦀️.rs"]
pub mod change_plated_patch_force;
#[path = "../🧱️change-plated-patch-length/🦀️.rs"]
pub mod change_plated_patch_length;
#[path = "../🧱️change-plated-patch-offset/🦀️.rs"]
pub mod change_plated_patch_offset;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_plated_patch_more::{change_plated_flange_thickness, change_plated_flange_width, change_plated_flange_fy, change_plated_longitudinal_offset, change_plated_longitudinal_inertia};
