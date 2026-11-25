use crate::change_connection_fuk::ChangeConnectionFUK;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionFUK, base: &En1995Snapshot) -> Option<crate::change_connection_fuk::ChangeConnectionFUK> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionFUK { connection_id: payload.connection_id.clone(), new_value: item.f_u_k })
}
