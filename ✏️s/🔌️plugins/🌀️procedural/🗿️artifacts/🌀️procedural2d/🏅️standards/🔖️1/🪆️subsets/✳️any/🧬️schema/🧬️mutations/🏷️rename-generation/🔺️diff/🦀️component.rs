//! 🔺️ Sparse diff for `RenameGeneration`, built directly from `(payload, base)`.
use super::mutation::RenameGeneration;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, widget_index, Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &RenameGeneration, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: payload.id.clone(), name: payload.name.clone() }])
}
//#endregion 🔖️Diff
