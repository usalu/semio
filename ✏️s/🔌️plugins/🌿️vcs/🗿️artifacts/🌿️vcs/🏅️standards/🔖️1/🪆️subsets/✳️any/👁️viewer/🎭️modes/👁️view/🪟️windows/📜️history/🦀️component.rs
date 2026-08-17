//! 📜️ VCS viewer — the History window: a read-only tree of checkpoints, built from the same
//! `HistoryView` the sibling editor surface's own History window renders as a swimlane graph — this
//! file renders it as a simpler `TreeWindowKit` tree instead (framework `📓️contract-freeze.md` §2.6:
//! this plugin is the one explicitly named for `TreeWindowKit`, "vcs history" being the natural fit for
//! a read-only checkpoint tree). Every checkpoint has exactly one parent
//! (`HistoryColumn.parent_checkpoint_id`), so the checkpoint DAG is genuinely a forest — no cycle/
//! multi-parent handling is needed to turn it into a `TreeView`. This file imports nothing from the
//! sibling editor module (`policyViewerPurityBreaches` forbids it outright).

use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{HistoryView, UiNode, WindowKindDefinition};
use std::collections::HashMap;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::vcs::create_vcs_viewer`.
pub fn definition() -> WindowKindDefinition {
    TreeWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `HistoryView -> UiNode` read: every checkpoint becomes one tree node, nested under its
/// parent (root checkpoints — `parent_checkpoint_id: None` — become tree roots). Alternative names and
/// per-row navigation actions (`checkoutCheckpoint`/`switchAlternative`, real app actions on the
/// editor's document panel) have no read-only counterpart here: a viewer declares no actions.
pub fn render(history: &HistoryView) -> UiNode {
    TreeWindowKit::render(&history_tree_view(history))
}

fn history_tree_view(history: &HistoryView) -> TreeView {
    let mut children_by_parent: HashMap<Option<String>, Vec<&store::HistoryColumn>> = HashMap::new();
    for column in &history.columns {
        children_by_parent.entry(column.parent_checkpoint_id.clone()).or_default().push(column);
    }
    TreeView { roots: history_tree_nodes(&None, &children_by_parent) }
}

fn history_tree_nodes(parent: &Option<String>, children_by_parent: &HashMap<Option<String>, Vec<&store::HistoryColumn>>) -> Vec<TreeNodeView> {
    children_by_parent
        .get(parent)
        .into_iter()
        .flatten()
        .map(|column| TreeNodeView {
            id: column.checkpoint_id.clone(),
            label: column.description.clone().unwrap_or_else(|| column.checkpoint_id.clone()),
            children: history_tree_nodes(&Some(column.checkpoint_id.clone()), children_by_parent),
        })
        .collect()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }

    #[test]
    fn render_nests_checkpoints_under_their_parent() {
        let history = HistoryView {
            columns: vec![
                store::HistoryColumn { checkpoint_id: "c1".into(), timestamp: "t1".into(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: None, description: Some("root".into()), lane: 0, alternative_ids: Vec::new() },
                store::HistoryColumn { checkpoint_id: "c2".into(), timestamp: "t2".into(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: Some("c1".into()), description: Some("child".into()), lane: 0, alternative_ids: Vec::new() },
            ],
            ..HistoryView::empty()
        };
        let view = history_tree_view(&history);
        assert_eq!(view.roots.len(), 1, "exactly one root checkpoint");
        assert_eq!(view.roots[0].id, "c1");
        assert_eq!(view.roots[0].children.len(), 1, "c2 nests under its parent c1");
        assert_eq!(view.roots[0].children[0].id, "c2");
    }
}
//#endregion 🧪️Tests
