//! 🔺️ Sparse diff construction for the `create-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::CreateAnalysisRecord;
use crate::artifacts::program::diff::ProgramAnalysesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.analysis_record.header.id.clone();
    if base.analyses.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An analysis record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(ProgramAnalysesDelta { added: vec![payload.analysis_record.clone()], ..Default::default() }), ..Default::default() })
}
