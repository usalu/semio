//! 🔗️ EN 1998 isolation system mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-isolation-system/🦀️.rs"]
pub mod insert_isolation_system;
#[path = "../➖️remove-isolation-system/🦀️.rs"]
pub mod remove_isolation_system;
#[path = "../✏️change-isolation-system-bridge-id/🦀️.rs"]
pub mod change_isolation_system_bridge_id;
#[path = "../✏️change-isolation-system-building-id/🦀️.rs"]
pub mod change_isolation_system_building_id;
#[path = "../📍️change-isolation-system-strength-n/🦀️.rs"]
pub mod change_isolation_system_strength_n;
#[path = "../📍️change-isolation-system-stiffness/🦀️.rs"]
pub mod change_isolation_system_stiffness;
#[path = "../📍️change-isolation-system-yield-disp-m/🦀️.rs"]
pub mod change_isolation_system_yield_disp_m;
#[path = "../📍️change-isolation-system-capacity-m/🦀️.rs"]
pub mod change_isolation_system_capacity_m;
//#endregion 🪢️Mounts
