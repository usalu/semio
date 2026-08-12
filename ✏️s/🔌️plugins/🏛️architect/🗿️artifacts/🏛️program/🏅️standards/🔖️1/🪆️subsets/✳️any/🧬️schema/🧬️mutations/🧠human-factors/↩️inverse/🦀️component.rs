//! ↩️ Inverse (undo) construction for the `human_factors` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateHumanFactorRequirement, DeleteHumanFactorRequirement, RenameHumanFactorRequirement, ReplaceHumanFactorRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateHumanFactorRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteHumanFactorRequirement(DeleteHumanFactorRequirement { id: payload.human_factor_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteHumanFactorRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.human_factors.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateHumanFactorRequirement(CreateHumanFactorRequirement { human_factor_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameHumanFactorRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.human_factors.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameHumanFactorRequirement(RenameHumanFactorRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.human_factors.iter().find(|row| row.header.id == payload.human_factor_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceHumanFactorRequirement(ReplaceHumanFactorRequirement { human_factor_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
