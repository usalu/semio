//! 🔺️ Sparse diff construction for the `rename-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::RenameReportRecord;
use crate::artifacts::program::diff::{ProgramReportsDelta, ProgramReportsPatchEntry};
use crate::artifacts::program::registers::ReportRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ReportRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
