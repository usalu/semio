use crate::change_fastener_offset_y::ChangeFastenerOffsetY;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerOffsetY, base: &En1995Snapshot) -> Option<crate::change_fastener_offset_y::ChangeFastenerOffsetY> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerOffsetY { fastener_id: payload.fastener_id.clone(), new_value: item.offset_y_m }),
        None => None,
    }
}
