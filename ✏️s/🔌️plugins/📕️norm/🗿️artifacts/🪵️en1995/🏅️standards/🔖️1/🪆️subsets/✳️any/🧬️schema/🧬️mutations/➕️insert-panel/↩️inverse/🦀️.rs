use crate::insert_panel::InsertPanel;
use crate::remove_panel;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertPanel, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_panel::RemovePanel> {
    let at = payload.index.min(base.panels.len());
    Ok(vec![Op::from(remove_panel::RemovePanel { index: at })])
}
