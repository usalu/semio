//! 🔺️ Sparse diff construction for the `analyses` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateAnalysisRecord, DeleteAnalysisRecord, RenameAnalysisRecord, ReplaceAnalysisRecord};
use crate::artifacts::program::diff::{ProgramAnalysesDelta, ProgramAnalysesPatchEntry};
use crate::artifacts::program::registers::AnalysisRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.analyses` on apply.
pub fn diff_create(payload: &CreateAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { added: vec![payload.analysis_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AnalysisRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceAnalysisRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.analyses.iter().find(|row| row.header.id == payload.analysis_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.analysis_record).expect("diff_patch always produces a full patch");
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.analysis_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
