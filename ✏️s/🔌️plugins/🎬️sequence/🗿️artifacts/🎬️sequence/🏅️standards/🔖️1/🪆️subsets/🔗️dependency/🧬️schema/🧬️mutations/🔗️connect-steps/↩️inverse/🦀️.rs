//! ↩️ Inverse for `ConnectSteps` — always a `disconnect-steps` of the edge id it created.
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectSteps, _base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    vec![crate::mutations::disconnect_steps(payload.id.clone())]
}
//#endregion 🔖️Inverse
