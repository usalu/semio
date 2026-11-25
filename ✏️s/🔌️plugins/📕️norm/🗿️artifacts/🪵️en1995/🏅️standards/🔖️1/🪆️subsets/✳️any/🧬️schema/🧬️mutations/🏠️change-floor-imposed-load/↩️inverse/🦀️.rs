use crate::change_floor_imposed_load::ChangeFloorImposedLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorImposedLoad, base: &En1995Snapshot) -> Option<crate::change_floor_imposed_load::ChangeFloorImposedLoad> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorImposedLoad { floor_id: payload.floor_id.clone(), new_value: item.imposed_load_pa }),
        None => None,
    }
}
