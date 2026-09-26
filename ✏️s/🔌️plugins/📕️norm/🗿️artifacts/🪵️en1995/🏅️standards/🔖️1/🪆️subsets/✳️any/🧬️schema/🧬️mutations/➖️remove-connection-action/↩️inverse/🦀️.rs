use super::RemoveConnectionAction;
use crate::mutations::{insert_connection_action, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &RemoveConnectionAction, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    if payload.index >= item.actions.len() { return Vec::new(); }
    vec![En1995Mutation::InsertConnectionAction(insert_connection_action::InsertConnectionAction { connection_id: payload.connection_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]
}
