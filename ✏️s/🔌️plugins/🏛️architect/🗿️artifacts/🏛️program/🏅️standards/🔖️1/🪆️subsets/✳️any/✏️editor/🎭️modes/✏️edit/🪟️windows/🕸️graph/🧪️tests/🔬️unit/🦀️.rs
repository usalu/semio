
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
    let node = render(&program, &ArchitectConfig::default()).expect("graph");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("graph surface") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.nodes.len(), program.elements.len());
    assert_eq!(scene.edges.len(), undirected_edges(&program).len());
    crate::editor::architect::testkit::project_render(Ok(node));
}

#[semio_framework_async_macros::async_test]
async fn every_element_becomes_a_node_and_every_adjacency_an_edge() {
    let program = sample_plugin();
    let (nodes, edges) = graph_media_json(&program, &GraphCamera { x: 0.0, y: 0.0, zoom: 1.0 });
    assert_eq!(nodes.len(), program.elements.len());
    assert_eq!(edges.len(), undirected_edges(&program).len());
}
