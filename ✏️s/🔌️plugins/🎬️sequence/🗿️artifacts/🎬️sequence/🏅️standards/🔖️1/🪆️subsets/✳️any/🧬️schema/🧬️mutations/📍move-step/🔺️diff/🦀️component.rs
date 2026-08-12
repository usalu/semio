//! 🔺️ Sparse diff builder for `MoveStep`.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceStepPatchEntry, SequenceStepsDelta};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveStep, _base: &SequenceSnapshot) -> SequenceDiff {
    let patch = SequenceStepPatch { x: Some(payload.x), y: Some(payload.y), ..Default::default() };
    SequenceDiff {
        steps: Some(SequenceStepsDelta { patched: vec![SequenceStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
