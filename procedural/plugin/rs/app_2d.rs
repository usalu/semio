//! 🎲 Procedural 2D plugin — procedural flow play app bundled as a hot-swappable WASM component.

use flow_core::{dag::DagFixture, flow_backed_node_graph_extras, flow_neuron_kind_infos_json, forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec}, FlowFixture, FlowHost, Widget};
use flow_module_draw::render_scene_json;
use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_canvas_2d_scene, build_node_graph_scene, create_default_layout, create_named_layout, handle_generation_command,
    render_generation_form_body, render_generation_preview_text, render_generations_tree, selected_generation,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, App, Canvas2dScene, CommandDescriptor, GenerationPlayState, NodeGraphScene, PluginApp, PluginBundle,
    UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";
const PROCEDURAL2D_PLAY_SURFACE_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_SURFACE_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_BODY_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_DOCUMENT: &str = "procedural2d.play.document";
const PROCEDURAL2D_PLAY_BODY_CATALOGUE: &str = "procedural2d.play.catalogue";
const PROCEDURAL2D_PLAY_BODY_INSPECTION: &str = "procedural2d.play.inspection";
const PROCEDURAL2D_PLAY_WINDOW_MAIN: &str = "procedural2d-main";
const PROCEDURAL2D_PLAY_WINDOW_PREVIEW: &str = "procedural2d-preview";
const PROCEDURAL2D_PLAY_WINDOW_GENERATIONS: &str = "procedural2d-generations";
const PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural2d-generate-form";
const PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural2d-generate-preview";
const PROCEDURAL2D_PLAY_BODY_GENERATIONS: &str = "procedural2d.play.generations";
const PROCEDURAL2D_PLAY_BODY_GENERATE_FORM: &str = "procedural2d.play.generate-form";
const PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
const PROCEDURAL2D_PLAY_SURFACE_GENERATIONS: &str = "procedural2d.play.generations";
const PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
const DEFAULT_PROCEDURAL2D_FIXTURE_JSON: &str = include_str!("../../2d/example/default.procedural2d.json");
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default = "default_show_mode")]
    show_mode: String,
    #[serde(default)]
    eval_outputs_json: String,
    #[serde(default)]
    undo_stack: Vec<FlowFixture>,
    #[serde(default)]
    redo_stack: Vec<FlowFixture>,
}

fn default_show_mode() -> String {
    "preview".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: Procedural2dPlayRuntime,
    #[serde(default)]
    generation: GenerationPlayState,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_fixture() -> FlowFixture {
    serde_json::from_str(DEFAULT_PROCEDURAL2D_FIXTURE_JSON).unwrap_or_default()
}

fn default_envelope() -> Procedural2dPlayEnvelope {
    Procedural2dPlayEnvelope {
        fixture: default_fixture(),
        runtime: Procedural2dPlayRuntime::default(),
        generation: GenerationPlayState::default(),
    }
}

fn parse_envelope(document_json: &str) -> Procedural2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Procedural2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn procedural2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PROCEDURAL2D_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &Procedural2dPlayEnvelope) -> FlowHost {
    let mut host = FlowHost::from_fixture(envelope.fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    host
}

fn push_undo(play: &mut Procedural2dPlayEnvelope) {
    play.runtime.undo_stack.push(play.fixture.clone());
    if play.runtime.undo_stack.len() > 32 {
        play.runtime.undo_stack.remove(0);
    }
    play.runtime.redo_stack.clear();
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("ids"))
                .and_then(|value| serde_json::from_value(value.clone()).ok())
        })
        .unwrap_or_default()
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<MediaGraphPortRecord>,
    outputs: Vec<MediaGraphPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| MediaGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node
                .inputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
            outputs: node
                .outputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
        })
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputStepper { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn collect_drawing_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if handle.starts_with("drawing-") {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

fn affine_transform_array(value: &Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

fn path_segments_from_node(node: &Value) -> Vec<Value> {
    if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
        return segments.clone();
    }
    for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
        if let Some(inner) = node.get(key) {
            if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                return segments.clone();
            }
        }
    }
    Vec::new()
}

fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = serde_json::from_str::<Value>(&scene_json) else {
        return Vec::new();
    };
    if scene.get("error").is_some() {
        return Vec::new();
    }
    let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
        return Vec::new();
    };
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let node_body = node.get("node").unwrap_or(node);
            json!({
                "id": format!("{prefix}-{handle}-{index}"),
                "transform": affine_transform_array(node.get("transform").unwrap_or(&Value::Null)),
                "segments": path_segments_from_node(node_body),
                "fill": node.get("fill").cloned().unwrap_or(Value::Null),
                "stroke": node.get("stroke").cloned().unwrap_or(Value::Null),
                "opacity": node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0),
                "blendMode": "normal",
                "visible": true,
                "needsKernel": false,
            })
        })
        .collect()
}

