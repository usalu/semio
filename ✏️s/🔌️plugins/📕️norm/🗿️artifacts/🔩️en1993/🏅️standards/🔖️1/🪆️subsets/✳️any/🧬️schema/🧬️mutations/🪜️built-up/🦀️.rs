//! 🪜️ EN 1993 built-up member mutation leaves: member id, label, kind and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪜️change-built-up-member-id/🦀️.rs"]
pub mod change_built_up_member_id;
#[path = "../🪜️change-built-up-label/🦀️.rs"]
pub mod change_built_up_label;
#[path = "../🪜️change-built-up-kind/🦀️.rs"]
pub mod change_built_up_kind;
#[path = "../🪜️change-built-up-chord-area/🦀️.rs"]
pub mod change_built_up_chord_area;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_built_up_more::{change_built_up_chord_inertia, change_built_up_chord_distance, change_built_up_panel_spacing, change_built_up_planes};
