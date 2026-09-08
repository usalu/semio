
use super::*;
use crate::editor::sequence::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_MAIN).await.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SEQUENCE_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn split_endpoint_defaults_port_to_next() {
    assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
    assert_eq!(split_endpoint("node"), ("node".to_string(), "next".to_string()));
}
