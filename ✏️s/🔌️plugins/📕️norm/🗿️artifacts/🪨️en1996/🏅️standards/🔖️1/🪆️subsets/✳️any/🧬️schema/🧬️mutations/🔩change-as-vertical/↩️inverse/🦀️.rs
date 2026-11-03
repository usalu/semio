//! ↩️ `change-as-vertical` inverse: restores the previous `as_vertical_m2`.

use crate::change_as_vertical::ChangeAsVertical;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeAsVertical, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_as_vertical::ChangeAsVertical> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAsVertical, base: &En1996Snapshot) -> Option<ChangeAsVertical> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAsVertical { id: payload.id.clone(), new_as_vertical_m2: r0.as_vertical_m2 })
}
