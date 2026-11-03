//! ↩️ `change-placement-x` inverse: restores the previous `x`.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_x::ChangePlacementX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlacementX, base: &En1991Snapshot) -> Option<ChangePlacementX> {
    let r0 = base.placements.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlacementX { id: payload.id.clone(), new_x: r0.x })
}
