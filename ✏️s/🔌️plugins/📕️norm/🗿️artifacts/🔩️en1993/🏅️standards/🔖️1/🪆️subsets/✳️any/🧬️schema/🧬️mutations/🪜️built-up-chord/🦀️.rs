//! 🪜️ EN 1993 built-up member mutation leaves: chord shape, chord depth, chord width and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪜️change-built-up-chord-shape/🦀️.rs"]
pub mod change_built_up_chord_shape;
#[path = "../🪜️change-built-up-chord-depth/🦀️.rs"]
pub mod change_built_up_chord_depth;
#[path = "../🪜️change-built-up-chord-width/🦀️.rs"]
pub mod change_built_up_chord_width;
#[path = "../🪜️change-built-up-chord-flange-thk/🦀️.rs"]
pub mod change_built_up_chord_flange_thk;
#[path = "../🪜️change-built-up-chord-web-thk/🦀️.rs"]
pub mod change_built_up_chord_web_thk;
//#endregion 🪢️Mounts
