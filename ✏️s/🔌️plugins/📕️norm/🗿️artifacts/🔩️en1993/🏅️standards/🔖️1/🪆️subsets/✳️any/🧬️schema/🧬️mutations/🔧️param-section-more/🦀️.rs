//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔧️change-param-width/🦀️.rs"]
pub mod change_param_width;
#[path = "../🔧️change-param-web-thickness/🦀️.rs"]
pub mod change_param_web_thickness;
#[path = "../🔧️change-param-flange-thickness/🦀️.rs"]
pub mod change_param_flange_thickness;
#[path = "../🔧️change-param-tolerance/🦀️.rs"]
pub mod change_param_tolerance;
//#endregion 🪢️Mounts
