use super::*;
use crate::editor::sequence::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_ARTIFACT).await.contains("sequence-play-document.steps"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(SEQUENCE_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use crate::editor::sequence::terminology::sequence_play_labels;
use crate::{SequenceHostSnapshot, SequenceStep, SlotRef};
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};

const STEPS_SECTION: &str = "sequence-play-document.steps";
const OVERSIZED: usize = 200;

fn step(id: String, kind: &str, slot: Option<SlotRef>) -> SequenceStep {
    SequenceStep { id, kind: kind.to_string(), params: Default::default(), x: 0.0, y: 0.0, slot, collapsed: false }
}

/// 🎬️ `count` root steps, the first of which is a `control.if` nesting `count` steps in its `then`
/// slot, each of those a `control.while` nesting `count` more in its `body` — three slot levels, every
/// one of them past the fixed child capacity.
fn oversized_sequence_document(count: usize) -> SequenceHostSnapshot {
    let mut steps = vec![step("root-if".to_string(), "control.if", None)];
    for index in 1..count {
        steps.push(step(format!("root-{index:03}"), "log.print", None));
    }
    for index in 0..count {
        let inner = format!("then-{index:03}");
        steps.push(step(inner.clone(), "control.while", Some(SlotRef { owner: "root-if".to_string(), name: "then".to_string() })));
        if index == 0 {
            for deep in 0..count {
                steps.push(step(format!("body-{deep:03}"), "log.print", Some(SlotRef { owner: inner.clone(), name: "body".to_string() })));
            }
        }
    }
    SequenceHostSnapshot { schema: SequenceHostSnapshot::default().schema, steps, edges: Vec::new() }
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: SEQUENCE_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
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
async fn an_oversized_document_stamps_the_full_total_at_every_slot_level() {
    let live = oversized_sequence_document(OVERSIZED);
    let view = window_law_view(vec![
        window_law_request(STEPS_SECTION, Some(true), 0, 8),
        window_law_request("sequence-play-document.slot.root-if.then", Some(true), 0, 4),
        window_law_request("sequence-play-document.slot.then-000.body", Some(true), 0, 3),
    ]);
    let tree = render(&live, sequence_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, SEQUENCE_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let steps = window_law_node(&tree, STEPS_SECTION);
    assert_eq!(window_law_extent(steps), (OVERSIZED as u32, 0));
    assert_eq!(steps.children.len(), 8);
    let then_slot = window_law_node(&tree, "sequence-play-document.slot.root-if.then");
    assert_eq!(window_law_extent(then_slot), (OVERSIZED as u32, 0));
    assert_eq!(then_slot.children.len(), 4);
    let body_slot = window_law_node(&tree, "sequence-play-document.slot.then-000.body");
    assert_eq!(window_law_extent(body_slot), (OVERSIZED as u32, 0));
    assert_eq!(body_slot.children.len(), 3);
    window_law_no_continuation(&tree);
}

/// 🎛️ A control step's own children stay the closed set `[collapse toggle, …declared slots]`, so the
/// toggle is never squeezed out by a window.
#[semio_framework_async_macros::async_test]
async fn a_control_step_keeps_its_collapse_toggle_beside_its_slot_rows() {
    let live = oversized_sequence_document(4);
    let tree = render(&live, sequence_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    let control = window_law_node(&tree, "root-if");
    assert_eq!(window_law_keys(&control.children.iter().next().map(|_| control).expect("the control row has children")), window_law_keys(control));
    assert_eq!(window_law_keys(control), vec!["sequence-play-document.collapse.root-if".to_string(), "sequence-play-document.slot.root-if.then".to_string(), "sequence-play-document.slot.root-if.else".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_slot_stamps_its_total_and_builds_no_children() {
    let live = oversized_sequence_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request("sequence-play-document.slot.root-if.then", Some(false), 0, 32)]);
    let tree = render(&live, sequence_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, SEQUENCE_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let then_slot = window_law_node(&tree, "sequence-play-document.slot.root-if.then");
    assert_eq!(window_law_extent(then_slot), (OVERSIZED as u32, 0));
    assert_eq!(then_slot.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_step_id() {
    let live = oversized_sequence_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request(STEPS_SECTION, Some(true), 50, 5)]);
    let tree = render(&live, sequence_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, SEQUENCE_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let steps = window_law_node(&tree, STEPS_SECTION);
    assert_eq!(window_law_extent(steps), (OVERSIZED as u32, 50));
    assert_eq!(window_law_keys(steps), (50..55).map(|index| format!("root-{index:03}")).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn pick_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let live = oversized_sequence_document(4);
    let tree = render(&live, sequence_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    let leaf = window_law_node(&tree, "root-001");
    let Component::TreeItem(props) = &leaf.component else { panic!("root-001 is not a tree row") };
    assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("step"));
    assert!(leaf.bindings.is_empty(), "a pick row must not pay for its own activate binding");
}

//#endregion 🪟️WindowLaws
