use crate::remove_panel::RemovePanel;
use crate::insert_panel;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemovePanel, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_panel::InsertPanel> {
    Ok(match base.panels.get(payload.index) {
        Some(item) => vec![Op::from(insert_panel::InsertPanel { index: payload.index, panel: item.clone() })],
        None => Vec::new(),
    })
}
