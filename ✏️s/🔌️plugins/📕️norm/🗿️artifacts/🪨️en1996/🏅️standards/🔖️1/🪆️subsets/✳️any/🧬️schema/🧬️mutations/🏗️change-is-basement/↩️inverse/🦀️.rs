//! ↩️ `change-is-basement` inverse: restores the previous `is_basement`.

use crate::change_is_basement::ChangeIsBasement;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeIsBasement, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_is_basement::ChangeIsBasement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsBasement, base: &En1996Snapshot) -> Option<ChangeIsBasement> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsBasement { id: payload.id.clone(), new_is_basement: r0.is_basement })
}
