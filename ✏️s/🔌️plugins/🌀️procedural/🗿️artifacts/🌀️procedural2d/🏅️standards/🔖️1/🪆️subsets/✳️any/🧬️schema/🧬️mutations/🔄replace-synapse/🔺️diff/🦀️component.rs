//! 🔺️ Sparse diff for `ReplaceSynapse`, built directly from `(payload, base)`.
use super::mutation::ReplaceSynapse;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::mutations::synapse_index;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSynapse, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    let Some(index) = synapse_index(&base.fixture, &payload.synapse.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.synapse.id), [payload.synapse.id.clone()]);
    };
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(index, payload.synapse.clone())] }, LayoutDiff::default(), None, None))
}
//#endregion 🔖️Diff
