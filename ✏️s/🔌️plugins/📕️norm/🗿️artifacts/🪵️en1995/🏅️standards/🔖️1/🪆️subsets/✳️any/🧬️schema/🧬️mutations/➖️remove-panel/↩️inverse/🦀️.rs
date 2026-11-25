use crate::remove_panel::RemovePanel;
use crate::insert_panel;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemovePanel, base: &En1995Snapshot) -> Option<crate::insert_panel::InsertPanel> {
    match base.panels.get(payload.index) {
        Some(item) => Some(insert_panel::InsertPanel { index: payload.index, panel: item.clone() }),
        None => None,
    }
}
