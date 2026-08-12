//! ↩️ Inverse (undo) construction for the `environmental` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateEnvironmentalRequirement, DeleteEnvironmentalRequirement, RenameEnvironmentalRequirement, ReplaceEnvironmentalRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateEnvironmentalRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteEnvironmentalRequirement(DeleteEnvironmentalRequirement { id: payload.environmental_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateEnvironmentalRequirement(CreateEnvironmentalRequirement { environmental_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameEnvironmentalRequirement(RenameEnvironmentalRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.environmental_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceEnvironmentalRequirement(ReplaceEnvironmentalRequirement { environmental_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
