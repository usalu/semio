//! ↩️ Inverse for `DeleteStep` — reconstructs the exact BASE step and edge order through
//! typed mutations. Missing target ⇒ `Vec::new()`.
use crate::mutations::SequenceMutation;
use crate::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStep, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    if !scene.steps.iter().any(|step| step.id == payload.id) {
        return Vec::new();
    }
    let mut mutations = Vec::new();
    for entry in &scene.steps {
        mutations.push(crate::mutations::delete_step(entry.id.clone()));
    }
    for entry in &scene.steps {
        mutations.push(crate::mutations::create_step(entry.clone()));
    }
    for edge in &scene.edges {
        mutations.push(crate::mutations::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone()));
    }
    mutations.reverse();
    mutations
}
//#endregion 🔖️Inverse
