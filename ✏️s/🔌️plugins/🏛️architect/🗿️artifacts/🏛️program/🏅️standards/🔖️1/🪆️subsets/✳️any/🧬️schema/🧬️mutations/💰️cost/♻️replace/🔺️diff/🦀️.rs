//! 🔺️ Sparse diff construction for the `replace-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::ReplaceCostRequirement;
use crate::diff::ProgramCostsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceCostRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.cost_requirement.header.id;
    let Some(position) = base.costs.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No cost requirement exists with this id.", [id.0.clone()]);
    };
    if base.costs[position] == payload.cost_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This cost requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramCostsDelta::removal(&base.costs, position);
    delta.absorb(ProgramCostsDelta::insertion(position, payload.cost_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { costs: Some(delta), ..Default::default() })
}
