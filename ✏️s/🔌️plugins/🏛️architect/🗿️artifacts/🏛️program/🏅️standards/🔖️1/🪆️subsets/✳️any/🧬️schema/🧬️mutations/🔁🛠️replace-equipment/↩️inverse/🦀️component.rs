//! ↩️ Inverse (undo) construction for the `replace-equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛠️equipment` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceEquipment, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.equipment.iter().find(|row| row.header.id == payload.equipment.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceEquipment(super::mutation::ReplaceEquipment { equipment: existing.clone() })],
        None => Vec::new(),
    }
}
