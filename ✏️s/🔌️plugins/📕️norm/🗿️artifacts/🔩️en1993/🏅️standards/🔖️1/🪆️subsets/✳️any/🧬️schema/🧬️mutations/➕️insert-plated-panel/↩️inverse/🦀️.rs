use super::InsertPlatedPanel;
use crate::mutations::{remove_plated_panel, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertPlatedPanel, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.plated_panels.len());
    vec![En1993Mutation::RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel { index: at })]
}
