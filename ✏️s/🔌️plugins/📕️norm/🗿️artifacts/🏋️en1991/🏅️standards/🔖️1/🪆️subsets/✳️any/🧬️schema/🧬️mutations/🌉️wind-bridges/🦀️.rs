//! 🌉 EN 1991 wind bridges mutation leaves: wind bridge, wind bridge deck depth, wind bridge deck width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-wind-bridge-deck-width/🦀️.rs"]
pub mod change_wind_bridge_deck_width;
#[path = "../🌉️change-wind-bridge-deck-depth/🦀️.rs"]
pub mod change_wind_bridge_deck_depth;
#[path = "../🌉️change-wind-bridge-parapet-kind/🦀️.rs"]
pub mod change_wind_bridge_parapet_kind;
#[path = "../🌉️change-wind-bridge-parapet-height/🦀️.rs"]
pub mod change_wind_bridge_parapet_height;
#[path = "../🌉️insert-wind-bridge/🦀️.rs"]
pub mod insert_wind_bridge;
#[path = "../🌉️remove-wind-bridge/🦀️.rs"]
pub mod remove_wind_bridge;
//#endregion 🪢️Mounts
