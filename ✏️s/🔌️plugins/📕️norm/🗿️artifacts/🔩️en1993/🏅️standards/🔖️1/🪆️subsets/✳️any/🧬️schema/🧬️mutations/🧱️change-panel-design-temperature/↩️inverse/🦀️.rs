//! ↩️ `change-panel-design-temperature` inverse: restores the previous `design_temperature`.

use crate::change_panel_design_temperature::ChangePanelDesignTemperature;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePanelDesignTemperature, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_design_temperature::ChangePanelDesignTemperature> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePanelDesignTemperature { id: payload.id.clone(), new_design_temperature: row.design_temperature })],
        None => Vec::new(),
    })
}
