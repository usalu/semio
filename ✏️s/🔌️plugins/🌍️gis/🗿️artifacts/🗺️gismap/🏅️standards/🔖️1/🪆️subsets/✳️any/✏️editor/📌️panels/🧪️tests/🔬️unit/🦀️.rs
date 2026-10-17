//! 🧪️ Aggregated unit tests of the `panels` leaves, one module per leaf.

mod artifact {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::{editor::gis2d::GIS2D_INTERACTION_DOMAIN, editor::gis2d::GIS2D_LAYER_GRANULARITY, editor::gis2d::GIS2D_PLAY_APP_ID, editor::gis2d::GIS_MAP_LAYER_IDS};
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use super::super::artifact::*;
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
    ViewModel { tree_windows: requests, tree_viewport_rows: Some(rows), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }
}

fn request(open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: GIS2D_PLAY_BODY_ARTIFACT.into(), node_key: GIS2D_PLAY_DOCUMENT_LAYERS.into(), open, offset, rows }
}

fn build(view: &ViewModel) -> BuiltNode {
    render(&MapWindowConfig::default(), gis2d_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::for_body(view, GIS2D_PLAY_BODY_ARTIFACT)).expect("gis2d document tree assembly")
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
}

mod catalogue {
use super::super::catalogue::*;
use crate::editor::gis2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_layer_toggles() {
    let mut app = app().await;
    assert!(render_body(&mut app, GIS2D_PLAY_BODY_CATALOGUE).await.contains("gis2d-play-catalogue.layer.water"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_catalogue_tab_to_this_body() {
    assert_eq!(definition().body_key.as_deref(), Some(GIS2D_PLAY_BODY_CATALOGUE));
}
}

mod inspection {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::{editor::gis2d::Gis2dInteractionSnapshot, editor::gis2d::GIS2D_FEATURE_GRANULARITY, editor::gis2d::GIS_MAP_LAYER_IDS};
use crate::{GisMapSnapshot, GIS_MAP_SCHEMA};
use semio_framework_plugin::PanelGroup;
use super::super::inspection::*;
use crate::editor::gis2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_schema_and_visible_count() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION).await;
    assert!(json.contains(GIS_MAP_SCHEMA));
    assert!(json.contains(&format!("{}/{}", GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_IDS.len())));
    drop(json);
    close(&mut app);
}

/// 🕹️ The interaction-view threading law for this panel: a `"layer"` pick in the framework-owned
/// `"features"` domain adds that layer's own detail rows, a `"feature"` pick adds the picked
/// feature's id and its document kind, and an empty domain renders the summary alone.
#[semio_framework_async_macros::async_test]
async fn the_inspector_detail_section_follows_the_features_selection() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let cfg = MapWindowConfig::default();
    let labels = crate::editor::gis2d::terminology::gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    // 🧩️ A `BuiltNode` carries retained page children, so it is projected through the fixture
    // transport (the same route `unit_tests::context::render` takes) rather than serialized directly.
    let json = |interaction: &Gis2dInteractionSnapshot| -> String {
        let tree = semio_framework_plugin::built_to_component_tree(render(&document, &cfg, interaction, labels).expect("inspector"));
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("inspector projection")
    };

    let empty = json(&Gis2dInteractionSnapshot::default());
    assert!(!empty.contains("gis2d-play-inspector.layer.id"), "an empty domain must render the summary alone: {empty}");

    let layer = Gis2dInteractionSnapshot { granularity: "layer".into(), ids: vec!["routes".into()], hovered_ids: Vec::new() };
    let layer_json = json(&layer);
    assert!(layer_json.contains("gis2d-play-inspector.layer.id"), "a layer pick must add the layer detail section: {layer_json}");
    assert!(layer_json.contains("routes"), "the layer detail section must name the picked layer: {layer_json}");

    let feature_id = document.positions.first().expect("the default fixture carries a position").id.clone();
    let feature = Gis2dInteractionSnapshot { granularity: GIS2D_FEATURE_GRANULARITY.into(), ids: vec![feature_id.clone()], hovered_ids: Vec::new() };
    let feature_json = json(&feature);
    assert!(feature_json.contains(&feature_id), "a feature pick must name the picked feature: {feature_json}");
    assert!(feature_json.contains("position"), "a feature pick must report the document kind it resolves to: {feature_json}");
}

/// 🪞️ B3a2: the "panel reflects state" half of the interaction bar. Before this, NO gis2d panel
/// projected the document — the artifact tree lists the eleven fixed layer ids and the inspector
/// summarised schema + layer visibility + selection size only, so `addFeature`/`deleteFeature` moved
/// the ledger and the edit count while every panel stayed byte-identical. The summary now carries
/// each collection's own extent, so a per-feature verb is visible in the UI it dispatched from.
#[semio_framework_async_macros::async_test]
async fn the_inspector_summary_projects_each_document_collection_extent() {
    let cfg = MapWindowConfig::default();
    let labels = crate::editor::gis2d::terminology::gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let json = |document: &GisMapSnapshot| -> String {
        let tree = semio_framework_plugin::built_to_component_tree(render(document, &cfg, &Gis2dInteractionSnapshot::default(), labels).expect("inspector"));
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("inspector projection")
    };

    let mut document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let before = json(&document);
    for row in ["gis2d-play-inspector.positions-count", "gis2d-play-inspector.routes-count", "gis2d-play-inspector.regions-count"] {
        assert!(before.contains(row), "the summary must carry {row}: {before}");
    }
    assert!(before.contains(&document.positions.len().to_string()), "the positions row must print the document's own extent: {before}");

    document.positions.push(crate::MapFeature { id: "position-probe".into(), data: semio_framework_value::DslValue::Null });
    let after = json(&document);
    assert_ne!(before, after, "appending one position must move the inspector projection");
    assert!(after.contains(&document.positions.len().to_string()), "the positions row must follow the document: {after}");
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_inspection_tab_to_this_body() {
    let definition = definition();
    assert!(matches!(definition.group, PanelGroup::Details));
    assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_INSPECTION));
}
}
