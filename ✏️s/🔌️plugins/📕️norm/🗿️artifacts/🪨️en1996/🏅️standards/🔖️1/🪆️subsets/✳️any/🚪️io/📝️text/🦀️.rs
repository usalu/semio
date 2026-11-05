//! 🚪️ Artifact representation module ownership.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;

pub use semio_s_artifact_norm_en1996_mutations::text as mutations;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
