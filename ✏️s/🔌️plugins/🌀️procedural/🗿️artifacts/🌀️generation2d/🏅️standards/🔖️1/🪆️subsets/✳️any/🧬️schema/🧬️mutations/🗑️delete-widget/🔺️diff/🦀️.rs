//! 🔺️ Sparse diff builder for `DeleteWidget` — a real id-keyed removal from the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dWidgetsDelta};
use crate::{widget_id, Generation2dSnapshot};

pub fn diff(payload: &super::DeleteWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(index) = base.host_snapshot.widgets.iter().position(|widget| widget_id(widget) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let outcome = protocol::MutationOutcome::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::removal(&base.host_snapshot.widgets, index)), ..Default::default() });
    let cascaded_synapse_ids: Vec<String> = base.host_snapshot.synapses.iter().filter(|synapse| synapse.from == payload.id || synapse.to == payload.id).map(|synapse| synapse.id.clone()).collect();
    if cascaded_synapse_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting widget \"{}\" leaves {} connected synapse(s) dangling: {}.", payload.id, cascaded_synapse_ids.len(), cascaded_synapse_ids.join(", ")))
    }
}
