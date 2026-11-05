//! 🔥️ EN 1993 fire exposure mutation leaves: member id, rating, protection thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔥️change-fire-member-id/🦀️.rs"]
pub mod change_fire_member_id;
#[path = "../🔥️change-fire-rating/🦀️.rs"]
pub mod change_fire_rating;
#[path = "../🔥️change-fire-protection-thickness/🦀️.rs"]
pub mod change_fire_protection_thickness;
#[path = "../🔥️change-fire-section-factor/🦀️.rs"]
pub mod change_fire_section_factor;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_fire_exposure_more::{change_fire_mu0, change_fire_design_temperature, change_fire_prot_conductivity, change_fire_protection_density, change_fire_prot_specific_heat};
