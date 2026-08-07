//! 🔺️ Sequence artifact — the operation diff (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::sequence::{SequenceEdge, SequenceEdgePatch, SequenceFixture, SequenceStep, SequenceStepPatch};
use protocol::{CollectionDiff, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: protocol::Identified<TId> + Clone + protocol::Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖️Collections

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceDiff {
    pub steps: Option<CollectionDiff<String, SequenceStepPatch, SequenceStep>>,
    pub edges: Option<CollectionDiff<String, SequenceEdgePatch, SequenceEdge>>,
}

impl OperationDiff<SequenceFixture> for SequenceDiff {
    fn apply(&self, projection: &SequenceFixture) -> SequenceFixture {
        let mut next = projection.clone();
        if let Some(diff) = &self.steps {
            apply_collection_diff(&mut next.steps, diff);
        }
        if let Some(diff) = &self.edges {
            apply_collection_diff(&mut next.edges, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        absorb_collection_diff(&mut self.steps, other.steps);
        absorb_collection_diff(&mut self.edges, other.edges);
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::op::SequenceOperation;
    use crate::artifacts::sequence::{default_fixture, StepParams};
    use protocol::Operation;

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries
    /// only the touched slot.
    #[test]
    fn steps_add_diff_applies_onto_the_base_projection() {
        let base = default_fixture();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let operation = SequenceOperation::StepsAdd { index: 2, item: step };
        let diff: SequenceDiff = operation.diff(&base);
        assert!(diff.steps.is_some(), "StepsAdd must produce a steps diff: {diff:?}");
        assert!(diff.edges.is_none(), "StepsAdd must touch only the steps slot: {diff:?}");
        assert_eq!(diff.apply(&base).steps.len(), base.steps.len() + 1);
    }
}
//#endregion 🧪️Tests
