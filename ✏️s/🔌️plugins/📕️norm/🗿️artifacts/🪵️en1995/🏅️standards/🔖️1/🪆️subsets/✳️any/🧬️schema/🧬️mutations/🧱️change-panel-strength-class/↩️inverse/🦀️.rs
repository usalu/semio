use crate::change_panel_strength_class::ChangePanelStrengthClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelStrengthClass, base: &En1995Snapshot) -> Option<crate::change_panel_strength_class::ChangePanelStrengthClass> {
    match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => Some(ChangePanelStrengthClass { panel_id: payload.panel_id.clone(), new_value: item.strength_class }),
        None => None,
    }
}
