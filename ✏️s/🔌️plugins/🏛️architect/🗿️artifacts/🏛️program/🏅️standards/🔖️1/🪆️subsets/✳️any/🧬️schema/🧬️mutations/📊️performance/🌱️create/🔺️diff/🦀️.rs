//! 🔺️ Sparse diff construction for the `create-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::CreatePerformanceCriterion;
use crate::diff::ProgramPerformanceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreatePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.performance_criterion.header.id;
    if base.performance.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A performance criterion already exists with this id.", [id.0.clone()]);
    }
    let length = base.performance.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the performance criterion list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.performance.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(ProgramPerformanceDelta { added: vec![payload.performance_criterion.clone()], reordered, ..Default::default() }), ..Default::default() })
}
