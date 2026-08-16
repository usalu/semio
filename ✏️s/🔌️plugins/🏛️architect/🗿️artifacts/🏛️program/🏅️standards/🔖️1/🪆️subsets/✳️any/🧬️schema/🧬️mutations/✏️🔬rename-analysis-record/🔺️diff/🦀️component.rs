//! 🔺️ Sparse diff construction for the `rename-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::RenameAnalysisRecord;
use crate::artifacts::program::diff::{ProgramAnalysesDelta, ProgramAnalysesPatchEntry};
use crate::artifacts::program::registers::AnalysisRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.analyses.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No analysis record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This analysis record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = AnalysisRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
