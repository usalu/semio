//! 🔺️ Sparse diff construction for the `replace-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::ReplaceAnalysisRecord;
use crate::diff::ProgramAnalysesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceAnalysisRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.analysis_record.header.id;
    let Some(position) = base.analyses.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No analysis record exists with this id.", [id.0.clone()]);
    };
    if base.analyses[position] == payload.analysis_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This analysis record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramAnalysesDelta::removal(&base.analyses, position);
    delta.absorb(ProgramAnalysesDelta::insertion(position, payload.analysis_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { analyses: Some(delta), ..Default::default() })
}
