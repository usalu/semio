//! 🥞️ EN 1997 soil layer mutation leaves: poisson ratio, cpt qc, spt n.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥞️change-layer-poisson-ratio/🦀️.rs"]
pub mod change_layer_poisson_ratio;
#[path = "../🥞️change-layer-cpt-qc/🦀️.rs"]
pub mod change_layer_cpt_qc;
#[path = "../🥞️change-layer-spt-n/🦀️.rs"]
pub mod change_layer_spt_n;
//#endregion 🪢️Mounts
