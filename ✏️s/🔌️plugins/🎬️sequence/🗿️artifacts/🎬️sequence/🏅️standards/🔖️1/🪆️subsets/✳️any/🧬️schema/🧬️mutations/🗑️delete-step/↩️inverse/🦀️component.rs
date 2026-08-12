//! ↩️ Inverse for `DeleteStep` — reconstructs a `create-step` of the captured BASE step, then
//! re-`connect-steps`s every edge BASE shows touching it (severed cascade). Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteStep, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.id) else {
        return Vec::new();
    };
    let mut mutations = vec![crate::artifacts::sequence::mutations::create_step::mutation::create_step(step.clone())];
    for edge in base.edges.iter().filter(|edge| edge.from == payload.id || edge.to == payload.id) {
        mutations.push(crate::artifacts::sequence::mutations::connect_steps::mutation::connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone()));
    }
    mutations
}
//#endregion 🔖️Inverse
