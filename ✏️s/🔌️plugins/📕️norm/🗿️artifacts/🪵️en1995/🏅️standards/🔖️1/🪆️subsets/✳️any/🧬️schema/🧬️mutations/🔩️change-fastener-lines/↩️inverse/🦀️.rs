use crate::change_fastener_lines::ChangeFastenerLines;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerLines, base: &En1995Snapshot) -> Option<crate::change_fastener_lines::ChangeFastenerLines> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerLines { fastener_id: payload.fastener_id.clone(), new_value: item.lines }),
        None => None,
    }
}
