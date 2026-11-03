//! 🥶️ EN 1993 cold-formed member mutation leaves: web height, web slope, opposite flange width and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥶️change-cf-web-height/🦀️.rs"]
pub mod change_cf_web_height;
#[path = "../🥶️change-cf-web-slope/🦀️.rs"]
pub mod change_cf_web_slope;
#[path = "../🥶️change-cf-opposite-flange-width/🦀️.rs"]
pub mod change_cf_opposite_flange_width;
#[path = "../🥶️change-cf-spring-factor/🦀️.rs"]
pub mod change_cf_spring_factor;
#[path = "../🥶️change-cf-corner-radius/🦀️.rs"]
pub mod change_cf_corner_radius;
#[path = "../🥶️change-cf-flange-stress/🦀️.rs"]
pub mod change_cf_flange_stress;
#[path = "../🥶️change-cf-shear-force/🦀️.rs"]
pub mod change_cf_shear_force;
//#endregion 🪢️Mounts
