//! 🔗️ EN 1998 tie beam mutation leaves: span, section, steel and relative displacement.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️change-tie-beam-displacement-limit/🦀️.rs"]
pub mod change_tie_beam_displacement_limit;
#[path = "../📍️change-tie-beam-relative-displacement/🦀️.rs"]
pub mod change_tie_beam_relative_displacement;
#[path = "../📍️change-tie-beam-steel-design-strength/🦀️.rs"]
pub mod change_tie_beam_steel_design_strength;
#[path = "../📍️change-tie-beam-steel-area/🦀️.rs"]
pub mod change_tie_beam_steel_area;
#[path = "../📍️change-tie-beam-depth/🦀️.rs"]
pub mod change_tie_beam_depth;
#[path = "../📍️change-tie-beam-width/🦀️.rs"]
pub mod change_tie_beam_width;
#[path = "../📍️change-tie-beam-span/🦀️.rs"]
pub mod change_tie_beam_span;
//#endregion 🪢️Mounts
