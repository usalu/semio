//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-deck-mass/🦀️.rs"]
pub mod change_bridge_deck_mass;
#[path = "../🌉️change-bridge-natural-frequency/🦀️.rs"]
pub mod change_bridge_natural_frequency;
#[path = "../🌉️change-bridge-end-rotation/🦀️.rs"]
pub mod change_bridge_end_rotation;
#[path = "../🌉️change-bridge-basic-wind-speed/🦀️.rs"]
pub mod change_bridge_basic_wind_speed;
//#endregion 🪢️Mounts
