use crate::remove_plated_panel::RemovePlatedPanel;
use crate::insert_plated_panel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemovePlatedPanel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_plated_panel::InsertPlatedPanel> {
    Ok((|| {
    if payload.index >= base.plated_panels.len() { return Vec::new(); }
    vec![Op::from(insert_plated_panel::InsertPlatedPanel { index: Some(payload.index), plated_panel: base.plated_panels[payload.index].clone() })]

    })())
}
