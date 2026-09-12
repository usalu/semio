use super::*;
use crate::editor::flow::unit_tests::context::{flow_app, main_window_measures, render as render_body};

#[semio_framework_async_macros::async_test]
async fn split_endpoint_defaults_port_to_out() {
    assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
    assert_eq!(split_endpoint("node"), ("node".to_string(), "out".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_MAIN).await.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_lod_proximity_and_grid() {
    let mut app = flow_app().await;
    let measures = main_window_measures(&mut app).await;
    assert_eq!(measures.len(), 3);
    assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.proximity")));
    assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "flow-play-measures.grid")));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, FLOW_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
