use crate::change_connection_shear_planes::ChangeConnectionShearPlanes;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionShearPlanes, base: &En1995Snapshot) -> Option<crate::change_connection_shear_planes::ChangeConnectionShearPlanes> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionShearPlanes { connection_id: payload.connection_id.clone(), new_value: item.shear_planes })
}
