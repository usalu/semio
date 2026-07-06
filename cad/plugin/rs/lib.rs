//! 📏 CAD plugin — spatial model play app bundled as a hot-swappable WASM component.

use cad_document::{empty_cad_projection, CadEnvelope, CadNode, CadOp, CadScene, CadStore, CAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    build_world_3d_scene, export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids, mesh_from_kind,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
    world3d_mesh_id_from_url, world3d_scene, world3d_selection_json, App,
    CommandDescriptor, MeshData, PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup,
    UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementOption,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::layout::WindowEngagementStatus;
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, DocumentVcsCommand};

//#region 🔖Constants
const CAD_PLAY_APP_ID: &str = "cad-play";
const CAD_PLAY_CONTROLLER_ID: &str = "cad-play";
const CAD_PLAY_BODY_SHAPE: &str = "cad.play.shape";
const CAD_PLAY_BODY_BUILDING: &str = "cad.play.building";
const CAD_PLAY_BODY_ENERGY: &str = "cad.play.energy";
const CAD_PLAY_BODY_STRUCTURE_CLASSIC: &str = "cad.play.structure-classic";
const CAD_PLAY_BODY_DOCUMENT: &str = "cad.play.document";
const CAD_PLAY_BODY_CATALOGUE: &str = "cad.play.catalogue";
const CAD_PLAY_BODY_PROPERTIES: &str = "cad.play.properties";
const CAD_PLAY_SURFACE_SHAPE: &str = "cad.play.scene3d/shape";
const CAD_PLAY_SURFACE_BUILDING: &str = "cad.play.scene3d/building";
const CAD_PLAY_SURFACE_ENERGY: &str = "cad.play.scene3d/energy";
const CAD_PLAY_SURFACE_STRUCTURE_CLASSIC: &str = "cad.play.scene3d/structure-classic";
const CAD_PLAY_WINDOW_SHAPE: &str = "cad-play-shape";
const CAD_PLAY_WINDOW_BUILDING: &str = "cad-play-building";
const CAD_PLAY_WINDOW_ENERGY: &str = "cad-play-energy";
const CAD_PLAY_WINDOW_STRUCTURE_CLASSIC: &str = "cad-play-structure-classic";
const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
const CAD_FALLBACK_MESH_KIND: &str = "box";

/// @emoji 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.
const CAD_MODEL_INDEX_BUILDING: usize = 0;
const CAD_MODEL_INDEX_ENERGY: usize = 1;
const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 2;
const CAD_MODEL_INDEX_SHAPE: usize = 3;

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const TYPOLOGY_MESH_URLS: &[(&str, &str)] = &[
    ("building.building.slab", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.column", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.beam", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.wall", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("spatial.shape.box", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
];

const TYPOLOGY_CATALOG: &[(&str, &str, &str)] = &[
    ("building.building.slab", "Slab", "square"),
    ("building.building.column", "Column", "columns"),
    ("building.building.beam", "Beam", "minus"),
    ("building.building.wall", "Wall", "panel-top"),
    ("spatial.shape.box", "Box", "box"),
];

const FOREST_LEFT_MODEL_JSON: &str =
    include_str!("../../asset/play/hexagonal-cut-concrete-forest-left.model.json");
//#endregion 🔖Constants

//#region 🔖BrepMeshes
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_engine::{block_on, BrepKernel, MeshTransfer};
use semio_framework_core::mesh_from_indexed;
use std::sync::{Mutex, OnceLock};

static CAD_BREP_KERNEL: OnceLock<Mutex<BrepkitKernel>> = OnceLock::new();

/// @emoji 📦 Universal fallback extent for typologies with no authored geometry to measure.
const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

fn cad_brep_kernel() -> &'static Mutex<BrepkitKernel> {
    CAD_BREP_KERNEL.get_or_init(|| Mutex::new(BrepkitKernel::new()))
}

/// @emoji 📐 Tessellates a typology's primitive sized from authored geometry (or a universal
/// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>) -> MeshData {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    let [ex, ey, ez] = extent.unwrap_or(CAD_DEFAULT_TYPOLOGY_EXTENT);
    let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
    let is_cylindrical = typology_mesh_kind(typology) == "cylinder";
    let handle = block_on(async {
        if is_cylindrical {
            kernel.cylinder_prim(width.max(depth) * 0.5, height).await
        } else {
            kernel.box_prim(width, depth, height).await
        }
    });
    let Ok(handle) = handle else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    let mesh: MeshTransfer = match block_on(kernel.tessellate(&handle, 0.1)) {
        Ok(mesh) => mesh,
        Err(_) => {
            let _ = block_on(kernel.dispose(&handle));
            return mesh_from_kind(typology_mesh_kind(typology));
        }
    };
    let _ = block_on(kernel.dispose(&handle));
    mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index)
}

