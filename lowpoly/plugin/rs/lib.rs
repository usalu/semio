//! 🔺 Lowpoly plugin — mesh editing play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, CommandDescriptor,
    PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiToggleNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, World3dScene, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_SURFACE_MAIN: &str = "lowpoly.play";
const LOWPOLY_PLAY_BODY_MAIN: &str = "lowpoly.play.main";
const LOWPOLY_PLAY_BODY_HIERARCHY: &str = "lowpoly.play.hierarchy";
const LOWPOLY_PLAY_BODY_CATALOGUE: &str = "lowpoly.play.catalogue";
const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
const LOWPOLY_PLAY_BODY_LAYERS: &str = "lowpoly.play.layers";
const LOWPOLY_PLAY_WINDOW_MAIN: &str = "lowpoly-main";
const LOWPOLY_FIXTURE_SCHEMA: &str = "lowpoly.fixture";

const DEFAULT_FIXTURE_JSON: &str = include_str!("../../example/default.lowpoly.json");

static LOWPOLY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[
    ("box", "Box", "box"),
    ("sphere", "Sphere", "circle"),
    ("ico", "Ico Sphere", "globe"),
    ("cylinder", "Cylinder", "cylinder"),
];
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LowpolyTransform {
    #[serde(default)]
    position: [f32; 3],
    #[serde(default)]
    rotation: [f32; 3],
    #[serde(default = "default_scale")]
    scale: [f32; 3],
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LowpolySelectionTargets {
    #[serde(default = "default_true")]
    mesh: bool,
    #[serde(default)]
    vertex: bool,
    #[serde(default)]
    edge: bool,
    #[serde(default)]
    face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LowpolySelection {
    #[serde(default)]
    targets: LowpolySelectionTargets,
    #[serde(default)]
    keys: Vec<String>,
    #[serde(default = "default_mesh_mode")]
    mode: String,
    #[serde(default)]
    ids: Vec<u32>,
}

fn default_mesh_mode() -> String {
    "mesh".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyPaintLayer {
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(default = "one_f32")]
    opacity: f32,
    #[serde(default = "default_blend")]
    blend_mode: String,
}

fn one_f32() -> f32 {
    1.0
}

fn default_blend() -> String {
    "normal".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyObject {
    id: String,
    name: String,
    #[serde(default)]
    transform: LowpolyTransform,
    #[serde(default)]
    smooth_shading: bool,
    #[serde(default = "default_mesh_json")]
    mesh_json: String,
    #[serde(default)]
    paint_layers: Vec<LowpolyPaintLayer>,
}

fn default_mesh_json() -> String {
    "{}".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyFixture {
    schema: String,
    #[serde(default)]
    objects: Vec<LowpolyObject>,
    #[serde(default)]
    active_object_id: String,
    #[serde(default)]
    selection: LowpolySelection,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyPlayRuntime {
    #[serde(default)]
    transform_tool: String,
    #[serde(default)]
    active_paint_layer: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyPlayEnvelope {
    fixture: LowpolyFixture,
    #[serde(default)]
    runtime: LowpolyPlayRuntime,
}

fn default_envelope() -> LowpolyPlayEnvelope {
    serde_json::from_str(DEFAULT_FIXTURE_JSON).unwrap_or_else(|_| LowpolyPlayEnvelope {
        fixture: LowpolyFixture {
            schema: LOWPOLY_FIXTURE_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: String::new(),
            selection: LowpolySelection {
                targets: LowpolySelectionTargets::default(),
                keys: Vec::new(),
                mode: default_mesh_mode(),
                ids: Vec::new(),
            },
        },
        runtime: LowpolyPlayRuntime::default(),
    })
}

fn parse_envelope(document_json: &str) -> LowpolyPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn next_object_id() -> String {
    let next = LOWPOLY_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("obj-{next}")
}

fn set_document_op(envelope: &LowpolyPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn lowpoly_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn active_object<'a>(fixture: &'a LowpolyFixture) -> Option<&'a LowpolyObject> {
    fixture
        .objects
        .iter()
        .find(|object| object.id == fixture.active_object_id)
        .or_else(|| fixture.objects.first())
}

fn world_instances_json(fixture: &LowpolyFixture) -> String {
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .enumerate()
        .map(|(index, object)| {
            let active = object.id == fixture.active_object_id
                || (fixture.active_object_id.is_empty() && index == 0);
            json!({
                "id": object.id,
                "x": object.transform.position[0] as f64,
                "y": object.transform.position[1] as f64,
                "z": object.transform.position[2] as f64,
                "scale": object.transform.scale[0] as f64,
                "label": object.name,
                "color": if active { "#60a5fa" } else { "#94a3b8" },
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn default_camera_json() -> String {
    r#"{"x":4.0,"y":-4.0,"z":3.0,"fov":45}"#.into()
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

fn build_hierarchy_tree(envelope: &LowpolyPlayEnvelope) -> UiNode {
    let items: Vec<UiTreeItemNode> = envelope
        .fixture
        .objects
        .iter()
        .map(|object| {
            tree_item_with_command(
                format!("lowpoly-object:{}", object.id),
                object.name.clone(),
                Some("box"),
                lowpoly_cmd("setActiveObject", Some(json!({ "objectId": object.id }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "lowpoly-play-hierarchy.objects".into(),
            label: Some("Objects".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = PRIMITIVE_CATALOG
        .iter()
        .map(|(kind, label, icon)| {
            tree_item_with_command(
                format!("lowpoly-play-catalogue.{kind}"),
                *label,
                Some(icon),
                lowpoly_cmd("addPrimitive", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "lowpoly-play-catalogue.primitives".into(),
            label: Some("Primitives".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_layers_tree(envelope: &LowpolyPlayEnvelope) -> UiNode {
    let object = active_object(&envelope.fixture);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let items: Vec<UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            tree_item_with_command(
                format!("lowpoly-layer:{index}"),
                layer.name.clone(),
                Some("layers"),
                lowpoly_cmd("setActivePaintLayer", Some(json!({ "layerIndex": index }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "lowpoly-play-layers.paint".into(),
            label: Some("Paint Layers".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(envelope: &LowpolyPlayEnvelope) -> UiNode {
    let Some(object) = active_object(&envelope.fixture) else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {LOWPOLY_FIXTURE_SCHEMA}")),
            ui_text("No active object".to_string()),
        ]);
    };
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: "Object".into(),
            default_open: None,
            fields: vec![
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: "Name".into(),
                    child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                        id: "lowpoly-play-inspector.object.name.input".into(),
                        input_kind: "text".into(),
                        value: object.name.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: lowpoly_cmd(
                            "patchObject",
                            Some(json!({ "objectId": object.id, "field": "name" })),
                        ),
                    }),
                }),
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: "Smooth Shading".into(),
                    child: UiControlNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        pressed: object.smooth_shading,
                        text: None,
                        on_change: lowpoly_cmd(
                            "patchObject",
                            Some(json!({ "objectId": object.id, "field": "smoothShading" })),
                        ),
                    }),
                }),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection",
                    "Selection Mode",
                    &envelope.fixture.selection.mode,
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.transform".into(),
            label: "Transform".into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field(
                "lowpoly-play-inspector.transform.tool",
                "Tool",
                &envelope.runtime.transform_tool,
            )],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖LowpolyPlayApp
struct LowpolyPlayApp;

impl PluginApp for LowpolyPlayApp {
    fn app_id(&self) -> &str {
        LOWPOLY_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("lowpoly envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(fixture) = serde_json::from_str::<LowpolyFixture>(json_text) {
                        envelope.fixture = fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if envelope.fixture.objects.iter().any(|object| object.id == object_id) {
                    envelope.fixture.active_object_id = object_id.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelection" => {
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let ids: Vec<u32> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.fixture.selection.mode = mode.into();
                envelope.fixture.selection.ids = ids;
                return vec![set_document_op(&envelope)];
            }
            "toggleSelectionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("mesh");
                match kind {
                    "vertex" => envelope.fixture.selection.targets.vertex = !envelope.fixture.selection.targets.vertex,
                    "edge" => envelope.fixture.selection.targets.edge = !envelope.fixture.selection.targets.edge,
                    "face" => envelope.fixture.selection.targets.face = !envelope.fixture.selection.targets.face,
                    _ => envelope.fixture.selection.targets.mesh = !envelope.fixture.selection.targets.mesh,
                }
                return vec![set_document_op(&envelope)];
            }
            "setTransformTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("translate");
                envelope.runtime.transform_tool = tool.into();
                return vec![set_document_op(&envelope)];
            }
            "setActivePaintLayer" => {
                let layer_index = args
                    .and_then(|value| value.get("layerIndex"))
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0) as u32;
                envelope.runtime.active_paint_layer = layer_index;
                return vec![set_document_op(&envelope)];
            }
            "addPrimitive" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("box");
                let id = next_object_id();
                let object = LowpolyObject {
                    id: id.clone(),
                    name: format!("{kind} {}", envelope.fixture.objects.len() + 1),
                    transform: LowpolyTransform::default(),
                    smooth_shading: false,
                    mesh_json: json!({ "kind": kind }).to_string(),
                    paint_layers: vec![LowpolyPaintLayer {
                        name: "Base".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                    }],
                };
                envelope.fixture.objects.push(object);
                envelope.fixture.active_object_id = id;
                return vec![set_document_op(&envelope)];
            }
            "patchObject" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                for object in &mut envelope.fixture.objects {
                    if object.id != object_id {
                        continue;
                    }
                    match field {
                        "name" => {
                            if let Some(name) = value.as_ref().and_then(|entry| entry.as_str()) {
                                object.name = name.into();
                            }
                        }
                        "smoothShading" => {
                            object.smooth_shading =
                                value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading);
                        }
                        _ => {}
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "toggleSmooth" => {
                if let Some(object) = active_object(&envelope.fixture).map(|entry| entry.id.clone()) {
                    return self.handle_command(
                        "patchObject",
                        Some(&json!({ "objectId": object, "field": "smoothShading" })),
                        document_json,
                        _view_state,
                    );
                }
            }
            "extrude" | "inset" | "bevel" | "subdivide" | "triangulate" | "mirror" | "worldPointerDown" => {
                return Vec::new();
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            LOWPOLY_PLAY_BODY_MAIN => build_world_3d_scene(
                LOWPOLY_PLAY_SURFACE_MAIN,
                LOWPOLY_PLAY_APP_ID,
                World3dScene {
                    camera_json: default_camera_json(),
                    meshes_json: "[]".into(),
                    instances_json: world_instances_json(&envelope.fixture),
                    selection_json: "[]".into(),
                },
            ),
            LOWPOLY_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            LOWPOLY_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            LOWPOLY_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            LOWPOLY_PLAY_BODY_LAYERS => build_layers_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖LowpolyPlayApp

//#region 🔖Manifest
fn create_lowpoly_app() -> App {
    App::from_builder(
        App::builder(LOWPOLY_PLAY_APP_ID, "Lowpoly")
            .icon_id("box")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(LOWPOLY_PLAY_WINDOW_MAIN, "Model", LOWPOLY_PLAY_BODY_MAIN)
            .default_layout(create_default_layout(
                &[LOWPOLY_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Model".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                LOWPOLY_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                LOWPOLY_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                LOWPOLY_PLAY_BODY_INSPECTION,
            )
            .panel_tab("framework.panel.layers", "Layers", "workbench", LOWPOLY_PLAY_BODY_LAYERS),
    )
    .example("default", "Default", DEFAULT_FIXTURE_JSON)
    .program("lowpoly", "Lowpoly", "mesh")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("lowpoly", "Lowpoly", "0.1.0").register_app(create_lowpoly_app(), || Box::new(LowpolyPlayApp))
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
    fn renders_world_scene() {
        let app = LowpolyPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LOWPOLY_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn catalogue_lists_primitives() {
        let app = LowpolyPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LOWPOLY_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lowpoly-play-catalogue.box"));
        assert!(json.contains("Ico Sphere"));
    }

    #[test]
    fn add_primitive_creates_object() {
        let mut app = LowpolyPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addPrimitive", Some(&json!({ "kind": "sphere" })), &document, &ViewState::default());
        let envelope: LowpolyPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.objects.iter().any(|object| object.mesh_json.contains("sphere")));
    }

    #[test]
    fn set_active_object_switches_selection() {
        let mut app = LowpolyPlayApp;
        let mut envelope = parse_envelope(&app.initial_document_json());
        let second_id = next_object_id();
        envelope.fixture.objects.push(LowpolyObject {
            id: second_id.clone(),
            name: "Second".into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json: "{}".into(),
            paint_layers: Vec::new(),
        });
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command("setActiveObject", Some(&json!({ "objectId": second_id })), &document, &ViewState::default());
        let next: LowpolyPlayEnvelope = apply_ops(&envelope, &ops);
        assert_eq!(next.fixture.active_object_id, second_id);
    }

    fn apply_ops(envelope: &LowpolyPlayEnvelope, ops: &[String]) -> LowpolyPlayEnvelope {
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
