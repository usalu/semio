use super::ChangeConnectionActionKind;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionActionKind, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };
    vec![En1995Mutation::ChangeConnectionActionKind(ChangeConnectionActionKind { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_value: item.kind.clone() })]
}
