//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdatePlatedInputs;
use crate::mutations::remove_plated_panel;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.plated_panels.iter().find(|x| x.id == payload.plated_panel.id) {
        vec![En1993Mutation::UpdatePlatedInputs(UpdatePlatedInputs { plated_panel: prior.clone() })]
    } else {
        vec![En1993Mutation::RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel { index: base.plated_panels.len() })]
    }
}
