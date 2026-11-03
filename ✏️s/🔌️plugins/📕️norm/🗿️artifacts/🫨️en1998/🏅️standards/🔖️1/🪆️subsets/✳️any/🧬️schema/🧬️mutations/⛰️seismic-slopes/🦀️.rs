//! ⛰️ EN 1998 seismic slope mutation leaves: geometry, strength, amplification and the slip circle.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️change-seismic-slope-circle-radius/🦀️.rs"]
pub mod change_seismic_slope_circle_radius;
#[path = "../📍️change-seismic-slope-circle-centre-z/🦀️.rs"]
pub mod change_seismic_slope_circle_centre_z;
#[path = "../📍️change-seismic-slope-circle-centre-x/🦀️.rs"]
pub mod change_seismic_slope_circle_centre_x;
#[path = "../📍️change-seismic-slope-topographic-factor/🦀️.rs"]
pub mod change_seismic_slope_topographic_factor;
#[path = "../📍️change-seismic-slope-cohesion/🦀️.rs"]
pub mod change_seismic_slope_cohesion;
#[path = "../📍️change-seismic-slope-friction-angle/🦀️.rs"]
pub mod change_seismic_slope_friction_angle;
#[path = "../📍️change-seismic-slope-angle/🦀️.rs"]
pub mod change_seismic_slope_angle;
#[path = "../📍️change-seismic-slope-height/🦀️.rs"]
pub mod change_seismic_slope_height;
//#endregion 🪢️Mounts
