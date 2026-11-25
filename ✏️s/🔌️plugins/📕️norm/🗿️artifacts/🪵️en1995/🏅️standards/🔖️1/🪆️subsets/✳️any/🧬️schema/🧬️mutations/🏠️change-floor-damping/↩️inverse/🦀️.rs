use crate::change_floor_damping::ChangeFloorDamping;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorDamping, base: &En1995Snapshot) -> Option<crate::change_floor_damping::ChangeFloorDamping> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorDamping { floor_id: payload.floor_id.clone(), new_value: item.damping_ratio }),
        None => None,
    }
}
