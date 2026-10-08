//! 🔺️ Sparse diff construction for the `create-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::CreatePerformanceCriterion;
use crate::diff::ProgramPerformanceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreatePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.performance_criterion.header.id;
    if base.performance.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A performance criterion already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.performance.len());
    if at > base.performance.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the performance criterion list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(ProgramPerformanceDelta::insertion(at, payload.performance_criterion.clone())), ..Default::default() })
}
