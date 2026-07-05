//! 🧱 Procedural 3D plugin — flow-based procedural brep editor bundled as a hot-swappable WASM component.

use flow_core::{dag::DagFixture, FlowFixture, FlowHost, Widget};
use semio_framework_plugin::{
    build_node_graph_scene, build_world_3d_scene, create_default_layout, export_mesh_glb_bytes,
    export_mesh_obj, merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    world3d_default_camera, world3d_meshes_json_from_kinds, world3d_scene, world3d_selection_json,
    CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle, UiControlNode, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_CONTROLLER_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
const PROCEDURAL_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
const PROCEDURAL_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_HIERARCHY: &str = "procedural.play.hierarchy";
const PROCEDURAL_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
const PROCEDURAL_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
const PROCEDURAL_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
const PROCEDURAL_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";

const PROCEDURAL_MESH_KIND: &str = "box";

const WIDGET_CATALOG: &[(&str, &str, &str)] = &[
    ("neuron", "Neuron", "cpu"),
    ("inputSlider", "Slider", "sliders-horizontal"),
    ("inputNote", "Note", "file-text"),
    ("outputPreview", "Preview", "eye"),
];
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural3dRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    lod_mode: String,
    #[serde(default = "default_show_mode")]
    show_mode: String,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_node_id: Option<String>,
}

fn default_show_mode() -> String {
    "solid".into()
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural3dEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: Procedural3dRuntime,
}

