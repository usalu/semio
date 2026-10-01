//! ↩️ Inverse for `DisconnectSteps` — reconstructs a `connect-steps` at the exact captured
//! (id, from, to) BASE showed. Missing target ⇒ `Vec::new()`.
use crate::mutations::SequenceMutation;
use crate::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DisconnectSteps, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    match scene.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![crate::mutations::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
