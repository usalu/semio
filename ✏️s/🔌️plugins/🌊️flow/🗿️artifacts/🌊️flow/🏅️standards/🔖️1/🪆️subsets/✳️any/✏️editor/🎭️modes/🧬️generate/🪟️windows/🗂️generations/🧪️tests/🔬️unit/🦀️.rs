use super::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_empty_generation_list_still_offers_the_add_action() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the generations window body is a
// `PanelTreeBuilder` tree outside `📌️panels`, so wave 2 missed it: its list was a `UiFixedList` that
// HARD-FAULTED (`ui.generations` / `items`) once a playbook run produced more than
// `UI_FIXED_LIST_ITEMS` generations. It is a windowed section now, and these laws pin that.
use semio_framework_plugin::{Component, TreeWindowRequest, ViewModel};

const GENERATIONS_SECTION: &str = "flow-play-generate.generations";
const OVERSIZED: usize = 200;

fn oversized_generations(count: usize) -> FlowWindowTransient {
    let generations = (0..count).map(|index| crate::playbook::FormGeneration { id: format!("gen-{index:03}"), name: format!("Generation {index}"), values: Default::default() }).collect();
    let state = crate::playbook::GenerationPlayState { generations, selected_generation_id: None, preview_text: None };
    FlowWindowTransient { generation_json: serde_json::to_string(&state).expect("the generation state serialises"), duplicate_widget_progress_json: String::new() }
}

fn generations_extent(node: &BuiltNode) -> (u32, u32) {
    let window = match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => None,
    };
    let window = window.unwrap_or_else(|| panic!("{} stamps no window", node.key.as_str()));
    (window.total, window.offset)
}

fn generations_node<'a>(root: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    fn walk<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| walk(child, key))
    }
    walk(root, key).unwrap_or_else(|| panic!("no node keyed {key}"))
}

fn generations_no_continuation(node: &BuiltNode) {
    assert!(!node.key.as_str().ends_with(".more"), "{} is a continuation row", node.key.as_str());
    if let Component::TreeItem(props) = &node.component {
        assert!(!props.label.0.as_str().starts_with('+'), "{} carries a +N label", node.key.as_str());
    }
    for child in node.children.iter() {
        generations_no_continuation(child);
    }
}

fn generations_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

#[test]
fn an_oversized_generation_list_stamps_its_whole_total_and_materialises_at_most_its_slice() {
    let transient = oversized_generations(OVERSIZED);
    let tree = render(&transient, Locale::En, Terminology::Native, &TreeWindows::unhosted()).expect("the generations tree builds");
    let section = generations_node(&tree, GENERATIONS_SECTION);
    let (total, offset) = generations_extent(section);
    assert_eq!(total as usize, OVERSIZED, "the section reports every generation");
    assert_eq!(offset, 0, "a cold render starts at the first row");
    assert!(section.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", section.children.len());
    generations_no_continuation(&tree);
}

#[test]
fn a_closed_generation_list_stamps_its_total_and_builds_no_children() {
    let transient = oversized_generations(OVERSIZED);
    let view = generations_view(vec![TreeWindowRequest { body_key: FLOW_PLAY_BODY_GENERATIONS.into(), node_key: GENERATIONS_SECTION.into(), open: Some(false), offset: 0, rows: 32 }]);
    let tree = render(&transient, Locale::En, Terminology::Native, &TreeWindows::for_body(&view, FLOW_PLAY_BODY_GENERATIONS)).expect("the generations tree builds");
    let section = generations_node(&tree, GENERATIONS_SECTION);
    assert_eq!(generations_extent(section), (OVERSIZED as u32, 0));
    assert_eq!(section.children.len(), 0, "a closed container materialises nothing");
}

#[test]
fn a_generation_window_request_materialises_exactly_its_slice_keyed_by_the_generation_id() {
    let transient = oversized_generations(OVERSIZED);
    let view = generations_view(vec![TreeWindowRequest { body_key: FLOW_PLAY_BODY_GENERATIONS.into(), node_key: GENERATIONS_SECTION.into(), open: Some(true), offset: 120, rows: 6 }]);
    let tree = render(&transient, Locale::En, Terminology::Native, &TreeWindows::for_body(&view, FLOW_PLAY_BODY_GENERATIONS)).expect("the generations tree builds");
    let section = generations_node(&tree, GENERATIONS_SECTION);
    assert_eq!(generations_extent(section), (OVERSIZED as u32, 120));
    let keys: Vec<String> = section.children.iter().map(|child| child.key.as_str().to_string()).collect();
    let expected: Vec<String> = (120..126).map(|index| format!("flow-play-generate.generation.gen-{index:03}")).collect();
    assert_eq!(keys, expected);
}
//#endregion 🪟️WindowLaws
