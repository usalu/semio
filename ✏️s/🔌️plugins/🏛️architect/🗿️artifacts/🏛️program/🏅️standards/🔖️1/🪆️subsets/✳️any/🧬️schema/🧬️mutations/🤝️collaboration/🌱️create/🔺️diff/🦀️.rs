//! 🔺️ Sparse diff construction for the `create-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::CreateCollaborationRecord;
use crate::diff::ProgramCollaborationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateCollaborationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.collaboration_record.header.id;
    if base.collaboration.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A collaboration record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.collaboration.len());
    if at > base.collaboration.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the collaboration record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { collaboration: Some(ProgramCollaborationDelta::insertion(at, payload.collaboration_record.clone())), ..Default::default() })
}
