use super::*;
use crate::editor::animate::unit_tests::context::{presentation_app, render as render_body};
use crate::editor::animate::PresentationCommand;

#[semio_framework_async_macros::async_test]
async fn document_lists_seeded_tiles() {
    use semio_framework_plugin::artifact_app_laws::meta;
    let mut app = presentation_app().await;
    crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 })).await;
    let document = render_body(&mut app, PRESENTATION_PLAY_BODY_ARTIFACT).await;
    assert!(document.contains("tile-r0-c0"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use crate::editor::animate::terminology::animate_presentation_labels;
use semio_framework_plugin::{Component, TreeWindowRequest, TreeWindows, ViewModel, INTERACTION_SELECT_ACTION_ID};

const TILES_SECTION: &str = "animate-presentation-play.tiles";
const OVERSIZED: usize = 200;

/// 🎞️ A deck far past one node's fixed child capacity — a real presentation outgrows it easily.
fn oversized_deck(count: usize) -> crate::PresentationSnapshot {
    let tiles: Vec<crate::FigureTileDraft> = (0..count)
        .map(|index| crate::FigureTileDraft { id: format!("tile-{index:03}"), name: format!("Tile {index}"), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } })
        .collect();
    crate::presentation_snapshot_with_tiles(&crate::default_figure_tile_source(), &tiles)
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: PRESENTATION_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
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
async fn an_oversized_deck_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let deck = oversized_deck(OVERSIZED);
    let tree = render(&deck, animate_presentation_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    let tiles = window_law_node(&tree, TILES_SECTION);
    assert_eq!(window_law_extent(tiles), (OVERSIZED as u32, 0));
    assert!(tiles.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", tiles.children.len());
    window_law_no_continuation(&tree);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let deck = oversized_deck(OVERSIZED);
    let view = window_law_view(vec![window_law_request(TILES_SECTION, Some(false), 0, 32)]);
    let tree = render(&deck, animate_presentation_labels(&ViewModel::default()), &TreeWindows::for_body(&view, PRESENTATION_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let tiles = window_law_node(&tree, TILES_SECTION);
    assert_eq!(window_law_extent(tiles), (OVERSIZED as u32, 0));
    assert_eq!(tiles.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_tile_id() {
    let deck = oversized_deck(OVERSIZED);
    let view = window_law_view(vec![window_law_request(TILES_SECTION, Some(true), 40, 6)]);
    let tree = render(&deck, animate_presentation_labels(&ViewModel::default()), &TreeWindows::for_body(&view, PRESENTATION_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let tiles = window_law_node(&tree, TILES_SECTION);
    assert_eq!(window_law_extent(tiles), (OVERSIZED as u32, 40));
    assert_eq!(window_law_keys(tiles), (40..46).map(|index| format!("tile-{index:03}")).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn pick_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let deck = oversized_deck(3);
    let tree = render(&deck, animate_presentation_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    for row in window_law_node(&tree, TILES_SECTION).children.iter() {
        let Component::TreeItem(props) = &row.component else { panic!("{} is not a tree row", row.key.as_str()) };
        assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("tile"));
        assert!(row.bindings.is_empty(), "{} must not pay for its own activate binding", row.key.as_str());
    }
}

//#endregion 🪟️WindowLaws
