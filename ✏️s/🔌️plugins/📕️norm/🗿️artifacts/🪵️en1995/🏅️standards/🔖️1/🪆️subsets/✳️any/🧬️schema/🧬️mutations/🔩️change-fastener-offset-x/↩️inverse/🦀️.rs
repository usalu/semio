use crate::change_fastener_offset_x::ChangeFastenerOffsetX;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerOffsetX, base: &En1995Snapshot) -> Option<crate::change_fastener_offset_x::ChangeFastenerOffsetX> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerOffsetX { fastener_id: payload.fastener_id.clone(), new_value: item.offset_x_m }),
        None => None,
    }
}
