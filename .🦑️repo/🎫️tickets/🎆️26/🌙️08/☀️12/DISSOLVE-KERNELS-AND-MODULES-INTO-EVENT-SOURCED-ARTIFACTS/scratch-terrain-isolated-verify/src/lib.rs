//! Scratch-only isolated verification harness for the DKM W2 terrain exemplar wave. Path-includes
//! the REAL, unmodified `🏔️terrain/🦀️component.rs` (no copy) to run its test suite without the
//! pre-existing, unrelated compile breakage currently blocking `cargo test` in
//! `semio-framework-surface`/`semio-framework-os-infinite` (see terrain-report.md). Not part of the
//! workspace, not a permanent artifact — ticket-folder scratch per repo rule #3.

#[path = "../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs"]
mod terrain;
