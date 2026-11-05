//! 🏞️ EN 1996 frost, cover, deflection, soil and fire isotherm mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❄️change-frost-class/🦀️.rs"]
pub mod change_frost_class;
#[path = "../🛡️change-cover/🦀️.rs"]
pub mod change_cover;
#[path = "../📉change-deflection-limit/🦀️.rs"]
pub mod change_deflection_limit;
#[path = "../🏞️change-soil-height/🦀️.rs"]
pub mod change_soil_height;
#[path = "../🪨change-soil-density/🦀️.rs"]
pub mod change_soil_density;
#[path = "../🔥change-isotherm-depth/🦀️.rs"]
pub mod change_isotherm_depth;
//#endregion 🪢️Mounts
