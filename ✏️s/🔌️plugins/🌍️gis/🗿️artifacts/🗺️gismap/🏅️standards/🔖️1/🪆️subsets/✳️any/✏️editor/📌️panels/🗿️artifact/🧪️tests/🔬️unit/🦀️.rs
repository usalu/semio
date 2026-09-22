use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_map_layers() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS2D_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains(GIS2D_PLAY_DOCUMENT_LAYERS), "the layer section is this tree's one container: {json}");
    assert!(json.contains("\"raster\""), "a row is keyed by the RAW layer id the `features` domain selects by: {json}");
    drop(json);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_document_tab_to_this_body() {
    let definition = definition();
    assert!(matches!(definition.kind, PanelTabKind::App(ref id) if id == FRAMEWORK_PANEL_TAB_ARTIFACT_ID));
    assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
use crate::editor::gis2d::terminology::gis2d_labels;
use semio_framework_plugin::plugin_app_close_prelude::{BuiltNode, Component};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID, TREE_WINDOW_DEFAULT_ROWS};
use semio_framework_ui_contract::TreeWindow;

/// 🪟️ The layer roster is a compile-time array, so "oversized" here means oversized RELATIVE TO THE
/// VIEWPORT: a host that measured room for `rows` rows must still be told the full extent of all
/// [`GIS_MAP_LAYER_IDS`] and must still receive only what it asked for.
fn viewport(rows: u32, requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, tree_viewport_rows: Some(rows), ..Default::default() }
}

fn request(open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: GIS2D_PLAY_BODY_ARTIFACT.into(), node_key: GIS2D_PLAY_DOCUMENT_LAYERS.into(), open, offset, rows }
}

fn build(view: &ViewModel) -> BuiltNode {
    render(&MapWindowConfig::default(), gis2d_labels(&ViewModel::default()), &TreeWindows::for_body(view, GIS2D_PLAY_BODY_ARTIFACT)).expect("gis2d document tree assembly")
}

fn projection(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("gis2d document tree projection")
}

fn layers_section(tree: &BuiltNode) -> &BuiltNode {
    tree.children.iter().find(|child| child.key.as_str() == GIS2D_PLAY_DOCUMENT_LAYERS).expect("the layer section")
}

fn window_of(node: &BuiltNode) -> TreeWindow {
    match &node.component {
        Component::TreeSection(props) => props.window.expect("a non-empty container stamps its window"),
        _ => panic!("a windowed container is a tree section"),
    }
}

fn row_keys(parent: &BuiltNode) -> Vec<&str> {
    parent.children.iter().map(|row| row.key.as_str()).collect()
}

/// 🪟️ Law (a): a roster past one viewport stamps its FULL extent, materialises no more than the
/// slice it was given, and never invents a `+N` row to stand in for the remainder.
#[semio_framework_async_macros::async_test]
async fn an_oversized_roster_stamps_its_extent_and_materialises_one_viewport() {
    let view = viewport(4, Vec::new());
    let tree = build(&view);
    let section = layers_section(&tree);
    assert_eq!(window_of(section).total as usize, GIS_MAP_LAYER_IDS.len(), "the section announces every layer it has");
    assert_eq!(section.children.len(), 4, "a first paint materialises the measured viewport, not the whole roster");
    assert!(section.children.len() <= TREE_WINDOW_DEFAULT_ROWS as usize);
    let json = projection(build(&view));
    assert!(!json.contains(".more"), "a windowed container never mints a continuation key: {json}");
    assert!(!json.contains("\"+"), "a windowed container never mints a `+N` label: {json}");
}

/// 🪟️ Law (b): a container the user closed is announced, not built.
#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_extent_and_builds_no_child() {
    let view = viewport(48, vec![request(Some(false), 0, 48)]);
    let tree = build(&view);
    let section = layers_section(&tree);
    assert_eq!(window_of(section).total as usize, GIS_MAP_LAYER_IDS.len(), "a closed section still announces its extent");
    assert!(section.children.is_empty(), "a closed section builds no row");
}

/// 🪟️ Law (c): the host names `{offset, rows}` and the guest materialises exactly that half-open
/// range, still keyed by the raw layer id.
#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_own_range() {
    let view = viewport(48, vec![request(None, 4, 3)]);
    let tree = build(&view);
    let section = layers_section(&tree);
    assert_eq!(window_of(section), TreeWindow { row_extent: Default::default(), total: GIS_MAP_LAYER_IDS.len() as u32, offset: 4 });
    let expected: Vec<&str> = GIS_MAP_LAYER_IDS[4..7].iter().map(|(id, _, _)| *id).collect();
    assert_eq!(row_keys(section), expected, "exactly entries [4, 7) keyed by the raw layer id");
}

/// 🪟️ Law (d): a domain-bound tree declares its picks on the rows and binds exactly once at the
/// root — no row carries an activate binding or an argument map of its own.
#[semio_framework_async_macros::async_test]
async fn rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select() {
    let view = viewport(48, vec![request(None, 0, 48)]);
    let tree = build(&view);
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(GIS2D_INTERACTION_DOMAIN));
    let binding = tree.bindings.iter().next().expect("the tree binds the domain select");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), GIS2D_PLAY_APP_ID);
    assert_eq!(tree.bindings.iter().count(), 1, "exactly one tree-level select, never one per row");
    for row in &layers_section(&tree).children {
        assert!(row.bindings.iter().next().is_none(), "a pick row binds no action of its own: {}", row.key.as_str());
        let Component::TreeItem(props) = &row.component else { panic!("tree item") };
        assert_eq!(props.granularity.as_ref().map(|text| text.as_str()), Some(GIS2D_LAYER_GRANULARITY));
    }
    let json = projection(build(&view));
    assert_eq!(json.matches(INTERACTION_SELECT_ACTION_ID).count(), 1, "the select is authored once for the whole tree: {json}");
}
//#endregion 🪟️WindowLaws
