//! 🏗 EN 1991 crane hoists mutation leaves: crane hoist, crane hoist rated load, crane hoist release device and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-crane-hoist-rated-load/🦀️.rs"]
pub mod change_crane_hoist_rated_load;
#[path = "../🏗️change-crane-hoist-release-fraction/🦀️.rs"]
pub mod change_crane_hoist_release_fraction;
#[path = "../🏗️change-crane-hoist-release-device/🦀️.rs"]
pub mod change_crane_hoist_release_device;
#[path = "../🏗️insert-crane-hoist/🦀️.rs"]
pub mod insert_crane_hoist;
#[path = "../🏗️remove-crane-hoist/🦀️.rs"]
pub mod remove_crane_hoist;
//#endregion 🪢️Mounts
