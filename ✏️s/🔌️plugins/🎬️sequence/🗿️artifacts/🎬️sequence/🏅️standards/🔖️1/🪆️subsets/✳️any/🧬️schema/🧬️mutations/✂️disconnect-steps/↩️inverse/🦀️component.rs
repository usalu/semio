//! ↩️ Inverse for `DisconnectSteps` — reconstructs a `connect-steps` at the exact captured
//! (id, from, to) BASE showed. Missing target ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectSteps, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    match base.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![crate::artifacts::sequence::mutations::connect_steps::mutation::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
