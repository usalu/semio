use crate::change_connection_t2::ChangeConnectionT2;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionT2, base: &En1995Snapshot) -> Option<crate::change_connection_t2::ChangeConnectionT2> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionT2 { connection_id: payload.connection_id.clone(), new_value: item.t2_m })
}
