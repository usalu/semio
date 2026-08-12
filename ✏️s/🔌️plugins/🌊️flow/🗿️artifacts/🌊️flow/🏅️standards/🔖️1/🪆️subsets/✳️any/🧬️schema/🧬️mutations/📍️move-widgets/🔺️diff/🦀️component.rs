//! 🔺️ Sparse `FlowDiff` construction for `move-widgets`.
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowLayoutMapDelta};
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::MoveWidgets;

pub fn diff(payload: &MoveWidgets, _base: &FlowSnapshot) -> FlowDiff {
    let entries = payload.entries.iter().map(|entry| (entry.id.clone(), entry.layout.clone())).collect();
    FlowDiff { layout: Some(FlowLayoutMapDelta { entries }), ..Default::default() }
}
