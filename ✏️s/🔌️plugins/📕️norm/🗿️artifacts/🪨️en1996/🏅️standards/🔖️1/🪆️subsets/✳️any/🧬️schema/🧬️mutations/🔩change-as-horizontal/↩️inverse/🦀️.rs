//! ↩️ `change-as-horizontal` inverse: restores the previous `as_horizontal_m2`.

use crate::change_as_horizontal::ChangeAsHorizontal;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeAsHorizontal, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_as_horizontal::ChangeAsHorizontal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAsHorizontal, base: &En1996Snapshot) -> Option<ChangeAsHorizontal> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAsHorizontal { id: payload.id.clone(), new_as_horizontal_m2: r0.as_horizontal_m2 })
}
