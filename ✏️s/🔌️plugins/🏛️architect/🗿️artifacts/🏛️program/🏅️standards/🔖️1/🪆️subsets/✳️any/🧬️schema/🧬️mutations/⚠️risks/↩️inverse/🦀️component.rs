//! ↩️ Inverse (undo) construction for the `risks` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateRisk, DeleteRisk, RenameRisk, ReplaceRisk};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateRisk, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRisk(DeleteRisk { id: payload.risk.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteRisk, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.risks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRisk(CreateRisk { risk: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameRisk, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.risks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameRisk(RenameRisk { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceRisk, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.risks.iter().find(|row| row.header.id == payload.risk.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRisk(ReplaceRisk { risk: existing.clone() })],
        None => Vec::new(),
    }
}
