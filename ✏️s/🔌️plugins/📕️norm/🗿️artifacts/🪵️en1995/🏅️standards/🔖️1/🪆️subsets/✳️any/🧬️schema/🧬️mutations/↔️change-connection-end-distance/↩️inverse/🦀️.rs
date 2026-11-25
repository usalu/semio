use crate::change_connection_end_distance::ChangeConnectionEndDistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionEndDistance, base: &En1995Snapshot) -> Option<crate::change_connection_end_distance::ChangeConnectionEndDistance> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionEndDistance { connection_id: payload.connection_id.clone(), new_value: item.end_distance_m })
}
