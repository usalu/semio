//! 🥞️ EN 1997 soil layer mutation leaves: soil type, depth top, depth bottom and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥞️change-layer-soil-type/🦀️.rs"]
pub mod change_layer_soil_type;
#[path = "../🥞️change-layer-depth-top/🦀️.rs"]
pub mod change_layer_depth_top;
#[path = "../🥞️change-layer-depth-bottom/🦀️.rs"]
pub mod change_layer_depth_bottom;
#[path = "../🥞️change-layer-gamma/🦀️.rs"]
pub mod change_layer_gamma;
#[path = "../🥞️change-layer-gamma-prime/🦀️.rs"]
pub mod change_layer_gamma_prime;
#[path = "../🥞️change-layer-cohesion-effective/🦀️.rs"]
pub mod change_layer_cohesion_effective;
#[path = "../🥞️change-layer-cohesion-undrained/🦀️.rs"]
pub mod change_layer_cohesion_undrained;
#[path = "../🥞️change-layer-cpt-nkt/🦀️.rs"]
pub mod change_layer_cpt_nkt;
//#endregion 🪢️Mounts
