use crate::change_fastener_connection::ChangeFastenerConnection;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerConnection, base: &En1995Snapshot) -> Option<crate::change_fastener_connection::ChangeFastenerConnection> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerConnection { fastener_id: payload.fastener_id.clone(), new_value: item.connection_id.clone() }),
        None => None,
    }
}
