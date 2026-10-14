//! 📚️ EN 1994 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🌉️composite-bridge-girder/🦀️.rs"]
pub mod composite_bridge_girder;
#[path = "🏢️composite-floor-beam/🦀️.rs"]
pub mod composite_floor_beam;
#[path = "🏢️composite-floor-beam-failing/🦀️.rs"]
pub mod composite_floor_beam_failing;
#[cfg(test)]
#[path = "🌉️composite-bridge-girder/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️Mounts
