use crate::change_fastener_offset_y::ChangeFastenerOffsetY;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerOffsetY, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_offset_y::ChangeFastenerOffsetY> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerOffsetY { fastener_id: payload.fastener_id.clone(), new_value: item.offset_y_m })],
        None => Vec::new(),
    })
}
