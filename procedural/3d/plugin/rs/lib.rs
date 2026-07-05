//! 🧱 Procedural 3D plugin — flow-based procedural brep editor bundled as a hot-swappable WASM component.

use flow_core::{dag::DagFixture, FlowFixture, FlowHost, Widget};
use flow_module_brep::tessellate_geometry_json;
use semio_framework_plugin::{
    build_node_graph_scene, build_world_3d_scene, create_default_layout, export_mesh_glb_bytes,
    export_mesh_obj, merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    world3d_default_camera, world3d_scene, world3d_selection_json,
    CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle, UiControlNode, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_core::mesh_from_indexed;
use std::collections::HashSet;
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

const PROCEDURAL_FALLBACK_MESH_KIND: &str = "box";
const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";

const HEX_COLUMN_EXAMPLE_JSON: &str = include_str!("../../example/hexagonal-mushroom-column.procedural.json");
const RECT_EXTRUDE_EXAMPLE_JSON: &str = include_str!("../../example/rectangle-extrude-volume.procedural.json");
const SPHERE_TORUS_EXAMPLE_JSON: &str = include_str!("../../example/sphere-cut-with-torus.procedural.json");

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
    envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(|| Procedural3dEnvelope {
        fixture: FlowFixture::default(),
        runtime: Procedural3dRuntime::default(),
    })
}

fn envelope_from_fixture_json(json_text: &str) -> Option<Procedural3dEnvelope> {
    serde_json::from_str::<FlowFixture>(json_text).ok().map(|fixture| Procedural3dEnvelope {
        fixture,
        runtime: Procedural3dRuntime::default(),
    })
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

fn neuron_mesh_kind(neuron_kind: &str) -> &'static str {
    match neuron_kind {
        "brep.prim3d.sphere" => "sphere",
        "brep.prim3d.cylinder" => "cylinder",
        "brep.prim3d.cone" => "cone",
        "brep.prim3d.torus" => "torus",
        "brep.prim3d.box" => "box",
        "brep.solid.extrude" | "brep.bool.cut" | "brep.bool.fuse" => "box",
        _ => PROCEDURAL_FALLBACK_MESH_KIND,
    }
}

fn widget_preview_mesh_kind(widget: &Widget) -> Option<&'static str> {
    match widget {
        Widget::Neuron { neuronKind, preview, .. } if *preview => Some(neuron_mesh_kind(neuronKind)),
        Widget::OutputPreview { .. } => Some(PROCEDURAL_FALLBACK_MESH_KIND),
        _ => None,
    }
}

fn widget_layout_position(fixture: &FlowFixture, widget_id: &str) -> (f64, f64) {
    fixture
        .layout
        .get(widget_id)
        .map(|layout| (layout.x, layout.y))
        .unwrap_or((0.0, 0.0))
}

fn is_brep_geometry_handle(handle: &str) -> bool {
    handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
}

fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
    let widget_eval = eval.get(widget_id)?;
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles.into_iter().next()
}

