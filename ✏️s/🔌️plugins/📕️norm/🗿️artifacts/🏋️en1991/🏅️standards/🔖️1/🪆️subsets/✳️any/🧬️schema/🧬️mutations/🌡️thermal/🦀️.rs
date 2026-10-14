//! 🌡️ EN 1991 thermal mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌡️change-assumed-delta-t/🦀️.rs"]
pub mod change_assumed_delta_t;
#[path = "../🕰️change-initial-temperature/🦀️.rs"]
pub mod change_initial_temperature;
#[path = "../📏change-linear-temperature-gradient/🦀️.rs"]
pub mod change_linear_temperature_gradient;
#[path = "../🌡️change-t-max/🦀️.rs"]
pub mod change_t_max;
#[path = "../🧊change-t-min/🦀️.rs"]
pub mod change_t_min;
#[path = "../🌉change-thermal-bridge-type/🦀️.rs"]
pub mod change_thermal_bridge_type;
#[path = "../🏗️change-thermal-element-type/🦀️.rs"]
pub mod change_thermal_element_type;
//#endregion 🪢️Mounts
