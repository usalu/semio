use crate::insert_panel_layer::InsertPanelLayer;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PanelDelta, En1995PanelPatch, En1995PanelLayerDelta};
pub fn diff(payload: &InsertPanelLayer, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown panel id.", vec![payload.panel_id.clone()]);
    };
    if parent.layers.iter().any(|row| row.id == payload.layer.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("PanelLayer id {} already exists.", payload.layer.id), [payload.layer.id.clone()]);
    }
    let at = payload.index.min(parent.layers.len());
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::modification(&parent.id, En1995PanelPatch { layers: En1995PanelLayerDelta::insertion(at, payload.layer.clone()), ..Default::default() }), ..Default::default() })
}
