use crate::change_panel_layer_orientation::ChangePanelLayerOrientation;
use crate::change_panel_layer_orientation;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePanelLayerOrientation, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_panel_layer_orientation::ChangePanelLayerOrientation> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else { return Ok(Vec::new()); };
    let Some(row) = parent.layers.iter().find(|row| row.id == payload.layer_id) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangePanelLayerOrientation { panel_id: payload.panel_id.clone(), layer_id: payload.layer_id.clone(), new_value: row.orientation })])
}
