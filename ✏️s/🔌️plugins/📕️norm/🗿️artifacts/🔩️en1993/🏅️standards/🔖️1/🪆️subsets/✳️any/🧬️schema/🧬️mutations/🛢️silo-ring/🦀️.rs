//! 🛢️ EN 1993 silo shell mutation leaves: ring spacing, ring area, ring inertia and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️change-silo-ring-spacing/🦀️.rs"]
pub mod change_silo_ring_spacing;
#[path = "../🛢️change-silo-ring-area/🦀️.rs"]
pub mod change_silo_ring_area;
#[path = "../🛢️change-silo-ring-inertia/🦀️.rs"]
pub mod change_silo_ring_inertia;
#[path = "../🛢️change-silo-ring-offset/🦀️.rs"]
pub mod change_silo_ring_offset;
//#endregion 🪢️Mounts
