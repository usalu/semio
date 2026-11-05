//! 🧱 EN 1997 investigation mutation leaves (pile groups, laboratory tests, layer water).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➖remove-lab-test/🦀️.rs"]
pub mod remove_lab_test;
#[path = "../➕insert-lab-test/🦀️.rs"]
pub mod insert_lab_test;
#[path = "../➖remove-layer-water/🦀️.rs"]
pub mod remove_layer_water;
#[path = "../➕insert-layer-water/🦀️.rs"]
pub mod insert_layer_water;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1997_mut_investigation_more::{remove_characteristic_basis, remove_ground_investigation, remove_pile_group, insert_characteristic_basis, insert_ground_investigation, insert_pile_group};
