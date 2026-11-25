use crate::change_floor_psi_2::ChangeFloorPsi2;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorPsi2, base: &En1995Snapshot) -> Option<crate::change_floor_psi_2::ChangeFloorPsi2> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorPsi2 { floor_id: payload.floor_id.clone(), new_value: item.psi_2 }),
        None => None,
    }
}
