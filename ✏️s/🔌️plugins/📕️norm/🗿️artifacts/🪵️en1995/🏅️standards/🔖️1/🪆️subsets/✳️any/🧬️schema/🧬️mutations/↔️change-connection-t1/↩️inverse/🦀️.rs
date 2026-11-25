use crate::change_connection_t1::ChangeConnectionT1;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionT1, base: &En1995Snapshot) -> Option<crate::change_connection_t1::ChangeConnectionT1> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionT1 { connection_id: payload.connection_id.clone(), new_value: item.t1_m })
}
