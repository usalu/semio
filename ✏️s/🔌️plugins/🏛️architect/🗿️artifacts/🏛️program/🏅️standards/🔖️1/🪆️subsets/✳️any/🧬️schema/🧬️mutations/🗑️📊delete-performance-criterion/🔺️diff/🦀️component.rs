//! 🔺️ Sparse diff construction for the `delete-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::DeletePerformanceCriterion;
use crate::artifacts::program::diff::ProgramPerformanceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeletePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.performance.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No performance criterion exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(ProgramPerformanceDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
