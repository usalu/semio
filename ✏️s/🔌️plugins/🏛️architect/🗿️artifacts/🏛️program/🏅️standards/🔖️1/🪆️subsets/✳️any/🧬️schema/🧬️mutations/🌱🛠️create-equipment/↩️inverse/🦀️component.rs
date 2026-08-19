//! ↩️ Inverse (undo) construction for the `create-equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛠️equipment` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateEquipment, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteEquipment(super::super::delete_equipment::mutation::DeleteEquipment { id: payload.equipment.header.id.clone() })]
}
