use super::InsertPlatedPanel;
use crate::mutations::{remove_plated_panel, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertPlatedPanel, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.plated_panels.len());
    vec![En1993Mutation::RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel { index: at })]

    })())
}
