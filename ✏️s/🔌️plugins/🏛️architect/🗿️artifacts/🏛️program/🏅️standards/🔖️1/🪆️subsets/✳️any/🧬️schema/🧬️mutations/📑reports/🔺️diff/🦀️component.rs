//! 🔺️ Sparse diff construction for the `reports` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateReportRecord, DeleteReportRecord, RenameReportRecord, ReplaceReportRecord};
use crate::artifacts::program::diff::{ProgramReportsDelta, ProgramReportsPatchEntry};
use crate::artifacts::program::registers::ReportRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.reports` on apply.
pub fn diff_create(payload: &CreateReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { reports: Some(ProgramReportsDelta { added: vec![payload.report_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { reports: Some(ProgramReportsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ReportRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceReportRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.report_record).expect("diff_patch always produces a full patch");
    ProgramDiff { reports: Some(ProgramReportsDelta { patched: vec![ProgramReportsPatchEntry { id: payload.report_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
