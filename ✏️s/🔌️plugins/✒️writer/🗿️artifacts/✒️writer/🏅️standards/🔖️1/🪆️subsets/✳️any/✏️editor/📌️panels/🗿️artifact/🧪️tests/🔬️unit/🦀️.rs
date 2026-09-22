use super::*;
use crate::editor::writer::unit_tests::context::{app_with_jack, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree_for_jack() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let node = app.render(WRITER_PLAY_BODY_ARTIFACT, Some(&crate::document_dsl::jack_example_json()), &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("render JSON");
    assert!(json.contains("\"type\":\"tree\""));
    assert!(json.contains("Query"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_and_its_children_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.children.len(), 2);
    assert!(definition.children.iter().all(|child| child.body_key.as_deref() == Some(WRITER_PLAY_BODY_ARTIFACT)));
}

/// 🌳️ The AST section only appears for `jack`-language documents (see `render`'s early return for
/// any other language) — load the jack fixture first.
#[semio_framework_async_macros::async_test]
async fn document_lists_the_ast_section_for_jack_documents() {
    let mut app = app_with_jack().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-play-document.ast"));
}

/// 📄️ A non-jack (default/plaintext) document renders the plain id/language fallback section
/// instead of the AST tree.
#[semio_framework_async_macros::async_test]
async fn document_falls_back_to_a_plain_section_for_non_jack_documents() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-document"));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use crate::editor::writer::terminology::writer_play_labels;
use crate::schema::JackAstNode;
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};

const AST_SECTION: &str = "writer-play-document.ast";
const OVERSIZED: usize = 200;

fn ast_node(id: &str, kind: &str, children: Vec<JackAstNode>) -> JackAstNode {
    JackAstNode { id: id.to_string(), kind: kind.to_string(), label: id.to_string(), start: 0, end: 1, children }
}

/// ✒️ A `query` root whose `match` child holds 200 leaves — the AST shape that used to fail the whole
/// render past the fixed child capacity. Built directly rather than parsed so the ids are pinnable.
fn oversized_ast(count: usize) -> JackAstNode {
    let leaves = (0..count).map(|index| ast_node(&format!("leaf-{index:03}"), "pattern", Vec::new())).collect();
    ast_node("root", "query", vec![ast_node("match", "match", leaves)])
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: WRITER_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🔑️ A windowed container nested inside another is addressed by its **window PATH** — the enclosing
/// windowed containers' keys, outermost first, then its own key, joined by
/// `TREE_WINDOW_PATH_SEPARATOR` (`TreeWindows::path_of`). A bare node key only ever matches a
/// top-level container, so a request filed as plain `"match"` silently misses and the level falls
/// back to its author default: the closed law materialised the whole first paint (47 rows) and the
/// window law read offset 0 instead of 30. These laws file the path the host really sends, exactly
/// like `🖨️raster`'s `nested_key` and `📐️cad`'s own nested document requests.
///
/// 🌳️ These two laws call `jack_ast_to_tree_item` DIRECTLY, so the AST root is the outermost
/// windowed container and the path of a level below it is `root␟<id>`.
fn ast_window_path(node_key: &str) -> String {
    format!("root{}{node_key}", semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR)
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
async fn an_oversized_ast_level_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let root = oversized_ast(OVERSIZED);
    let node = super::jack_ast_to_tree_item(&root, &TreeWindows::unhosted()).expect("the AST row builds");
    let matched = window_law_node(&node, "match");
    assert_eq!(window_law_extent(matched), (OVERSIZED as u32, 0));
    assert!(matched.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", matched.children.len());
    window_law_no_continuation(&node);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_ast_level_stamps_its_total_and_builds_no_children() {
    let root = oversized_ast(OVERSIZED);
    let view = window_law_view(vec![window_law_request(&ast_window_path("match"), Some(false), 0, 32)]);
    let node = super::jack_ast_to_tree_item(&root, &TreeWindows::for_body(&view, WRITER_PLAY_BODY_ARTIFACT)).expect("the AST row builds");
    let matched = window_law_node(&node, "match");
    assert_eq!(window_law_extent(matched), (OVERSIZED as u32, 0));
    assert_eq!(matched.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_ast_id() {
    let root = oversized_ast(OVERSIZED);
    let view = window_law_view(vec![window_law_request(&ast_window_path("match"), Some(true), 30, 4)]);
    let node = super::jack_ast_to_tree_item(&root, &TreeWindows::for_body(&view, WRITER_PLAY_BODY_ARTIFACT)).expect("the AST row builds");
    let matched = window_law_node(&node, "match");
    assert_eq!(window_law_extent(matched), (OVERSIZED as u32, 30));
    assert_eq!(window_law_keys(matched), (30..34).map(|index| format!("leaf-{index:03}")).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn ast_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let document = crate::WriterSnapshot { language_id: "jack".into(), document: crate::document_child_handle_with_text("doc", "match Wall\n", "jack"), ..crate::WriterSnapshot::default() };
    let tree = render(&document, writer_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    let ast = window_law_node(&tree, AST_SECTION);
    let row = ast.children.iter().next().expect("an AST root row");
    let Component::TreeItem(props) = &row.component else { panic!("{} is not a tree row", row.key.as_str()) };
    assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("node"));
    assert!(row.bindings.is_empty(), "a pick row must not pay for its own activate binding");
}

//#endregion 🪟️WindowLaws
