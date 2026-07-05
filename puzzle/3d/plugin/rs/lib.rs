//! 🧊 Puzzle 3D plugin — 3D puzzle assembly play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, export_mesh_glb_bytes, export_mesh_obj,
    merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls,
    world3d_scene, world3d_selection_json, App, CommandDescriptor, MeshData, PluginApp, PluginBundle,
    UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use puzzle_3d::{BrushPlacePayload, Puzzle3dPrecomputeSession};
use std::collections::{HashMap, HashSet};
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
const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
const PUZZLE3D_ENGAGEMENT_TOOL_BRUSH: &str = "puzzle3d.tool.brush";
const PUZZLE3D_ENGAGEMENT_TOOL_SELECT: &str = "puzzle3d.tool.select";
const PUZZLE3D_ENGAGEMENT_TOOL_FILL: &str = "puzzle3d.tool.fill";

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dVortex {
    id: String,
    #[serde(default, rename = "vortexKind")]
    vortex_kind: Option<String>,
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    direction: Option<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dObject {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default, rename = "objectKind")]
    object_kind: Option<String>,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    vortices: Vec<Puzzle3dVortex>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixtureMeta {
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    kind_compatibility: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dTargetVolume {
    id: String,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
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
    meta: Puzzle3dFixtureMeta,
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
    #[serde(default, rename = "targetVolumes")]
    target_volumes: Vec<Puzzle3dTargetVolume>,
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
    #[serde(default)]
    target_volume_ids: Vec<String>,
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
    #[serde(default = "default_overlap_budget")]
    overlap_budget: f64,
    #[serde(default)]
    fill_count: u32,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default)]
    object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_kind_weights: HashMap<String, f64>,
}

fn default_overlap_budget() -> f64 {
    0.02
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
        meta: Puzzle3dFixtureMeta::default(),
        objects: Vec::new(),
        attractions: Vec::new(),
        target_volumes: Vec::new(),
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
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": 45.0,
    })
    .to_string()
}

fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    let kind_id = object.object_kind.as_deref()?;
    let catalogs = meta.kind_catalogs.as_ref()?;
    let objects = catalogs.get("objects")?.as_array()?;
    for entry in objects {
        if entry.get("id").and_then(|v| v.as_str()) == Some(kind_id) {
            return entry
                .get("meshUrl")
                .and_then(|v| v.as_str())
                .map(str::to_string);
        }
    }
    None
}

fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in &fixture.objects {
        if let Some(url) = resolve_object_mesh_url(object, &fixture.meta) {
            urls.insert(url);
        }
    }
    if let Some(catalogs) = fixture.meta.kind_catalogs.as_ref() {
        if let Some(objects) = catalogs.get("objects").and_then(|v| v.as_array()) {
            for entry in objects {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls.into_iter().collect()
}

fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match &object.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [
            values[0].as_f64().unwrap_or(1.0),
            values[1].as_f64().unwrap_or(1.0),
            values[2].as_f64().unwrap_or(1.0),
        ],
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_instances_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
    let selection = &runtime.selection;
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let selected = selection.object_ids.contains(&object.id);
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
            let mesh_id = resolve_object_mesh_url(object, &fixture.meta)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into());
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": object_scale_json(object),
                "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                "color": if selected { "#f59e0b" } else if hovered { "#fbbf24" } else { "#94a3b8" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(fixture))
}

fn scene_config_json(envelope: &Puzzle3dEnvelope) -> String {
    json!({
        "fixture": {
            "objects": envelope.fixture.objects,
            "attractions": envelope.fixture.attractions,
            "targetVolumes": envelope.fixture.target_volumes,
        },
        "kindCatalogs": envelope.fixture.meta.kind_catalogs,
        "kindCompatibility": envelope.fixture.meta.kind_compatibility.unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "hostRules": {},
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        }
    })
    .to_string()
}

fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dEnvelope) {
    let _ = session.set_scene(&scene_config_json(envelope));
}

fn world_selection_json(runtime: &Puzzle3dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selection.object_ids,
        runtime.hovered_object_id.as_deref(),
    )
}

