//! ↩️ `change-built-up-panel-spacing` inverse: restores the previous `panel_spacing`.

use crate::change_built_up_panel_spacing::ChangeBuiltUpPanelSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpPanelSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_panel_spacing::ChangeBuiltUpPanelSpacing> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpPanelSpacing { id: payload.id.clone(), new_panel_spacing: row.panel_spacing })],
        None => Vec::new(),
    })
}
