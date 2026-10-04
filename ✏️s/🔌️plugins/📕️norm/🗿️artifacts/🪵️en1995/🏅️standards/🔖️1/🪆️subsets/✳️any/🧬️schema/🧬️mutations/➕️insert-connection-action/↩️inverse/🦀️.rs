use super::InsertConnectionAction;
use crate::mutations::{remove_connection_action, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &InsertConnectionAction, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    let at = payload.index.min(item.actions.len());
    vec![En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction { connection_id: payload.connection_id.clone(), index: at })]

    })())
}
