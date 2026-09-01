//! 🔺️ Sparse diff construction for the `create-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::CreatePerformanceCriterion;
use crate::artifacts::program::diff::ProgramPerformanceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreatePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.performance_criterion.header.id.clone();
    if base.performance.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A performance criterion already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(ProgramPerformanceDelta { added: vec![payload.performance_criterion.clone()], ..Default::default() }), ..Default::default() })
}
