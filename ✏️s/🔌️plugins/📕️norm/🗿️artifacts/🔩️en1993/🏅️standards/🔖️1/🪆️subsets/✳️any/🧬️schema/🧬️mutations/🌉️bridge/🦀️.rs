//! 🌉️ EN 1993 bridge fatigue mutation leaves: member id, lambda, phi2 and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-member-id/🦀️.rs"]
pub mod change_bridge_member_id;
#[path = "../🌉️change-bridge-lambda/🦀️.rs"]
pub mod change_bridge_lambda;
#[path = "../🌉️change-bridge-phi2/🦀️.rs"]
pub mod change_bridge_phi2;
#[path = "../🌉️change-bridge-delta-sigma-p/🦀️.rs"]
pub mod change_bridge_delta_sigma_p;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_bridge_more::{change_bridge_category, change_bridge_method, change_bridge_heavy_vehicle_weight, change_bridge_reference_weight, change_bridge_consequence};
