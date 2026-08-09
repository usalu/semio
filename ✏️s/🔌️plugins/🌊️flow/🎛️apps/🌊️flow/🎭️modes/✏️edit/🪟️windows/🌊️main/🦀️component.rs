//! 🌊️ Flow play app — the main node-graph window: the editable flow canvas.

use crate::apps::flow::config::FlowConfig;
use crate::apps::flow::modes::edit::windows::main::options;
use crate::apps::flow::terminology::FlowPlayLabels;
use crate::apps::flow::FLOW_PLAY_APP_ID;
use crate::artifacts::flow::engine::{fixture_to_workflow, host_from_snapshot};
use crate::artifacts::flow::FlowSnapshot;
use flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
pub const FLOW_PLAY_BODY_MAIN: &str = "flow.play.main";
const FLOW_PLAY_SURFACE_MAIN: &str = "flow.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::flow::create_flow_app`. `options.measures` stays
/// empty here on purpose: flow's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Flow"),
        body_key: FLOW_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "flow-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(config: &FlowConfig, labels: &FlowPlayLabels) -> Vec<WindowMeasure> {
    vec![options::lod::measure(config, labels), options::proximity::measure(config, labels), options::grid::measure(config, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> UiNode {
    let host = host_from_snapshot(fixture, config, session);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let fixture_json = serde_json::to_string(fixture).ok();
    let selection = config.selected_node_ids.clone();
    let flow_extras = flow_backed_node_graph_extras(&fixture.to_fixture(), &config.lod_mode, config.proximity_distance, config.grid_visible, config.grid_snap_enabled, config.grid_factor, Some(session));
    let preview_off_json = if config.preview_off_node_ids.is_empty() { None } else { serde_json::to_string(&config.preview_off_node_ids).ok() };
    build_node_graph_scene(
        FLOW_PLAY_SURFACE_MAIN,
        FLOW_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: flow_extras.operators,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json.or(fixture_json),
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            selection,
            preview_off_json,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{flow_app, main_window_measures, render as render_body};

    #[test]
    fn renders_node_graph_scene() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_MAIN).contains("node-graph"));
    }

    #[test]
    fn window_measures_surface_lod_proximity_and_grid() {
        let mut app = flow_app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 3);
        assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.proximity")));
        assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "flow-play-measures.grid")));
    }

    #[test]
    fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, FLOW_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
