//! ↩️ Inverse (undo) construction for the `create-collaboration-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🤝collaboration` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateCollaborationRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteCollaborationRecord(super::super::delete_collaboration_record::mutation::DeleteCollaborationRecord { id: payload.collaboration_record.header.id.clone() })]
}
