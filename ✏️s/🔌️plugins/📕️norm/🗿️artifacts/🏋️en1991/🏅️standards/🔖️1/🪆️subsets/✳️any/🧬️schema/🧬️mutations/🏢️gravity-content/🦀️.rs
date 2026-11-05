//! 🏢 EN 1991 gravity content mutation leaves: floor assumed partitions, floor assumed qk conc, floor category and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢️change-floor-category/🦀️.rs"]
pub mod change_floor_category;
#[path = "../🏢️change-floor-assumed-qk-conc/🦀️.rs"]
pub mod change_floor_assumed_qk_conc;
#[path = "../🏢️change-floor-assumed-partitions/🦀️.rs"]
pub mod change_floor_assumed_partitions;
#[path = "../🧱️change-self-weight-material/🦀️.rs"]
pub mod change_self_weight_material;
//#endregion 🪢️Mounts
