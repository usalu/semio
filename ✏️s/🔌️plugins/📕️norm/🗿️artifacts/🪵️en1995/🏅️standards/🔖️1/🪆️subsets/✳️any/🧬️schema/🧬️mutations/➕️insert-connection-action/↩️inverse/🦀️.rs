use crate::insert_connection_action::InsertConnectionAction;
use crate::remove_connection_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertConnectionAction, base: &En1995Snapshot) -> Option<crate::remove_connection_action::RemoveConnectionAction> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    let at = payload.index.min(item.actions.len());
    Some(remove_connection_action::RemoveConnectionAction { connection_id: payload.connection_id.clone(), index: at })
}
