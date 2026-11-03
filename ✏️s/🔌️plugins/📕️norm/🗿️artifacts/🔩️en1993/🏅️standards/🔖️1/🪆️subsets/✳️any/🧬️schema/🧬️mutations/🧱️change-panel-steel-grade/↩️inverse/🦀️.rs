//! ↩️ `change-panel-steel-grade` inverse: restores the previous `steel_grade`.

use crate::change_panel_steel_grade::ChangePanelSteelGrade;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePanelSteelGrade, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_steel_grade::ChangePanelSteelGrade> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePanelSteelGrade { id: payload.id.clone(), new_steel_grade: row.steel_grade.clone() })],
        None => Vec::new(),
    })
}
