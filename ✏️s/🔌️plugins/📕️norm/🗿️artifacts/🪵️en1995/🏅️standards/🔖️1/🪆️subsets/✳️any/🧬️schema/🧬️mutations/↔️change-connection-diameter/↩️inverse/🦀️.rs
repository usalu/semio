use crate::change_connection_diameter::ChangeConnectionDiameter;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionDiameter, base: &En1995Snapshot) -> Option<crate::change_connection_diameter::ChangeConnectionDiameter> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionDiameter { connection_id: payload.connection_id.clone(), new_value: item.diameter_m })
}
