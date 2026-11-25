use crate::change_floor_ei_b::ChangeFloorEiB;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorEiB, base: &En1995Snapshot) -> Option<crate::change_floor_ei_b::ChangeFloorEiB> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorEiB { floor_id: payload.floor_id.clone(), new_value: item.ei_b_n_m2_per_m }),
        None => None,
    }
}
