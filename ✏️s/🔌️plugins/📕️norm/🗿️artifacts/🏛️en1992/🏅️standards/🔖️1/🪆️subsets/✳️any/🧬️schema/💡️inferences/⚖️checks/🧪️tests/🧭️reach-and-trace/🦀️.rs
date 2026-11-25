//! 🧭️ EN 1992 reach and trace laws: the route table of the catalogue entries and the spec trace, run against the public surface of the checks crate.

#![allow(async_fn_in_trait)]
#![cfg(test)]

pub use semio_s_artifact_norm_en1992_checks::*;
use semio_s_plugin_norm_compliance::{ndp_is_verified, AnnexChoice};

#[path = "../📍️reach/🦀️.rs"]
mod reach_tests;

#[path = "../🧭️spec-trace/🦀️.rs"]
mod spec_trace_tests;
