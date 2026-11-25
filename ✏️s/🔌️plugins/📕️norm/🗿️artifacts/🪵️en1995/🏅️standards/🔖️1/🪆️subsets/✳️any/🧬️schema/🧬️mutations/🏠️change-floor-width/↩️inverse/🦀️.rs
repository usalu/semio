use crate::change_floor_width::ChangeFloorWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorWidth, base: &En1995Snapshot) -> Option<crate::change_floor_width::ChangeFloorWidth> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorWidth { floor_id: payload.floor_id.clone(), new_value: item.width_m }),
        None => None,
    }
}
