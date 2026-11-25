use crate::change_connection_steel_plate::ChangeConnectionSteelPlate;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionSteelPlate, base: &En1995Snapshot) -> Option<crate::change_connection_steel_plate::ChangeConnectionSteelPlate> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionSteelPlate { connection_id: payload.connection_id.clone(), new_value: item.steel_plate })
}
