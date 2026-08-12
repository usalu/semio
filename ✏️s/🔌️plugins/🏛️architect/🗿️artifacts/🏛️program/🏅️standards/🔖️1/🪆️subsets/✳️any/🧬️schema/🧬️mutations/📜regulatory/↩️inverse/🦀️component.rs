//! ↩️ Inverse (undo) construction for the `regulatory` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateRegulatoryRequirement, DeleteRegulatoryRequirement, RenameRegulatoryRequirement, ReplaceRegulatoryRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateRegulatoryRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRegulatoryRequirement(DeleteRegulatoryRequirement { id: payload.regulatory_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteRegulatoryRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.regulatory.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRegulatoryRequirement(CreateRegulatoryRequirement { regulatory_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameRegulatoryRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.regulatory.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameRegulatoryRequirement(RenameRegulatoryRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.regulatory.iter().find(|row| row.header.id == payload.regulatory_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRegulatoryRequirement(ReplaceRegulatoryRequirement { regulatory_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
