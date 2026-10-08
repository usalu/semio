//! 🔺️ Sparse diff construction for the `replace-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::ReplaceCommunicationRequirement;
use crate::diff::ProgramCommunicationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.communication_requirement.header.id;
    let Some(position) = base.communication.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No communication requirement exists with this id.", [id.0.clone()]);
    };
    if base.communication[position] == payload.communication_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This communication requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramCommunicationDelta::removal(&base.communication, position);
    delta.absorb(ProgramCommunicationDelta::insertion(position, payload.communication_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { communication: Some(delta), ..Default::default() })
}
