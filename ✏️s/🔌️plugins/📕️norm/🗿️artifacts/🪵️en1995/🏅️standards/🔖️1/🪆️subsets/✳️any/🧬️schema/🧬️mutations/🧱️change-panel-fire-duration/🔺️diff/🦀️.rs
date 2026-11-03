use crate::change_panel_fire_duration::ChangePanelFireDuration;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PanelDelta, En1995PanelPatch};
pub fn diff(payload: &ChangePanelFireDuration, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.panels.iter().find(|item| item.id == payload.panel_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown panel id.", vec![payload.panel_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::modification(&row.id, En1995PanelPatch { fire_duration_s: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
