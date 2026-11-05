//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-member-bridge-n-obs/🦀️.rs"]
pub mod change_member_bridge_n_obs;
#[path = "../🌉️change-member-bridge-tl-years/🦀️.rs"]
pub mod change_member_bridge_tl_years;
#[path = "../🌊️change-member-damping-xi/🦀️.rs"]
pub mod change_member_damping_xi;
#[path = "../⚖️change-member-mass-kg-per-m/🦀️.rs"]
pub mod change_member_mass_kg_per_m;
#[path = "../⚖️change-member-mass-kg-per-m2/🦀️.rs"]
pub mod change_member_mass_kg_per_m2;
//#endregion 🪢️Mounts
