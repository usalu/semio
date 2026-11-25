use crate::insert_panel::InsertPanel;
use crate::remove_panel;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertPanel, base: &En1995Snapshot) -> Option<crate::remove_panel::RemovePanel> {
    let at = payload.index.min(base.panels.len());
    Some(remove_panel::RemovePanel { index: at })
}
