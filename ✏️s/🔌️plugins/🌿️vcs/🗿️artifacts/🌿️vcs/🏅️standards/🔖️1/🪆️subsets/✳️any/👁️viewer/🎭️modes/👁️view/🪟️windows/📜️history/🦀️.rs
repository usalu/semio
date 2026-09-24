//! 📜️ VCS viewer — the History window: a read-only tree of checkpoints, built from the same
//! `HistoryView` the sibling editor surface's own History window renders as a swimlane graph — this
//! file renders it as a simpler `TreeWindowKit` tree instead (framework `📓️contract-freeze.md` §2.6:
//! this plugin is the one explicitly named for `TreeWindowKit`, "vcs history" being the natural fit for
//! a read-only checkpoint tree). Every checkpoint has exactly one parent
//! (`HistoryColumn.parent_checkpoint_id`), so the checkpoint DAG is genuinely a forest — no cycle/
//! multi-parent handling is needed to turn it into a `TreeView`. This file imports nothing from the
//! sibling editor module (`policyViewerPurityBreaches` forbids it outright).

use crate::VcsSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, HistoryView, UiAssemblyResult, WindowKindDefinition};
use std::collections::HashMap;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
/// 🆔️ The document row's key. A tree row needs a real, non-empty key that no checkpoint id spells:
/// an empty id falls back to its positional `#0` key, which no `TreeWindowRequest` can name.
pub const VCS_DOCUMENT_NODE_ID: &str = "$";
/// ➖️ The document row's label while the document has no title — the same language-neutral mark the
/// editor's summary prints for empty notes.
const VCS_UNTITLED_LABEL: &str = "—";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::vcs::create_vcs_viewer`.
pub fn definition() -> WindowKindDefinition {
    TreeWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `(VcsSnapshot, HistoryView) -> BuiltNode` read: the document itself is the one root row,
/// and every checkpoint becomes one tree node nested under its parent (root checkpoints —
/// `parent_checkpoint_id: None` — nest directly under the document). A tree of checkpoints alone
/// painted NOTHING for a document that has none yet — every freshly opened viewer (S15, session 11)
/// — because `TreeWindowKit` renders no rows for an empty roster. Alternative names and per-row
/// navigation actions (`checkoutCheckpoint`/`switchAlternative`, real app actions on the editor's
/// document panel) have no read-only counterpart here: a viewer declares no actions.
pub fn render(document: &VcsSnapshot, history: &HistoryView) -> UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render(&history_tree_view(document, history))
}

fn history_tree_view(document: &VcsSnapshot, history: &HistoryView) -> TreeView {
    let mut children_by_parent: HashMap<Option<String>, Vec<&store::HistoryColumn>> = HashMap::new();
    for column in &history.columns {
        children_by_parent.entry(column.parent_checkpoint_id.clone()).or_default().push(column);
    }
    let label = if document.title.is_empty() { VCS_UNTITLED_LABEL.to_string() } else { document.title.clone() };
    TreeView { roots: vec![TreeNodeView { id: VCS_DOCUMENT_NODE_ID.to_string(), label, children: history_tree_nodes(&None, &children_by_parent) }] }
}

fn history_tree_nodes(parent: &Option<String>, children_by_parent: &HashMap<Option<String>, Vec<&store::HistoryColumn>>) -> Vec<TreeNodeView> {
    children_by_parent
        .get(parent)
        .into_iter()
        .flatten()
        .map(|column| TreeNodeView { id: column.checkpoint_id.clone(), label: column.description.clone().unwrap_or_else(|| column.checkpoint_id.clone()), children: history_tree_nodes(&Some(column.checkpoint_id.clone()), children_by_parent) })
        .collect()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
