use super::*;
use crate::editor::flow::unit_tests::context::{flow_app, flow_app_closing, main_window_measures, render as render_body, select_graph};

fn rendered_node_graph_scene(rendered: &str) -> serde_json::Value {
    let scene: NodeGraphScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(rendered).expect("rendered nodeGraph scene");
    serde_json::to_value(scene).expect("nodeGraph scene json")
}

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
async fn main_scene_declares_its_scoped_graph_interaction_targets() {
    let mut app = flow_app_closing().await;
    let rendered = render_body(&mut app, FLOW_PLAY_BODY_MAIN).await;
    let scene = rendered_node_graph_scene(&rendered);
    assert_eq!(
        scene["interactionDomain"],
        serde_json::json!({
            "id": "graph",
            "nodeTargetPrefix": "flow-play-document.widget.",
            "edgeTargetPrefix": "flow-play-document.synapse.",
            "handleTargetPrefix": "flow-play-document.handle."
        }),
        "the scene must own the app-specific target mapping used by generic NodeGraph producers"
    );
}

#[semio_framework_async_macros::async_test]
async fn framework_graph_selection_projects_back_to_raw_scene_node_ids() {
    let mut app = flow_app_closing().await;
    select_graph(&mut app, &["add"], &[]).await;
    let selected = app.interaction_state().await;
    assert_eq!(
        selected.selection.get(crate::editor::flow::FLOW_INTERACTION_GRAPH).map(|selection| selection.ids.as_slice()),
        Some([crate::editor::flow::flow_graph_node_target_id("add")].as_slice()),
        "the settled reserved job must publish the app-scoped target before render"
    );
    let rendered = render_body(&mut app, FLOW_PLAY_BODY_MAIN).await;
    let scene = rendered_node_graph_scene(&rendered);
    assert_eq!(scene["selection"], serde_json::json!(["add"]), "the main scene must strip its scoped topology prefix while projecting framework-owned selection");
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
