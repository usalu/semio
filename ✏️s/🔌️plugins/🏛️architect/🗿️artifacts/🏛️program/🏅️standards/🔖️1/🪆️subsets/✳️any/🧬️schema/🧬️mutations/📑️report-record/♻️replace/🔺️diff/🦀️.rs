//! 🔺️ Sparse diff construction for the `replace-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::ReplaceReportRecord;
use crate::diff::ProgramReportsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceReportRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.report_record.header.id;
    let Some(position) = base.reports.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No report record exists with this id.", [id.0.clone()]);
    };
    if base.reports[position] == payload.report_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This report record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramReportsDelta::removal(&base.reports, position);
    delta.absorb(ProgramReportsDelta::insertion(position, payload.report_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { reports: Some(delta), ..Default::default() })
}
