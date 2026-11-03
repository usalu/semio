//! ↩️ `change-mortar-class` inverse: restores the previous `mortar_class`.

use crate::change_mortar_class::ChangeMortarClass;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeMortarClass, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mortar_class::ChangeMortarClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMortarClass, base: &En1996Snapshot) -> Option<ChangeMortarClass> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMortarClass { id: payload.id.clone(), new_mortar_class: r0.mortar_class.clone() })
}
