//! ↩️ `change-frost-class` inverse: restores the previous `frost_class`.

use crate::change_frost_class::ChangeFrostClass;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeFrostClass, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frost_class::ChangeFrostClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFrostClass, base: &En1996Snapshot) -> Option<ChangeFrostClass> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFrostClass { id: payload.id.clone(), new_frost_class: r0.frost_class.clone() })
}
