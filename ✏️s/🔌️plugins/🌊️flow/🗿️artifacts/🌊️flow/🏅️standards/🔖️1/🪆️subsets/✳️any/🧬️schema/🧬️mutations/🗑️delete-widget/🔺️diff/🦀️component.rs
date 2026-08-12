//! 🔺️ Sparse `FlowDiff` construction for `delete-widget`. Cascades into severed synapses and the
//! widget's layout entry (taxonomy `delete` — "captures cascade").
use crate::artifacts::flow::schema::diff::text::{widgets_delta_from_collection_mutation, FlowDiff, FlowLayoutMapDelta, FlowSynapsesDelta};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;
use std::collections::BTreeMap;

use super::mutation::DeleteWidget;

pub fn diff(payload: &DeleteWidget, base: &FlowSnapshot) -> FlowDiff {
    let widgets_delta = widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Remove { id: payload.id.clone() });

    let severed: Vec<String> = base.synapses.iter().filter(|synapse| synapse.from == payload.id || synapse.to == payload.id).map(|synapse| synapse.id.clone()).collect();
    let synapses = if severed.is_empty() { None } else { Some(FlowSynapsesDelta { removed: severed, ..Default::default() }) };

    let layout = if base.layout.contains_key(&payload.id) {
        let mut entries: BTreeMap<String, Option<flow::WidgetLayout>> = BTreeMap::new();
        entries.insert(payload.id.clone(), None);
        Some(FlowLayoutMapDelta { entries })
    } else {
        None
    };

    FlowDiff { widgets: Some(widgets_delta), synapses, layout, ..Default::default() }
}
