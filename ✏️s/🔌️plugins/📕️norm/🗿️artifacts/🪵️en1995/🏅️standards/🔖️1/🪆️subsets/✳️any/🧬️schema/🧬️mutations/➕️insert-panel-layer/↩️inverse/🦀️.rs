use crate::insert_panel_layer::InsertPanelLayer;
use crate::remove_panel_layer;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertPanelLayer, base: &En1995Snapshot) -> Option<crate::remove_panel_layer::RemovePanelLayer> {
    Some(remove_panel_layer::RemovePanelLayer { panel_id: payload.panel_id.clone(), id: payload.layer.id.clone() })
}
