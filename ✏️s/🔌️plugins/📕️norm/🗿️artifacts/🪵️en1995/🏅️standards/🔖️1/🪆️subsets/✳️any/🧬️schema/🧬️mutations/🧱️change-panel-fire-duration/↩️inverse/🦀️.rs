use crate::change_panel_fire_duration::ChangePanelFireDuration;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelFireDuration, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_fire_duration::ChangePanelFireDuration> {
    Ok(match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => vec![Op::from(ChangePanelFireDuration { panel_id: payload.panel_id.clone(), new_value: item.fire_duration_s })],
        None => Vec::new(),
    })
}
