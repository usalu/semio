//! 🌡 EN 1991 thermal sections mutation leaves: thermal assumed range, thermal section, thermal section alpha micro and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌡️change-thermal-section-deck-type/🦀️.rs"]
pub mod change_thermal_section_deck_type;
#[path = "../🌡️change-thermal-section-surfacing/🦀️.rs"]
pub mod change_thermal_section_surfacing;
#[path = "../🌡️change-thermal-section-length/🦀️.rs"]
pub mod change_thermal_section_length;
#[path = "../🌡️change-thermal-section-depth/🦀️.rs"]
pub mod change_thermal_section_depth;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_thermal_sections_more::{change_thermal_section_alpha_micro, change_thermal_assumed_range, insert_thermal_section, remove_thermal_section};
