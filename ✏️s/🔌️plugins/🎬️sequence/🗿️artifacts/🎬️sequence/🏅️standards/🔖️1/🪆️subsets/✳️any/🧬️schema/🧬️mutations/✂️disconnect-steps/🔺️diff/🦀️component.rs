//! 🔺️ Sparse diff builder for `DisconnectSteps`.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceEdgesDelta};
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectSteps, _base: &SequenceSnapshot) -> SequenceDiff {
    SequenceDiff { edges: Some(SequenceEdgesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
