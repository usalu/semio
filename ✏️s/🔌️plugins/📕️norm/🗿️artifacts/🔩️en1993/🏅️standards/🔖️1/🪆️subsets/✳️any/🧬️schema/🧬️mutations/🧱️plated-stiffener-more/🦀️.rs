//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-compression-stress/🦀️.rs"]
pub mod change_plated_compression_stress;
#[path = "../🧱️change-plated-half-width/🦀️.rs"]
pub mod change_plated_half_width;
#[path = "../🧱️change-plated-equivalent-span/🦀️.rs"]
pub mod change_plated_equivalent_span;
#[path = "../🧱️change-plated-hogging/🦀️.rs"]
pub mod change_plated_hogging;
//#endregion 🪢️Mounts
