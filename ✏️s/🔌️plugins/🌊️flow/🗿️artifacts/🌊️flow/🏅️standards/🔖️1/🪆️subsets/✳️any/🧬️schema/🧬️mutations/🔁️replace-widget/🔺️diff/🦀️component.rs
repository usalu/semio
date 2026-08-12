//! 🔺️ Sparse `FlowDiff` construction for `replace-widget` — a real whole-value patch entry
//! construction (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowWidgetPatchEntry, FlowWidgetsDelta};
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::ReplaceWidget;

pub fn diff(payload: &ReplaceWidget, _base: &FlowSnapshot) -> FlowDiff {
    let delta = FlowWidgetsDelta {
        patched: vec![FlowWidgetPatchEntry { id: payload.id.clone(), patch: payload.widget.clone() }],
        ..Default::default()
    };
    FlowDiff { widgets: Some(delta), ..Default::default() }
}
