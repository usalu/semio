//! 📚️ EN 1996 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🧱️loadbearing-wall/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[cfg(test)]
#[path = "❌️multi-fail-masonry/🧪️tests/🧩️example/🦀️.rs"]
mod example_multi_fail;
#[path = "🧱️loadbearing-wall/🦀️.rs"]
pub mod loadbearing_wall;
#[path = "❌️multi-fail-masonry/🦀️.rs"]
pub mod multi_fail_masonry;
//#endregion 🪢️Mounts
