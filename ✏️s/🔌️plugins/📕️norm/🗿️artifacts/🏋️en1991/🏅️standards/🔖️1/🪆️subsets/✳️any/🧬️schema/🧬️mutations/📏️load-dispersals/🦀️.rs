//! 📏 EN 1991 load dispersals mutation leaves: load dispersal, load dispersal pavement thickness, load dispersal slab thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-load-dispersal-wheel-length/🦀️.rs"]
pub mod change_load_dispersal_wheel_length;
#[path = "../📏️change-load-dispersal-wheel-width/🦀️.rs"]
pub mod change_load_dispersal_wheel_width;
#[path = "../📏️change-load-dispersal-paving-thickness/🦀️.rs"]
pub mod change_load_dispersal_paving_thickness;
#[path = "../📏️change-load-dispersal-slab-thickness/🦀️.rs"]
pub mod change_load_dispersal_slab_thickness;
#[path = "../📏️insert-load-dispersal/🦀️.rs"]
pub mod insert_load_dispersal;
#[path = "../📏️remove-load-dispersal/🦀️.rs"]
pub mod remove_load_dispersal;
//#endregion 🪢️Mounts
