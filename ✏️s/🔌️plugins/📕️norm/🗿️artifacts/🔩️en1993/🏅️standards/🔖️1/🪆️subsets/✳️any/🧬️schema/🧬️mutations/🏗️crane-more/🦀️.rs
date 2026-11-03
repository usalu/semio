//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-crane-fy/🦀️.rs"]
pub mod change_crane_fy;
#[path = "../🏗️change-crane-phi/🦀️.rs"]
pub mod change_crane_phi;
#[path = "../🏗️change-crane-max-wheel-load/🦀️.rs"]
pub mod change_crane_max_wheel_load;
#[path = "../🏗️change-crane-load-factor/🦀️.rs"]
pub mod change_crane_load_factor;
//#endregion 🪢️Mounts
