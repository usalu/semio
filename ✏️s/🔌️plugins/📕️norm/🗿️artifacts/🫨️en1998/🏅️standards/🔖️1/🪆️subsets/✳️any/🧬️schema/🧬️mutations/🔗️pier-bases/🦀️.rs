//! 🔗️ EN 1998 pier base mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-pier-base/🦀️.rs"]
pub mod insert_pier_base;
#[path = "../➖️remove-pier-base/🦀️.rs"]
pub mod remove_pier_base;
#[path = "../✏️change-pier-base-bridge-id/🦀️.rs"]
pub mod change_pier_base_bridge_id;
#[path = "../✏️change-pier-base-pier-id/🦀️.rs"]
pub mod change_pier_base_pier_id;
#[path = "../📍️change-pier-base-radius-m/🦀️.rs"]
pub mod change_pier_base_radius_m;
#[path = "../📍️change-pier-base-soil-vs-m-s/🦀️.rs"]
pub mod change_pier_base_soil_vs_m_s;
#[path = "../📍️change-pier-base-soil-unit-weight-n-m3/🦀️.rs"]
pub mod change_pier_base_soil_unit_weight_n_m3;
#[path = "../📍️change-pier-base-poisson-ratio/🦀️.rs"]
pub mod change_pier_base_poisson_ratio;
//#endregion 🪢️Mounts
