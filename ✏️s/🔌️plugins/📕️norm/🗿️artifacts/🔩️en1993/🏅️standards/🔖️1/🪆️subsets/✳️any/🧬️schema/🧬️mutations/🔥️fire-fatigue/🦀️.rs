//! 🔥️ EN 1993 fire fatigue mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-bridge-fatigue/🦀️.rs"]
pub mod insert_bridge_fatigue;
#[path = "../➕️insert-fatigue-detail/🦀️.rs"]
pub mod insert_fatigue_detail;
#[path = "../➕️insert-fire-exposure/🦀️.rs"]
pub mod insert_fire_exposure;
#[path = "../➖️remove-bridge-fatigue/🦀️.rs"]
pub mod remove_bridge_fatigue;
#[path = "../➖️remove-fatigue-detail/🦀️.rs"]
pub mod remove_fatigue_detail;
#[path = "../➖️remove-fire-exposure/🦀️.rs"]
pub mod remove_fire_exposure;
#[path = "../🌉️update-bridge-inputs/🦀️.rs"]
pub mod update_bridge_inputs;
#[path = "../🔁️update-fatigue-inputs/🦀️.rs"]
pub mod update_fatigue_inputs;
#[path = "../🔥️update-fire-inputs/🦀️.rs"]
pub mod update_fire_inputs;
//#endregion 🪢️Mounts
