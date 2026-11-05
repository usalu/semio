//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-lambda1/🦀️.rs"]
pub mod change_bridge_lambda_1;
#[path = "../🌉️change-bridge-lambda-max/🦀️.rs"]
pub mod change_bridge_lambda_max;
#[path = "../🌉️change-bridge-detail-id/🦀️.rs"]
pub mod change_bridge_detail_id;
#[path = "../🌉️change-bridge-plate-thickness/🦀️.rs"]
pub mod change_bridge_plate_thickness;
//#endregion 🪢️Mounts
