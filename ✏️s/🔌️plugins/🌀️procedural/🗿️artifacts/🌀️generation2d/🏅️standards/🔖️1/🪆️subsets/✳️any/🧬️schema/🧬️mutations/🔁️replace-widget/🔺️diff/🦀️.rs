//! 🔺️ Sparse diff for `ReplaceWidget`, built directly from `(payload, base)`.
use super::ReplaceWidget;
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::diff::Generation2dWidgetPatch;
use semio_s_artifact_procedural_2d_model::diff::Generation2dWidgetModification;
use semio_s_artifact_procedural_2d_model::diff::Generation2dWidgetsDelta;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let id = widget_id(&payload.widget);
    if !base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{id}\" does not exist."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { modified: vec![Generation2dWidgetModification { id: id.to_string(), patch: Generation2dWidgetPatch::Replace { widget: payload.widget.clone() } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
