//! ↩️ Inverse (undo) construction for the `resources` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateResource, DeleteResource, RenameResource, ReplaceResource};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateResource, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteResource(DeleteResource { id: payload.resource.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteResource, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resources.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateResource(CreateResource { resource: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameResource, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resources.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameResource(RenameResource { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceResource, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resources.iter().find(|row| row.header.id == payload.resource.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceResource(ReplaceResource { resource: existing.clone() })],
        None => Vec::new(),
    }
}
