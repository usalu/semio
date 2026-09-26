use super::ChangeConnectionEdgeDistance;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionEdgeDistance, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![En1995Mutation::ChangeConnectionEdgeDistance(ChangeConnectionEdgeDistance { connection_id: payload.connection_id.clone(), new_value: item.edge_distance_m })]
}
