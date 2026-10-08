//! 🔺️ `delete-widget` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Generation3dWidgetsDelta};
use crate::standards::v1::subsets::any::schema::mutations::delete_widget::DeleteWidget;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget by id.
pub fn diff(payload: &DeleteWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if widget_index(&base.host_snapshot, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
