use crate::change_panel_strength_class::ChangePanelStrengthClass;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PanelDelta, En1995PanelPatch};
pub fn diff(payload: &ChangePanelStrengthClass, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.panels.iter().find(|item| item.id == payload.panel_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown panel id.", vec![payload.panel_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::modification(&row.id, En1995PanelPatch { strength_class: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
