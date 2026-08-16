//! 🔺️ Sparse diff construction for the `create-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::CreateCollaborationRecord;
use crate::artifacts::program::diff::ProgramCollaborationDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateCollaborationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.collaboration_record.header.id.clone();
    if base.collaboration.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A collaboration record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { collaboration: Some(ProgramCollaborationDelta { added: vec![payload.collaboration_record.clone()], ..Default::default() }), ..Default::default() })
}