fn fixture_from_engine_json(envelope: &Puzzle3dEnvelope, fixture_json: &str) -> Option<Puzzle3dEnvelope> {
    let parsed: Value = serde_json::from_str(fixture_json).ok()?;
    let mut next = envelope.clone();
    next.fixture.objects = serde_json::from_value(parsed.get("objects")?.clone()).ok()?;
    next.fixture.attractions = parsed
        .get("attractions")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    next.fixture.target_volumes = parsed
        .get("targetVolumes")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    Some(next)
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
struct Puzzle3dPlayApp {
    precompute: Puzzle3dPrecomputeSession,
}

impl Default for Puzzle3dPlayApp {
    fn default() -> Self {
        Self {
            precompute: Puzzle3dPrecomputeSession::new(),
        }
    }
}

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
                let mesh_url = envelope
                    .fixture
                    .meta
                    .kind_catalogs
                    .as_ref()
                    .and_then(|catalogs| {
                        catalogs.get("objects")?.as_array()?.iter().find_map(|entry| {
                            if entry.get("id").and_then(|v| v.as_str()) == Some(object_kind) {
                                entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string)
                            } else {
                                None
                            }
                        })
                    });
                envelope.fixture.objects.push(Puzzle3dObject {
                    id: id.clone(),
                    label: Some(object_kind.into()),
                    object_kind: Some(object_kind.into()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url,
                    vortices: Vec::new(),
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
            "engagementPossibleSelect" => {
                let possible_id = args
                    .and_then(|value| value.get("possibleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope.runtime.active_tool = match possible_id {
                    PUZZLE3D_ENGAGEMENT_TOOL_BRUSH => "brush",
                    PUZZLE3D_ENGAGEMENT_TOOL_FILL => "fill",
                    _ => "select",
                }
                .into();
                return vec![set_document_op(&envelope)];
            }
            "addBrushObject" => {
                sync_precompute_session(&mut self.precompute, &envelope);
                if let Some(payload_value) = args {
                    if let Ok(payload) = serde_json::from_value::<BrushPlacePayload>(payload_value.clone()) {
                        if let Ok(fixture_json) =
                            self.precompute.apply_brush_placement_rust(&serde_json::to_string(&payload).unwrap_or_default())
                        {
                            if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                                envelope = next;
                                return vec![set_document_op(&envelope)];
                            }
                        }
                    }
                }
            }
            "setFillCount" => {
                sync_precompute_session(&mut self.precompute, &envelope);
                let count = args
                    .and_then(|value| value.get("count"))
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0) as u32;
                envelope.runtime.fill_count = count;
                if count > 0 {
                    if let Ok(fixture_json) = self.precompute.apply_fill_count_rust(count) {
                        if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                            envelope = next;
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "setBrushPlacementOverlapBudget" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                    envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                    sync_precompute_session(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setObjectKindWeight" | "setVortexKindWeight" => {
                let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                if command == "setObjectKindWeight" {
                    envelope.runtime.object_kind_weights.insert(kind_id.into(), value);
                } else {
                    envelope.runtime.vortex_kind_weights.insert(kind_id.into(), value);
                }
                sync_precompute_session(&mut self.precompute, &envelope);
                return vec![set_document_op(&envelope)];
            }
            "cycleBrushCandidate" => {
                envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                return vec![set_document_op(&envelope)];
            }
            "registerBrushMesh" => {
                if let (Some(url), Some(positions), Some(indices)) = (
                    args.and_then(|v| v.get("url")).and_then(|v| v.as_str()),
                    args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()),
                    args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()),
                ) {
                    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                    self.precompute.register_mesh(url, &positions, &indices);
                }
                return Vec::new();
            }
            "worldPointerDown" => return Vec::new(),
            _ => {}
        };
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
                    world_meshes_json(&envelope.fixture),
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
    PluginBundle::new("puzzle3d", "Puzzle 3D", "0.1.0")
        .register_app(create_puzzle3d_app(), || Box::new(Puzzle3dPlayApp::default()))
}

fn register_puzzle3d_exports() {
    register_os_media_export_handler("3d.puzzle", OsMediaExportFormat::Obj, |_doc| {
        let mesh = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
        let (data, mime_type) = export_mesh_obj(&mesh, "puzzle");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "puzzle.obj".into(),
        })
    });
    register_os_media_export_handler("3d.puzzle", OsMediaExportFormat::Glb, |_doc| {
        let mesh = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
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
