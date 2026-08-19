//! 🔺️ `create-widget` sparse diff construction — a single `WidgetsDiff.set` entry, never a
//! snapshot clone.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::procedural3d::mutations::widget_index;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};

/// 🏗️ Builds the sparse fixture delta for one `create-widget` payload.
pub async fn diff(payload: &CreateWidget, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let id = widget_id(&payload.widget);
    if widget_index(&base.fixture, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(
        base,
        WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] },
        SynapsesDiff::default(),
        LayoutDiff::default(),
        None,
        None,
    ))
}
