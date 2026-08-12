//! 🔺️ Sparse `FlowDiff` construction for `reorder-widgets`.
use crate::artifacts::flow::schema::diff::text::{widgets_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;

use super::mutation::ReorderWidgets;

pub fn diff(payload: &ReorderWidgets, base: &FlowSnapshot) -> FlowDiff {
    let delta = widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index });
    FlowDiff { widgets: Some(delta), ..Default::default() }
}
