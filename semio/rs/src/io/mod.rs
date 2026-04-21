//! I/O backends for kit persistence. Each backend implements methods on
//! [`crate::kit::KitStore`] behind its own cfg, keeping the domain layer free of
//! transport concerns.

pub mod json;

#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite;

#[cfg(not(target_arch = "wasm32"))]
pub mod zip;
