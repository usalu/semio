use crate::change_panel_load::ChangePanelLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelLoad, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_load::ChangePanelLoad> {
    Ok(match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => vec![Op::from(ChangePanelLoad { panel_id: payload.panel_id.clone(), new_value: item.load_pa })],
        None => Vec::new(),
    })
}
