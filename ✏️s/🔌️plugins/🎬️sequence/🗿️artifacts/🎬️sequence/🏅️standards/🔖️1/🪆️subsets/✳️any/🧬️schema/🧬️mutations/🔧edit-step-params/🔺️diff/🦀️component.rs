//! 🔺️ Sparse diff builder for `EditStepParams`.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceStepPatchEntry, SequenceStepsDelta};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditStepParams, _base: &SequenceSnapshot) -> SequenceDiff {
    let patch = SequenceStepPatch { params: Some(payload.params.clone()), ..Default::default() };
    SequenceDiff {
        steps: Some(SequenceStepsDelta { patched: vec![SequenceStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
