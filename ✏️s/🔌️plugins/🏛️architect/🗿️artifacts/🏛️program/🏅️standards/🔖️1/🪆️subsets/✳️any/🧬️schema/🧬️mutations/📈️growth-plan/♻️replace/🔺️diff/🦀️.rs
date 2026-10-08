//! 🔺️ Sparse diff construction for the `replace-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::ReplaceGrowthPlan;
use crate::diff::ProgramGrowthDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.growth_plan.header.id;
    let Some(position) = base.growth.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No growth plan exists with this id.", [id.0.clone()]);
    };
    if base.growth[position] == payload.growth_plan {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This growth plan already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramGrowthDelta::removal(&base.growth, position);
    delta.absorb(ProgramGrowthDelta::insertion(position, payload.growth_plan.clone()));
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(delta), ..Default::default() })
}
