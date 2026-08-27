//! ↩️ Inverse for `MoveStep` — the OLD `(x, y)` looked up from BASE. Missing target ⇒ `Vec::new()`.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{sequence_working_scene, SequenceSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MoveStep, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let scene = sequence_working_scene(base);
    match scene.steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![super::move_step(payload.id.clone(), step.x, step.y)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
