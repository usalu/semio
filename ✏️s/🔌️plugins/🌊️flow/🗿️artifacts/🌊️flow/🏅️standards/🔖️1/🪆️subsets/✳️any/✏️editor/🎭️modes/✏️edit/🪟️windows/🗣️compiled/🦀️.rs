//! 🗣️ Flow play app — the compiled-DAG window: the read-only wire literal of the current snapshot.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::FlowSnapshot;
use flow::FlowEvalSession;
use semio_plugin::scene_surface;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_ui_contract::SurfaceKind as ContractSurfaceKind;
use semio_ui_declaration::ui::TextEditorScene;

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_COMPILED: &str = "flow-compiled-dag";
pub const FLOW_PLAY_BODY_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_SURFACE_COMPILED: &str = "flow.play.compiled-dag";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: FLOW_PLAY_WINDOW_COMPILED.into(),
        label: LocalizedLabel::native("DSL", "DSL"),
        body_key: FLOW_PLAY_BODY_COMPILED.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "code".into(),
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
pub fn render(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> UiAssemblyResult<BuiltNode> {
    let live=snapshot.read_source().map_err(|error|semio_plugin::PluginAssemblyError::new("flow.source",error.into_message()))?;
    render_source(&live,config,session)
}

/// 🗣️ Projects the already retained execution source.
pub fn render_source(live:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,_config:&FlowMainWindowConfig,_session:&FlowEvalSession)->UiAssemblyResult<BuiltNode>{
    let wire=flow::FlowHost::project_compiled_wire_literal(live,&flow::flow_neuron_kind_info_map());
    let scene = TextEditorScene::base(wire, Some("wire".into()), None);
    scene_surface(FLOW_PLAY_SURFACE_COMPILED, ContractSurfaceKind::TextEditor, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
