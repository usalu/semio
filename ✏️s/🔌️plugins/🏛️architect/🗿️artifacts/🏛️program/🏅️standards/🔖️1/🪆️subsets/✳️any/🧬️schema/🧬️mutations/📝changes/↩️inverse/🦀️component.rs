//! ↩️ Inverse (undo) construction for the `changes` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateChangeRecord, DeleteChangeRecord, RenameChangeRecord, ReplaceChangeRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateChangeRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteChangeRecord(DeleteChangeRecord { id: payload.change_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteChangeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.changes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateChangeRecord(CreateChangeRecord { change_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameChangeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.changes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameChangeRecord(RenameChangeRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceChangeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.changes.iter().find(|row| row.header.id == payload.change_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceChangeRecord(ReplaceChangeRecord { change_record: existing.clone() })],
        None => Vec::new(),
    }
}
