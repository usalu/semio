//! 🌉 EN 1990 bridge sls mutation leaves: bridge deck accel limit, bridge deck acceleration, bridge deck twist and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-member-id/🦀️.rs"]
pub mod change_bridge_member_id;
#[path = "../🌉️change-bridge-deck-acceleration/🦀️.rs"]
pub mod change_bridge_deck_acceleration;
#[path = "../🌉️change-bridge-deck-accel-limit/🦀️.rs"]
pub mod change_bridge_deck_accel_limit;
#[path = "../🌉️change-bridge-deck-twist/🦀️.rs"]
pub mod change_bridge_deck_twist;
#[path = "../🌉️change-bridge-deck-twist-limit/🦀️.rs"]
pub mod change_bridge_deck_twist_limit;
#[path = "../🌉️change-bridge-deflection/🦀️.rs"]
pub mod change_bridge_deflection;
#[path = "../🌉️change-bridge-deflection-limit/🦀️.rs"]
pub mod change_bridge_deflection_limit;
//#endregion 🪢️Mounts