/// @emoji 🎯 Centroid of an object's authored vertices (matched by id prefix), or the origin.
fn object_origin_from_vertices(object_id: &str, vertices: &[Value]) -> [f64; 3] {
    let bim_token = object_id.strip_prefix("object-").unwrap_or(object_id);
    let prefix = format!("{bim_token}-");
    let mut count = 0usize;
    let mut sum = [0.0f64; 3];
    for vertex in vertices {
        let vertex_id = vertex.get("id").and_then(|value| value.as_str()).unwrap_or("");
        if !vertex_id.starts_with(&prefix) {
            continue;
        }
        let Some(position) = vertex.get("position").and_then(|value| value.as_array()) else {
            continue;
        };
        if position.len() < 3 {
            continue;
        }
        sum[0] += position[0].as_f64().unwrap_or(0.0);
        sum[1] += position[1].as_f64().unwrap_or(0.0);
        sum[2] += position[2].as_f64().unwrap_or(0.0);
        count += 1;
    }
    if count == 0 {
        return [0.0, 0.0, 0.0];
    }
    let n = count as f64;
    [sum[0] / n, sum[1] / n, sum[2] / n]
}

/// @emoji 📏 Authored bounding-box extent of an object's vertices (matched by id prefix).
fn object_extent_from_vertices(object_id: &str, vertices: &[Value]) -> Option<[f64; 3]> {
    let bim_token = object_id.strip_prefix("object-").unwrap_or(object_id);
    let prefix = format!("{bim_token}-");
    let mut min = [f64::MAX; 3];
    let mut max = [f64::MIN; 3];
    let mut count = 0usize;
    for vertex in vertices {
        let vertex_id = vertex.get("id").and_then(|value| value.as_str()).unwrap_or("");
        if !vertex_id.starts_with(&prefix) {
            continue;
        }
        let Some(position) = vertex.get("position").and_then(|value| value.as_array()) else {
            continue;
        };
        if position.len() < 3 {
            continue;
        }
        for axis in 0..3 {
            let value = position[axis].as_f64().unwrap_or(0.0);
            min[axis] = min[axis].min(value);
            max[axis] = max[axis].max(value);
        }
        count += 1;
    }
    if count == 0 {
        return None;
    }
    Some([
        (max[0] - min[0]).max(0.05),
        (max[1] - min[1]).max(0.05),
        (max[2] - min[2]).max(0.05),
    ])
}

/// @emoji 🗃️ Reads one pane's objects (with origin/extent derived from authored geometry) from
/// the shared quad fixture's `models[modelIndex]` model definition.
fn cad_document_pane_objects(source_json: &str, model_index: usize) -> Vec<CadObject> {
    let Ok(root) = serde_json::from_str::<Value>(source_json) else {
        return Vec::new();
    };
    let Some(objects) = root
        .pointer(&format!("/models/{model_index}/model/objects"))
        .and_then(|value| value.as_array())
    else {
        return Vec::new();
    };
    let vertices = root
        .pointer(&format!("/models/{model_index}/model/geometry/vertices"))
        .and_then(|value| value.as_array())
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    objects
        .iter()
        .filter_map(|entry| {
            let object_id = entry.get("id")?.as_str()?;
            let typology = entry.get("typology")?.as_str()?;
            let label = object_id
                .split('-')
                .last()
                .map(str::to_string)
                .unwrap_or_else(|| object_id.to_string());
            Some(CadObject {
                id: object_id.into(),
                label,
                typology: typology.into(),
                visible: true,
                origin: object_origin_from_vertices(object_id, vertices),
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: TYPOLOGY_MESH_URLS
                    .iter()
                    .find(|(kind, _)| *kind == typology)
                    .map(|(_, url)| url.to_string()),
                extent: object_extent_from_vertices(object_id, vertices),
            })
        })
        .collect()
}
//#endregion 🔖BrepMeshes

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CadCamera {
    #[serde(default = "default_camera_position")]
    position: [f64; 3],
    #[serde(default = "default_camera_target")]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
    #[serde(default = "default_fov")]
    fov: f64,
}

