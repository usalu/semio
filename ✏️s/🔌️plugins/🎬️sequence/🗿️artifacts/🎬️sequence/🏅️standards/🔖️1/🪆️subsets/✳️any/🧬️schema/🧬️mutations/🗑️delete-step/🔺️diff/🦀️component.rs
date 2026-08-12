//! 🔺️ Sparse diff builder for `DeleteStep` — a real cascade-aware removal (step + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceEdgesDelta, SequenceStepsDelta};
use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteStep, base: &SequenceSnapshot) -> SequenceDiff {
    let severed: Vec<String> = base.edges.iter().filter(|edge| edge.from == payload.id || edge.to == payload.id).map(|edge| edge.id.clone()).collect();
    SequenceDiff {
        steps: Some(SequenceStepsDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        edges: if severed.is_empty() { None } else { Some(SequenceEdgesDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    }
}
//#endregion 🔖️Diff
