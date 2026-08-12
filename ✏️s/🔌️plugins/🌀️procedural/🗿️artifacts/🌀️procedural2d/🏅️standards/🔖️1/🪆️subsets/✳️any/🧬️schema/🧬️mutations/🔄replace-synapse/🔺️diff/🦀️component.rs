//! 🔺️ Sparse diff for `ReplaceSynapse`, built directly from `(payload, base)`.
use super::mutation::ReplaceSynapse;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, widget_index, Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSynapse, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    let index = synapse_index(&base.fixture, &payload.synapse.id).unwrap_or(base.fixture.synapses.len());
            diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(index, payload.synapse.clone())] }, LayoutDiff::default(), None, None)
}
//#endregion 🔖️Diff
