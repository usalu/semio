use crate::change_fastener_edge_distance::ChangeFastenerEdgeDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerEdgeDistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_edge_distance::ChangeFastenerEdgeDistance> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerEdgeDistance { fastener_id: payload.fastener_id.clone(), new_value: item.edge_distance_m })],
        None => Vec::new(),
    })
}
