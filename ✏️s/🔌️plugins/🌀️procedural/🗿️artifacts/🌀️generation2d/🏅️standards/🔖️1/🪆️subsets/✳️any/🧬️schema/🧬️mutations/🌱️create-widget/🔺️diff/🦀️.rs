//! 🔺️ Sparse diff builder for `CreateWidget` — a real id-keyed upsert into the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dWidgetsDelta};
use crate::{widget_id, Generation2dSnapshot};

pub fn diff(payload: &super::CreateWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let id = widget_id(&payload.widget);
    if base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    let reordered = ((payload.index) < base.host_snapshot.widgets.len()).then(|| { let mut order: Vec<String> = base.host_snapshot.widgets.iter().map(|entry| widget_id(entry).to_string()).collect(); order.insert(payload.index, id.to_string()); order });
    protocol::MutationOutcome::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { added: vec![payload.widget.clone()], reordered, ..Default::default() }), ..Default::default() })
}
