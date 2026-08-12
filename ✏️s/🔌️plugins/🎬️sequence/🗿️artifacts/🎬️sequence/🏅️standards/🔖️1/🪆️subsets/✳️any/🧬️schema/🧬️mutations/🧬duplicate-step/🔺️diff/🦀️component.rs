//! 🔺️ Sparse diff builder for `DuplicateStep` — a real copy-from-BASE insert (never a
//! whole-snapshot capture). Missing source ⇒ empty diff.
use crate::artifacts::sequence::diff::{SequenceDiff, SequenceStepsDelta};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DuplicateStep, base: &SequenceSnapshot) -> SequenceDiff {
    let Some(source) = base.steps.iter().find(|step| step.id == payload.source_id) else {
        return SequenceDiff::default();
    };
    let copy = SequenceStep { id: payload.new_id.clone(), kind: source.kind.clone(), params: source.params.clone(), x: payload.x, y: payload.y, slot: None, collapsed: source.collapsed };
    SequenceDiff { steps: Some(SequenceStepsDelta { added: vec![copy], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
