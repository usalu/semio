//! ↩️ Inverse (undo) construction for the `constraints` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateConstraintRecord, DeleteConstraintRecord, RenameConstraintRecord, ReplaceConstraintRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateConstraintRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteConstraintRecord(DeleteConstraintRecord { id: payload.constraint_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateConstraintRecord(CreateConstraintRecord { constraint_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameConstraintRecord(RenameConstraintRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceConstraintRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.constraints.iter().find(|row| row.header.id == payload.constraint_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceConstraintRecord(ReplaceConstraintRecord { constraint_record: existing.clone() })],
        None => Vec::new(),
    }
}
