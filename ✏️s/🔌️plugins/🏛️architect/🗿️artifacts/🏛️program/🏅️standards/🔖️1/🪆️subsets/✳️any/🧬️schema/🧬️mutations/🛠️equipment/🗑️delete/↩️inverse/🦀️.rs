//! ↩️ Inverse (undo) construction for the `delete-equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛠️equipment` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteEquipment, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.equipment.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateEquipment(super::super::create_equipment::CreateEquipment { equipment: base.equipment[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
