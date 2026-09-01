//! 🔺️ Sparse diff construction for the `rename-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::RenameCommunicationRequirement;
use crate::artifacts::program::diff::{ProgramCommunicationDelta, ProgramCommunicationPatchEntry};
use crate::artifacts::program::registers::CommunicationRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.communication.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No communication requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This communication requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = CommunicationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
