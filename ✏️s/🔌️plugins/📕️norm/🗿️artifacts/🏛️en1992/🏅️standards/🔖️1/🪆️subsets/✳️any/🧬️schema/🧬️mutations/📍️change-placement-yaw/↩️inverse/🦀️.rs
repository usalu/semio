//! ↩️ `change-placement-yaw` inverse: restores the previous `yaw`.

use crate::change_placement_yaw::ChangePlacementYaw;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementYaw, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_yaw::ChangePlacementYaw> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlacementYaw, base: &En1992Snapshot) -> Option<ChangePlacementYaw> {
    let r0 = base.placements.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlacementYaw { id: payload.id.clone(), new_yaw: r0.yaw })
}
