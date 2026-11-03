//! ↩️ `change-unit-group` inverse: restores the previous `unit_group`.

use crate::change_unit_group::ChangeUnitGroup;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitGroup, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_group::ChangeUnitGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitGroup, base: &En1996Snapshot) -> Option<ChangeUnitGroup> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitGroup { id: payload.id.clone(), new_unit_group: r0.unit_group.clone() })
}
