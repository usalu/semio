//! ↩️ `change-unit-length` inverse: restores the previous `unit_length_m`.

use crate::change_unit_length::ChangeUnitLength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_length::ChangeUnitLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitLength, base: &En1996Snapshot) -> Option<ChangeUnitLength> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitLength { id: payload.id.clone(), new_unit_length_m: r0.unit_length_m })
}
