//! ↩️ Inverse for `DisconnectSteps` — reconstructs a `connect-steps` at the exact captured
//! (id, from, to) BASE showed. Missing target ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DisconnectSteps, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    match scene.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![crate::artifacts::sequence::mutations::connect_steps::mutation::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
