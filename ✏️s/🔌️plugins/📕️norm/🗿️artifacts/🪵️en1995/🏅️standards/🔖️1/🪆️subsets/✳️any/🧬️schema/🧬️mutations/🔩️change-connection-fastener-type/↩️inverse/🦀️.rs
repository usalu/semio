use crate::change_connection_fastener_type::ChangeConnectionFastenerType;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionFastenerType, base: &En1995Snapshot) -> Option<crate::change_connection_fastener_type::ChangeConnectionFastenerType> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionFastenerType { connection_id: payload.connection_id.clone(), new_value: item.fastener_type })
}
