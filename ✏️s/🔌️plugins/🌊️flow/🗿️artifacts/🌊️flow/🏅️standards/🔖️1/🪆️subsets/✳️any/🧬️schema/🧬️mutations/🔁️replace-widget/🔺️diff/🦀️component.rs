//! 🔺️ Sparse `FlowDiff` construction for `replace-widget`.
use crate::artifacts::flow::schema::diff::text::{widgets_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;

use super::mutation::ReplaceWidget;

pub fn diff(payload: &ReplaceWidget, base: &FlowSnapshot) -> FlowDiff {
    let delta = widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Patch { id: payload.id.clone(), patch: payload.widget.clone() });
    FlowDiff { widgets: Some(delta), ..Default::default() }
}
