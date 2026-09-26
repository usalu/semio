use super::ChangeConnectionSteelPlate;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionSteelPlate, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };
    vec![En1995Mutation::ChangeConnectionSteelPlate(ChangeConnectionSteelPlate { connection_id: payload.connection_id.clone(), new_value: item.steel_plate })]
}
