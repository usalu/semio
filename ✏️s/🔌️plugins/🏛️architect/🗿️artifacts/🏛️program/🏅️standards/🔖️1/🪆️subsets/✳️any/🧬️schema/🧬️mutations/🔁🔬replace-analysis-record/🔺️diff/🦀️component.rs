//! 🔺️ Sparse diff construction for the `replace-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::ReplaceAnalysisRecord;
use crate::artifacts::program::diff::{ProgramAnalysesDelta, ProgramAnalysesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.analyses.iter().find(|row| row.header.id == payload.analysis_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No analysis record exists with this id.", [payload.analysis_record.header.id.0.clone()]);
    };
    if existing == &payload.analysis_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This analysis record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.analysis_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.analysis_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
