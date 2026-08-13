//! 🔺️ Sparse diff for `DeleteGeneration`, built directly from `(payload, base)`.
use super::mutation::DeleteGeneration;
use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, Procedural2dDiff, Procedural2dSnapshot};
use crate::artifacts::procedural2d::mutations::{widget_index};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &DeleteGeneration, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }])
}
//#endregion 🔖️Diff