fn default_envelope() -> Procedural3dEnvelope {
    Procedural3dEnvelope {
        fixture: FlowFixture::default(),
        runtime: Procedural3dRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> Procedural3dEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Procedural3dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn procedural_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PROCEDURAL_3D_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &Procedural3dEnvelope) -> FlowHost {
    FlowHost::from_fixture(envelope.fixture.clone())
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<Value> = fixture
        .nodes
        .iter()
        .map(|node| {
            json!({
                "id": node.id,
                "label": if node.name.is_empty() { &node.id } else { &node.name },
                "x": node.x,
                "y": node.y,
                "width": node.width,
                "height": node.height,
            })
        })
        .collect();
    let edges: Vec<Value> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            json!({
                "id": edge.id,
                "sourceNodeId": source_node_id,
                "sourcePortId": source_port_id,
                "targetNodeId": target_node_id,
                "targetPortId": target_port_id,
            })
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

fn preview_instances_json(host: &FlowHost, runtime: &Procedural3dRuntime) -> String {
    let instances: Vec<Value> = host
        .dag
        .fixture
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let selected = runtime.selected_node_ids.contains(&node.id);
            let hovered = runtime.hovered_node_id.as_deref() == Some(node.id.as_str());
            json!({
                "id": node.id,
                "meshId": PROCEDURAL_MESH_KIND,
                "position": [node.x * 0.01, node.y * 0.01, index as f64 * 0.5],
                "scale": [1.0, 1.0, 1.0],
                "label": node.name,
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn preview_meshes_json() -> String {
    world3d_meshes_json_from_kinds(&[PROCEDURAL_MESH_KIND.into()])
}

fn preview_selection_json(runtime: &Procedural3dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_node_ids,
        runtime.hovered_node_id.as_deref(),
    )
}

fn export_mesh_from_envelope(_envelope: &Procedural3dEnvelope) -> semio_framework_plugin::MeshData {
    mesh_from_kind(PROCEDURAL_MESH_KIND)
}
//#endregion 🔖Document

//#region 🔖Panels
fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: CommandDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command: Some(command),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(fixture: &FlowFixture, selected_node_ids: &[String]) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_command(
                format!("procedural-widget:{id}"),
                id.clone(),
                Some("cpu"),
                procedural_cmd("setSelection", Some(json!({ "ids": [id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural-play-hierarchy.widgets".into(),
            label: Some("Widgets".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: Some(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = WIDGET_CATALOG
        .iter()
        .map(|(kind, label, icon)| {
            tree_item_with_command(
                format!("procedural-play-catalogue.{kind}"),
                *label,
                Some(icon),
                procedural_cmd("addWidget", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural-play-catalogue.widgets".into(),
            label: Some("Widgets".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(fixture: &FlowFixture, selected_node_ids: &[String]) -> UiNode {
    let Some(selected_id) = selected_node_ids.first() else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", fixture.schema)),
            ui_text(format!("Widgets: {}", fixture.widgets.len())),
        ]);
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return ui_text("No selection".to_string());
    };
    let mut fields = vec![ui_inspector_readonly_field(
        "procedural-play-inspector.id",
        "Id",
        widget_id(widget),
    )];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let mixed = ui_inspector_mixed_number(&[*value]);
        fields.push(UiNode::Field(UiFieldNode {
            id: "procedural-play-inspector.value".into(),
            label: "Value".into(),
            child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                id: "procedural-play-inspector.value.input".into(),
                input_kind: "number".into(),
                value: mixed.value.to_string(),
                placeholder: None,
                commit: None,
                on_change: procedural_cmd(
                    "patchFlowWidgets",
                    Some(json!({ "widgetIds": [selected_id], "field": "value" })),
                ),
            }),
        }));
        fields.push(ui_inspector_readonly_field(
            "procedural-play-inspector.range",
            "Range",
            &format!("{min}..{max}"),
        ));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "procedural-play-inspector.widget".into(),
        label: "Widget".into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖Panels

//#region 🔖Procedural3dPlayApp
struct Procedural3dPlayApp;

impl PluginApp for Procedural3dPlayApp {
    fn app_id(&self) -> &str {
        PROCEDURAL_3D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("procedural3d envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let mut host = host_from_envelope(&envelope);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" => {
                envelope.runtime.selected_node_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("instanceId")).and_then(|value| value.as_str()) {
                    envelope.runtime.lod_mode = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.show_mode = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if host.move_widget(node_id, x, y).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => json!({ "kind": "neuron", "neuronKind": "math.add" }).to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    envelope.fixture = host.fixture;
                    envelope.runtime.selected_node_ids = vec![id];
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                for widget in envelope.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    if let (Widget::InputSlider { value: ref mut slider_value, .. }, Some(value)) =
                        (widget, raw_value.and_then(|entry| entry.as_f64()))
                    {
                        if field == "value" {
                            *slider_value = value;
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "reorganize" => {
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    envelope.fixture = host.fixture;
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_node_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_node_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_node_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setSelectionMethod" => {
                let method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "worldPointerDown" | "graphPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let host = host_from_envelope(&envelope);
        match body_key {
            PROCEDURAL_3D_PLAY_BODY_MAIN => {
                let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
                let viewport_json =
                    serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
                build_node_graph_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    NodeGraphScene {
                        nodes_json,
                        edges_json,
                        viewport_json,
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_PREVIEW => build_world_3d_scene(
                PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                PROCEDURAL_3D_PLAY_APP_ID,
                world3d_scene(
                    world3d_default_camera(),
                    preview_meshes_json(),
                    preview_instances_json(&host, &envelope.runtime),
                    preview_selection_json(&envelope.runtime),
                ),
            ),
            PROCEDURAL_3D_PLAY_BODY_HIERARCHY => {
                build_hierarchy_tree(&envelope.fixture, &envelope.runtime.selected_node_ids)
            }
            PROCEDURAL_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            PROCEDURAL_3D_PLAY_BODY_INSPECTION => {
                build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids)
            }
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}
//#endregion 🔖Procedural3dPlayApp

//#region 🔖Manifest
fn create_procedural3d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D")
            .icon_id("workflow")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL_3D_PLAY_WINDOW_MAIN, "Flow", PROCEDURAL_3D_PLAY_BODY_MAIN)
            .window_kind(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL_3D_PLAY_BODY_PREVIEW)
            .default_layout(create_default_layout(
                &[PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "Preview".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                PROCEDURAL_3D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PROCEDURAL_3D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PROCEDURAL_3D_PLAY_BODY_INSPECTION,
            ),
    )
    .example("demo", "Demo", &serde_json::to_string(&default_envelope()).unwrap())
    .program("procedural3d", "Procedural 3D", "brep")
}

fn bundle() -> PluginBundle {
    register_procedural3d_exports();
    PluginBundle::new("procedural3d", "Procedural 3D", "0.1.0")
        .register_app(create_procedural3d_app(), || Box::new(Procedural3dPlayApp))
}

fn register_procedural3d_exports() {
    register_os_media_export_handler("3d.procedural", OsMediaExportFormat::Obj, |doc| {
        let envelope: Procedural3dEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (data, mime_type) = export_mesh_obj(&mesh, "procedural");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "procedural.obj".into(),
        })
    });
    register_os_media_export_handler("3d.procedural", OsMediaExportFormat::Glb, |doc| {
        let envelope: Procedural3dEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "procedural.glb".into(),
        })
    });
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_node_graph_scene() {
        let app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_world_preview_scene() {
        let app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn add_widget_command_appends_widget() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let before = parse_envelope(&document).fixture.widgets.len();
        let ops = app.handle_command("addWidget", Some(&json!({ "kind": "inputNote" })), &document, &ViewState::default());
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.widgets.len() > before);
    }

    fn apply_ops(envelope: &Procedural3dEnvelope, ops: &[String]) -> Procedural3dEnvelope {
        let mut next = envelope.clone();
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        next = parsed;
                    }
                }
            }
        }
        next
    }
}
//#endregion 🧪Tests
