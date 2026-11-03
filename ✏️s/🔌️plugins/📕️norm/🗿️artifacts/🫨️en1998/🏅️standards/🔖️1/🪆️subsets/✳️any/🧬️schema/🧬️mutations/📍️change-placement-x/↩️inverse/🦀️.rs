//! ↩️ `change-placement-x` inverse: restores the previous `x`.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementX, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_x::ChangePlacementX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlacementX, base: &En1998Snapshot) -> Option<ChangePlacementX> {
    let r0 = base.placements.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlacementX { id: payload.id.clone(), new_x: r0.x })
}
