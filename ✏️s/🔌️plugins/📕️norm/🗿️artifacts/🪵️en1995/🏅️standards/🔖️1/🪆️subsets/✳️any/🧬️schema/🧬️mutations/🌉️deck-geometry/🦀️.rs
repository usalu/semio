//! 🌉️ EN 1995 deck-geometry mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-deck-lamination-product/🦀️.rs"]
pub mod change_deck_lamination_product;
#[path = "../🌉️change-deck-bar-product/🦀️.rs"]
pub mod change_deck_bar_product;
#[path = "../🌉️change-deck-span/🦀️.rs"]
pub mod change_deck_span;
#[path = "../🌉️change-deck-deck-width/🦀️.rs"]
pub mod change_deck_deck_width;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_deck_geometry_more::{change_deck_deck_depth, change_deck_lamination_width, change_deck_bar_spacing, change_deck_bar_diameter, change_deck_plate_side};
