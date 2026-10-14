use crate::change_connection_edge_distance::ChangeConnectionEdgeDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeConnectionEdgeDistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connection_edge_distance::ChangeConnectionEdgeDistance> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![Op::from(ChangeConnectionEdgeDistance { connection_id: payload.connection_id.clone(), new_value: item.edge_distance_m })]

    })())
}
