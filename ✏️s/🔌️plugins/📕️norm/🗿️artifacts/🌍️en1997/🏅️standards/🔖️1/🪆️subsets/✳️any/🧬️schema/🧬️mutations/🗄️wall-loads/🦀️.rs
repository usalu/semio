//! 🗄️ EN 1997 retaining wall mutation leaves: wall movement, ocr, concrete gamma and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗄️change-wall-wall-movement/🦀️.rs"]
pub mod change_wall_wall_movement;
#[path = "../🗄️change-wall-ocr/🦀️.rs"]
pub mod change_wall_ocr;
#[path = "../🗄️change-wall-concrete-gamma/🦀️.rs"]
pub mod change_wall_concrete_gamma;
#[path = "../🗄️change-wall-surcharge/🦀️.rs"]
pub mod change_wall_surcharge;
#[path = "../🗄️change-wall-vertical-permanent/🦀️.rs"]
pub mod change_wall_vertical_permanent;
#[path = "../🗄️change-wall-horizontal-permanent/🦀️.rs"]
pub mod change_wall_horizontal_permanent;
//#endregion 🪢️Mounts
