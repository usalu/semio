use crate::change_diaphragm_panels::ChangeDiaphragmPanels;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmPanels, base: &En1995Snapshot) -> Option<crate::change_diaphragm_panels::ChangeDiaphragmPanels> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmPanels { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.panels }),
        None => None,
    }
}
