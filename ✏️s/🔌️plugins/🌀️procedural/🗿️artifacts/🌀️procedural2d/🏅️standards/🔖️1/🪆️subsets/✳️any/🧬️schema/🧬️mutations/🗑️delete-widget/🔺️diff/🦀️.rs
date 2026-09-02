//! 🔺️ Sparse diff builder for `DeleteWidget` — a real id-keyed removal from the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};

pub fn diff(payload: &super::DeleteWidget, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    if !base.fixture.widgets.iter().any(|widget| widget_id(widget) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let outcome = protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![payload.id.clone()], set: vec![] }, SynapsesDiff::default(), LayoutDiff::default(), None, None));
    let cascaded_synapse_ids: Vec<String> = base.fixture.synapses.iter().filter(|synapse| synapse.from == payload.id || synapse.to == payload.id).map(|synapse| synapse.id.clone()).collect();
    if cascaded_synapse_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting widget \"{}\" leaves {} connected synapse(s) dangling: {}.", payload.id, cascaded_synapse_ids.len(), cascaded_synapse_ids.join(", ")))
    }
}
