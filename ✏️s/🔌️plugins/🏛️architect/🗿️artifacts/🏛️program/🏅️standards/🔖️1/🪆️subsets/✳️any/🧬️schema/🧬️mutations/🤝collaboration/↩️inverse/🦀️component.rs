//! ↩️ Inverse (undo) construction for the `collaboration` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateCollaborationRecord, DeleteCollaborationRecord, RenameCollaborationRecord, ReplaceCollaborationRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateCollaborationRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteCollaborationRecord(DeleteCollaborationRecord { id: payload.collaboration_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteCollaborationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.collaboration.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateCollaborationRecord(CreateCollaborationRecord { collaboration_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameCollaborationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.collaboration.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameCollaborationRecord(RenameCollaborationRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceCollaborationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCollaborationRecord(ReplaceCollaborationRecord { collaboration_record: existing.clone() })],
        None => Vec::new(),
    }
}
