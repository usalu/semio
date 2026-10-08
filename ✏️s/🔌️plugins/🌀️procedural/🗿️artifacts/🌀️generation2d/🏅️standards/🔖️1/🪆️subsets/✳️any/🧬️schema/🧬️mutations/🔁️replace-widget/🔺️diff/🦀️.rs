//! 🔺️ Sparse diff for `ReplaceWidget`, built directly from `(payload, base)`.
use super::ReplaceWidget;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dWidgetPatch, Generation2dWidgetModification, Generation2dWidgetsDelta};
use crate::{widget_id, Generation2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let id = widget_id(&payload.widget);
    if !base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{id}\" does not exist."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { modified: vec![Generation2dWidgetModification { id: id.to_string(), patch: Generation2dWidgetPatch::Replace { widget: payload.widget.clone() } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
