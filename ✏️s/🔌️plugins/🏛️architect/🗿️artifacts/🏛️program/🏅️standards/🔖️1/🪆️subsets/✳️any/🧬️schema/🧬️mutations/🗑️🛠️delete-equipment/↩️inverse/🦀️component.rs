//! ↩️ Inverse (undo) construction for the `delete-equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛠️equipment` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteEquipment, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.equipment.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateEquipment(super::super::create_equipment::mutation::CreateEquipment { equipment: existing.clone() })],
        None => Vec::new(),
    }
}
