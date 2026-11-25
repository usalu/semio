use crate::change_fastener_load_angle::ChangeFastenerLoadAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerLoadAngle, base: &En1995Snapshot) -> Option<crate::change_fastener_load_angle::ChangeFastenerLoadAngle> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerLoadAngle { fastener_id: payload.fastener_id.clone(), new_value: item.load_angle_rad }),
        None => None,
    }
}