fn mesh_from_tessellation_json(mesh_json: &str) -> Option<semio_framework_plugin::MeshData> {
    let parsed: Value = serde_json::from_str(mesh_json).ok()?;
    if parsed.get("error").is_some() {
        return None;
    }
    let positions: Vec<f32> = parsed
        .get("position")
        .or_else(|| parsed.get("positions"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
        .filter(|items: &Vec<f32>| !items.is_empty())?;
    let normals: Vec<f32> = parsed
        .get("normal")
        .or_else(|| parsed.get("normals"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
        .unwrap_or_default();
    let indices: Vec<u32> = parsed
        .get("index")
        .or_else(|| parsed.get("indices"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_u64().map(|number| number as u32)).collect())
        .filter(|items: &Vec<u32>| !items.is_empty())?;
    Some(mesh_from_indexed(&positions, &normals, &indices))
}

fn evaluated_preview_payload(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let Some(handle) = geometry_handle_for_widget(&eval, &id) else {
            continue;
        };
        let mesh_id = format!("eval-{id}");
        if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            let tessellation = tessellate_geometry_json(&handle, 0.05);
            if let Some(data) = mesh_from_tessellation_json(&tessellation) {
                meshes.push(json!({ "id": mesh_id, "data": data }));
            }
        }
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            let (x, y) = widget_layout_position(fixture, &id);
            let selected = runtime.selected_node_ids.contains(&id);
            let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
            instances.push(json!({
                "id": id,
                "meshId": mesh_id,
                "position": [x * 0.01, -y * 0.01, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": selected,
                "hovered": hovered,
            }));
        }
    }
    if meshes.is_empty() {
        let fallback = preview_meshes_json_fallback(fixture);
        let fallback_instances = preview_instances_json_fallback(fixture, runtime);
        return (fallback, fallback_instances);
    }
    (
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()),
    )
}

fn preview_instances_json_fallback(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> String {
    let instances: Vec<Value> = fixture
        .widgets
        .iter()
        .filter_map(|widget| {
            let mesh_kind = widget_preview_mesh_kind(widget)?;
            let id = widget_id(widget).to_string();
            let (x, y) = widget_layout_position(fixture, &id);
            let selected = runtime.selected_node_ids.contains(&id);
            let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
            Some(json!({
                "id": id,
                "meshId": mesh_kind,
                "position": [x * 0.01, -y * 0.01, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": selected,
                "hovered": hovered,
            }))
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn preview_meshes_json_fallback(fixture: &FlowFixture) -> String {
    let kinds: Vec<String> = fixture
        .widgets
        .iter()
        .filter_map(|widget| widget_preview_mesh_kind(widget).map(str::to_string))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let fallback_kinds = if kinds.is_empty() {
        vec![PROCEDURAL_FALLBACK_MESH_KIND.into()]
    } else {
        kinds
    };
    let meshes: Vec<Value> = fallback_kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn preview_selection_json(runtime: &Procedural3dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_node_ids,
        runtime.hovered_node_id.as_deref(),
    )
}

fn export_mesh_from_envelope(envelope: &Procedural3dEnvelope) -> semio_framework_plugin::MeshData {
    let (meshes_json, _) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
    if let Ok(meshes) = serde_json::from_str::<Vec<Value>>(&meshes_json) {
        if let Some(first) = meshes.first() {
            if let Ok(data) = serde_json::from_value(first.get("data").cloned().unwrap_or(Value::Null)) {
                return data;
            }
        }
    }
    let kind = envelope
        .fixture
        .widgets
        .iter()
        .find_map(|widget| widget_preview_mesh_kind(widget))
        .unwrap_or(PROCEDURAL_FALLBACK_MESH_KIND);
    mesh_from_kind(kind)
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
        hover_command: None,
        unhover_command: None,
        actions: None,
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
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    Procedural3dEnvelope {
                        fixture: FlowFixture::default(),
                        runtime: Procedural3dRuntime::default(),
                    }
                } else if example_id == PROCEDURAL_EXAMPLE_HEX_COLUMN || example_id == "demo" {
                    envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else if example_id == PROCEDURAL_EXAMPLE_RECT_EXTRUDE {
                    envelope_from_fixture_json(RECT_EXTRUDE_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else if example_id == PROCEDURAL_EXAMPLE_SPHERE_TORUS {
                    envelope_from_fixture_json(SPHERE_TORUS_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setSelection" | "selectNode" => {
                envelope.runtime.selected_node_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
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
        NodeGraphScene::base(nodes_json, edges_json, viewport_json),
                )
            }
            PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                let (meshes_json, instances_json) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
                build_world_3d_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    world3d_scene(
                        world3d_default_camera(),
                        meshes_json,
                        instances_json,
                        preview_selection_json(&envelope.runtime),
                    ),
                )
            }
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
    .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", HEX_COLUMN_EXAMPLE_JSON)
    .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", RECT_EXTRUDE_EXAMPLE_JSON)
    .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", SPHERE_TORUS_EXAMPLE_JSON)
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
    fn set_lod_mode_reads_value_arg() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("setLodMode", Some(&json!({ "value": "wireframe" })), &document, &ViewState::default());
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.lod_mode, "wireframe");
    }

    #[test]
    fn set_active_example_loads_sphere_fixture() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &document,
            &ViewState::default(),
        );
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuronKind, .. } if neuronKind == "brep.prim3d.sphere")));
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
