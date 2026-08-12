//! ↩️ Inverse for `ConnectSteps` — always a `disconnect-steps` of the edge id it created.
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectSteps, _base: &SequenceSnapshot) -> Vec<SequenceMutation> {
    vec![crate::artifacts::sequence::mutations::disconnect_steps::mutation::disconnect_steps(payload.id.clone())]
}
//#endregion 🔖️Inverse
