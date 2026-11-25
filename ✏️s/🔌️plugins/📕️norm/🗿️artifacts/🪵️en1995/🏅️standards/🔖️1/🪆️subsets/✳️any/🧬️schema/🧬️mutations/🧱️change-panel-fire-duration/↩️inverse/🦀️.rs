use crate::change_panel_fire_duration::ChangePanelFireDuration;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelFireDuration, base: &En1995Snapshot) -> Option<crate::change_panel_fire_duration::ChangePanelFireDuration> {
    match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => Some(ChangePanelFireDuration { panel_id: payload.panel_id.clone(), new_value: item.fire_duration_s }),
        None => None,
    }
}
