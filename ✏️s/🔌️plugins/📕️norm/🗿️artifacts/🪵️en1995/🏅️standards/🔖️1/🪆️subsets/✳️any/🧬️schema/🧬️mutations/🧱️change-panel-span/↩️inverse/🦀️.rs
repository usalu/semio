use crate::change_panel_span::ChangePanelSpan;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelSpan, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_span::ChangePanelSpan> {
    Ok(match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => vec![Op::from(ChangePanelSpan { panel_id: payload.panel_id.clone(), new_value: item.span_m })],
        None => Vec::new(),
    })
}
