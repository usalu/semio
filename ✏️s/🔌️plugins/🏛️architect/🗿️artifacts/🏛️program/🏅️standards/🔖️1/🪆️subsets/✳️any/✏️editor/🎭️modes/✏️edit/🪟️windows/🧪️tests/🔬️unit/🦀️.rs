//! 🧪️ Aggregated unit tests of the `windows` leaves, one module per leaf.

mod adjacency {
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use super::super::adjacency::*;
use crate::standards::v1::subsets::any::schema::snapshot::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_adjacency_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_ADJACENCY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn the_matrix_renders_a_triangle_strip_with_element_labels() {
    let json = project_render(render(&sample_plugin(), &config::ArchitectAdjacencyWindowConfig::default(), &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains('▲'));
    assert!(json.contains("Reception"));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_renders_the_placeholder() {
    let json = project_render(render(&empty_plugin(), &config::ArchitectAdjacencyWindowConfig::default(), &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("Add program elements"));
}

fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
}
}

mod graph {
use crate::standards::v1::subsets::any::schema::inferences::undirected_edges;
use semio_framework_plugin::NodeGraphScene;
use semio_framework_plugin::SurfaceKind;
use super::super::graph::*;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_GRAPH);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn the_graph_body_emits_a_node_graph_scene() {
    let program = sample_plugin();
    let node = render(&program, &config::ArchitectGraphWindowConfig::default()).expect("graph");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("graph surface") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.nodes.len(), program.elements.len());
    assert_eq!(scene.edges.len(), undirected_edges(&program).len());
    project_render(Ok(node));
}

/// 📏️ The narrowest Graph pane this window is given in the play grid (measured 469 × 907 CSS px on
/// the `semio-tech play` :6033 grid, `getBoundingClientRect` of `window:architect-graph`).
const SMALLEST_GRAPH_PANE_PX: f64 = 469.0;

/// 🖼️ An untouched Graph window carries `ArchitectGraphWindowConfig::default().viewport` — `{x: 0,
/// y: 0, zoom: 1}` — so scene coordinates ARE window coordinates. Every node of a program this
/// window can reasonably show must therefore land inside the narrowest pane the grid gives it, in
/// the positive quadrant. The fixed-radius layout put a two-element program's second node at
/// x = 540 in a 469-px window: the pane rendered one cropped label and read as empty.
#[semio_framework_async_macros::async_test]
async fn the_default_viewport_frames_every_node_without_panning() {
    let program = sample_plugin();
    assert!(program.elements.len() >= 2, "the sample program must exercise the multi-node ring");
    let (nodes, _) = graph_media_json(&program);
    let left = nodes.iter().map(|node| node.x).fold(f64::INFINITY, f64::min);
    let top = nodes.iter().map(|node| node.y).fold(f64::INFINITY, f64::min);
    let right = nodes.iter().map(|node| node.x + node.width).fold(f64::NEG_INFINITY, f64::max);
    let bottom = nodes.iter().map(|node| node.y + node.height).fold(f64::NEG_INFINITY, f64::max);
    // 📐️ The RING is anchored so its own bounding circle starts one margin from the scene origin; the
    // node boxes therefore never reach negative coordinates, and on the axis where the ring's extreme
    // point carries a node (always x, where `cos` attains −1 for an even count) the box bound IS the
    // margin. Asserting equality on BOTH axes would be wrong: two elements sit on a horizontal
    // diameter, so nothing is drawn at the ring's top and the vertical bound is the ring centre less
    // half a node.
    assert!(left >= ARCHITECT_GRAPH_MARGIN - 1e-9, "the node bounding box must stay in the positive quadrant, one margin clear of the scene origin, not at x={left}");
    assert!(top >= ARCHITECT_GRAPH_MARGIN - 1e-9, "the node bounding box must stay in the positive quadrant, one margin clear of the scene origin, not at y={top}");
    assert!(right <= SMALLEST_GRAPH_PANE_PX, "node bounding box runs to x={right}, past the {SMALLEST_GRAPH_PANE_PX}px pane");
    assert!(bottom <= SMALLEST_GRAPH_PANE_PX, "node bounding box runs to y={bottom}, past the {SMALLEST_GRAPH_PANE_PX}px pane");
    if program.elements.len() % 2 == 0 {
        let radius = graph_ring_radius(program.elements.len());
        assert!((left - ARCHITECT_GRAPH_MARGIN).abs() < 1e-9, "an even element count puts a node at the ring's left extreme, so the box bound IS the margin, not {left}");
        assert!((right - (ARCHITECT_GRAPH_MARGIN + 2.0 * radius + ARCHITECT_GRAPH_NODE_WIDTH)).abs() < 1e-9, "the ring's own width plus one node box is the drawn extent, not {right}");
    }
}

/// ↔️ Neighbours on the ring never overlap: the chord between two adjacent node centres clears a
/// whole node box (measured on its diagonal) plus the declared gap, for every element count the ring
/// is used for.
#[semio_framework_async_macros::async_test]
async fn the_ring_radius_keeps_neighbouring_node_boxes_clear() {
    assert_eq!(graph_ring_radius(0), 0.0);
    assert_eq!(graph_ring_radius(1), 0.0);
    let clearance = ARCHITECT_GRAPH_NODE_WIDTH.hypot(ARCHITECT_GRAPH_NODE_HEIGHT) + ARCHITECT_GRAPH_NODE_GAP;
    for count in 2..=32usize {
        let radius = graph_ring_radius(count);
        let chord = 2.0 * radius * (std::f64::consts::PI / count as f64).sin();
        assert!(chord + 1e-9 >= clearance, "{count} nodes land {chord} apart, inside the {clearance} clearance");
        assert!(radius < graph_ring_radius(count + 1), "the ring must grow with the element count");
    }
}

#[semio_framework_async_macros::async_test]
async fn every_element_becomes_a_node_and_every_adjacency_an_edge() {
    let program = sample_plugin();
    let (nodes, edges) = graph_media_json(&program);
    assert_eq!(nodes.len(), program.elements.len());
    assert_eq!(edges.len(), undirected_edges(&program).len());
}

fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
}
}

