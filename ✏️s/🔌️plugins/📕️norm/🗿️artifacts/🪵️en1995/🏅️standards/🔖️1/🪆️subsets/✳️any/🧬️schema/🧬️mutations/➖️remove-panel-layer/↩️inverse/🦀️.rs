use crate::remove_panel_layer::RemovePanelLayer;
use crate::insert_panel_layer;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemovePanelLayer, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_panel_layer::InsertPanelLayer> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else { return Ok(Vec::new()); };
    let Some(at) = parent.layers.iter().position(|row| row.id == payload.id) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(insert_panel_layer::InsertPanelLayer { panel_id: payload.panel_id.clone(), index: at, layer: parent.layers[at].clone() })])
}
