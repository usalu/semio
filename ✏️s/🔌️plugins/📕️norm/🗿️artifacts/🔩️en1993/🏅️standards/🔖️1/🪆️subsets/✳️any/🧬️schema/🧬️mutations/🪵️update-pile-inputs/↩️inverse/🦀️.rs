//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_pile_inputs::UpdatePileInputs;
use crate::remove_pile;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdatePileInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_pile_inputs::UpdatePileInputs> + From<crate::remove_pile::RemovePile> {
    Ok((|| {
    if let Some(prior) = base.piles.iter().find(|x| x.id == payload.pile.id) {
        vec![Op::from(UpdatePileInputs { pile: prior.clone() })]
    } else {
        vec![Op::from(remove_pile::RemovePile { index: base.piles.len() })]
    }

    })())
}
