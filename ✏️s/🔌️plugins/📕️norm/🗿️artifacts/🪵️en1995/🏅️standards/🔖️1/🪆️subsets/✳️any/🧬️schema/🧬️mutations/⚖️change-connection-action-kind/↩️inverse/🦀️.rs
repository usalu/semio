use crate::change_connection_action_kind::ChangeConnectionActionKind;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionActionKind, base: &En1995Snapshot) -> Option<crate::change_connection_action_kind::ChangeConnectionActionKind> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return None; };
    Some(ChangeConnectionActionKind { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_value: item.kind })
}
