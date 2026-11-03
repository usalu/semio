//! ⭕ EN 1992 bar layers mutation leaves: bar layer aggregate size, bar layer anchorage length, bar layer bond condition and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⭕️change-bar-layer-position/🦀️.rs"]
pub mod change_bar_layer_position;
#[path = "../⭕️change-bar-layer-anchorage-length/🦀️.rs"]
pub mod change_bar_layer_anchorage_length;
#[path = "../⭕️change-bar-layer-lap-length/🦀️.rs"]
pub mod change_bar_layer_lap_length;
#[path = "../⭕️change-bar-layer-bond-condition/🦀️.rs"]
pub mod change_bar_layer_bond_condition;
#[path = "../⭕️change-bar-layer-aggregate-size/🦀️.rs"]
pub mod change_bar_layer_aggregate_size;
#[path = "../⭕️change-bar-layer-lap-share/🦀️.rs"]
pub mod change_bar_layer_lap_share;
//#endregion 🪢️Mounts
