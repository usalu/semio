//! 🔺️ Sparse diff builder for `ConnectSteps`.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceEdgesDelta};
use crate::artifacts::sequence::{SequenceEdge, SequenceSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectSteps, _base: &SequenceSnapshot) -> SequenceDiff {
    let edge = SequenceEdge { id: payload.id.clone(), from: payload.from.clone(), to: payload.to.clone() };
    SequenceDiff { edges: Some(SequenceEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
