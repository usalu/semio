use crate::change_panel_strength_class::ChangePanelStrengthClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelStrengthClass, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_strength_class::ChangePanelStrengthClass> {
    Ok(match base.panels.iter().find(|item| item.id == payload.panel_id) {
        Some(item) => vec![Op::from(ChangePanelStrengthClass { panel_id: payload.panel_id.clone(), new_value: item.strength_class })],
        None => Vec::new(),
    })
}
