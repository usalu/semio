use crate::change_floor_permanent_load::ChangeFloorPermanentLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorPermanentLoad, base: &En1995Snapshot) -> Option<crate::change_floor_permanent_load::ChangeFloorPermanentLoad> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorPermanentLoad { floor_id: payload.floor_id.clone(), new_value: item.permanent_load_pa }),
        None => None,
    }
}
