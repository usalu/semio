//! 🔺️ Sparse diff builder for `CreateStep` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceStepsDelta};
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateStep, _base: &SequenceSnapshot) -> SequenceDiff {
    SequenceDiff { steps: Some(SequenceStepsDelta { added: vec![payload.step.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
