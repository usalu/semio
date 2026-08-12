//! 🔺️ Sparse `FlowDiff` construction for `create-widget`.
use crate::artifacts::flow::schema::diff::text::{widgets_delta_from_collection_mutation, FlowDiff};
use crate::artifacts::flow::FlowSnapshot;
use protocol::CollectionMutation;

use super::mutation::CreateWidget;

pub fn diff(payload: &CreateWidget, base: &FlowSnapshot) -> FlowDiff {
    let delta = widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Add { index: payload.index, item: payload.widget.clone() });
    FlowDiff { widgets: Some(delta), ..Default::default() }
}
