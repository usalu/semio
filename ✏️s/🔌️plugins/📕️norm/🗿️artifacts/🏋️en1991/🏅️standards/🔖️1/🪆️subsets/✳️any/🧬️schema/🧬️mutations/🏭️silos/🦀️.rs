//! 🏭️ EN 1991 silos mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📦change-assumed-silo-patch/🦀️.rs"]
pub mod change_assumed_silo_patch;
#[path = "../🧱change-assumed-silo-wall-friction/🦀️.rs"]
pub mod change_assumed_silo_wall_friction;
#[path = "../🏭change-silo-claimed/🦀️.rs"]
pub mod change_silo_claimed;
#[path = "../⭕change-silo-hydraulic-radius/🦀️.rs"]
pub mod change_silo_hydraulic_radius;
#[path = "../⚖️change-silo-kind/🦀️.rs"]
pub mod change_silo_kind;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_silos_2::{change_assumed_silo_pressure, change_silo_bulk_density, change_silo_height, change_silo_k, change_silo_mu};