mod register {
use semio_framework_plugin::BlockListScene;
use semio_framework_plugin::SurfaceKind;
use super::super::register::*;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_block_list_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_REGISTER);
    assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
}

#[semio_framework_async_macros::async_test]
async fn the_active_registers_rows_become_block_list_steps() {
    let node = render(&sample_plugin(), &config::ArchitectRegisterWindowConfig::default()).expect("register");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("register surface") };
    let scene: BlockListScene = semio_framework_ui_scene::decode(props).expect("packed register");
    let steps: serde_json::Value = serde_json::to_value(&scene.steps).expect("independent step oracle");
    assert!(steps.to_string().contains("Reception"));
    project_render(Ok(node));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_register_renders_the_placeholder() {
    let cfg = config::ArchitectRegisterWindowConfig { active_register: "risks".into() };
    let json = project_render(render(&sample_plugin(), &cfg));
    assert!(json.contains("No entities in register 'risks'"), "{json}");
}

fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
}
}

mod report {
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TreeWindows;
use super::super::report::*;
use crate::editor::architect::catalog::report_record_from;
use crate::standards::v1::subsets::any::schema::registers::ReportKind;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;
use crate::standards::v1::subsets::any::schema::inferences::build_report;
use semio_framework_plugin::ViewModel;

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_REPORT);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_selected_authored_record_renders_localized_metadata() {
    let mut program = sample_plugin();
    let report = build_report(&program, ReportKind::ExecutiveSummary);
    let record = report_record_from(&program, ReportKind::ExecutiveSummary, &report);
    let selected_report_id = record.header.id.clone();
    program.reports.push(record);
    let cfg = config::ArchitectReportWindowConfig { selected_report_id: Some(selected_report_id) };
    let json = project_render(render(&program, &cfg, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("Overview"));
    assert!(json.contains("architect-report.section"));
    let de = ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), ..ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
    let json = project_render(render(&program, &cfg, &de, &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("Art:"));
    assert!(json.contains("Erstellt:"));
    assert!(json.contains("Version:"));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_empty_selection_renders_a_localized_prompt() {
    let json = project_render(render(&sample_plugin(), &config::ArchitectReportWindowConfig::default(), &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("Generate a report in this window"));
    let de = ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), ..ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
    let json = project_render(render(&sample_plugin(), &config::ArchitectReportWindowConfig::default(), &de, &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("Erstellen Sie in diesem Fenster einen Bericht"));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_deleted_selection_renders_a_localized_missing_state() {
    let cfg = config::ArchitectReportWindowConfig { selected_report_id: Some(crate::standards::v1::subsets::any::schema::kernel::EntityId("missing-report".into())) };
    let de = ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), ..ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag("de").expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
    let json = project_render(render(&sample_plugin(), &cfg, &de, &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("ist nicht verfügbar"));
}

fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
}
}
