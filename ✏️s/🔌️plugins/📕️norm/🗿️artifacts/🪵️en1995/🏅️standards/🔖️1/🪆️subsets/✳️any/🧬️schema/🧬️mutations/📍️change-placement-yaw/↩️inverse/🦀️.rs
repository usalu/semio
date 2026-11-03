use crate::change_placement_yaw::ChangePlacementYaw;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePlacementYaw, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_yaw::ChangePlacementYaw> {
    Ok(match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![Op::from(ChangePlacementYaw { id: payload.id.clone(), new_value: item.yaw })],
        None => Vec::new(),
    })
}
