//! ↩️ Inverse (undo) construction for the `status_records` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateStatusRecord, DeleteStatusRecord, RenameStatusRecord, ReplaceStatusRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateStatusRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStatusRecord(DeleteStatusRecord { id: payload.status_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteStatusRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.status_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStatusRecord(CreateStatusRecord { status_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameStatusRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.status_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameStatusRecord(RenameStatusRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceStatusRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.status_records.iter().find(|row| row.header.id == payload.status_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStatusRecord(ReplaceStatusRecord { status_record: existing.clone() })],
        None => Vec::new(),
    }
}
