use crate::change_floor_joist_spacing::ChangeFloorJoistSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorJoistSpacing, base: &En1995Snapshot) -> Option<crate::change_floor_joist_spacing::ChangeFloorJoistSpacing> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorJoistSpacing { floor_id: payload.floor_id.clone(), new_value: item.joist_spacing_m }),
        None => None,
    }
}
