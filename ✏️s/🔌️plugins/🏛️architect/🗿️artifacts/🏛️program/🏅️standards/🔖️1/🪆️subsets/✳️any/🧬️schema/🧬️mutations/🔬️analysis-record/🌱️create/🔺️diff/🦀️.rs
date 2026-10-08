//! 🔺️ Sparse diff construction for the `create-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::CreateAnalysisRecord;
use crate::diff::ProgramAnalysesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.analysis_record.header.id;
    if base.analyses.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An analysis record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.analyses.len());
    if at > base.analyses.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the analysis record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(ProgramAnalysesDelta::insertion(at, payload.analysis_record.clone())), ..Default::default() })
}
