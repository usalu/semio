use crate::change_connection_number::ChangeConnectionNumber;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionNumber, base: &En1995Snapshot) -> Option<crate::change_connection_number::ChangeConnectionNumber> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionNumber { connection_id: payload.connection_id.clone(), new_value: item.number })
}
