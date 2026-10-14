//! 🔺️ `create-widget` sparse diff construction — a single added widget row with the
//! requested order, never a snapshot clone.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta;
use crate::create_widget::CreateWidget;
use crate::widget_index;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta for one `create-widget` payload.
pub fn diff(payload: &CreateWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = widget_id(&payload.widget);
    if widget_index(&base.host_snapshot, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    let at = payload.index.min(base.host_snapshot.widgets.len());
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta::insertion(at, payload.widget.clone())), ..Default::default() })
}
