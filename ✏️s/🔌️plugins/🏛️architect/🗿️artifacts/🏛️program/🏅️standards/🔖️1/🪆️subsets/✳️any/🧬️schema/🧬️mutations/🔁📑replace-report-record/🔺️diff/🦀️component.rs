//! 🔺️ Sparse diff construction for the `replace-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::ReplaceReportRecord;
use crate::artifacts::program::diff::{ProgramReportsDelta, ProgramReportsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceReportRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No report record exists with this id.", [payload.report_record.header.id.0.clone()]);
    };
    if existing == &payload.report_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This report record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.report_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.report_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
