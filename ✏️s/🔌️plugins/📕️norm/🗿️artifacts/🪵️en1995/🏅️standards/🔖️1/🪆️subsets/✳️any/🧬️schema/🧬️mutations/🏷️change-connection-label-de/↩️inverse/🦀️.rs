use crate::change_connection_label_de::ChangeConnectionLabelDe;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionLabelDe, base: &En1995Snapshot) -> Option<crate::change_connection_label_de::ChangeConnectionLabelDe> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionLabelDe { connection_id: payload.connection_id.clone(), new_value: item.label_de.clone() })
}
