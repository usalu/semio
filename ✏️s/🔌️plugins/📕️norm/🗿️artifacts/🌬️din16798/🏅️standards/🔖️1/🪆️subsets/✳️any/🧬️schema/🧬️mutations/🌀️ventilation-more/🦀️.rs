//! more ventilation mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📛change-vent-name/🦀️.rs"]
pub mod change_vent_name;
#[path = "../🪪change-vent-id/🦀️.rs"]
pub mod change_vent_id;
#[path = "../🌪️change-vent-fan-flow/🦀️.rs"]
pub mod change_vent_fan_flow;
#[path = "../⏱️change-vent-fan-run-hours/🦀️.rs"]
pub mod change_vent_fan_run_hours;
#[path = "../🧪change-vent-duct-test-pressure/🦀️.rs"]
pub mod change_vent_duct_test_pressure;
#[path = "../💦change-vent-humidification-required/🦀️.rs"]
pub mod change_vent_humidification_required;
#[path = "../🚿change-vent-humidification-provided/🦀️.rs"]
pub mod change_vent_humidification_provided;
//#endregion 🪢️Mounts
