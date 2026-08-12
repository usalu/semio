//! ↩️ Inverse (undo) construction for the `validations` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateValidationRecord, DeleteValidationRecord, RenameValidationRecord, ReplaceValidationRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateValidationRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteValidationRecord(DeleteValidationRecord { id: payload.validation_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateValidationRecord(CreateValidationRecord { validation_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameValidationRecord(RenameValidationRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.validation_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceValidationRecord(ReplaceValidationRecord { validation_record: existing.clone() })],
        None => Vec::new(),
    }
}
