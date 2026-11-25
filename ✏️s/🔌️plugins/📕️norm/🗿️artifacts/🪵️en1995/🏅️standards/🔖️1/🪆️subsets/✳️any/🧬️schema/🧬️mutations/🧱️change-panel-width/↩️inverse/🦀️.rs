use crate::change_panel_width::ChangePanelWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelWidth, base: &En1995Snapshot) -> Option<crate::change_panel_width::ChangePanelWidth> {
    match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => Some(ChangePanelWidth { panel_id: payload.panel_id.clone(), new_value: item.width_m }),
        None => None,
    }
}
