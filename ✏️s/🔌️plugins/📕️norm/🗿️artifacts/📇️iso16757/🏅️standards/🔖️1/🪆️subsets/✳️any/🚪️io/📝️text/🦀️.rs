//! 🚪️ Artifact representation module ownership.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

pub use semio_s_artifact_norm_iso16757_diff::text as diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
