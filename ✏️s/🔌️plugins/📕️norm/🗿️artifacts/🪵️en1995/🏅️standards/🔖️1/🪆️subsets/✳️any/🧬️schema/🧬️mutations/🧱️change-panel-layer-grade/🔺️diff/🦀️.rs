use crate::change_panel_layer_grade::ChangePanelLayerGrade;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PanelDelta, En1995PanelPatch, En1995PanelLayerDelta, En1995PanelLayerPatch};
pub fn diff(payload: &ChangePanelLayerGrade, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(parent) = base.panels.iter().find(|row| row.id == payload.panel_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown panel id.", vec![payload.panel_id.clone()]);
    };
    let Some(row) = parent.layers.iter().find(|row| row.id == payload.layer_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown layer id.", vec![payload.panel_id.clone(), payload.layer_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::modification(&parent.id, En1995PanelPatch { layers: En1995PanelLayerDelta::modification(&row.id, En1995PanelLayerPatch { grade: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
