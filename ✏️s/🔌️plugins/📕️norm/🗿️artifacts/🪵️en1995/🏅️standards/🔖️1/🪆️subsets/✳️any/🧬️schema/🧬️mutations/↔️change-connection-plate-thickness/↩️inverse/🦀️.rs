use crate::change_connection_plate_thickness::ChangeConnectionPlateThickness;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionPlateThickness, base: &En1995Snapshot) -> Option<crate::change_connection_plate_thickness::ChangeConnectionPlateThickness> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionPlateThickness { connection_id: payload.connection_id.clone(), new_value: item.steel_plate_thickness_m })
}
