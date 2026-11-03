//! ⚖️ EN 1995 floor-loads mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏠️change-floor-mass/🦀️.rs"]
pub mod change_floor_mass;
#[path = "../🏠️change-floor-damping/🦀️.rs"]
pub mod change_floor_damping;
#[path = "../🏠️change-floor-permanent-load/🦀️.rs"]
pub mod change_floor_permanent_load;
#[path = "../🏠️change-floor-imposed-load/🦀️.rs"]
pub mod change_floor_imposed_load;
#[path = "../🏠️change-floor-psi2/🦀️.rs"]
pub mod change_floor_psi_2;
//#endregion 🪢️Mounts
