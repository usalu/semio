use crate::change_diaphragm_panel_width::ChangeDiaphragmPanelWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmPanelWidth, base: &En1995Snapshot) -> Option<crate::change_diaphragm_panel_width::ChangeDiaphragmPanelWidth> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmPanelWidth { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.panel_width_m }),
        None => None,
    }
}
