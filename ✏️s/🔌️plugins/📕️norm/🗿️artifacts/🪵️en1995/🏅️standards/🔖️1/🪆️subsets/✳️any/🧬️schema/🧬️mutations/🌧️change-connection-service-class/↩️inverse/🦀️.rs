use crate::change_connection_service_class::ChangeConnectionServiceClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionServiceClass, base: &En1995Snapshot) -> Option<crate::change_connection_service_class::ChangeConnectionServiceClass> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionServiceClass { connection_id: payload.connection_id.clone(), new_value: item.service_class })
}
