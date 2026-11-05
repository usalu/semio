//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-deck-bar-resistance/🦀️.rs"]
pub mod change_deck_bar_resistance;
#[path = "../🌉️change-deck-shear-stress/🦀️.rs"]
pub mod change_deck_shear_stress;
#[path = "../🌉️change-deck-compression-c90/🦀️.rs"]
pub mod change_deck_compression_c90;
#[path = "../🌉️change-deck-wheel-load/🦀️.rs"]
pub mod change_deck_wheel_load;
#[path = "../🌉️change-deck-wheel-contact/🦀️.rs"]
pub mod change_deck_wheel_contact;
//#endregion 🪢️Mounts
