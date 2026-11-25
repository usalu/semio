use crate::change_diaphragm_fastener_spacing::ChangeDiaphragmFastenerSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmFastenerSpacing, base: &En1995Snapshot) -> Option<crate::change_diaphragm_fastener_spacing::ChangeDiaphragmFastenerSpacing> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmFastenerSpacing { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.fastener_spacing_m }),
        None => None,
    }
}
