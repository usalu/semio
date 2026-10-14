//! 🔺️ Sparse diff builder for `CreateWidget` — a real id-keyed upsert into the fixture's widget
//! collection helper (never a whole-snapshot capture).

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::diff::Generation2dWidgetsDelta;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn diff(payload: &super::CreateWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let id = widget_id(&payload.widget);
    if base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    let at = payload.index.min(base.host_snapshot.widgets.len());
    protocol::MutationOutcome::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::insertion(at, payload.widget.clone())), ..Default::default() })
}
