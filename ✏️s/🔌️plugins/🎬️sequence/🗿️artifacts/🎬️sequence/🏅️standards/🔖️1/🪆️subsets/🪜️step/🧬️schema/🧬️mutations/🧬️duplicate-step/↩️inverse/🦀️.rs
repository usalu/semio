//! ↩️ Inverse for `DuplicateStep` — always a `delete-step` of the id it created (the payload
//! itself carries the id, so no BASE lookup is needed to know what to undo).
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DuplicateStep, _base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    vec![crate::mutations::delete_step(payload.new_id.clone())]
}
//#endregion 🔖️Inverse
