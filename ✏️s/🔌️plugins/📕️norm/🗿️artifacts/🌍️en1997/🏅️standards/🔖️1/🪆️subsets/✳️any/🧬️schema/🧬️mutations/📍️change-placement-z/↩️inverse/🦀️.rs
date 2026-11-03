//! ↩️ `change-placement-z` inverse: restores the previous `z`.

use crate::change_placement_z::ChangePlacementZ;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementZ, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_z::ChangePlacementZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlacementZ, base: &En1997Snapshot) -> Option<ChangePlacementZ> {
    let r0 = base.placements.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlacementZ { id: payload.id.clone(), new_z: r0.z })
}
