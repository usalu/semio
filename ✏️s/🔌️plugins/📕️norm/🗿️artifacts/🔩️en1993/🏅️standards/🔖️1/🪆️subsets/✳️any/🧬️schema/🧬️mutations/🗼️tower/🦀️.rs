//! 🗼️ EN 1993 tower leg mutation leaves: member id, force coefficient, dynamic factor and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗼️change-tower-member-id/🦀️.rs"]
pub mod change_tower_member_id;
#[path = "../🗼️change-tower-force-coefficient/🦀️.rs"]
pub mod change_tower_force_coefficient;
#[path = "../🗼️change-tower-dynamic-factor/🦀️.rs"]
pub mod change_tower_dynamic_factor;
#[path = "../🗼️change-tower-lattice-kind/🦀️.rs"]
pub mod change_tower_lattice_kind;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_tower_more::{change_tower_slenderness, change_tower_diameter, change_tower_height, change_tower_bending_stiffness};
