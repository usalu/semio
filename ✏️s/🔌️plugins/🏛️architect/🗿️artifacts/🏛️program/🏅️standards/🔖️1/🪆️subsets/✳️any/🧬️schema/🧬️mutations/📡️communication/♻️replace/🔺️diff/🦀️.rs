//! 🔺️ Sparse diff construction for the `replace-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::ReplaceCommunicationRequirement;
use crate::diff::ProgramCommunicationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.communication_requirement.header.id;
    let Some(position) = base.communication.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No communication requirement exists with this id.", [id.0.clone()]);
    };
    if base.communication[position] == payload.communication_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This communication requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.communication.len()).then(|| base.communication.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { communication: Some(ProgramCommunicationDelta { removed: vec![id.0.clone()], added: vec![payload.communication_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
