use super::RemovePlatedPanel;
use crate::mutations::{insert_plated_panel, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemovePlatedPanel, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.plated_panels.len() { return Vec::new(); }
    vec![En1993Mutation::InsertPlatedPanel(insert_plated_panel::InsertPlatedPanel { index: payload.index, plated_panel: base.plated_panels[payload.index].clone() })]
}
