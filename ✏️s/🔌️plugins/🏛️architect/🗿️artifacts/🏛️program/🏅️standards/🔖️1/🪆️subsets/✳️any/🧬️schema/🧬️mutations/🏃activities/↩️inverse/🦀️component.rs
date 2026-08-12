//! ↩️ Inverse (undo) construction for the `activities` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateActivity, DeleteActivity, RenameActivity, ReplaceActivity};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateActivity, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteActivity(DeleteActivity { id: payload.activity.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteActivity, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.activities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateActivity(CreateActivity { activity: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameActivity, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.activities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameActivity(RenameActivity { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceActivity, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.activities.iter().find(|row| row.header.id == payload.activity.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceActivity(ReplaceActivity { activity: existing.clone() })],
        None => Vec::new(),
    }
}
