//! 🛤️ EN 1995 mutation bridges: the round trip of every mutation kind through the production codec and applier, and the committed vector replay.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod bridge_tests;
