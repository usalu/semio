//! ↩️ Inverse (undo) construction for the `equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateEquipment, DeleteEquipment, RenameEquipment, ReplaceEquipment};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateEquipment, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteEquipment(DeleteEquipment { id: payload.equipment.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteEquipment, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.equipment.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateEquipment(CreateEquipment { equipment: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameEquipment, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.equipment.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameEquipment(RenameEquipment { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceEquipment, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.equipment.iter().find(|row| row.header.id == payload.equipment.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceEquipment(ReplaceEquipment { equipment: existing.clone() })],
        None => Vec::new(),
    }
}
