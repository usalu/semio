//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-soil-layer-n160/🦀️.rs"]
pub mod change_soil_layer_n1_60;
#[path = "../✏️change-soil-layer-fines-percent/🦀️.rs"]
pub mod change_soil_layer_fines_percent;
#[path = "../➕️insert-soil-layer/🦀️.rs"]
pub mod insert_soil_layer;
#[path = "../➖️remove-soil-layer/🦀️.rs"]
pub mod remove_soil_layer;
//#endregion 🪢️Mounts
