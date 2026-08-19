//! 🔺️ Sparse diff construction for the `delete-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::DeleteReportRecord;
use crate::artifacts::program::diff::ProgramReportsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteReportRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.reports.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No report record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { reports: Some(ProgramReportsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
