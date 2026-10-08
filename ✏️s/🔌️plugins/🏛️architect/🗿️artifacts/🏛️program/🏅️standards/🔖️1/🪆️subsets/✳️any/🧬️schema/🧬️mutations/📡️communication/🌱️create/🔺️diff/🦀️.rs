//! 🔺️ Sparse diff construction for the `create-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::CreateCommunicationRequirement;
use crate::diff::ProgramCommunicationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateCommunicationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.communication_requirement.header.id;
    if base.communication.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A communication requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.communication.len());
    if at > base.communication.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the communication requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { communication: Some(ProgramCommunicationDelta::insertion(at, payload.communication_requirement.clone())), ..Default::default() })
}
