//! ↩️ Inverse for `DeleteStep` — reconstructs the exact BASE step and edge order through
//! typed mutations. Missing target ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteStep, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    if !scene.steps.iter().any(|step| step.id == payload.id) {
        return Vec::new();
    }
    let mut mutations = Vec::new();
    for entry in &scene.steps {
        mutations.push(crate::artifacts::sequence::mutations::delete_step::mutation::delete_step(entry.id.clone()));
    }
    for entry in &scene.steps {
        mutations.push(crate::artifacts::sequence::mutations::create_step::mutation::create_step(entry.clone()));
    }
    for edge in &scene.edges {
        mutations.push(crate::artifacts::sequence::mutations::connect_steps::mutation::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone()));
    }
    mutations.reverse();
    mutations
}
//#endregion 🔖️Inverse
