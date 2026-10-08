//! 🔺️ `create-widget` sparse diff construction — a single added widget row with the
//! requested order, never a snapshot clone.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Generation3dWidgetsDelta};
use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::{widget_id, Generation3dSnapshot};

/// 🏗️ Builds the sparse fixture delta for one `create-widget` payload.
pub fn diff(payload: &CreateWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = widget_id(&payload.widget);
    if widget_index(&base.host_snapshot, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    let reordered = ((payload.index) < base.host_snapshot.widgets.len()).then(|| { let mut order: Vec<String> = base.host_snapshot.widgets.iter().map(|entry| widget_id(entry).to_string()).collect(); order.insert(payload.index, widget_id(&payload.widget).to_string()); order });
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { added: vec![payload.widget.clone()], reordered, ..Default::default() }), ..Default::default() })
}
