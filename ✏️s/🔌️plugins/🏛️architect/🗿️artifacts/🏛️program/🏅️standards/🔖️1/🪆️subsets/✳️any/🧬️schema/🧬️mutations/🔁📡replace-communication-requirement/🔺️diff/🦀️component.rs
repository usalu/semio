//! 🔺️ Sparse diff construction for the `replace-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::ReplaceCommunicationRequirement;
use crate::artifacts::program::diff::{ProgramCommunicationDelta, ProgramCommunicationPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.communication.iter().find(|row| row.header.id == payload.communication_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No communication requirement exists with this id.", [payload.communication_requirement.header.id.0.clone()]);
    };
    if existing == &payload.communication_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This communication requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.communication_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff {
        communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.communication_requirement.header.id.0.clone(), patch }], ..Default::default() }),
        ..Default::default()
    })
}
