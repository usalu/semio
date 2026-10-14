use crate::insert_plated_panel::InsertPlatedPanel;
use crate::remove_plated_panel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertPlatedPanel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_plated_panel::RemovePlatedPanel> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.plated_panels.len());
    vec![Op::from(remove_plated_panel::RemovePlatedPanel { index: at })]

    })())
}
