//! 🔺️ Sparse diff construction for the `replace-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::ReplaceReportRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramReportsDelta, ProgramReportsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceReportRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.report_record).expect("diff_patch always produces a full patch");
    ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.report_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
