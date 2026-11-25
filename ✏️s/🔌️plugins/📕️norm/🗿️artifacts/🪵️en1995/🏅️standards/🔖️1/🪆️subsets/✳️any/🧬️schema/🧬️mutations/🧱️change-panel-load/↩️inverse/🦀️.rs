use crate::change_panel_load::ChangePanelLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelLoad, base: &En1995Snapshot) -> Option<crate::change_panel_load::ChangePanelLoad> {
    match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => Some(ChangePanelLoad { panel_id: payload.panel_id.clone(), new_value: item.load_pa }),
        None => None,
    }
}