fn eval_preview_layers(play: &Procedural2dPlayEnvelope, preview: bool) -> String {
    let mut host = host_from_envelope(play);
    let eval_json = if play.runtime.eval_outputs_json.is_empty() {
        host.evaluate().unwrap_or_default()
    } else {
        host.apply_eval_outputs_json(&play.runtime.eval_outputs_json);
        play.runtime.eval_outputs_json.clone()
    };
    let prefix = if preview { "procedural2d-preview" } else { "procedural2d-main" };
    let mut layers = Vec::new();
    if let Ok(outputs) = serde_json::from_str::<Value>(&eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    if play.runtime.show_mode == "wire" {
        let offset = if preview { 240.0 } else { 0.0 };
        for widget in &play.fixture.widgets {
            let id = widget_id(widget).to_string();
            if play.runtime.selected_ids.is_empty() || play.runtime.selected_ids.iter().any(|selected| selected == &id) {
                let (x, y) = play
                    .fixture
                    .layout
                    .get(&id)
                    .map(|layout| (layout.x, layout.y))
                    .unwrap_or((offset + 48.0, 240.0));
                layers.push(json!({
                    "id": format!("widget-{id}"),
                    "kind": "node",
                    "name": id,
                    "x": x,
                    "y": y,
                    "width": 96.0,
                    "height": 48.0,
                }));
            }
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

fn evaluate_generation_preview(play: &Procedural2dPlayEnvelope, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(&play.fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| play.fixture.clone());
    let mut host = FlowHost::from_fixture(fixture);
    host.evaluate().unwrap_or_default()
}

fn generation_preview_layers(play: &Procedural2dPlayEnvelope, eval_json: &str) -> String {
    let prefix = "procedural2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

fn refresh_generation_preview(play: &mut Procedural2dPlayEnvelope) {
    let Some(generation) = selected_generation(&play.generation) else {
        play.generation.preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(play, &generation.values);
    play.generation.preview_text = Some(preview.clone());
    play.runtime.eval_outputs_json = preview;
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>, command: Option<CommandDescriptor>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        command,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_document_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = play
        .fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item(
                format!("procedural2d-play-document.widget.{id}"),
                id.clone(),
                Some(procedural2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural2d-play-document.widgets".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if widget_items.is_empty() {
                vec![tree_item("procedural2d-play-document.empty", "(none)", None)]
            } else {
                widget_items
            },
        }],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("procedural2d-play-document.widget.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(procedural2d_cmd("setSelection", None)),
        drop_command: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let sources = [("inputSlider", "Slider"), ("inputStepper", "Stepper"), ("inputNote", "Note")];
    let components = [("math.add", "Add"), ("logic.and", "And"), ("text.concat", "Concat")];
    let sinks = [("outputPreview", "Preview"), ("outputExport", "Export")];
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "procedural2d-play-catalogue.sources".into(),
                label: Some("Sources".into()),
                default_open: Some(true),
                items: sources
                    .iter()
                    .map(|(kind, label)| {
                        tree_item(
                            format!("procedural2d-play-catalogue.source.{kind}"),
                            *label,
                            Some(procedural2d_cmd("addWidget", Some(json!({ "kind": kind })))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "procedural2d-play-catalogue.components".into(),
                label: Some("Components".into()),
                default_open: Some(true),
                items: components
                    .iter()
                    .map(|(kind, label)| {
                        tree_item(
                            format!("procedural2d-play-catalogue.component.{kind}"),
                            *label,
                            Some(procedural2d_cmd(
                                "addWidget",
                                Some(json!({ "kind": "neuron", "neuronKind": kind })),
                            )),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "procedural2d-play-catalogue.sinks".into(),
                label: Some("Sinks".into()),
                default_open: Some(true),
                items: sinks
                    .iter()
                    .map(|(kind, label)| {
                        tree_item(
                            format!("procedural2d-play-catalogue.sink.{kind}"),
                            *label,
                            Some(procedural2d_cmd("addWidget", Some(json!({ "kind": kind })))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "procedural2d-play-catalogue.modes".into(),
                label: Some("Show mode".into()),
                default_open: Some(false),
                items: ["preview", "generate", "wire"]
                    .iter()
                    .map(|mode| {
                        tree_item(
                            format!("procedural2d-play-catalogue.mode.{mode}"),
                            format!("Show {mode}"),
                            Some(procedural2d_cmd("setShowMode", Some(json!({ "value": mode })))),
                        )
                    })
                    .collect(),
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_command: None,
    })
}

fn build_inspector_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    if play.runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![
            ui_text("Schema: flow.fixture"),
            ui_text(format!("Widgets: {}", play.fixture.widgets.len())),
            ui_text(format!("Show mode: {}", play.runtime.show_mode)),
        ]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "procedural2d-play-inspector.selection".into(),
        label: "Selection".into(),
        default_open: Some(true),
        fields: vec![ui_inspector_readonly_field(
            "procedural2d-play-inspector.ids",
            "Ids",
            play.runtime.selected_ids.join(", "),
        )],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(play: &Procedural2dPlayEnvelope) -> UiNode {
    let host = host_from_envelope(play);
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&play.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if play.runtime.selected_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&play.runtime.selected_ids).ok()
    };
    let flow_extras = flow_backed_node_graph_extras(&play.fixture, "", 0.0);
    build_node_graph_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: flow_extras.operators_json,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            selection_json,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","command":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
            ),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_main_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: play.fixture.camera.x,
            camera_y: play.fixture.camera.y,
            zoom: play.fixture.camera.zoom,
            layers_json: eval_preview_layers(play, false),
        },
    )
}

fn render_preview_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: play.fixture.camera.x,
            camera_y: play.fixture.camera.y,
            zoom: play.fixture.camera.zoom,
            layers_json: eval_preview_layers(play, true),
        },
    )
}

fn render_generate_generations(play: &Procedural2dPlayEnvelope) -> UiNode {
    render_generations_tree(
        PROCEDURAL2D_PLAY_APP_ID,
        "procedural2d-play-generate",
        &play.generation.generations,
        play.generation.selected_generation_id.as_deref(),
    )
}

fn render_generate_form(play: &Procedural2dPlayEnvelope) -> UiNode {
    let spec = flow_fixture_to_form_spec(&play.fixture);
    let Some(generation) = selected_generation(&play.generation) else {
        return ui_text("Add a generation to edit input values.");
    };
    render_generation_form_body(
        &spec,
        &generation.values,
        PROCEDURAL2D_PLAY_APP_ID,
        "updateGenerationValues",
        &generation.id,
    )
}

fn render_generate_preview(play: &Procedural2dPlayEnvelope) -> UiNode {
    let eval_json = play
        .generation
        .preview_text
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("");
    if eval_json.is_empty() {
        return ui_text("(evaluate a generation to preview output)");
    }
    let layers = generation_preview_layers(play, eval_json);
    if layers == "[]" {
        return render_generation_preview_text(
            PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW,
            PROCEDURAL2D_PLAY_APP_ID,
            eval_json,
        );
    }
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: play.fixture.camera.x,
            camera_y: play.fixture.camera.y,
            zoom: play.fixture.camera.zoom,
            layers_json: layers,
        },
    )
}
//#endregion 🔖Render

//#region 🔖Procedural2dPlayApp
pub struct Procedural2dPlayApp;

impl PluginApp for Procedural2dPlayApp {
    fn app_id(&self) -> &str {
        PROCEDURAL2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("procedural2d envelope json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<Procedural2dPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "nodeGraphHover" => return Vec::new(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str(viewport_json) {
                        play.fixture.camera = camera;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "nodeGraphEdit" => {
                let mut host = host_from_envelope(&play);
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut changed = false;
                for op in ops {
                    match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                    push_undo(&mut play);
                                    play.fixture = fixture;
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            for id in play.runtime.selected_ids.clone() {
                                push_undo(&mut play);
                                if host.remove_widget(&id).is_ok() {
                                    changed = true;
                                }
                            }
                            if changed {
                                play.fixture = host.fixture.clone();
                                play.runtime.selected_ids.clear();
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) =
                                (from, from_port, to, to_port)
                            {
                                push_undo(&mut play);
                                if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                    play.fixture = host.fixture.clone();
                                    changed = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    return vec![set_document_op(&play)];
                }
            }
            "moveMediaNode" => {
                let mut host = host_from_envelope(&play);
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    push_undo(&mut play);
                    if host.move_widget(node_id, x, y).is_ok() {
                        play.fixture = host.fixture;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "addWidget" => {
                let mut host = host_from_envelope(&play);
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => json!({
                        "kind": "neuron",
                        "neuronKind": args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add"),
                    })
                    .to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                push_undo(&mut play);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    play.fixture = host.fixture;
                    play.runtime.selected_ids = vec![id];
                    return vec![set_document_op(&play)];
                }
            }
            "removeWidget" => {
                let mut host = host_from_envelope(&play);
                let widget_id = args.and_then(|value| value.get("widgetId")).and_then(|value| value.as_str());
                if let Some(widget_id) = widget_id {
                    push_undo(&mut play);
                    if host.remove_widget(widget_id).is_ok() {
                        play.fixture = host.fixture;
                        play.runtime.selected_ids.retain(|id| id != widget_id);
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "connectMediaPorts" => {
                let mut host = host_from_envelope(&play);
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str());
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                    push_undo(&mut play);
                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                        play.fixture = host.fixture;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "reorganize" => {
                let mut host = host_from_envelope(&play);
                push_undo(&mut play);
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    play.fixture = host.fixture;
                    return vec![set_document_op(&play)];
                }
            }
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    play.runtime.show_mode = mode.into();
                    return vec![set_document_op(&play)];
                }
            }
            "generate" => {
                push_undo(&mut play);
                let mut host = host_from_envelope(&play);
                play.runtime.eval_outputs_json = host.evaluate().unwrap_or_default();
                play.runtime.show_mode = "generate".into();
                return vec![set_document_op(&play)];
            }
            "setEvalOutputs" => {
                if let Some(outputs) = args.and_then(|value| value.get("outputs")) {
                    play.runtime.eval_outputs_json = outputs.to_string();
                    return vec![set_document_op(&play)];
                }
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    play.runtime.eval_outputs_json = json_text.into();
                    return vec![set_document_op(&play)];
                }
            }
            "undo" => {
                if let Some(previous) = play.runtime.undo_stack.pop() {
                    play.runtime.redo_stack.push(play.fixture.clone());
                    play.fixture = previous;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.runtime.redo_stack.pop() {
                    play.runtime.undo_stack.push(play.fixture.clone());
                    play.fixture = next;
                    return vec![set_document_op(&play)];
                }
            }
            "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => {}
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(&play.fixture);
                if handle_generation_command(command, args, &mut play.generation, &spec, PROCEDURAL2D_PLAY_APP_ID) {
                    if matches!(command, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                        refresh_generation_preview(&mut play);
                    }
                    return vec![set_document_op(&play)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            PROCEDURAL2D_PLAY_BODY_MAIN => render_main_graph(&play),
            PROCEDURAL2D_PLAY_BODY_PREVIEW => render_preview_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_GENERATIONS => render_generate_generations(&play),
            PROCEDURAL2D_PLAY_BODY_GENERATE_FORM => render_generate_form(&play),
            PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&play),
            PROCEDURAL2D_PLAY_BODY_DOCUMENT => build_document_tree(&play),
            PROCEDURAL2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            PROCEDURAL2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Procedural2dPlayApp

//#region 🔖AppFactory
pub fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D").document(["semio", "procedural", "2d"])
            .icon_id("procedural2d")
            .mode("edit", "Edit")
            .mode("generate", "Generate")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, "Flow", PROCEDURAL2D_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL2D_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d)
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATIONS,
                "Generations",
                PROCEDURAL2D_PLAY_BODY_GENERATIONS,
                SurfaceKind::Canvas2d,
            )
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, "Form", PROCEDURAL2D_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d)
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::Canvas2d,
            )
            .default_layout(create_default_layout(
                &[PROCEDURAL2D_PLAY_WINDOW_MAIN.into(), PROCEDURAL2D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[55.0, 45.0]),
                Some(&["Main".into(), "Preview".into()]),
            ))
            .named_layout(create_named_layout(
                "procedural2d-generate",
                "Generate",
                create_default_layout(
                    &[
                        PROCEDURAL2D_PLAY_WINDOW_GENERATIONS.into(),
                        PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM.into(),
                        PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                    ],
                    "row",
                    Some(&[22.0, 43.0, 35.0]),
                    Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                ),
                "builtin",
                Some("sparkles".into()),
                None,
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                PROCEDURAL2D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                PROCEDURAL2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                PROCEDURAL2D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", "Default", serde_json::to_string(&default_envelope()).unwrap())
    .program("procedural2d", "Procedural 2D", "layout")
}

fn procedural2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Procedural 2D", 1024, 768)
}

pub fn register_procedural2d_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.procedural", "procedural2d", procedural2d_document_json_to_svg);
}
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_main_graph_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn main_graph_scene_exports_flow_backed_node_graph_fields() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let value: Value = serde_json::from_str(&serde_json::to_string(&node).unwrap()).expect("ui node json");
        let graph = value.get("nodeGraph").expect("nodeGraph");
        assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
        assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some());
        assert!(graph.get("capabilitiesJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow")));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn document_lists_widgets() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-document.widget.rect"));
    }

    #[test]
    fn catalogue_lists_show_modes() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-catalogue.mode.preview"));
    }

    #[test]
    fn generate_command_sets_eval_outputs() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("generate", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.show_mode, "generate");
    }

    #[test]
    fn set_show_mode_updates_runtime() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setShowMode", Some(&json!({ "value": "wire" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.show_mode, "wire");
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let generations = app.render(PROCEDURAL2D_PLAY_BODY_GENERATIONS, &document, &ViewState::default());
        assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
    }

    #[test]
    fn add_generation_evaluates_preview() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addGeneration", None, &document, &ViewState::default());
        let updated: Procedural2dPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(updated.generation.generations.len(), 1);
        assert!(updated.generation.preview_text.as_deref().unwrap_or("").len() > 2);
    }
}
//#endregion 🧪Tests
