//! 🔺️ Sparse diff construction for the `replace-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::ReplacePerformanceCriterion;
use crate::diff::ProgramPerformanceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplacePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.performance_criterion.header.id;
    let Some(position) = base.performance.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No performance criterion exists with this id.", [id.0.clone()]);
    };
    if base.performance[position] == payload.performance_criterion {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This performance criterion already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramPerformanceDelta::removal(&base.performance, position);
    delta.absorb(ProgramPerformanceDelta::insertion(position, payload.performance_criterion.clone()));
    protocol::MutationOutcome::new(ProgramDiff { performance: Some(delta), ..Default::default() })
}
