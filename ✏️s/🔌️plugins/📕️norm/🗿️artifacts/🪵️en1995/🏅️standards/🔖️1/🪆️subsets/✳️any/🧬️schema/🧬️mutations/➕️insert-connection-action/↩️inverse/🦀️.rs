use crate::insert_connection_action::InsertConnectionAction;
use crate::remove_connection_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertConnectionAction, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_connection_action::RemoveConnectionAction> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    let at = payload.index.min(item.actions.len());
    vec![Op::from(remove_connection_action::RemoveConnectionAction { connection_id: payload.connection_id.clone(), index: at })]

    })())
}
