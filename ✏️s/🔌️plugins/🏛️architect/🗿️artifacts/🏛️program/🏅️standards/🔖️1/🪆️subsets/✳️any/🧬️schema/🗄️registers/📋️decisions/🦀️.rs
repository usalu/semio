//! 📋️ Architect decision registers: requirements, priorities, scenarios, option evaluations, decisions, validations, performance, quality, artifacts, changes, collaboration, analyses, reports and search filters; the shared enums live in `kernel` and the patch macros in [`super`].

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
use protocol::Identified;
use protocol::list_delta::RowPatch;

#[path = "🎯️requirements-and-decisions/🦀️.rs"]
mod requirements_and_decisions;
pub use requirements_and_decisions::*;

#[path = "📈️quality-change-and-reports/🦀️.rs"]
mod quality_change_and_reports;
pub use quality_change_and_reports::*;


