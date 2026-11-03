//! ↩️ `change-unit-height` inverse: restores the previous `unit_height_m`.

use crate::change_unit_height::ChangeUnitHeight;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_height::ChangeUnitHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitHeight, base: &En1996Snapshot) -> Option<ChangeUnitHeight> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitHeight { id: payload.id.clone(), new_unit_height_m: r0.unit_height_m })
}
