//! 🚪️ Artifact representation module ownership.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
