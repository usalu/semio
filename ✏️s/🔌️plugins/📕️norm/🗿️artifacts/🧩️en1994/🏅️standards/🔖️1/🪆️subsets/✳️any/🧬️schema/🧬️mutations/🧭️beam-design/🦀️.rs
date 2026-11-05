//! 🧭️ EN 1994 beam design input mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../↕️change-beam-end-plate-height-m/🦀️.rs"]
pub mod change_beam_end_plate_height_m;
#[path = "../⬜️change-beam-end-plate-width-m/🦀️.rs"]
pub mod change_beam_end_plate_width_m;
#[path = "../➡️change-beam-end-zone-length-m/🦀️.rs"]
pub mod change_beam_end_zone_length_m;
#[path = "../📏️change-beam-stud-edge-distance-m/🦀️.rs"]
pub mod change_beam_stud_edge_distance_m;
#[path = "../↔️change-beam-stud-cross-spacing-m/🦀️.rs"]
pub mod change_beam_stud_cross_spacing_m;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1994_mut_beam_design_bridge::{change_beam_transverse_as_top, change_beam_adjacent_span_m, change_beam_bridge_lanes, change_beam_patch_bearing_length_m, change_beam_cracked_length_fraction};
