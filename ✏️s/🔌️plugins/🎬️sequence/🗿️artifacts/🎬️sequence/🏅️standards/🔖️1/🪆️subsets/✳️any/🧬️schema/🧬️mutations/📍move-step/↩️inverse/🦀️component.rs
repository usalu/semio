//! ↩️ Inverse for `MoveStep` — the OLD `(x, y)` looked up from BASE. Missing target ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::MoveStep, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    match base.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![super::mutation::move_step(payload.id.clone(), step.x, step.y)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
