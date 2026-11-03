//! 🏗️ EN 1993 crane runway mutation leaves: rail radius, rail contact width, crane nu and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-crane-rail-radius/🦀️.rs"]
pub mod change_crane_rail_radius;
#[path = "../🏗️change-crane-rail-contact-width/🦀️.rs"]
pub mod change_crane_rail_contact_width;
#[path = "../🏗️change-crane-crane-nu/🦀️.rs"]
pub mod change_crane_crane_nu;
#[path = "../🏗️change-crane-crane-kq/🦀️.rs"]
pub mod change_crane_crane_kq;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_crane_rail_radius_more::{change_crane_damage_exponent, change_crane_stress_range, change_crane_detail_cat_stress, change_crane_fatigue_phi};
