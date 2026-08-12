//! ↩️ Inverse (undo) construction for the `sustainability` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateSustainabilityRequirement, DeleteSustainabilityRequirement, RenameSustainabilityRequirement, ReplaceSustainabilityRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateSustainabilityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSustainabilityRequirement(DeleteSustainabilityRequirement { id: payload.sustainability_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteSustainabilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.sustainability.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSustainabilityRequirement(CreateSustainabilityRequirement { sustainability_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameSustainabilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.sustainability.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameSustainabilityRequirement(RenameSustainabilityRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.sustainability.iter().find(|row| row.header.id == payload.sustainability_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSustainabilityRequirement(ReplaceSustainabilityRequirement { sustainability_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
