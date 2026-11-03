use crate::change_panel_layer_thickness::ChangePanelLayerThickness;
use crate::change_panel_layer_thickness;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelLayerThickness, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_layer_thickness::ChangePanelLayerThickness> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else { return Ok(Vec::new()); };
    let Some(row) = parent.layers.iter().find(|row| row.id == payload.layer_id) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangePanelLayerThickness { panel_id: payload.panel_id.clone(), layer_id: payload.layer_id.clone(), new_value: row.thickness_m })])
}
