//! 🔺️ Sparse diff for `ReplaceWidget`, built directly from `(payload, base)`.
use super::mutation::ReplaceWidget;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::mutations::widget_index;
use crate::artifacts::procedural2d::{widget_id, Procedural2dDiff, Procedural2dSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceWidget, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    let id = widget_id(&payload.widget);
    let Some(index) = widget_index(&base.fixture, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{id}\" does not exist."), [id.to_string()]);
    };
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
//#endregion 🔖️Diff
