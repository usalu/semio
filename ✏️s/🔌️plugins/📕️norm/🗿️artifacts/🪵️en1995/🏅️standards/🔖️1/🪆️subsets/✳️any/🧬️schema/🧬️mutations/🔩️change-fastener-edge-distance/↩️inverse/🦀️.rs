use crate::change_fastener_edge_distance::ChangeFastenerEdgeDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerEdgeDistance, base: &En1995Snapshot) -> Option<crate::change_fastener_edge_distance::ChangeFastenerEdgeDistance> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerEdgeDistance { fastener_id: payload.fastener_id.clone(), new_value: item.edge_distance_m }),
        None => None,
    }
}
