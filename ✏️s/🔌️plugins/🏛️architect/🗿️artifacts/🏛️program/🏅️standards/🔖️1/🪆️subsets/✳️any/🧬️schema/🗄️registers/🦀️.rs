//! 🏛️ Architect program artifact — the typed register entities for all 65 feature areas (the
//! document model's row types; the `ProgramSnapshot` document that holds them lives in `🦀️.rs`).

//! 🏛️ Architectural programming register entities — typed domain model for all 65 feature areas.

pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::registers::kernel;
pub use kernel::*;
pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::program_structure;
pub use program_structure::*;
pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::operations_requirements;
pub use operations_requirements::*;
pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::context_requirements;
pub use context_requirements::*;
pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::decisions;
pub use decisions::*;
#[path = "🗃️records/🦀️.rs"]
pub mod records;
pub use records::*;
