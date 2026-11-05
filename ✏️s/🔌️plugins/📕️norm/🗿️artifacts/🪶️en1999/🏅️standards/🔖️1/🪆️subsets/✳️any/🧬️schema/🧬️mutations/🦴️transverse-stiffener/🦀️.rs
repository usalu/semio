//! 🦴️ EN 1999 transverse stiffener mutation leaves: girder id, position, width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🦴️change-transverse-girder-id/🦀️.rs"]
pub mod change_transverse_girder_id;
#[path = "../🦴️change-transverse-position/🦀️.rs"]
pub mod change_transverse_position;
#[path = "../🦴️change-transverse-width/🦀️.rs"]
pub mod change_transverse_width;
#[path = "../🦴️change-transverse-thickness/🦀️.rs"]
pub mod change_transverse_thickness;
#[path = "../🦴️change-conn-load-eccentricity/🦀️.rs"]
pub mod change_conn_load_eccentricity;
#[path = "../🦴️change-conn-hole-stagger/🦀️.rs"]
pub mod change_conn_hole_stagger;
#[path = "../🦴️change-transverse-sides/🦀️.rs"]
pub mod change_transverse_sides;
//#endregion 🪢️Mounts
