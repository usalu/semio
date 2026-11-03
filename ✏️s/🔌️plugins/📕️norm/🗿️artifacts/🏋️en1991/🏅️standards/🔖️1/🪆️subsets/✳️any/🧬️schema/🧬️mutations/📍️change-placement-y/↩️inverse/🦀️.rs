//! ↩️ `change-placement-y` inverse: restores the previous `y`.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_y::ChangePlacementY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlacementY, base: &En1991Snapshot) -> Option<ChangePlacementY> {
    let r0 = base.placements.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlacementY { id: payload.id.clone(), new_y: r0.y })
}
