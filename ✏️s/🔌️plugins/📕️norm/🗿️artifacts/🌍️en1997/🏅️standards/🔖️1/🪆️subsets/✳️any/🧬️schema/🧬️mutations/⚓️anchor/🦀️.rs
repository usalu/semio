//! ⚓️ EN 1997 anchor mutation leaves: wall id, tendon, head depth and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚓️change-anchor-wall-id/🦀️.rs"]
pub mod change_anchor_wall_id;
#[path = "../⚓️change-anchor-tendon/🦀️.rs"]
pub mod change_anchor_tendon;
#[path = "../⚓️change-anchor-head-depth/🦀️.rs"]
pub mod change_anchor_head_depth;
#[path = "../⚓️change-anchor-inclination-deg/🦀️.rs"]
pub mod change_anchor_inclination_deg;
#[path = "../⚓️change-anchor-free-length/🦀️.rs"]
pub mod change_anchor_free_length;
#[path = "../⚓️change-anchor-fixed-length/🦀️.rs"]
pub mod change_anchor_fixed_length;
#[path = "../⚓️change-anchor-fixed-diameter/🦀️.rs"]
pub mod change_anchor_fixed_diameter;
//#endregion 🪢️Mounts
