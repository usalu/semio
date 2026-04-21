//! Integration-style tests (in-crate) for JSON/hash and I/O helpers.

mod io_json;
mod diff;
mod events;
mod entities;
mod flatten;
mod invalidation;
mod validation;

#[cfg(not(target_arch = "wasm32"))]
mod io_sqlite;

#[cfg(not(target_arch = "wasm32"))]
mod io_zip;
