//! 🔺️ Sparse `FlowDiff` construction for `reorder-widgets` — recomputes the widget id order from
//! `base` directly (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowWidgetsDelta};
use crate::artifacts::flow::FlowSnapshot;
use protocol::Identified;

use super::mutation::ReorderWidgets;

pub fn diff(payload: &ReorderWidgets, base: &FlowSnapshot) -> FlowDiff {
    let mut ids: Vec<String> = base.widgets.iter().map(|widget| widget.id().clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    FlowDiff { widgets: Some(FlowWidgetsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
