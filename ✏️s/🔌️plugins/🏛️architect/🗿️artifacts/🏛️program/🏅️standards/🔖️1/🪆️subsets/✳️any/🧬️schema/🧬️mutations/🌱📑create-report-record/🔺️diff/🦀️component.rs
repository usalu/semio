//! 🔺️ Sparse diff construction for the `create-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::CreateReportRecord;
use crate::artifacts::program::diff::ProgramReportsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.reports` on apply.
pub fn diff(payload: &CreateReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { reports: Some(ProgramReportsDelta { added: vec![payload.report_record.clone()], ..Default::default() }), ..Default::default() }
}
