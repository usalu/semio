//! 🔺️ Sparse diff construction for the `replace-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::ReplaceAnalysisRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAnalysesDelta, ProgramAnalysesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceAnalysisRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.analyses.iter().find(|row| row.header.id == payload.analysis_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.analysis_record).expect("diff_patch always produces a full patch");
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.analysis_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
