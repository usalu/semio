//! 📚️ Layout examples: the example documents the subset advertises.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[cfg(test)]
#[path = "🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod demo_tests;
//#endregion 🪢️TaxonomyMounts
