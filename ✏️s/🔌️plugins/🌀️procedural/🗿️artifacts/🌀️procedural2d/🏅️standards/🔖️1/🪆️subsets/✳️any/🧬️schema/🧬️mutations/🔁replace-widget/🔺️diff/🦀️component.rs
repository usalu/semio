//! 🔺️ Sparse diff for `ReplaceWidget`, built directly from `(payload, base)`.
use super::mutation::ReplaceWidget;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, widget_index, Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceWidget, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    let index = widget_index(&base.fixture, widget_id(&payload.widget)).unwrap_or(base.fixture.widgets.len());
            diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None)
}
//#endregion 🔖️Diff
