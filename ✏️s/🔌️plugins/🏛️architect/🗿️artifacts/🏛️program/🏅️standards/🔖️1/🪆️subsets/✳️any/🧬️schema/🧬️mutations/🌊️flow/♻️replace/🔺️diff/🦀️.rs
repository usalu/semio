//! 🔺️ Sparse diff construction for the `replace-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::ReplaceFlowRequirement;
use crate::diff::ProgramFlowsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceFlowRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flow_requirement.header.id;
    let Some(position) = base.flows.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flow requirement exists with this id.", [id.0.clone()]);
    };
    if base.flows[position] == payload.flow_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This flow requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramFlowsDelta::removal(&base.flows, position);
    delta.absorb(ProgramFlowsDelta::insertion(position, payload.flow_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { flows: Some(delta), ..Default::default() })
}
