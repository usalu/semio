//! 🚪️ Artifact representation module ownership.
#![allow(clippy::result_large_err)]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;

pub use semio_s_artifact_procedural_2d_mutations::text as mutations;

pub use semio_s_artifact_procedural_2d_model::diff_text as diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
