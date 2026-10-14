//! 🔺️ `delete-widget` sparse diff construction.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta;
use crate::delete_widget::DeleteWidget;
use crate::widget_index;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget by id.
pub fn diff(payload: &DeleteWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(index) = widget_index(&base.host_snapshot, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta::removal(&base.host_snapshot.widgets, index)), ..Default::default() })
}
