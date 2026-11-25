use crate::change_connection_rows::ChangeConnectionRows;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionRows, base: &En1995Snapshot) -> Option<crate::change_connection_rows::ChangeConnectionRows> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionRows { connection_id: payload.connection_id.clone(), new_value: item.rows })
}
