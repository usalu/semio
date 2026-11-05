//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪢️change-tension-cable-weight/🦀️.rs"]
pub mod change_tension_cable_weight;
#[path = "../🪢️change-tension-cable-horiz-force/🦀️.rs"]
pub mod change_tension_cable_horiz_force;
#[path = "../🪢️change-tension-cable-modulus/🦀️.rs"]
pub mod change_tension_cable_modulus;
#[path = "../🪢️change-tension-cable-stress/🦀️.rs"]
pub mod change_tension_cable_stress;
#[path = "../🪢️change-tension-cable-unit-weight/🦀️.rs"]
pub mod change_tension_cable_unit_weight;
//#endregion 🪢️Mounts
