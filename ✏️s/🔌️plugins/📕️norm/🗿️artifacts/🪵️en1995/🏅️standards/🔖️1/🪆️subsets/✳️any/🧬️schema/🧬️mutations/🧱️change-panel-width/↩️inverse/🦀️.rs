use crate::change_panel_width::ChangePanelWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_width::ChangePanelWidth> {
    Ok(match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => vec![Op::from(ChangePanelWidth { panel_id: payload.panel_id.clone(), new_value: item.width_m })],
        None => Vec::new(),
    })
}
