use crate::change_panel_layer_orientation::ChangePanelLayerOrientation;
use crate::change_panel_layer_orientation;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePanelLayerOrientation, base: &En1995Snapshot) -> Option<crate::change_panel_layer_orientation::ChangePanelLayerOrientation> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else { return None; };
    let Some(row) = parent.layers.iter().find(|row| row.id == payload.layer_id) else { return None; };
    Some(ChangePanelLayerOrientation { panel_id: payload.panel_id.clone(), layer_id: payload.layer_id.clone(), new_value: row.orientation })
}
