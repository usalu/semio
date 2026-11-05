//! ⭕️ EN 1999 ring stiffener mutation leaves: shell id, x, depth and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⭕️change-ring-shell-id/🦀️.rs"]
pub mod change_ring_shell_id;
#[path = "../⭕️change-ring-x/🦀️.rs"]
pub mod change_ring_x;
#[path = "../⭕️change-ring-depth/🦀️.rs"]
pub mod change_ring_depth;
#[path = "../⭕️change-ring-thickness/🦀️.rs"]
pub mod change_ring_thickness;
#[path = "../⭕️change-ring-internal/🦀️.rs"]
pub mod change_ring_internal;
//#endregion 🪢️Mounts
