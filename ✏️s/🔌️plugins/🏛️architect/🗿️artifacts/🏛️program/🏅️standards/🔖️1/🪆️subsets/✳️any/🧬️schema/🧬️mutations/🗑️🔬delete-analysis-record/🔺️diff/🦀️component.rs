//! 🔺️ Sparse diff construction for the `delete-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::DeleteAnalysisRecord;
use crate::artifacts::program::diff::ProgramAnalysesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.analyses.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No analysis record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(ProgramAnalysesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
