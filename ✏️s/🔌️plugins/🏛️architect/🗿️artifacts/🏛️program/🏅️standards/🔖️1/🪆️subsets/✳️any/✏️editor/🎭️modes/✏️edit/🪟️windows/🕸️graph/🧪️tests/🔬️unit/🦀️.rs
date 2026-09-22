use super::*;
use crate::sample_plugin;

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
    crate::editor::architect::unit_tests::context::project_render(Ok(node));
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
