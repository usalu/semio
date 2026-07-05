//! 🧊 Puzzle 3D plugin — 3D puzzle assembly play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, export_mesh_glb_bytes, export_mesh_obj,
    merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_meshes_json_from_kinds, world3d_scene, world3d_selection_json, App,
    CommandDescriptor, MeshData, PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup,
    UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
const PUZZLE3D_PLAY_BODY_COMPOSITE: &str = "puzzle3d.play.composite";
const PUZZLE3D_PLAY_BODY_HIERARCHY: &str = "puzzle.3d.play.hierarchy";
const PUZZLE3D_PLAY_BODY_KINDS: &str = "puzzle.3d.play.kinds";
const PUZZLE3D_PLAY_BODY_INSPECTOR: &str = "puzzle.3d.play.inspector";
const PUZZLE3D_PLAY_WINDOW_MAIN: &str = "puzzle3d-main";
const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";

const PUZZLE3D_MESH_KIND: &str = "box";

const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../example/concrete-forest.3d.json");
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dCamera {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dObject {
    id: String,
    #[serde(default, rename = "objectKind")]
    object_kind: Option<String>,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dAttraction {
    id: String,
    attracting: String,
    attracted: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixture {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    camera: Puzzle3dCamera,
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSelection {
    #[serde(default)]
    object_ids: Vec<String>,
    #[serde(default)]
    vortex_ids: Vec<String>,
    #[serde(default)]
    attraction_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dRuntime {
    #[serde(default)]
    selection: Puzzle3dSelection,
    #[serde(default)]
    active_tool: String,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dEnvelope {
    fixture: Puzzle3dFixture,
    #[serde(default)]
    runtime: Puzzle3dRuntime,
}

static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

fn empty_fixture() -> Puzzle3dFixture {
    Puzzle3dFixture {
        schema: PUZZLE3D_FIXTURE_SCHEMA.into(),
        domain: "architecture".into(),
        camera: Puzzle3dCamera::default(),
        objects: Vec::new(),
        attractions: Vec::new(),
    }
}

fn default_envelope() -> Puzzle3dEnvelope {
    serde_json::from_str::<Puzzle3dFixture>(CONCRETE_FOREST_EXAMPLE_JSON)
        .map(|fixture| Puzzle3dEnvelope {
            fixture,
            runtime: Puzzle3dRuntime::default(),
        })
        .unwrap_or_else(|_| Puzzle3dEnvelope {
            fixture: empty_fixture(),
            runtime: Puzzle3dRuntime::default(),
        })
}

fn parse_envelope(document_json: &str) -> Puzzle3dEnvelope {
    if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(document_json) {
        return Puzzle3dEnvelope {
            fixture,
            runtime: Puzzle3dRuntime::default(),
        };
    }
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Puzzle3dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle3d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn camera_json(camera: &Puzzle3dCamera) -> String {
    json!({
        "x": camera.position.first().copied().unwrap_or(0.0),
        "y": camera.position.get(1).copied().unwrap_or(-5.0),
        "z": camera.position.get(2).copied().unwrap_or(3.0),
        "fov": 45.0,
    })
    .to_string()
}

fn world_instances_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
    let selection = &runtime.selection;
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .enumerate()
        .map(|(index, object)| {
            let selected = selection.object_ids.contains(&object.id);
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
            json!({
                "id": object.id,
                "meshId": PUZZLE3D_MESH_KIND,
                "position": [
                    object.origin.first().copied().unwrap_or(index as f64),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "scale": [1.0, 1.0, 1.0],
                "label": object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                "color": if selected { "#f59e0b" } else { "#94a3b8" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json() -> String {
    world3d_meshes_json_from_kinds(&[PUZZLE3D_MESH_KIND.into()])
}

fn world_selection_json(runtime: &Puzzle3dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selection.object_ids,
        runtime.hovered_object_id.as_deref(),
    )
}

fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
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

fn build_hierarchy_tree(envelope: &Puzzle3dEnvelope) -> UiNode {
    let object_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .objects
        .iter()
        .map(|object| {
            tree_item_with_command(
                format!("puzzle3d-object:{}", object.id),
                object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                Some("box"),
                puzzle3d_cmd(
                    "setSelection",
                    Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } })),
                ),
            )
        })
        .collect();
    let attraction_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .attractions
        .iter()
        .map(|attraction| {
            tree_item_with_command(
                format!("puzzle3d-attraction:{}", attraction.id),
                format!("{} → {}", attraction.attracting, attraction.attracted),
                Some("link"),
                puzzle3d_cmd(
                    "setSelection",
                    Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [attraction.id] } })),
                ),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "puzzle3d-play-hierarchy.objects".into(),
                label: Some("Objects".into()),
                default_open: Some(true),
                items: object_items,
            },
            UiTreeSectionNode {
                id: "puzzle3d-play-hierarchy.attractions".into(),
                label: Some("Attractions".into()),
                default_open: Some(false),
                items: attraction_items,
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_kinds_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "puzzle3d-play-kinds.objects".into(),
            label: Some("Object Kinds".into()),
            default_open: Some(true),
            items: vec![
                kind_item("Hexagonal Cut Concrete Forest Left"),
                kind_item("Hexagonal Cut Concrete Forest Right"),
            ],
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn kind_item(kind: &str) -> UiTreeItemNode {
    tree_item_with_command(
        format!("puzzle3d-kind:{kind}"),
        kind,
        Some("box"),
        puzzle3d_cmd("addObjectKind", Some(json!({ "objectKind": kind }))),
    )
}

fn build_inspector_tree(envelope: &Puzzle3dEnvelope) -> UiNode {
    if let Some(object_id) = envelope.runtime.selection.object_ids.first() {
        if let Some(object) = envelope.fixture.objects.iter().find(|entry| &entry.id == object_id) {
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
                id: "puzzle3d-play-inspector.object".into(),
                label: "Object".into(),
                default_open: None,
                fields: vec![
                    ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", "Id", &object.id),
                    ui_inspector_readonly_field(
                        "puzzle3d-play-inspector.object.kind",
                        "Kind",
                        object.object_kind.as_deref().unwrap_or(""),
                    ),
                    UiNode::Field(UiFieldNode {
                        id: "puzzle3d-play-inspector.object.origin".into(),
                        label: "Origin".into(),
                        child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                            id: "puzzle3d-play-inspector.object.origin.input".into(),
                            input_kind: "text".into(),
                            value: format!(
                                "{:.2}, {:.2}, {:.2}",
                                object.origin.first().copied().unwrap_or(0.0),
                                object.origin.get(1).copied().unwrap_or(0.0),
                                object.origin.get(2).copied().unwrap_or(0.0),
                            ),
                            placeholder: None,
                            commit: None,
                            on_change: puzzle3d_cmd("setSelection", None),
                        }),
                    }),
                ],
            }]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.fixture.schema)),
        ui_text(format!("Domain: {}", envelope.fixture.domain)),
        ui_text(format!("Objects: {}", envelope.fixture.objects.len())),
    ])
}
//#endregion 🔖Panels

//#region 🔖Puzzle3dPlayApp
struct Puzzle3dPlayApp;

impl PluginApp for Puzzle3dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE3D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle3d envelope json")
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
                    if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
                        envelope.fixture = fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    Puzzle3dEnvelope {
                        fixture: empty_fixture(),
                        runtime: Puzzle3dRuntime::default(),
                    }
                } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    default_envelope()
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setSelection" => {
                if let Some(selection) = args.and_then(|value| value.get("selection")) {
                    if let Ok(parsed) = serde_json::from_value(selection.clone()) {
                        envelope.runtime.selection = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("select");
                envelope.runtime.active_tool = tool.into();
                return vec![set_document_op(&envelope)];
            }
            "addObjectKind" => {
                let object_kind = args
                    .and_then(|value| value.get("objectKind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Object");
                let id = next_object_id();
                envelope.fixture.objects.push(Puzzle3dObject {
                    id: id.clone(),
                    object_kind: Some(object_kind.into()),
                    origin: [0.0, 0.0, 0.0],
                    mesh_url: None,
                });
                envelope.runtime.selection.object_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                let ids: Vec<String> = envelope.runtime.selection.object_ids.clone();
                envelope.fixture.objects.retain(|object| !ids.contains(&object.id));
                envelope.runtime.selection.object_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.fixture.camera = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selection.object_ids =
                    merge_world_selection_ids(&envelope.runtime.selection.object_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_object_id = args
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
            "worldPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            PUZZLE3D_PLAY_BODY_COMPOSITE => build_world_3d_scene(
                PUZZLE3D_PLAY_SURFACE_VIEWPORT,
                PUZZLE3D_PLAY_APP_ID,
                world3d_scene(
                    camera_json(&envelope.fixture.camera),
                    world_meshes_json(),
                    world_instances_json(&envelope.fixture, &envelope.runtime),
                    world_selection_json(&envelope.runtime),
                ),
            ),
            PUZZLE3D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            PUZZLE3D_PLAY_BODY_KINDS => build_kinds_tree(),
            PUZZLE3D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Puzzle3dPlayApp

//#region 🔖Manifest
fn create_puzzle3d_app() -> App {
    App::from_builder(
        App::builder(PUZZLE3D_PLAY_APP_ID, "Puzzle 3D")
            .icon_id("puzzle")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE)
            .default_layout(create_default_layout(
                &[PUZZLE3D_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Puzzle 3D".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                PUZZLE3D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PUZZLE3D_PLAY_BODY_KINDS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PUZZLE3D_PLAY_BODY_INSPECTOR,
            ),
    )
    .example(
        PUZZLE3D_EXAMPLE_CONCRETE_FOREST,
        "Concrete Forest",
        CONCRETE_FOREST_EXAMPLE_JSON,
    )
    .program("puzzle3d", "Puzzle 3D", "model")
}

fn bundle() -> PluginBundle {
    register_puzzle3d_exports();
    PluginBundle::new("puzzle3d", "Puzzle 3D", "0.1.0").register_app(create_puzzle3d_app(), || Box::new(Puzzle3dPlayApp))
}

fn register_puzzle3d_exports() {
    register_os_media_export_handler("3d.puzzle", OsMediaExportFormat::Obj, |_doc| {
        let mesh = mesh_from_kind(PUZZLE3D_MESH_KIND);
        let (data, mime_type) = export_mesh_obj(&mesh, "puzzle");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "puzzle.obj".into(),
        })
    });
    register_os_media_export_handler("3d.puzzle", OsMediaExportFormat::Glb, |_doc| {
        let mesh = mesh_from_kind(PUZZLE3D_MESH_KIND);
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "puzzle.glb".into(),
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
    fn renders_world_scene() {
        let app = Puzzle3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn concrete_forest_example_parses() {
        let envelope = default_envelope();
        assert_eq!(envelope.fixture.schema, PUZZLE3D_FIXTURE_SCHEMA);
        assert!(!envelope.fixture.objects.is_empty());
    }

    #[test]
    fn hierarchy_lists_objects() {
        let app = Puzzle3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle3d-object:"));
    }

    #[test]
    fn add_object_kind_appends_object() {
        let mut app = Puzzle3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addObjectKind",
            Some(&json!({ "objectKind": "Test Kind" })),
            &document,
            &ViewState::default(),
        );
        let envelope: Puzzle3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.objects.iter().any(|object| object.object_kind.as_deref() == Some("Test Kind")));
    }

    fn apply_ops(envelope: &Puzzle3dEnvelope, ops: &[String]) -> Puzzle3dEnvelope {
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
