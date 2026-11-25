use crate::change_connection_strength_class::ChangeConnectionStrengthClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeConnectionStrengthClass, base: &En1995Snapshot) -> Option<crate::change_connection_strength_class::ChangeConnectionStrengthClass> {
    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return None; };
    Some(ChangeConnectionStrengthClass { connection_id: payload.connection_id.clone(), new_value: item.strength_class })
}
