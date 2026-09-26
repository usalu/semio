use super::ChangeConnectionFUK;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionFUK, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![En1995Mutation::ChangeConnectionFUK(ChangeConnectionFUK { connection_id: payload.connection_id.clone(), new_value: item.f_u_k })]
}
