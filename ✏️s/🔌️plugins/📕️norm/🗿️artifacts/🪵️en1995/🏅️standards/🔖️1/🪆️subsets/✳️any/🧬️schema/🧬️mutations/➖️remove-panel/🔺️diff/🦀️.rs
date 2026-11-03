use crate::remove_panel::RemovePanel;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PanelDelta;
pub fn diff(payload: &RemovePanel, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.panels.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Panel index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::removal(&base.panels, payload.index), ..Default::default() })
}
