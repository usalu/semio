use crate::insert_panel::InsertPanel;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PanelDelta;
pub fn diff(payload: &InsertPanel, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.panels.iter().any(|existing| existing.id == payload.panel.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Panel id {} already exists.", payload.panel.id), [payload.panel.id.clone()]);
    }
    let at = payload.index.min(base.panels.len());
    protocol::MutationOutcome::new(En1995Diff { panels: En1995PanelDelta::insertion(at, payload.panel.clone()), ..Default::default() })
}
