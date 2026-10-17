//! 🗃️ Architect record registers: status, workshops, surveys, issues, audit events, templates, knowledge, benchmarks, assumptions, constraints, compliance, approvals, meetings and governance; the shared enums live in `kernel` and the patch macros in [`super`].

use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
use protocol::Identified;
use protocol::list_delta::RowPatch;

#[path = "📝️status-workshop-and-audit-records/🦀️.rs"]
mod status_workshop_and_audit_records;
pub use status_workshop_and_audit_records::*;

#[path = "🧾️benchmark-compliance-and-governance/🦀️.rs"]
mod benchmark_compliance_and_governance;
pub use benchmark_compliance_and_governance::*;


