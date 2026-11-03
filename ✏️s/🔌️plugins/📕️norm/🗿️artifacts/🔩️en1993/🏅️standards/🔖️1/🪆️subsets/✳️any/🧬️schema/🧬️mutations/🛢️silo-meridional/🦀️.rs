//! 🛢️ EN 1993 silo shell mutation leaves: meridional stress, hoop stress, shell shear stress and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️change-silo-meridional-stress/🦀️.rs"]
pub mod change_silo_meridional_stress;
#[path = "../🛢️change-silo-hoop-stress/🦀️.rs"]
pub mod change_silo_hoop_stress;
#[path = "../🛢️change-silo-shell-shear-stress/🦀️.rs"]
pub mod change_silo_shell_shear_stress;
#[path = "../🛢️change-silo-boundary-factor-axial/🦀️.rs"]
pub mod change_silo_boundary_factor_axial;
#[path = "../🛢️change-silo-boundary-factor-hoop/🦀️.rs"]
pub mod change_silo_boundary_factor_hoop;
#[path = "../🛢️change-silo-boundary-factor-shear/🦀️.rs"]
pub mod change_silo_boundary_factor_shear;
#[path = "../🛢️change-silo-wall-friction/🦀️.rs"]
pub mod change_silo_wall_friction;
//#endregion 🪢️Mounts
