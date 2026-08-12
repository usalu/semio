//! 🔺️ Sparse diff builder for `ChangeStepCollapsed`.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceStepPatchEntry, SequenceStepsDelta};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeStepCollapsed, _base: &SequenceSnapshot) -> SequenceDiff {
    let patch = SequenceStepPatch { collapsed: Some(payload.collapsed), ..Default::default() };
    SequenceDiff {
        steps: Some(SequenceStepsDelta { patched: vec![SequenceStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
