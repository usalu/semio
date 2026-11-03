//! ↩️ `change-mortar-type` inverse: restores the previous `mortar_type`.

use crate::change_mortar_type::ChangeMortarType;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeMortarType, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mortar_type::ChangeMortarType> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMortarType, base: &En1996Snapshot) -> Option<ChangeMortarType> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMortarType { id: payload.id.clone(), new_mortar_type: r0.mortar_type.clone() })
}
