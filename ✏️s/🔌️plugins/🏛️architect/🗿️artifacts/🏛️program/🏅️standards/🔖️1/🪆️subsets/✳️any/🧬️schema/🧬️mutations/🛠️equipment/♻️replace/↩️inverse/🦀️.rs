//! ↩️ Inverse (undo) construction for the `replace-equipment` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛠️equipment` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceEquipment, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.equipment.iter().find(|row| row.header.id == payload.equipment.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceEquipment(super::ReplaceEquipment { equipment: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
