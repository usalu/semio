use crate::change_fastener_end_distance::ChangeFastenerEndDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerEndDistance, base: &En1995Snapshot) -> Option<crate::change_fastener_end_distance::ChangeFastenerEndDistance> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerEndDistance { fastener_id: payload.fastener_id.clone(), new_value: item.end_distance_m }),
        None => None,
    }
}
