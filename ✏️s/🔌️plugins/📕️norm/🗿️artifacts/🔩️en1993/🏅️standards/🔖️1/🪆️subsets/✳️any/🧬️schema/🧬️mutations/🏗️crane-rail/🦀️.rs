//! 🏗️ EN 1993 crane runway mutation leaves: rail inertia, rail fixed, flange effective width and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-crane-rail-inertia/🦀️.rs"]
pub mod change_crane_rail_inertia;
#[path = "../🏗️change-crane-rail-fixed/🦀️.rs"]
pub mod change_crane_rail_fixed;
#[path = "../🏗️change-crane-flange-eff-width/🦀️.rs"]
pub mod change_crane_flange_eff_width;
#[path = "../🏗️change-crane-flange-thickness/🦀️.rs"]
pub mod change_crane_flange_thickness;
#[path = "../🏗️change-crane-wheel-eccentricity/🦀️.rs"]
pub mod change_crane_wheel_eccentricity;
#[path = "../🏗️change-crane-torsion-constant/🦀️.rs"]
pub mod change_crane_torsion_constant;
#[path = "../🏗️change-crane-longitudinal-stress/🦀️.rs"]
pub mod change_crane_longitudinal_stress;
//#endregion 🪢️Mounts
