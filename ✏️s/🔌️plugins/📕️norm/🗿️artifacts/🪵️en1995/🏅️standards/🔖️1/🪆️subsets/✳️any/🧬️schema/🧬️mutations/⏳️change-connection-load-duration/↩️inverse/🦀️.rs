use crate::change_connection_load_duration::ChangeConnectionLoadDuration;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionLoadDuration, base: &En1995Snapshot) -> Option<crate::change_connection_load_duration::ChangeConnectionLoadDuration> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return None; };
    Some(ChangeConnectionLoadDuration { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_value: item.load_duration })
}
