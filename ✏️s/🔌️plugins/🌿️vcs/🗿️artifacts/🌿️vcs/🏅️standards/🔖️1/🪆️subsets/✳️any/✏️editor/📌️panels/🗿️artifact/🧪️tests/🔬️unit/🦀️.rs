use super::*;
use crate::editor::vcs::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_checkpoints() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("vcs-play-document.checkpoint"));
}

#[semio_framework_async_macros::async_test]
async fn vcs_labels_resolve_native_english_by_default() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("Alternatives"));
    assert!(json.contains("checkpoints"));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, TreeWindows, ViewModel, INTERACTION_SELECT_ACTION_ID};

const CHECKPOINTS_SECTION: &str = "vcs-play-document.checkpoints";
const OVERSIZED: usize = 200;

/// 🌿️ A history deep enough to outgrow one node's fixed child capacity — commit history is the one
/// list in this audit that is unbounded by construction. Columns are newest-last, the panel reverses.
fn oversized_history(count: usize) -> HistoryView {
    let columns = (0..count)
        .map(|index| store::HistoryColumn {
            checkpoint_id: format!("checkpoint-{index:03}"),
            timestamp: format!("2026-09-16T00:00:{:02}Z", index % 60),
            labels: Vec::new(),
            authors: Vec::new(),
            parent_checkpoint_id: None,
            description: Some(format!("Edit {index}")),
            lane: 0,
            alternative_ids: vec!["main".to_string()],
        })
        .collect();
    HistoryView { columns, ..HistoryView::empty() }
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: VCS_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn window_law_node<'a>(root: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    fn walk<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| walk(child, key))
    }
    walk(root, key).unwrap_or_else(|| panic!("no node keyed {key}"))
}

fn window_law_extent(node: &BuiltNode) -> (u32, u32) {
    let window = match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => None,
    };
    let window = window.unwrap_or_else(|| panic!("{} stamps no window", node.key.as_str()));
    (window.total, window.offset)
}

fn window_law_keys(node: &BuiltNode) -> Vec<String> {
    node.children.iter().map(|child| child.key.as_str().to_string()).collect()
}

fn window_law_no_continuation(node: &BuiltNode) {
    assert!(!node.key.as_str().ends_with(".more"), "{} is a continuation row", node.key.as_str());
    if let Component::TreeItem(props) = &node.component {
        assert!(!props.label.0.as_str().starts_with('+'), "{} carries a +N label", node.key.as_str());
    }
    for child in node.children.iter() {
        window_law_no_continuation(child);
    }
}

fn window_law_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn an_oversized_history_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let history = oversized_history(OVERSIZED);
    let tree = render(&history, crate::editor::vcs::terminology::vcs_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    let checkpoints = window_law_node(&tree, CHECKPOINTS_SECTION);
    assert_eq!(window_law_extent(checkpoints), (OVERSIZED as u32, 0));
    assert!(checkpoints.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", checkpoints.children.len());
    window_law_no_continuation(&tree);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let history = oversized_history(OVERSIZED);
    let view = window_law_view(vec![window_law_request(CHECKPOINTS_SECTION, Some(false), 0, 32)]);
    let tree = render(&history, crate::editor::vcs::terminology::vcs_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, VCS_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let checkpoints = window_law_node(&tree, CHECKPOINTS_SECTION);
    assert_eq!(window_law_extent(checkpoints), (OVERSIZED as u32, 0));
    assert_eq!(checkpoints.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_newest_first_slice() {
    let history = oversized_history(OVERSIZED);
    let view = window_law_view(vec![window_law_request(CHECKPOINTS_SECTION, Some(true), 10, 4)]);
    let tree = render(&history, crate::editor::vcs::terminology::vcs_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, VCS_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let checkpoints = window_law_node(&tree, CHECKPOINTS_SECTION);
    assert_eq!(window_law_extent(checkpoints), (OVERSIZED as u32, 10));
    let expected: Vec<String> = (0..4).map(|row| format!("vcs-play-document.checkpoint.checkpoint-{:03}", OVERSIZED - 1 - (10 + row))).collect();
    assert_eq!(window_law_keys(checkpoints), expected);
}

/// 🕹️ vcs is the domain-bound tree whose rows KEEP their own app action (`checkoutCheckpoint` is
/// navigation, not selection), so law (d) reduces to: the tree still carries exactly one
/// `interactionSelect`, and every row's own binding is its checkout action.
#[semio_framework_async_macros::async_test]
async fn the_tree_root_carries_one_interaction_select_and_rows_keep_their_checkout_action() {
    let history = oversized_history(3);
    let tree = render(&history, crate::editor::vcs::terminology::vcs_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    for row in window_law_node(&tree, CHECKPOINTS_SECTION).children.iter() {
        assert_eq!(row.bindings.iter().next().expect("a checkout binding").action.name.as_str(), "checkoutCheckpoint");
    }
}

//#endregion 🪟️WindowLaws
