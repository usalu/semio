//! ↩️ Inverse (undo) construction for the `relationships` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateRelationship, DeleteRelationship, RenameRelationship, ReplaceRelationship};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateRelationship, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRelationship(DeleteRelationship { id: payload.relationship.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteRelationship, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.relationships.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRelationship(CreateRelationship { relationship: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameRelationship, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.relationships.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameRelationship(RenameRelationship { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceRelationship, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.relationships.iter().find(|row| row.header.id == payload.relationship.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRelationship(ReplaceRelationship { relationship: existing.clone() })],
        None => Vec::new(),
    }
}
