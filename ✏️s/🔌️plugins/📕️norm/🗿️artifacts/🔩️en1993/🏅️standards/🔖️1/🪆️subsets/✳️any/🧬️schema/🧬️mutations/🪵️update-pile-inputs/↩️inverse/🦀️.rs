//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdatePileInputs;
use crate::mutations::remove_pile;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdatePileInputs, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if let Some(prior) = base.piles.iter().find(|x| x.id == payload.pile.id) {
        vec![En1993Mutation::UpdatePileInputs(UpdatePileInputs { pile: prior.clone() })]
    } else {
        vec![En1993Mutation::RemovePile(remove_pile::RemovePile { index: base.piles.len() })]
    }

    })())
}
