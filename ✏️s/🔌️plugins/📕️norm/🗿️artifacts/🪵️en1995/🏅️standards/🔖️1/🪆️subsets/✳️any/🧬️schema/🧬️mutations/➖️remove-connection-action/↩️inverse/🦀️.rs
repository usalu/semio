use crate::remove_connection_action::RemoveConnectionAction;
use crate::insert_connection_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveConnectionAction, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_connection_action::InsertConnectionAction> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    if payload.index >= item.actions.len() { return Vec::new(); }
    vec![Op::from(insert_connection_action::InsertConnectionAction { connection_id: payload.connection_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]

    })())
}
