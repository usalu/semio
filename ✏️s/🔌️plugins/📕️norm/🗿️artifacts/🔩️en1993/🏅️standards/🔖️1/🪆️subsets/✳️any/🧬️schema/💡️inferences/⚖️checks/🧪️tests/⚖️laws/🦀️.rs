//! ⚖️ EN 1993 check laws: the compliance and special-wiring suites run against the public surface of the checks crate.

#![allow(async_fn_in_trait)]
#![cfg(test)]

pub use semio_s_artifact_norm_en1993_checks::*;

#[path = "../../../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;

#[path = "../🔭️special-wiring/🦀️.rs"]
mod special_wiring_tests;
