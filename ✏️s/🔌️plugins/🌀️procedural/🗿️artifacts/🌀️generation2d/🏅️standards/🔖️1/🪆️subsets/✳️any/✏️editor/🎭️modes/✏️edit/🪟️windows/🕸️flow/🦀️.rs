//! 🕸️ Generation2d play app — the main node-graph window: the editable flow canvas.

use crate::editor::generation2d::config::Generation2dConfig;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use crate::standards::v1::subsets::any::schema::{fixture_to_workflow, with_host};
use crate::Generation2dSnapshot;
use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_MAIN: &str = "generation2d-main";
pub const GENERATION2D_PLAY_BODY_MAIN: &str = "generation2d.play.main";
const GENERATION2D_PLAY_SURFACE_MAIN: &str = "generation2d.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION2D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Fluss"),
        body_key: GENERATION2D_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "flow-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Generation2dSnapshot, config: &Generation2dConfig, session: &FlowEvalSession) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let fixture = &document.fixture;
    let (nodes, edges) = with_host(fixture, |host| fixture_to_workflow(&host.dag.fixture));
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let flow_extras = flow_backed_node_graph_extras(fixture, "", 0.0, true, false, semio_framework_ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    // 🕹️ `render` carries no `InteractionView` and `NodeGraphScene` has no `interaction_domain` field
    // for the wrapper to stamp post-render either (see the `🧊️3d` sibling window's identical note,
    // ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `selection` is left at
    // `NodeGraphScene::base`'s empty default until a future wave threads interaction into rendering.
    let _ = GENERATION2D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION2D_PLAY_SURFACE_MAIN,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph,
        &NodeGraphScene {
            editable: Some(true),
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🕸️flow/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
