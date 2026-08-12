//! 🔺️ Sparse `FlowDiff` construction for `create-widget` — a real append-only insert (never a
//! whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowWidgetsDelta};
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::CreateWidget;

pub fn diff(payload: &CreateWidget, _base: &FlowSnapshot) -> FlowDiff {
    FlowDiff { widgets: Some(FlowWidgetsDelta { added: vec![payload.widget.clone()], ..Default::default() }), ..Default::default() }
}
