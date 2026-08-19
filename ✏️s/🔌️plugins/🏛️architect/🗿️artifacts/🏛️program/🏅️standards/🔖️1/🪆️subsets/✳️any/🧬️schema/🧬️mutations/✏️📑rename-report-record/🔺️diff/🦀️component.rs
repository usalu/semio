//! 🔺️ Sparse diff construction for the `rename-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::RenameReportRecord;
use crate::artifacts::program::diff::{ProgramReportsDelta, ProgramReportsPatchEntry};
use crate::artifacts::program::registers::ReportRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameReportRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.reports.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No report record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This report record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ReportRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
