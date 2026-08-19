//! 🔺️ Sparse diff construction for the `create-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::CreateCommunicationRequirement;
use crate::artifacts::program::diff::ProgramCommunicationDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.communication_requirement.header.id.clone();
    if base.communication.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A communication requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { communication: Some(ProgramCommunicationDelta { added: vec![payload.communication_requirement.clone()], ..Default::default() }), ..Default::default() })
}
