use crate::change_panel_span::ChangePanelSpan;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelSpan, base: &En1995Snapshot) -> Option<crate::change_panel_span::ChangePanelSpan> {
    match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => Some(ChangePanelSpan { panel_id: payload.panel_id.clone(), new_value: item.span_m }),
        None => None,
    }
}
