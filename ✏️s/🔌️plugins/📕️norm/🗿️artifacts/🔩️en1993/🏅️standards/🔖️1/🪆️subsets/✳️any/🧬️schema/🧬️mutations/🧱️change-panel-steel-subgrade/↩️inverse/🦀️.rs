//! ↩️ `change-panel-steel-subgrade` inverse: restores the previous `steel_subgrade`.

use crate::change_panel_steel_subgrade::ChangePanelSteelSubgrade;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePanelSteelSubgrade, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_steel_subgrade::ChangePanelSteelSubgrade> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePanelSteelSubgrade { id: payload.id.clone(), new_steel_subgrade: row.steel_subgrade.clone() })],
        None => Vec::new(),
    })
}
