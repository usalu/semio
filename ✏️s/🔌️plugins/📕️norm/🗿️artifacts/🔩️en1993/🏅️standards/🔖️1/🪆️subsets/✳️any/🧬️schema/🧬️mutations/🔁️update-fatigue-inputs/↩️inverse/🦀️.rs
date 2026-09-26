//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateFatigueInputs;
use crate::mutations::remove_fatigue_detail;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.fatigue_details.iter().find(|x| x.id == payload.fatigue_detail.id) {
        vec![En1993Mutation::UpdateFatigueInputs(UpdateFatigueInputs { fatigue_detail: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail { index: base.fatigue_details.len() })]
    }
}
