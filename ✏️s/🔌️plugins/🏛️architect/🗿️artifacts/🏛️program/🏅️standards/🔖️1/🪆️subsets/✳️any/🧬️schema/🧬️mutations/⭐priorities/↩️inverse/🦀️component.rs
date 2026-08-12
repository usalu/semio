//! ↩️ Inverse (undo) construction for the `priorities` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreatePriorityRecord, DeletePriorityRecord, RenamePriorityRecord, ReplacePriorityRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreatePriorityRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeletePriorityRecord(DeletePriorityRecord { id: payload.priority_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeletePriorityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.priorities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePriorityRecord(CreatePriorityRecord { priority_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenamePriorityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.priorities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenamePriorityRecord(RenamePriorityRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplacePriorityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePriorityRecord(ReplacePriorityRecord { priority_record: existing.clone() })],
        None => Vec::new(),
    }
}
