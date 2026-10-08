//! 🔺️ Sparse diff construction for the `create-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::CreateReportRecord;
use crate::diff::ProgramReportsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateReportRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.report_record.header.id;
    if base.reports.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A report record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.reports.len());
    if at > base.reports.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the report record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { reports: Some(ProgramReportsDelta::insertion(at, payload.report_record.clone())), ..Default::default() })
}