fn default_camera_position() -> [f64; 3] {
    [12.0, -12.0, 8.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_transform_tool() -> String {
    "move".into()
}

fn typology_mesh_kind(typology: &str) -> &'static str {
    match typology {
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" => "cylinder",
        _ => "box",
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadObject {
    id: String,
    label: String,
    typology: String,
    #[serde(default)]
    visible: bool,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<[f64; 3]>,
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    /// @emoji 📏 Authored bounding-box size, used to derive brep primitive dimensions.
    #[serde(default)]
    extent: Option<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayDocument {
    schema: String,
    id: String,
    #[serde(default)]
    camera: CadCamera,
    /// @emoji 📐 Shape pane objects (also the default single-pane / addObject target).
    #[serde(default)]
    objects: Vec<CadObject>,
    #[serde(default)]
    nodes: Vec<CadNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_tool: Option<String>,
    /// @emoji 🏢 Building pane objects.
    #[serde(default)]
    building_objects: Vec<CadObject>,
    /// @emoji 🔥 Energy pane objects.
    #[serde(default)]
    energy_objects: Vec<CadObject>,
    /// @emoji 🏗️ Structure classic pane objects.
    #[serde(default)]
    structure_classic_objects: Vec<CadObject>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayRuntime {
    #[serde(default)]
    selected_object_ids: Vec<String>,
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
    /// @emoji 🕹️ Active gumball transform mode: "move" | "rotate" | "scale".
    #[serde(default = "default_transform_tool")]
    transform_tool: String,
}

impl Default for CadPlayRuntime {
    fn default() -> Self {
        Self {
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            transform_tool: default_transform_tool(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayEnvelope {
    document: CadPlayDocument,
    #[serde(default)]
    runtime: CadPlayRuntime,
    /// @emoji 🗄️ Node add/rename history, replayed through `vcs::DocumentVcsCommand`.
    #[serde(default = "default_cad_history")]
    history: CadEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
}

fn default_document() -> CadPlayDocument {
    CadPlayDocument {
        schema: "cad.document".into(),
        id: "cad".into(),
        camera: CadCamera {
            position: default_camera_position(),
            target: default_camera_target(),
            zoom: 1.0,
            fov: default_fov(),
        },
        objects: vec![CadObject {
            id: "object-box-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: Some("/mesh/hexagonal-cut-concrete-forest-left.glb".into()),
            extent: None,
        }],
        nodes: vec![
            CadNode {
                id: "node-root".into(),
                label: "Model".into(),
                kind: "group".into(),
            },
            CadNode {
                id: "node-box".into(),
                label: "Box".into(),
                kind: "solid".into(),
            },
        ],
        active_tool: Some("selectDirect".into()),
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
    }
}

/// @emoji 🪟 Builds the quad play document: shape/building/energy/structure-classic panes each
/// sourced from their own model definition inside the shared fixture JSON.
fn forest_play_document(source_json: &str, id: &str) -> CadPlayDocument {
    let shape_objects = cad_document_pane_objects(source_json, CAD_MODEL_INDEX_SHAPE);
    if shape_objects.is_empty() {
        return default_document();
    }
    CadPlayDocument {
        schema: "cad.document".into(),
        id: id.into(),
        camera: CadCamera {
            position: [12.0, -12.0, 8.0],
            target: [5.4, 2.34, 1.5],
            zoom: 1.0,
            fov: 50.0,
        },
        objects: shape_objects,
        nodes: vec![CadNode {
            id: "node-root".into(),
            label: "Concrete Forest Left".into(),
            kind: "group".into(),
        }],
        active_tool: Some("selectDirect".into()),
        building_objects: cad_document_pane_objects(source_json, CAD_MODEL_INDEX_BUILDING),
        energy_objects: cad_document_pane_objects(source_json, CAD_MODEL_INDEX_ENERGY),
        structure_classic_objects: cad_document_pane_objects(source_json, CAD_MODEL_INDEX_STRUCTURE_CLASSIC),
    }
}

fn seed_cad_history(document: &CadPlayDocument) -> CadEnvelope {
    create_document_vcs_envelope(
        CAD_DOCUMENT_SCHEMA,
        "cad-play-nodes",
        CadScene {
            schema: CAD_DOCUMENT_SCHEMA.into(),
            id: document.id.clone(),
            nodes: document.nodes.clone(),
        },
        None,
    )
}

fn default_cad_history() -> CadEnvelope {
    seed_cad_history(&default_document())
}

fn forest_play_envelope() -> CadPlayEnvelope {
    let document = forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT);
    CadPlayEnvelope {
        history: seed_cad_history(&document),
        document,
        runtime: CadPlayRuntime::default(),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
    }
}

fn default_envelope() -> CadPlayEnvelope {
    let document = default_document();
    CadPlayEnvelope {
        history: seed_cad_history(&document),
        document,
        runtime: CadPlayRuntime::default(),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> CadPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn next_cad_id(prefix: &str) -> String {
    let next = CAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn set_document_op(envelope: &CadPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn cad_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: CAD_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn camera_json(camera: &CadCamera) -> String {
    semio_framework_core::world3d_camera_json(camera.position, camera.target, camera.fov)
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|ids: &Vec<String>| !ids.is_empty())
        .unwrap_or_else(|| fallback.to_vec())
}

fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

//#region 🔖PaneHelpers
/// @emoji 🧭 The 4 pane object lists, in document field order: shape/building/energy/structure.
fn cad_pane_lists(document: &CadPlayDocument) -> [&Vec<CadObject>; 4] {
    [
        &document.objects,
        &document.building_objects,
        &document.energy_objects,
        &document.structure_classic_objects,
    ]
}

fn cad_pane_lists_mut(document: &mut CadPlayDocument) -> [&mut Vec<CadObject>; 4] {
    [
        &mut document.objects,
        &mut document.building_objects,
        &mut document.energy_objects,
        &mut document.structure_classic_objects,
    ]
}

/// @emoji 🌐 Objects across all 4 panes (ids are globally unique across the whole fixture).
fn cad_all_objects(document: &CadPlayDocument) -> impl Iterator<Item = &CadObject> {
    cad_pane_lists(document).into_iter().flatten()
}
//#endregion 🔖PaneHelpers

fn apply_cad_translate(envelope: &mut CadPlayEnvelope, args: Option<&Value>) -> bool {
    let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    for object in cad_pane_lists_mut(&mut envelope.document).into_iter().flatten() {
        if ids.contains(&object.id) {
            object.origin[0] += dx;
            object.origin[1] += dy;
            object.origin[2] += dz;
        }
    }
    !ids.is_empty()
}

fn apply_cad_rotate(envelope: &mut CadPlayEnvelope, args: Option<&Value>) -> bool {
    let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let delta = quat_from_axis_angle(ax, ay, az, angle);
    for object in cad_pane_lists_mut(&mut envelope.document).into_iter().flatten() {
        if ids.contains(&object.id) {
            let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            object.orientation = Some(quat_mul(delta, current));
        }
    }
    !ids.is_empty()
}

fn apply_cad_scale(envelope: &mut CadPlayEnvelope, args: Option<&Value>) -> bool {
    let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    for object in cad_pane_lists_mut(&mut envelope.document).into_iter().flatten() {
        if ids.contains(&object.id) {
            let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
            object.scale = Some([current[0] * sx, current[1] * sy, current[2] * sz]);
        }
    }
    !ids.is_empty()
}

fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    TYPOLOGY_MESH_URLS
        .iter()
        .find(|(typology, _)| *typology == object.typology)
        .map(|(_, url)| url.to_string())
}

fn collect_mesh_urls(objects: &[CadObject]) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in objects {
        if let Some(url) = resolve_object_mesh_url(object) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

fn object_scale_json(object: &CadObject) -> [f64; 3] {
    object.scale.unwrap_or([1.0, 1.0, 1.0])
}

//#region 🔖Gumball
/// @emoji 🕹️ Whether a visible gumball engagement should render for the current selection.
fn gumball_active(runtime: &CadPlayRuntime) -> bool {
    !runtime.selected_object_ids.is_empty()
}

/// @emoji 🎯 World-space pivot for the gumball: centroid of selected objects across all panes.
fn gumball_target_for(document: &CadPlayDocument, selected_ids: &[String]) -> Option<[f64; 3]> {
    let mut sum = [0.0; 3];
    let mut count = 0usize;
    for object in cad_all_objects(document) {
        if selected_ids.contains(&object.id) {
            sum[0] += object.origin[0];
            sum[1] += object.origin[1];
            sum[2] += object.origin[2];
            count += 1;
        }
    }
    if count == 0 {
        return None;
    }
    let n = count as f64;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}
//#endregion 🔖Gumball

fn world_instances_json(objects: &[CadObject], runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| typology_mesh_kind(&object.typology).to_string());
            let selected = runtime.selected_object_ids.contains(&object.id);
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
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
                "label": object.label,
                "color": if selected { "#3b82f6" } else { "#64748b" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(objects: &[CadObject]) -> String {
    let urls = collect_mesh_urls(objects);
    if !urls.is_empty() {
        return semio_framework_plugin::world3d_meshes_json_from_urls(&urls);
    }
    let mut kinds: Vec<String> = objects
        .iter()
        .map(|object| typology_mesh_kind(&object.typology).to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if kinds.is_empty() {
        kinds.push(CAD_FALLBACK_MESH_KIND.into());
    }
    let meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let representative = objects.iter().find(|object| typology_mesh_kind(&object.typology) == kind.as_str());
            let typology = representative.map(|object| object.typology.as_str()).unwrap_or("spatial.shape.box");
            let extent = representative.and_then(|object| object.extent);
            let data = typology_brep_mesh(typology, extent);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_selection_json(document: &CadPlayDocument, runtime: &CadPlayRuntime) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_object_ids,
        runtime.hovered_object_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("transformTool".into(), json!(runtime.transform_tool));
        object.insert("gumballActive".into(), json!(gumball_active(runtime)));
        if let Some(target) = gumball_target_for(document, &runtime.selected_object_ids) {
            object.insert("gumballTarget".into(), json!(target));
        }
    }
    value.to_string()
}

fn export_mesh_from_envelope(envelope: &CadPlayEnvelope) -> MeshData {
    let selected = cad_all_objects(&envelope.document)
        .find(|object| envelope.runtime.selected_object_ids.contains(&object.id));
    let typology = selected.map(|object| object.typology.as_str()).unwrap_or("spatial.shape.box");
    let extent = selected.and_then(|object| object.extent);
    typology_brep_mesh(typology, extent)
}

//#region 🔖NodeHistory
/// @emoji 🗄️ Reconstructs the node-history VCS store from the persisted envelope state.
fn cad_history_store(envelope: &CadPlayEnvelope) -> CadStore {
    let mut store = CadStore::new(envelope.history.clone());
    store.set_state(envelope.history.clone(), envelope.applied_edit_ids.clone(), envelope.redo_edit_ids.clone());
    store
}

/// @emoji 💾 Persists the store's materialized nodes + history + undo/redo stacks back onto the envelope.
fn sync_cad_history(envelope: &mut CadPlayEnvelope, store: &CadStore) {
    if let Ok(scene) = store.projection() {
        envelope.document.nodes = scene.nodes;
    }
    envelope.history = store.envelope().clone();
    envelope.applied_edit_ids = store.applied_edit_ids().to_vec();
    envelope.redo_edit_ids = store.redo_edit_ids().to_vec();
}
//#endregion 🔖NodeHistory
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

fn pane_document_section(label: &str, id_suffix: &str, objects: &[CadObject]) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: format!("cad-play-document.{id_suffix}"),
        label: Some(label.into()),
        default_open: Some(true),
        items: objects
            .iter()
            .map(|object| {
                tree_item_with_command(
                    format!("cad-object:{id_suffix}:{}", object.id),
                    object.label.clone(),
                    Some("box"),
                    cad_cmd("setSelection", Some(json!({ "objectIds": [object.id] }))),
                )
            })
            .collect(),
    }
}

fn build_document_tree(envelope: &CadPlayEnvelope) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = envelope
        .document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_command(
                format!("cad-node:{}", node.id),
                node.label.clone(),
                Some("git-branch"),
                cad_cmd("setNodeSelection", Some(json!({ "nodeIds": [node.id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            pane_document_section("Shape", "shape", &envelope.document.objects),
            pane_document_section("Building", "building", &envelope.document.building_objects),
            pane_document_section("Energy", "energy", &envelope.document.energy_objects),
            pane_document_section("Structure Classic", "structure-classic", &envelope.document.structure_classic_objects),
            UiTreeSectionNode {
                id: "cad-play-document.nodes".into(),
                label: Some("Nodes".into()),
                default_open: Some(true),
                items: node_items,
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = TYPOLOGY_CATALOG
        .iter()
        .map(|(typology, label, icon)| {
            tree_item_with_command(
                format!("cad-play-catalogue.{typology}"),
                *label,
                Some(icon),
                cad_cmd("addObject", Some(json!({ "typology": typology }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "cad-play-catalogue.typologies".into(),
            label: Some("Typologies".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_properties_panel(envelope: &CadPlayEnvelope) -> UiNode {
    if let Some(object_id) = envelope.runtime.selected_object_ids.first() {
        if let Some(object) = cad_all_objects(&envelope.document).find(|entry| &entry.id == object_id) {
            return ui_inspector_groups_to_tree(&[object_inspector_group(object)]);
        }
    }
    if let Some(node_id) = envelope.runtime.selected_node_ids.first() {
        if let Some(node) = envelope.document.nodes.iter().find(|entry| &entry.id == node_id) {
            return ui_inspector_groups_to_tree(&[node_inspector_group(node)]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.document.schema)),
        ui_text(format!(
            "Tool: {}",
            envelope
                .document
                .active_tool
                .clone()
                .unwrap_or_else(|| "selectDirect".into())
        )),
        ui_text(format!("Objects: {}", envelope.document.objects.len())),
    ])
}

fn object_inspector_group(object: &CadObject) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: "Object".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: object.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_cmd(
                        "patchObject",
                        Some(json!({ "objectId": object.id, "field": "label" })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("cad-play-inspector.object.typology", "Typology", &object.typology),
        ],
    }
}

fn node_inspector_group(node: &CadNode) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.node".into(),
        label: "Node".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.node.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "cad-play-inspector.node.label.input".into(),
                    input_kind: "text".into(),
                    value: node.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_cmd(
                        "renameNode",
                        Some(json!({ "nodeId": node.id })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("cad-play-inspector.node.kind", "Kind", &node.kind),
        ],
    }
}

/// @emoji 🕹️ Visible gumball engagement: move/rotate/scale toggle buttons + selection status.
fn cad_window_engagement(envelope: &CadPlayEnvelope) -> WindowEngagement {
    let transform = envelope.runtime.transform_tool.clone();
    let selected_count = envelope.runtime.selected_object_ids.len();
    WindowEngagement {
        session_active: Some(true),
        options: Some(vec![
            WindowEngagementOption {
                id: "cad.opt.move".into(),
                label: Some("Move".into()),
                icon_id: Some("move".into()),
                pressed: Some(transform == "move"),
                disabled: None,
                command: Some(cad_cmd("setTransformTool", Some(json!({ "tool": "move" })))),
            },
            WindowEngagementOption {
                id: "cad.opt.rotate".into(),
                label: Some("Rotate".into()),
                icon_id: Some("rotate-cw".into()),
                pressed: Some(transform == "rotate"),
                disabled: None,
                command: Some(cad_cmd("setTransformTool", Some(json!({ "tool": "rotate" })))),
            },
            WindowEngagementOption {
                id: "cad.opt.scale".into(),
                label: Some("Scale".into()),
                icon_id: Some("maximize-2".into()),
                pressed: Some(transform == "scale"),
                disabled: None,
                command: Some(cad_cmd("setTransformTool", Some(json!({ "tool": "scale" })))),
            },
        ]),
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "cad-status".into(),
            text: format!("{selected_count} selected"),
        }]),
        possible_engagements: None,
    }
}
//#endregion 🔖Panels

//#region 🔖CadApp
struct CadApp;

impl PluginApp for CadApp {
    fn app_id(&self) -> &str {
        CAD_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("cad envelope json")
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
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    let document = CadPlayDocument {
                        schema: "cad.document".into(),
                        id: "cad".into(),
                        camera: CadCamera {
                            position: default_camera_position(),
                            target: default_camera_target(),
                            zoom: 1.0,
                            fov: default_fov(),
                        },
                        objects: Vec::new(),
                        nodes: Vec::new(),
                        active_tool: Some("selectDirect".into()),
                        building_objects: Vec::new(),
                        energy_objects: Vec::new(),
                        structure_classic_objects: Vec::new(),
                    };
                    CadPlayEnvelope {
                        history: seed_cad_history(&document),
                        document,
                        runtime: CadPlayRuntime::default(),
                        applied_edit_ids: Vec::new(),
                        redo_edit_ids: Vec::new(),
                    }
                } else if example_id == "default" {
                    default_envelope()
                } else if example_id == CAD_EXAMPLE_FOREST_LEFT || example_id == "forest-left" {
                    forest_play_envelope()
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.document.active_tool = Some(tool.into());
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelection" => {
                let object_ids: Vec<String> = args
                    .and_then(|value| value.get("objectIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_object_ids = object_ids;
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setNodeSelection" => {
                let node_ids: Vec<String> = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_node_ids = node_ids;
                envelope.runtime.selected_object_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setTransformTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.transform_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "translateSelection" => {
                if apply_cad_translate(&mut envelope, args) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                if apply_cad_rotate(&mut envelope, args) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                if apply_cad_scale(&mut envelope, args) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "addObject" => {
                let typology = args.and_then(|value| value.get("typology")).and_then(|value| value.as_str()).unwrap_or("spatial.shape.box");
                let label = TYPOLOGY_CATALOG
                    .iter()
                    .find(|(id, _, _)| *id == typology)
                    .map(|(_, name, _)| *name)
                    .unwrap_or("Object");
                let id = next_cad_id("object");
                envelope.document.objects.push(CadObject {
                    id: id.clone(),
                    label: format!("{label} {}", envelope.document.objects.len() + 1),
                    typology: typology.into(),
                    visible: true,
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: TYPOLOGY_MESH_URLS
                        .iter()
                        .find(|(entry, _)| *entry == typology)
                        .map(|(_, url)| url.to_string()),
                    extent: None,
                });
                envelope.runtime.selected_object_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "patchObject" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                for object in cad_pane_lists_mut(&mut envelope.document).into_iter().flatten() {
                    if object.id != object_id {
                        continue;
                    }
                    if field == "label" {
                        if let Some(label) = value.as_ref().and_then(|entry| entry.as_str()) {
                            object.label = label.into();
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("solid");
                let id = next_cad_id("node");
                let label = format!("Node {}", envelope.document.nodes.len() + 1);
                let mut store = cad_history_store(&envelope);
                let node = CadNode { id: id.clone(), label, kind: kind.into() };
                if store
                    .dispatch(DocumentVcsCommand::Apply { operations: vec![CadOp::AddNode { node }], description: None })
                    .is_ok()
                {
                    sync_cad_history(&mut envelope, &store);
                    envelope.runtime.selected_node_ids = vec![id];
                    return vec![set_document_op(&envelope)];
                }
            }
            "renameNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if !node_id.is_empty() && !label.is_empty() {
                    let mut store = cad_history_store(&envelope);
                    if store
                        .dispatch(DocumentVcsCommand::Apply {
                            operations: vec![CadOp::RenameNode { node_id: node_id.into(), label: label.into() }],
                            description: None,
                        })
                        .is_ok()
                    {
                        sync_cad_history(&mut envelope, &store);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "undo" => {
                let mut store = cad_history_store(&envelope);
                if store.dispatch(DocumentVcsCommand::Undo).is_ok() {
                    sync_cad_history(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                let mut store = cad_history_store(&envelope);
                if store.dispatch(DocumentVcsCommand::Redo).is_ok() {
                    sync_cad_history(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_object_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_object_ids, &ids, merge);
                envelope.runtime.selected_node_ids.clear();
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
            CAD_PLAY_BODY_SHAPE => build_world_3d_scene(
                CAD_PLAY_SURFACE_SHAPE,
                CAD_PLAY_APP_ID,
                world3d_scene(
                    camera_json(&envelope.document.camera),
                    world_meshes_json(&envelope.document.objects),
                    world_instances_json(&envelope.document.objects, &envelope.runtime),
                    world_selection_json(&envelope.document, &envelope.runtime),
                ),
            ),
            CAD_PLAY_BODY_BUILDING => build_world_3d_scene(
                CAD_PLAY_SURFACE_BUILDING,
                CAD_PLAY_APP_ID,
                world3d_scene(
                    camera_json(&envelope.document.camera),
                    world_meshes_json(&envelope.document.building_objects),
                    world_instances_json(&envelope.document.building_objects, &envelope.runtime),
                    world_selection_json(&envelope.document, &envelope.runtime),
                ),
            ),
            CAD_PLAY_BODY_ENERGY => build_world_3d_scene(
                CAD_PLAY_SURFACE_ENERGY,
                CAD_PLAY_APP_ID,
                world3d_scene(
                    camera_json(&envelope.document.camera),
                    world_meshes_json(&envelope.document.energy_objects),
                    world_instances_json(&envelope.document.energy_objects, &envelope.runtime),
                    world_selection_json(&envelope.document, &envelope.runtime),
                ),
            ),
            CAD_PLAY_BODY_STRUCTURE_CLASSIC => build_world_3d_scene(
                CAD_PLAY_SURFACE_STRUCTURE_CLASSIC,
                CAD_PLAY_APP_ID,
                world3d_scene(
                    camera_json(&envelope.document.camera),
                    world_meshes_json(&envelope.document.structure_classic_objects),
                    world_instances_json(&envelope.document.structure_classic_objects, &envelope.runtime),
                    world_selection_json(&envelope.document, &envelope.runtime),
                ),
            ),
            CAD_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            CAD_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            CAD_PLAY_BODY_PROPERTIES => build_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        let engagement = cad_window_engagement(&envelope);
        HashMap::from([
            (CAD_PLAY_WINDOW_SHAPE.to_string(), engagement.clone()),
            (CAD_PLAY_WINDOW_BUILDING.to_string(), engagement.clone()),
            (CAD_PLAY_WINDOW_ENERGY.to_string(), engagement.clone()),
            (CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(), engagement),
        ])
    }
}
//#endregion 🔖CadApp

//#region 🔖Manifest
/// @emoji 🪟 One quadrant of the quad layout: a stack holding a single window kind.
fn cad_window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode {
            kind: "window".into(),
            window_kind_id: window_kind_id.into(),
            title: Some(title.into()),
            instance_id: None,
            template_id: None,
        }],
    })
}

/// @emoji 🪟 Quad play layout: shape/building left column, energy/structure classic right column.
fn cad_quad_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![
                        cad_window_stack(CAD_PLAY_WINDOW_SHAPE, "Shape", Some(0.5)),
                        cad_window_stack(CAD_PLAY_WINDOW_BUILDING, "Building", Some(0.5)),
                    ],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![
                        cad_window_stack(CAD_PLAY_WINDOW_ENERGY, "Energy", Some(0.5)),
                        cad_window_stack(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", Some(0.5)),
                    ],
                }),
            ],
        }),
    }
}

fn create_cad_app() -> App {
    App::from_builder(
        App::builder(CAD_PLAY_APP_ID, "CAD").document(["semio", "cad"])
            .icon_id("box")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(CAD_PLAY_WINDOW_SHAPE, "Shape", CAD_PLAY_BODY_SHAPE)
            .window_kind(CAD_PLAY_WINDOW_BUILDING, "Building", CAD_PLAY_BODY_BUILDING)
            .window_kind(CAD_PLAY_WINDOW_ENERGY, "Energy", CAD_PLAY_BODY_ENERGY)
            .window_kind(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", CAD_PLAY_BODY_STRUCTURE_CLASSIC)
            .default_layout(cad_quad_layout())
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                "workbench",
                CAD_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                CAD_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                CAD_PLAY_BODY_PROPERTIES,
            ),
    )
    .example("default", "Default", &serde_json::to_string(&default_envelope()).unwrap())
    .example(
        CAD_EXAMPLE_FOREST_LEFT,
        "Hexagonal Cut Concrete Forest Left",
        &serde_json::to_string(&forest_play_envelope()).unwrap(),
    )
    .program("cad", "CAD", "model")
}

fn bundle() -> PluginBundle {
    register_cad_exports();
    PluginBundle::new("cad", "CAD", "0.1.0").register_app(create_cad_app(), || Box::new(CadApp))
}

fn register_cad_exports() {
    register_os_media_export_handler("3d.cad", OsMediaExportFormat::Obj, |doc| {
        let envelope: CadPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (data, mime_type) = export_mesh_obj(&mesh, "cad");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "cad.obj".into(),
        })
    });
    register_os_media_export_handler("3d.cad", OsMediaExportFormat::Glb, |doc| {
        let envelope: CadPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "cad.glb".into(),
        })
    });
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn forest_example_uses_mesh_urls_and_origins() {
        let envelope = forest_play_envelope();
        let json = world_instances_json(&envelope.document.building_objects, &envelope.runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        assert!(json.contains("4.05") || json.contains("8.10"));
        let meshes = world_meshes_json(&envelope.document.building_objects);
        assert!(meshes.contains("hexagonal-cut-concrete-forest-left.glb"));
        assert!(envelope.document.building_objects.len() > 5);
    }

    #[test]
    fn quad_panes_each_populate_distinct_objects() {
        let envelope = forest_play_envelope();
        assert!(!envelope.document.objects.is_empty(), "shape pane");
        assert!(!envelope.document.building_objects.is_empty(), "building pane");
        assert!(!envelope.document.energy_objects.is_empty(), "energy pane");
        assert!(!envelope.document.structure_classic_objects.is_empty(), "structure classic pane");
    }

    #[test]
    fn renders_world_scene_for_each_pane() {
        let app = CadApp;
        let document = serde_json::to_string(&forest_play_envelope()).unwrap();
        for body_key in [
            CAD_PLAY_BODY_SHAPE,
            CAD_PLAY_BODY_BUILDING,
            CAD_PLAY_BODY_ENERGY,
            CAD_PLAY_BODY_STRUCTURE_CLASSIC,
        ] {
            let node = app.render(body_key, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
        }
    }

    #[test]
    fn document_lists_objects_and_nodes() {
        let app = CadApp;
        let document = app.initial_document_json();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-object:"));
        assert!(json.contains("cad-node:"));
    }

    #[test]
    fn add_object_command_appends_object() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addObject",
            Some(&json!({ "typology": "building.building.column" })),
            &document,
            &ViewState::default(),
        );
        let envelope: CadPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope
            .document
            .objects
            .iter()
            .any(|object| object.typology == "building.building.column"));
    }

    #[test]
    fn cad_document_schema_matches_domain() {
        let scene = empty_cad_projection();
        assert_eq!(scene.schema, CAD_DOCUMENT_SCHEMA);
    }

    #[test]
    fn gumball_fields_present_when_selection_active() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setSelection",
            Some(&json!({ "objectIds": ["object-box-1"] })),
            &document,
            &ViewState::default(),
        );
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let selection = world_selection_json(&envelope.document, &envelope.runtime);
        assert!(selection.contains("\"transformTool\":\"move\""));
        assert!(selection.contains("\"gumballActive\":true"));
        assert!(selection.contains("\"gumballTarget\""));
    }

    #[test]
    fn gumball_inactive_without_selection() {
        let app = CadApp;
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let selection = world_selection_json(&envelope.document, &envelope.runtime);
        assert!(selection.contains("\"gumballActive\":false"));
        assert!(!selection.contains("\"gumballTarget\""));
    }

    #[test]
    fn set_transform_tool_updates_runtime_and_engagement() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setTransformTool",
            Some(&json!({ "tool": "rotate" })),
            &document,
            &ViewState::default(),
        );
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.transform_tool, "rotate");
        let engagements = app.window_engagements(&serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let shape_engagement = engagements.get(CAD_PLAY_WINDOW_SHAPE).expect("shape engagement");
        let rotate_option = shape_engagement
            .options
            .as_ref()
            .and_then(|options| options.iter().find(|option| option.id == "cad.opt.rotate"))
            .expect("rotate option");
        assert_eq!(rotate_option.pressed, Some(true));
    }

    #[test]
    fn window_engagements_registered_for_all_four_panes() {
        let app = CadApp;
        let document = app.initial_document_json();
        let engagements = app.window_engagements(&document, &ViewState::default());
        for window_kind in [
            CAD_PLAY_WINDOW_SHAPE,
            CAD_PLAY_WINDOW_BUILDING,
            CAD_PLAY_WINDOW_ENERGY,
            CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
        ] {
            assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
        }
    }

    #[test]
    fn undo_redo_round_trips_added_node() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let before_count = parse_envelope(&document).document.nodes.len();

        let add_ops = app.handle_command("addNode", Some(&json!({ "kind": "solid" })), &document, &ViewState::default());
        let after_add = apply_ops(&parse_envelope(&document), &add_ops);
        assert_eq!(after_add.document.nodes.len(), before_count + 1);
        let after_add_json = serde_json::to_string(&after_add).unwrap();

        let undo_ops = app.handle_command("undo", None, &after_add_json, &ViewState::default());
        assert!(!undo_ops.is_empty(), "undo should produce an op");
        let after_undo = apply_ops(&after_add, &undo_ops);
        assert_eq!(after_undo.document.nodes.len(), before_count);
        let after_undo_json = serde_json::to_string(&after_undo).unwrap();

        let redo_ops = app.handle_command("redo", None, &after_undo_json, &ViewState::default());
        assert!(!redo_ops.is_empty(), "redo should produce an op");
        let after_redo = apply_ops(&after_undo, &redo_ops);
        assert_eq!(after_redo.document.nodes.len(), before_count + 1);
    }

    #[test]
    fn typology_extent_derives_from_authored_geometry() {
        let envelope = forest_play_envelope();
        let column = envelope
            .document
            .building_objects
            .iter()
            .find(|object| object.typology == "building.building.column")
            .expect("column object");
        let extent = column.extent.expect("column extent derived from geometry");
        assert!(extent[2] > 0.05, "authored column height should be measurable");
        assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
    }

    fn apply_ops(envelope: &CadPlayEnvelope, ops: &[String]) -> CadPlayEnvelope {
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
