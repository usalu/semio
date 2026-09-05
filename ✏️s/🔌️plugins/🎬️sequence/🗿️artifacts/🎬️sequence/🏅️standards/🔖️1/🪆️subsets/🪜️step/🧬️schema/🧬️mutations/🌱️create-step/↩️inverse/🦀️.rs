//! ↩️ Inverse for `CreateStep` — always a `delete-step` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateStep, _base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    vec![crate::artifacts::sequence::mutations::delete_step(payload.step.id.clone())]
}
//#endregion 🔖️Inverse
