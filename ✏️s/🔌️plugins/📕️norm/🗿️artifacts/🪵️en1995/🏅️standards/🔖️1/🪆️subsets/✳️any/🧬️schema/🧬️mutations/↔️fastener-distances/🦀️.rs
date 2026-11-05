//! ↔️ EN 1995 fastener-distances mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-fastener-spacing-par/🦀️.rs"]
pub mod change_fastener_spacing_par;
#[path = "../🔩️change-fastener-spacing-perp/🦀️.rs"]
pub mod change_fastener_spacing_perp;
#[path = "../🔩️change-fastener-end-distance/🦀️.rs"]
pub mod change_fastener_end_distance;
#[path = "../🔩️change-fastener-edge-distance/🦀️.rs"]
pub mod change_fastener_edge_distance;
#[path = "../🔩️change-fastener-offset-x/🦀️.rs"]
pub mod change_fastener_offset_x;
#[path = "../🔩️change-fastener-offset-y/🦀️.rs"]
pub mod change_fastener_offset_y;
#[path = "../🔩️change-fastener-load-angle/🦀️.rs"]
pub mod change_fastener_load_angle;
//#endregion 🪢️Mounts
