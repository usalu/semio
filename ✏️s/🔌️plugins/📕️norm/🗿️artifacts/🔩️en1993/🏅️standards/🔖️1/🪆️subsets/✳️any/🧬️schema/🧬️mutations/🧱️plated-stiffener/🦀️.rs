//! 🧱️ EN 1993 plated panel mutation leaves: stiffener inertia, stiffener area, stiffener eccentricity and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-stiffener-inertia/🦀️.rs"]
pub mod change_plated_stiffener_inertia;
#[path = "../🧱️change-plated-stiffener-area/🦀️.rs"]
pub mod change_plated_stiffener_area;
#[path = "../🧱️change-plated-stiffener-ecc/🦀️.rs"]
pub mod change_plated_stiffener_ecc;
#[path = "../🧱️change-plated-stiffener-alpha/🦀️.rs"]
pub mod change_plated_stiffener_alpha;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_plated_stiffener_more::{change_plated_compression_stress, change_plated_half_width, change_plated_equivalent_span, change_plated_hogging};
