//! 🪵️ EN 1993 pile mutation leaves: section id, material id, driving stress and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪵️change-pile-section-id/🦀️.rs"]
pub mod change_pile_section_id;
#[path = "../🪵️change-pile-material-id/🦀️.rs"]
pub mod change_pile_material_id;
#[path = "../🪵️change-pile-driving-stress/🦀️.rs"]
pub mod change_pile_driving_stress;
#[path = "../🪵️change-pile-embedded-length/🦀️.rs"]
pub mod change_pile_embedded_length;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_pile_more::{change_pile_shaft_perimeter, change_pile_environment, change_pile_design_life, change_pile_wall_thickness};
