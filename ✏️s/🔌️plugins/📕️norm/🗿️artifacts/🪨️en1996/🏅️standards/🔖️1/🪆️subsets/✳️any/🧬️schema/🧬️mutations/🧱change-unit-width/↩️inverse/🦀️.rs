//! ↩️ `change-unit-width` inverse: restores the previous `unit_width_m`.

use crate::change_unit_width::ChangeUnitWidth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitWidth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_width::ChangeUnitWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitWidth, base: &En1996Snapshot) -> Option<ChangeUnitWidth> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitWidth { id: payload.id.clone(), new_unit_width_m: r0.unit_width_m })
}
