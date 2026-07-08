//! 📏 CAD plugin — spatial model play app bundled as a hot-swappable WASM component.

mod interaction;
mod transformation;

use cad_document::{
    cad_all_objects, cad_find_object_pane, cad_pane_from_model_definition_id, cad_pane_objects,
    empty_cad_projection, CadCamera, CadEnvelope, CadNode, CadObject, CadObjectPatch, CadOp, CadPaneId,
    CadPrimitiveSlot, CadReference, CadReferencePatch, CadScene, CadStore, CAD_DOCUMENT_SCHEMA,
    CAD_PLAY_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{PanelGroup, 
    build_world_3d_scene, export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids, mesh_from_kind,
    tool_button, tool_collection, tool_separator, tool_toggle, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_mixed_vec3, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_chunking_json, world3d_mesh_id_from_url, world3d_scene_extended, world3d_selection_json, App,
    CommandDescriptor, MeshData, PluginApp, PluginBundle, ToolCategory, ToolNode, UiControlNode, UiFieldNode,
    UiInspectorFieldGroup, UiInputNode, UiNode, UiSelectItem, UiSelectNode, UiTreeItemAction, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput, WindowEngagementOption,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::layout::{WindowEngagementPossible, WindowEngagementStatus};
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use interaction::{
    apply_event, can_commit, commit_object, keyed_transitions, list_interactions_for_model_definition,
    parse_repl_line, preview_display_items, resolve_interaction_key, start_session, CadEngagementSession,
};
use transformation::{
    apply_from_building, apply_typology_fallback, run_derive_from_geometry, solid_for_object,
};
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
const CAD_MODEL_INDEX_SHAPE: usize = 0;
const CAD_MODEL_INDEX_BUILDING: usize = 1;
const CAD_MODEL_INDEX_ENERGY: usize = 2;
const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const TYPOLOGY_MESH_URLS: &[(&str, &str)] = &[
    ("building.building.slab", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.column", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.beam", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.wall", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("structure.structure.onewayreinforcedconcreteslab", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("structure.structure.reinforcedconcretecolumn", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("energy.energy.externalwall", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("spatial.shape.primitive.box", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
];

struct CadTypologyEntry {
    typology: &'static str,
    label: &'static str,
    icon: &'static str,
    model_definition_id: &'static str,
}

const TYPOLOGY_CATALOG: &[CadTypologyEntry] = &[
    CadTypologyEntry {
        typology: "spatial.shape.primitive.box",
        label: "Box",
        icon: "box",
        model_definition_id: CAD_MODEL_DEFINITION_SHAPE,
    },
    CadTypologyEntry {
        typology: "building.building.slab",
        label: "Slab",
        icon: "square",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.column",
        label: "Column",
        icon: "columns",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.beam",
        label: "Beam",
        icon: "minus",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.wall",
        label: "Wall",
        icon: "panel-top",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "energy.energy.externalwall",
        label: "External Wall",
        icon: "panel-top",
        model_definition_id: CAD_MODEL_DEFINITION_ENERGY,
    },
    CadTypologyEntry {
        typology: "structure.structure.onewayreinforcedconcreteslab",
        label: "Slab",
        icon: "square",
        model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
    },
    CadTypologyEntry {
        typology: "structure.structure.reinforcedconcretecolumn",
        label: "Column",
        icon: "columns",
        model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
    },
];

const FOREST_LEFT_MODEL_JSON: &str =
    include_str!("../../asset/play/hexagonal-cut-concrete-forest-left.model.json");

const CAD_MODEL_DEFINITION_SHAPE: &str = "spatial.shape";
const CAD_MODEL_DEFINITION_BUILDING: &str = "aec.building";
const CAD_MODEL_DEFINITION_ENERGY: &str = "aec.building.energy";
const CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC: &str = "aec.building.structure.classic";

const CAD_CONCRETE_FOREST_REFERENCE_URL: &str = "/cad-fixture/concrete-forest-reference.png";

struct CadTransformationSpec {
    id: &'static str,
    label: &'static str,
    source_model_definition_id: &'static str,
    target_model_definition_id: &'static str,
    mode: TransformationMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TransformationMode {
    DeriveFromGeometry,
    FromBuilding,
    TypologyFallback,
}

const CAD_TRANSFORMATION_SPECS: &[CadTransformationSpec] = &[
    CadTransformationSpec {
        id: "from_geometry",
        label: "From Geometry",
        source_model_definition_id: CAD_MODEL_DEFINITION_SHAPE,
        target_model_definition_id: CAD_MODEL_DEFINITION_ENERGY,
        mode: TransformationMode::DeriveFromGeometry,
    },
    CadTransformationSpec {
        id: "from_building",
        label: "From Building",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::FromBuilding,
    },
    CadTransformationSpec {
        id: "classic",
        label: "Classic",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::TypologyFallback,
    },
];
//#endregion 🔖Constants

//#region 🔖BrepMeshes
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_engine::{block_on, BrepKernel, MeshTransfer};
use semio_framework_core::{SurfaceKind, mesh_from_indexed;
use std::sync::{Mutex, OnceLock};

static CAD_BREP_KERNEL: OnceLock<Mutex<BrepkitKernel>> = OnceLock::new();

/// @emoji 📦 Universal fallback extent for typologies with no authored geometry to measure.
const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

fn cad_brep_kernel() -> &'static Mutex<BrepkitKernel> {
    CAD_BREP_KERNEL.get_or_init(|| Mutex::new(BrepkitKernel::new()))
}

/// @emoji 📐 Tessellates a typology's primitive sized from authored geometry (or a universal
/// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>, solid_handle: Option<&str>) -> MeshData {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    if let Some(handle_id) = solid_handle {
        let handle = kernel_3d_engine::GeometryHandle(handle_id.into());
        if let Ok(mesh) = block_on(kernel.tessellate(&handle, 0.1)) {
            return mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index);
        }
    }
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
                locked: false,
                origin: object_origin_from_vertices(object_id, vertices),
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: TYPOLOGY_MESH_URLS
                    .iter()
                    .find(|(kind, _)| *kind == typology)
                    .map(|(_, url)| url.to_string()),
                extent: object_extent_from_vertices(object_id, vertices),
                solid_handle: None,
                primitives: primitives_from_json(entry),
            })
        })
        .collect()
}

fn primitives_from_json(entry: &Value) -> Vec<CadPrimitiveSlot> {
    let Some(primitives) = entry.get("primitives") else {
        return Vec::new();
    };
    if let Some(map) = primitives.as_object() {
        return map
            .iter()
            .map(|(slot, value)| CadPrimitiveSlot {
                slot: slot.clone(),
                primitive_id: value.as_str().unwrap_or_default().into(),
                kind: slot.clone(),
            })
            .collect();
    }
    if let Some(rows) = primitives.as_array() {
        return rows
            .iter()
            .filter_map(|row| {
                let kind = row.get("kind")?.as_str()?;
                let primitive_id = row.get("id")?.as_str()?;
                let slot = row
                    .get("slot")
                    .and_then(|value| value.as_str())
                    .unwrap_or(kind);
                Some(CadPrimitiveSlot {
                    slot: slot.into(),
                    primitive_id: primitive_id.into(),
                    kind: kind.into(),
                })
            })
            .collect();
    }
    Vec::new()
}

fn forest_references_for_model_definitions() -> HashMap<String, Vec<CadReference>> {
    CadPaneId::all()
        .into_iter()
        .map(|pane| {
            (
                pane.model_definition_id().into(),
                vec![CadReference {
                    id: "ref-concrete-forest".into(),
                    source_url: CAD_CONCRETE_FOREST_REFERENCE_URL.into(),
                    media_kind: "image".into(),
                    origin: [-24.0, -18.0, 0.01],
                    orientation: None,
                    scale: None,
                    width_world: 22.0,
                    hidden: false,
                    locked: false,
                    opacity: Some(1.0),
                }],
            )
        })
        .collect()
}
//#endregion 🔖BrepMeshes

//#region 🔖Document
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
    #[serde(default = "default_transform_tool")]
    transform_tool: String,
    #[serde(default)]
    engagement_input: String,
    #[serde(default)]
    engagement_step: String,
    #[serde(default)]
    active_example_id: Option<String>,
    #[serde(default)]
    selected_reference_model_definition_id: Option<String>,
    #[serde(default)]
    selected_reference_id: Option<String>,
    #[serde(default)]
    engagement_pane: Option<String>,
    #[serde(default)]
    engagement_session: Option<CadEngagementSession>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pending_export: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pending_export_filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pending_export_mime: Option<String>,
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_transform_tool() -> String {
    "move".into()
}

impl Default for CadPlayRuntime {
    fn default() -> Self {
        Self {
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            transform_tool: default_transform_tool(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            engagement_pane: None,
            engagement_session: None,
            pending_export: None,
            pending_export_filename: None,
            pending_export_mime: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayEnvelope {
    document: CadScene,
    #[serde(default)]
    runtime: CadPlayRuntime,
    #[serde(default = "default_cad_history")]
    history: CadEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
}

fn typology_mesh_kind(typology: &str) -> &'static str {
    match typology {
        "building.building.column"
        | "structure.structure.reinforcedconcretecolumn"
        | "aec.building.column" => "cylinder",
        _ => "box",
    }
}

fn default_document() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        camera: CadCamera {
            position: [12.0, -12.0, 8.0],
            target: [0.0, 0.0, 0.0],
            zoom: 1.0,
            fov: 50.0,
        },
        objects: vec![CadObject {
            id: "object-box-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.primitive.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: Some("/mesh/hexagonal-cut-concrete-forest-left.glb".into()),
            extent: Some([1.0, 1.0, 1.0]),
            solid_handle: None,
            primitives: vec![CadPrimitiveSlot {
                slot: "solid".into(),
                primitive_id: "box-solid".into(),
                kind: "solid".into(),
            }],
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
        references_by_model_definition_id: HashMap::new(),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

/// @emoji 🪟 Builds the quad play document: shape/building/energy/structure-classic panes each
/// sourced from their own model definition inside the shared fixture JSON.
fn forest_play_document(source_json: &str, id: &str) -> CadScene {
    let shape_objects = cad_document_pane_objects(source_json, CAD_MODEL_INDEX_SHAPE);
    if shape_objects.is_empty() {
        return default_document();
    }
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
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
        references_by_model_definition_id: forest_references_for_model_definitions(),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

fn seed_cad_history(document: &CadScene) -> CadEnvelope {
    create_document_vcs_envelope(
        CAD_DOCUMENT_SCHEMA,
        "cad-play",
        document.clone(),
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
        runtime: CadPlayRuntime {
            active_example_id: Some(CAD_EXAMPLE_FOREST_LEFT.into()),
            ..CadPlayRuntime::default()
        },
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

//#region 🔖PaneHelpers
fn cad_pane_lists(document: &CadScene) -> [&Vec<CadObject>; 4] {
    [
        &document.objects,
        &document.building_objects,
        &document.energy_objects,
        &document.structure_classic_objects,
    ]
}

fn cad_pane_id_from_suffix(id_suffix: &str) -> CadPaneId {
    match id_suffix {
        "building" => CadPaneId::Building,
        "energy" => CadPaneId::Energy,
        "structure-classic" => CadPaneId::StructureClassic,
        _ => CadPaneId::Shape,
    }
}

fn cad_pane_suffix(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shape",
        CadPaneId::Building => "building",
        CadPaneId::Energy => "energy",
        CadPaneId::StructureClassic => "structure-classic",
    }
}

fn dispatch_cad_ops(envelope: &mut CadPlayEnvelope, operations: Vec<CadOp>) -> bool {
    if operations.is_empty() {
        return false;
    }
    let mut store = cad_history_store(envelope);
    if store
        .dispatch(DocumentVcsCommand::Apply {
            operations,
            description: None,
        })
        .is_ok()
    {
        sync_cad_history(envelope, &store);
        true
    } else {
        false
    }
}

fn qualified_transformation_id(model_definition_id: &str, transformation_id: &str) -> String {
    format!("{model_definition_id}.{transformation_id}")
}

fn transfers_to_for_model_definition(active_model_definition_id: &str) -> Vec<&'static CadTransformationSpec> {
    CAD_TRANSFORMATION_SPECS
        .iter()
        .filter(|spec| spec.source_model_definition_id == active_model_definition_id)
        .collect()
}

fn transfers_from_for_model_definition(active_model_definition_id: &str) -> Vec<&'static CadTransformationSpec> {
    CAD_TRANSFORMATION_SPECS
        .iter()
        .filter(|spec| spec.target_model_definition_id == active_model_definition_id)
        .collect()
}

fn ensure_object_solid_handle(kernel: &mut BrepkitKernel, object: &mut CadObject) {
    if object.solid_handle.is_some() {
        return;
    }
    if let Some(handle) = solid_for_object(kernel, object) {
        let primitive_id = handle.0.clone();
        object.solid_handle = Some(primitive_id.clone());
        if object.primitives.is_empty() {
            object.primitives.push(CadPrimitiveSlot {
                slot: "solid".into(),
                primitive_id,
                kind: "solid".into(),
            });
        }
    }
}

fn apply_transformation_to_envelope(envelope: &mut CadPlayEnvelope, qid: &str) -> bool {
    let Some((model_definition_id, transformation_id)) = qid.rsplit_once('.') else {
        return false;
    };
    let Some(spec) = CAD_TRANSFORMATION_SPECS.iter().find(|entry| {
        entry.source_model_definition_id == model_definition_id && entry.id == transformation_id
    }) else {
        return false;
    };
    let Some(source_pane) = cad_pane_from_model_definition_id(spec.source_model_definition_id) else {
        return false;
    };
    let Some(target_pane) = cad_pane_from_model_definition_id(spec.target_model_definition_id) else {
        return false;
    };
    let objects = if envelope.runtime.active_example_id.as_deref() == Some(CAD_EXAMPLE_FOREST_LEFT) {
        let model_index = match target_pane {
            CadPaneId::Building => CAD_MODEL_INDEX_BUILDING,
            CadPaneId::Energy => CAD_MODEL_INDEX_ENERGY,
            CadPaneId::StructureClassic => CAD_MODEL_INDEX_STRUCTURE_CLASSIC,
            CadPaneId::Shape => CAD_MODEL_INDEX_SHAPE,
        };
        cad_document_pane_objects(FOREST_LEFT_MODEL_JSON, model_index)
    } else {
        let source_objects: Vec<CadObject> = cad_pane_objects(&envelope.document, source_pane)
            .iter()
            .cloned()
            .collect();
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return false;
        };
        let mut prepared = source_objects;
        for object in &mut prepared {
            ensure_object_solid_handle(&mut kernel, object);
        }
        match spec.mode {
            TransformationMode::DeriveFromGeometry => {
                run_derive_from_geometry(&mut kernel, &prepared, "derived-energy")
            }
            TransformationMode::FromBuilding => apply_from_building(&prepared, "derived-structure"),
            TransformationMode::TypologyFallback => apply_typology_fallback(
                &prepared,
                &[
                    "building.building.slab",
                    "building.building.column",
                    "building.building.beam",
                    "building.building.wall",
                ],
                "derived-fallback",
            ),
        }
    };
    let ops_ok = dispatch_cad_ops(
        envelope,
        vec![CadOp::SetPaneObjects {
            pane: target_pane,
            objects,
        }],
    );
    if ops_ok {
        envelope.document.active_model_definition_id = spec.target_model_definition_id.into();
    }
    ops_ok
}

fn export_step_for_pane(envelope: &CadPlayEnvelope, pane: CadPaneId) -> Option<(String, String)> {
    let objects = cad_pane_objects(&envelope.document, pane);
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return None;
    };
    let mut solids = Vec::new();
    for object in objects {
        let mut next = object.clone();
        if let Some(handle) = solid_for_object(&mut kernel, &mut next) {
            solids.push(handle);
        }
    }
    if solids.is_empty() {
        return None;
    }
    let step = kernel.export_step_sync(&solids).ok()?;
    let stem = pane.model_definition_id().replace('.', "-");
    Some((format!("cad-{}.stp", stem), step))
}

fn export_step_modelspace(envelope: &CadPlayEnvelope) -> Option<(String, String)> {
    let Ok(kernel_mutex) = cad_brep_kernel().lock() else {
        return None;
    };
    let mut kernel = kernel_mutex;
    let mut solids = Vec::new();
    for pane in CadPaneId::all() {
        for object in cad_pane_objects(&envelope.document, pane) {
            let mut next = object.clone();
            if let Some(handle) = solid_for_object(&mut kernel, &mut next) {
                solids.push(handle);
            }
        }
    }
    if solids.is_empty() {
        return None;
    }
    let step = kernel.export_step_sync(&solids).ok()?;
    Some(("cad.modelspace.stp".into(), step))
}

fn export_spatial_json(envelope: &CadPlayEnvelope, mode: &str) -> Value {
    let models: Vec<Value> = CadPaneId::all()
        .into_iter()
        .map(|pane| {
            json!({
                "id": pane.model_definition_id(),
                "model": {
                    "schema": "spatial.model",
                    "revision": 1,
                    "objects": cad_pane_objects(&envelope.document, pane),
                }
            })
        })
        .collect();
    match mode {
        "selected" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                .unwrap_or(CadPaneId::Shape);
            let selected: Vec<&CadObject> = envelope
                .runtime
                .selected_object_ids
                .iter()
                .filter_map(|id| {
                    cad_all_objects(&envelope.document)
                        .find(|(object, _)| &object.id == id)
                        .map(|(object, _)| object)
                })
                .collect();
            let model = json!({
                "schema": "spatial.model",
                "revision": 1,
                "objects": selected,
            });
            let model_space = json!({
                "schema": "spatial.modelspace",
                "revision": 1,
                "models": [{
                    "id": pane.model_definition_id(),
                    "model": model,
                }],
            });
            json!({
                "model": model,
                "modelSpace": model_space,
                "activeModelDefinitionId": pane.model_definition_id(),
            })
        }
        "current" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                .unwrap_or(CadPaneId::Shape);
            json!({
                "schema": "spatial.model",
                "revision": 1,
                "modelDefinitionId": pane.model_definition_id(),
                "objects": cad_pane_objects(&envelope.document, pane),
            })
        }
        _ => json!({
            "schema": "spatial.modelspace",
            "revision": 1,
            "activeModelDefinitionId": envelope.document.active_model_definition_id,
            "models": models,
        }),
    }
}

fn unwrap_spatial_load_payload(raw: &Value) -> Option<Value> {
    if raw.get("modelSpace").is_some() {
        return raw.get("modelSpace").cloned();
    }
    if raw.get("model").is_some() {
        return raw.get("model").cloned();
    }
    if raw.get("raw").is_some() {
        return raw.get("raw").cloned();
    }
    Some(raw.clone())
}

fn scene_from_spatial_payload(payload: &Value) -> Option<CadScene> {
    if payload.get("schema").and_then(|value| value.as_str()) == Some("spatial.modelspace") {
        let models = payload.get("models")?.as_array()?;
        let mut scene = default_document();
        for entry in models {
            let model_definition_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("");
            let objects_value = entry.pointer("/model/objects")?;
            let objects: Vec<CadObject> = serde_json::from_value(objects_value.clone()).ok()?;
            match model_definition_id {
                CAD_MODEL_DEFINITION_SHAPE => scene.objects = objects,
                CAD_MODEL_DEFINITION_BUILDING => scene.building_objects = objects,
                CAD_MODEL_DEFINITION_ENERGY => scene.energy_objects = objects,
                CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC => scene.structure_classic_objects = objects,
                _ => {}
            }
        }
        if let Some(active) = payload.get("activeModelDefinitionId").and_then(|value| value.as_str()) {
            scene.active_model_definition_id = active.into();
        }
        return Some(scene);
    }
    if payload.get("schema").and_then(|value| value.as_str()) == Some("spatial.model") {
        let objects: Vec<CadObject> = serde_json::from_value(payload.get("objects")?.clone()).ok()?;
        let mut scene = default_document();
        let pane = payload
            .get("modelDefinitionId")
            .and_then(|value| value.as_str())
            .and_then(cad_pane_from_model_definition_id)
            .unwrap_or(CadPaneId::Shape);
        match pane {
            CadPaneId::Shape => scene.objects = objects,
            CadPaneId::Building => scene.building_objects = objects,
            CadPaneId::Energy => scene.energy_objects = objects,
            CadPaneId::StructureClassic => scene.structure_classic_objects = objects,
        }
        scene.active_model_definition_id = pane.model_definition_id().into();
        return Some(scene);
    }
    None
}

fn export_download_ops(envelope: &CadPlayEnvelope) -> Vec<String> {
    let Some(data) = envelope.runtime.pending_export.clone() else {
        return Vec::new();
    };
    let filename = envelope
        .runtime
        .pending_export_filename
        .clone()
        .unwrap_or_else(|| "cad.spatial.json".into());
    let mime_type = envelope
        .runtime
        .pending_export_mime
        .clone()
        .unwrap_or_else(|| "application/json".into());
    let payload = match data {
        Value::String(text) => text,
        other => serde_json::to_string(&other).unwrap_or_default(),
    };
    vec![json!({
        "op": "downloadMediaExport",
        "filename": filename,
        "mimeType": mime_type,
        "data": payload,
    })
    .to_string()]
}
//#endregion 🔖PaneHelpers

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
fn gumball_target_for(document: &CadScene, selected_ids: &[String]) -> Option<[f64; 3]> {
    let mut sum = [0.0; 3];
    let mut count = 0usize;
    for (object, _) in cad_all_objects(document) {
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
            let typology = representative.map(|object| object.typology.as_str()).unwrap_or("spatial.shape.primitive.box");
            let extent = representative.and_then(|object| object.extent);
            let solid_handle = representative.and_then(|object| object.solid_handle.as_deref());
            let data = typology_brep_mesh(typology, extent, solid_handle);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_selection_json(document: &CadScene, runtime: &CadPlayRuntime) -> String {
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

fn world_references_json(document: &CadScene, pane: CadPaneId) -> Option<String> {
    let references = document
        .references_by_model_definition_id
        .get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<Value> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source_url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
                "opacity": reference.opacity.unwrap_or(1.0),
            })
        })
        .collect();
    Some(serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()))
}

fn build_world_scene_for_pane(envelope: &CadPlayEnvelope, pane: CadPaneId, surface_id: &str) -> UiNode {
    let objects = cad_pane_objects(&envelope.document, pane);
    let preview = envelope
        .runtime
        .engagement_session
        .as_ref()
        .filter(|session| session.pane == pane)
        .map(preview_display_items)
        .filter(|items| !items.is_empty())
        .map(|items| serde_json::to_string(&items).unwrap_or_else(|_| "[]".into()));
    build_world_3d_scene(
        surface_id,
        CAD_PLAY_APP_ID,
        world3d_scene_extended(
            camera_json(&envelope.document.camera),
            world_meshes_json(objects),
            world_instances_json(objects, &envelope.runtime),
            world_selection_json(&envelope.document, &envelope.runtime),
            None,
            None,
            None,
            world_references_json(&envelope.document, pane),
            None,
            preview,
            None,
            Some(world3d_chunking_json(256.0, 8000.0)),
        ),
    )
}

fn export_mesh_from_envelope(envelope: &CadPlayEnvelope) -> MeshData {
    let selected = cad_all_objects(&envelope.document)
        .find(|(object, _)| envelope.runtime.selected_object_ids.contains(&object.id));
    let typology = selected
        .map(|(object, _)| object.typology.as_str())
        .unwrap_or("spatial.shape.primitive.box");
    let extent = selected.and_then(|(object, _)| object.extent);
    let solid_handle = selected.and_then(|(object, _)| object.solid_handle.as_deref());
    typology_brep_mesh(typology, extent, solid_handle)
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
        envelope.document = scene;
    }
    envelope.history = store.envelope().clone();
    envelope.applied_edit_ids = store.applied_edit_ids().to_vec();
    envelope.redo_edit_ids = store.redo_edit_ids().to_vec();
}
//#endregion 🔖NodeHistory
//#endregion 🔖Document

//#region 🔖Panels
fn object_tree_item(id_suffix: &str, object: &CadObject) -> UiTreeItemNode {
    let primitive_items: Vec<UiTreeItemNode> = object
        .primitives
        .iter()
        .map(|primitive| {
            let mut item = tree_item_with_command(
                format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id),
                format!("{}: {}", primitive.slot, primitive.primitive_id),
                Some("hexagon"),
                cad_cmd(
                    "setPrimitiveSelection",
                    Some(json!({
                        "objectId": object.id,
                        "primitiveId": primitive.primitive_id,
                        "kind": primitive.kind,
                    })),
                ),
            );
            item.hover_command = Some(cad_cmd("worldHover", Some(json!({ "id": object.id }))));
            item.unhover_command = Some(cad_cmd("worldHover", None));
            item
        })
        .collect();
    let mut item = tree_item_with_command(
        format!("cad-object:{id_suffix}:{}", object.id),
        object.label.clone(),
        Some("box"),
        cad_cmd("setSelection", Some(json!({ "objectIds": [object.id] }))),
    );
    item.hover_command = Some(cad_cmd("worldHover", Some(json!({ "id": object.id }))));
    item.unhover_command = Some(cad_cmd("worldHover", None));
    item.is_hidden = Some(!object.visible);
    item.draggable = Some(!object.locked);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if object.visible { "eye-off" } else { "eye" }.into(),
            label: Some(if object.visible { "Hide" } else { "Show" }.into()),
            command: cad_cmd(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "hidden", "value": object.visible })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: if object.locked { "unlock" } else { "lock" }.into(),
            label: Some(if object.locked { "Unlock" } else { "Lock" }.into()),
            command: cad_cmd(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "locked", "value": !object.locked })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "copy".into(),
            label: Some("Duplicate".into()),
            command: cad_cmd("duplicateObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "trash-2".into(),
            label: Some("Delete".into()),
            command: cad_cmd("deleteObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
    ]);
    if !primitive_items.is_empty() {
        item.items = Some(primitive_items);
        item.default_open = Some(false);
    }
    item
}

fn reference_tree_item(model_definition_id: &str, reference: &CadReference) -> UiTreeItemNode {
    let mut item = tree_item_with_command(
        format!("cad-reference:{model_definition_id}:{}", reference.id),
        reference.id.clone(),
        Some("image"),
        cad_cmd(
            "setReferenceSelection",
            Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
        ),
    );
    item.description = Some(reference.source_url.clone());
    item.hover_command = Some(cad_cmd(
        "referenceHover",
        Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
    ));
    item.unhover_command = Some(cad_cmd("referenceHover", None));
    item.is_hidden = Some(reference.hidden);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if reference.hidden { "eye" } else { "eye-off" }.into(),
            label: Some(if reference.hidden { "Show" } else { "Hide" }.into()),
            command: cad_cmd(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "hidden",
                    "value": !reference.hidden,
                })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: if reference.locked { "unlock" } else { "lock" }.into(),
            label: Some(if reference.locked { "Unlock" } else { "Lock" }.into()),
            command: cad_cmd(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "locked",
                    "value": !reference.locked,
                })),
            ),
            reveal_on_hover: Some(true),
        },
    ]);
    item
}

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
        items: objects.iter().map(|object| object_tree_item(id_suffix, object)).collect(),
    }
}

fn references_section(model_definition_id: &str, references: &[CadReference]) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: format!("cad-play-document.references.{model_definition_id}"),
        label: Some("References".into()),
        default_open: Some(false),
        items: if references.is_empty() {
            vec![tree_item_with_command(
                format!("cad-play-document.references.{model_definition_id}.empty"),
                "(none)",
                None,
                cad_cmd("noop", None),
            )]
        } else {
            references
                .iter()
                .map(|reference| reference_tree_item(model_definition_id, reference))
                .collect()
        },
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
    let mut sections = vec![
        pane_document_section("Shape", "shape", &envelope.document.objects),
        references_section(
            CAD_MODEL_DEFINITION_SHAPE,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_SHAPE)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
        ),
        pane_document_section("Building", "building", &envelope.document.building_objects),
        references_section(
            CAD_MODEL_DEFINITION_BUILDING,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_BUILDING)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
        ),
        pane_document_section("Energy", "energy", &envelope.document.energy_objects),
        references_section(
            CAD_MODEL_DEFINITION_ENERGY,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_ENERGY)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
        ),
        pane_document_section(
            "Structure Classic",
            "structure-classic",
            &envelope.document.structure_classic_objects,
        ),
        references_section(
            CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
        ),
        UiTreeSectionNode {
            id: "cad-play-document.nodes".into(),
            label: Some("Nodes".into()),
            default_open: Some(true),
            items: node_items,
        },
    ];
    let _ = &mut sections;
    UiNode::Tree(UiTreeNode {
        sections,
        selected_ids: None,
        highlighted_ids: envelope
            .runtime
            .hovered_object_id
            .as_ref()
            .map(|id| vec![format!("cad-object:shape:{id}")]),
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = TYPOLOGY_CATALOG
        .iter()
        .map(|entry| {
            tree_item_with_command(
                format!("cad-play-catalogue.{}", entry.typology),
                entry.label,
                Some(entry.icon),
                cad_cmd("addObject", Some(json!({ "typology": entry.typology }))),
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
    if !envelope.runtime.selected_object_ids.is_empty() {
        let selected: Vec<&CadObject> = envelope
            .runtime
            .selected_object_ids
            .iter()
            .filter_map(|id| {
                cad_all_objects(&envelope.document)
                    .find(|(object, _)| &object.id == id)
                    .map(|(object, _)| object)
            })
            .collect();
        if !selected.is_empty() {
            return ui_inspector_groups_to_tree(&[object_inspector_group(&selected)]);
        }
    }
    if let (Some(model_definition_id), Some(reference_id)) = (
        envelope.runtime.selected_reference_model_definition_id.as_deref(),
        envelope.runtime.selected_reference_id.as_deref(),
    ) {
        if let Some(reference) = envelope
            .document
            .references_by_model_definition_id
            .get(model_definition_id)
            .and_then(|rows| rows.iter().find(|row| row.id == reference_id))
        {
            return ui_inspector_groups_to_tree(&[reference_inspector_group(
                model_definition_id,
                reference,
            )]);
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

fn inspector_number_field(
    id: &str,
    label: &str,
    values: &[f64],
    object_ids: &[String],
    field: &str,
) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform {
                mixed.value.to_string()
            } else {
                String::new()
            },
            placeholder: if mixed.uniform { None } else { Some("—".into()) },
            commit: None,
            on_change: cad_cmd(
                "patchSelection",
                Some(json!({ "objectIds": object_ids, "field": field })),
            ),
        }),
    })
}

fn inspector_vec3_field(
    id: &str,
    label: &str,
    values: &[[f64; 3]],
    object_ids: &[String],
    field: &str,
) -> UiNode {
    let mixed = ui_inspector_mixed_vec3(values);
    let value = mixed.value.unwrap_or([0.0, 0.0, 0.0]);
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: if mixed.uniform {
                format!("[{}, {}, {}]", value[0], value[1], value[2])
            } else {
                String::new()
            },
            placeholder: if mixed.uniform { None } else { Some("—".into()) },
            commit: None,
            on_change: cad_cmd(
                "patchSelection",
                Some(json!({ "objectIds": object_ids, "field": field })),
            ),
        }),
    })
}

fn object_inspector_group(objects: &[&CadObject]) -> UiInspectorFieldGroup {
    let object_ids: Vec<String> = objects.iter().map(|object| object.id.clone()).collect();
    let labels: Vec<String> = objects.iter().map(|object| object.label.clone()).collect();
    let typologies: Vec<String> = objects.iter().map(|object| object.typology.clone()).collect();
    let hidden: Vec<bool> = objects.iter().map(|object| !object.visible).collect();
    let locked: Vec<bool> = objects.iter().map(|object| object.locked).collect();
    let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
    let scales: Vec<[f64; 3]> = objects
        .iter()
        .map(|object| object.scale.unwrap_or([1.0, 1.0, 1.0]))
        .collect();
    let label_mixed = ui_inspector_mixed_text(&labels);
    let typology_mixed = ui_inspector_mixed_text(&typologies);
    let hidden_mixed = ui_inspector_mixed_toggle(&hidden);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: if objects.len() == 1 {
            "Object".into()
        } else {
            format!("{} Objects", objects.len())
        },
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: label_mixed.value.clone(),
                    placeholder: label_mixed.placeholder.clone(),
                    commit: None,
                    on_change: cad_cmd(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "label" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.typology".into(),
                label: "Typology".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "cad-play-inspector.object.typology.select".into(),
                    value: typology_mixed.value.clone(),
                    items: TYPOLOGY_CATALOG
                        .iter()
                        .map(|entry| UiSelectItem {
                            value: entry.typology.into(),
                            label: entry.label.into(),
                        })
                        .collect(),
                    placeholder: typology_mixed.placeholder.clone(),
                    on_change: cad_cmd(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "typology" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.hidden".into(),
                label: "Hidden".into(),
                child: UiControlNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.hidden.toggle".into(),
                    icon_id: "eye-off".into(),
                    pressed: hidden_mixed.pressed,
                    text: None,
                    on_change: cad_cmd(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "hidden" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.locked".into(),
                label: "Locked".into(),
                child: UiControlNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.pressed,
                    text: None,
                    on_change: cad_cmd(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "locked" })),
                    ),
                }),
            }),
            inspector_vec3_field(
                "cad-play-inspector.object.origin",
                "Position",
                &origins,
                &object_ids,
                "origin",
            ),
            inspector_vec3_field(
                "cad-play-inspector.object.scale",
                "Scale",
                &scales,
                &object_ids,
                "scale",
            ),
        ],
    }
}

fn reference_inspector_group(model_definition_id: &str, reference: &CadReference) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.reference".into(),
        label: "Reference".into(),
        default_open: None,
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.reference.id", "Id", &reference.id),
            ui_inspector_readonly_field(
                "cad-play-inspector.reference.source",
                "Source",
                &reference.source_url,
            ),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.reference.widthWorld".into(),
                label: "Width (world)".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "cad-play-inspector.reference.widthWorld.input".into(),
                    input_kind: "number".into(),
                    value: reference.width_world.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_cmd(
                        "patchCadPlayReference",
                        Some(json!({
                            "modelDefinitionId": model_definition_id,
                            "referenceId": reference.id,
                            "field": "widthWorld",
                        })),
                    ),
                }),
            }),
            inspector_vec3_field(
                "cad-play-inspector.reference.origin",
                "Position",
                &[reference.origin],
                &[reference.id.clone()],
                "origin",
            ),
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

fn cad_window_engagement(envelope: &CadPlayEnvelope, pane: CadPaneId) -> WindowEngagement {
    let transform = envelope.runtime.transform_tool.clone();
    let selected_count = envelope.runtime.selected_object_ids.len();
    let model_definition_id = pane.model_definition_id();
    let session_active = envelope.runtime.engagement_session.is_some();
    let possible_engagements: Vec<WindowEngagementPossible> =
        if let Some(session) = envelope.runtime.engagement_session.as_ref() {
            keyed_transitions(session)
                .into_iter()
                .map(|transition| WindowEngagementPossible {
                    id: transition.event_kind.clone(),
                    label: transition.label,
                    detail: Some(transition.key),
                    command: Some(cad_cmd(
                        "engagementPossibleSelect",
                        Some(json!({
                            "pane": cad_pane_suffix(pane),
                            "possibleId": transition.event_kind,
                        })),
                    )),
                })
                .collect()
        } else {
            list_interactions_for_model_definition(model_definition_id)
                .into_iter()
                .map(|entry| WindowEngagementPossible {
                    id: entry.id.into(),
                    label: entry.label.into(),
                    detail: Some(entry.key.into()),
                    command: Some(cad_cmd(
                        "engagementPossibleSelect",
                        Some(json!({ "pane": cad_pane_suffix(pane), "possibleId": entry.id })),
                    )),
                })
                .collect()
        };
    let step_text = envelope
        .runtime
        .engagement_session
        .as_ref()
        .map(|session| session.state.clone())
        .unwrap_or_else(|| envelope.runtime.engagement_step.clone());
    WindowEngagement {
        session_active: Some(session_active || true),
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
        input: Some(WindowEngagementInput {
            id: Some("engagement-input".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some("Command".into()),
            disabled: None,
            on_change: Some(cad_cmd(
                "engagementInput",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_submit: Some(cad_cmd(
                "engagementSubmit",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_repeat_last: Some(cad_cmd(
                "engagementRepeatLast",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_abort: Some(cad_cmd(
                "engagementAbort",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus {
                id: "cad-status".into(),
                text: format!("{selected_count} selected"),
            },
            WindowEngagementStatus {
                id: "cad-step".into(),
                text: format!("Step: {step_text}"),
            },
            WindowEngagementStatus {
                id: "cad-response".into(),
                text: envelope
                    .runtime
                    .engagement_session
                    .as_ref()
                    .and_then(|session| session.last_response.clone())
                    .unwrap_or_else(|| "OK".into()),
            },
        ]),
        possible_engagements: Some(possible_engagements),
    }
}

fn build_cad_play_toolbar(envelope: &CadPlayEnvelope) -> Vec<ToolNode> {
    let active = envelope.document.active_model_definition_id.as_str();
    let view_tools: Vec<ToolNode> = CadPaneId::all()
        .into_iter()
        .enumerate()
        .map(|(index, pane)| {
            tool_toggle(
                format!("cad.play.view.{}", pane.model_definition_id()),
                "box",
                pane.model_definition_id(),
                active == pane.model_definition_id(),
                cad_cmd(
                    "focusModelDefinition",
                    Some(json!({ "modelDefinitionId": pane.model_definition_id() })),
                ),
            )
            .with_order(index as u32)
        })
        .collect();
    let save_tools = vec![
        tool_button(
            "cad.play.save.selected",
            "save",
            "Selected",
            cad_cmd("saveSelected", None),
        )
        .with_disabled(envelope.runtime.selected_object_ids.is_empty()),
        tool_button(
            "cad.play.save.modelspace",
            "hard-drive",
            "Model space",
            cad_cmd("saveInPlay", None),
        ),
        tool_button(
            "cad.play.save.current",
            "save",
            "Current",
            cad_cmd("saveCurrent", None),
        ),
        tool_button(
            "cad.play.save.load",
            "folder-open",
            "Load",
            cad_cmd("loadRawRequest", None),
        ),
    ];
    let transfers_to = transfers_to_for_model_definition(active);
    let transfers_from = transfers_from_for_model_definition(active);
    let mut transfer_tools: Vec<ToolNode> = transfers_to
        .iter()
        .enumerate()
        .map(|(index, spec)| {
            tool_button(
                format!(
                    "cad.play.transfer.to.{}",
                    qualified_transformation_id(spec.source_model_definition_id, spec.id)
                ),
                "arrow-right",
                format!("→ {}", spec.label),
                cad_cmd(
                    "applyTransformation",
                    Some(json!({
                        "qid": qualified_transformation_id(spec.source_model_definition_id, spec.id),
                    })),
                ),
            )
            .with_order(index as u32)
        })
        .collect();
    if !transfers_to.is_empty() && !transfers_from.is_empty() {
        transfer_tools.push(tool_separator("cad.play.transfer.separator"));
    }
    transfer_tools.extend(transfers_from.iter().enumerate().map(|(index, spec)| {
        tool_button(
            format!(
                "cad.play.transfer.from.{}",
                qualified_transformation_id(spec.source_model_definition_id, spec.id)
            ),
            "arrow-left",
            format!("← {}", spec.label),
            cad_cmd(
                "applyTransformation",
                Some(json!({
                    "qid": qualified_transformation_id(spec.source_model_definition_id, spec.id),
                })),
            ),
        )
        .with_order((transfers_to.len() + index + 1) as u32)
    }));
    let mut tools = vec![
        tool_collection("view", "layout-grid", "View", view_tools).with_category(ToolCategory::Tools),
        tool_collection("save", "save", "Save", save_tools).with_category(ToolCategory::Commands),
    ];
    if !transfer_tools.is_empty() {
        tools.push(
            tool_collection(
                "transfer",
                "arrow-right-left",
                "Transfer",
                transfer_tools,
            )
            .with_category(ToolCategory::Commands),
        );
    }
    tools
}

trait ToolNodeExt {
    fn with_pressed(self, pressed: bool) -> Self;
    fn with_order(self, order: u32) -> Self;
    fn with_disabled(self, disabled: bool) -> Self;
}

impl ToolNodeExt for ToolNode {
    fn with_pressed(mut self, pressed: bool) -> Self {
        if let ToolNode::Toggle { pressed: slot, .. } = &mut self {
            *slot = Some(pressed);
        }
        self
    }

    fn with_order(mut self, order: u32) -> Self {
        match &mut self {
            ToolNode::Button { order: slot, .. }
            | ToolNode::Toggle { order: slot, .. }
            | ToolNode::Collection { order: slot, .. }
            | ToolNode::Separator { order: slot, .. } => *slot = Some(order),
        }
        self
    }

    fn with_disabled(mut self, disabled: bool) -> Self {
        match &mut self {
            ToolNode::Button { disabled: slot, .. }
            | ToolNode::Toggle { disabled: slot, .. }
            | ToolNode::Collection { disabled: slot, .. }
            | ToolNode::Separator { disabled: slot, .. } => *slot = Some(disabled),
        }
        self
    }
}
//#endregion 🔖Panels

fn object_patch_from_field(field: &str, value: Option<&Value>) -> Option<CadObjectPatch> {
    match field {
        "label" | "name" => value
            .and_then(|entry| entry.as_str())
            .map(|label| CadObjectPatch {
                label: Some(label.into()),
                ..Default::default()
            }),
        "typology" => value
            .and_then(|entry| entry.as_str())
            .map(|typology| CadObjectPatch {
                typology: Some(typology.into()),
                ..Default::default()
            }),
        "hidden" => value
            .and_then(|entry| entry.as_bool())
            .map(|hidden| CadObjectPatch {
                visible: Some(!hidden),
                ..Default::default()
            }),
        "locked" => value.and_then(|entry| entry.as_bool()).map(|locked| CadObjectPatch {
            locked: Some(locked),
            ..Default::default()
        }),
        "origin" => value.and_then(parse_vec3_value).map(|origin| CadObjectPatch {
            origin: Some(origin),
            ..Default::default()
        }),
        "scale" => value.and_then(parse_vec3_value).map(|scale| CadObjectPatch {
            scale: Some(scale),
            ..Default::default()
        }),
        _ => None,
    }
}

fn parse_vec3_value(value: &Value) -> Option<[f64; 3]> {
    if let Some(array) = value.as_array() {
        if array.len() >= 3 {
            return Some([
                array[0].as_f64().unwrap_or(0.0),
                array[1].as_f64().unwrap_or(0.0),
                array[2].as_f64().unwrap_or(0.0),
            ]);
        }
    }
    if let Some(text) = value.as_str() {
        let trimmed = text.trim().trim_start_matches('[').trim_end_matches(']');
        let parts: Vec<f64> = trimmed
            .split(',')
            .filter_map(|part| part.trim().parse().ok())
            .collect();
        if parts.len() >= 3 {
            return Some([parts[0], parts[1], parts[2]]);
        }
    }
    None
}

fn patch_objects_in_envelope(
    envelope: &mut CadPlayEnvelope,
    object_ids: &[String],
    field: &str,
    value: Option<&Value>,
) -> bool {
    let patch = match object_patch_from_field(field, value) {
        Some(patch) => patch,
        None => return false,
    };
    let mut operations = Vec::new();
    for object_id in object_ids {
        let Some(pane) = cad_find_object_pane(&envelope.document, object_id) else {
            continue;
        };
        operations.push(CadOp::PatchObject {
            pane,
            object_id: object_id.clone(),
            patch: patch.clone(),
        });
    }
    dispatch_cad_ops(envelope, operations)
}

fn make_object_for_typology(typology: &str, label_count: usize, pane: CadPaneId) -> CadObject {
    let label = TYPOLOGY_CATALOG
        .iter()
        .find(|entry| entry.typology == typology)
        .map(|entry| entry.label)
        .unwrap_or("Object");
    let extent = match typology {
        t if t.contains("column") => Some([0.5, 0.5, 3.0]),
        t if t.contains("slab") => Some([4.0, 4.0, 0.25]),
        t if t.contains("wall") => Some([4.0, 0.2, 3.0]),
        _ => Some([1.0, 1.0, 1.0]),
    };
    let mut object = CadObject {
        id: next_cad_id("object"),
        label: format!("{label} {}", label_count + 1),
        typology: typology.into(),
        visible: true,
        locked: false,
        origin: [0.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: TYPOLOGY_MESH_URLS
            .iter()
            .find(|(entry, _)| *entry == typology)
            .map(|(_, url)| url.to_string()),
        extent,
        solid_handle: None,
        primitives: Vec::new(),
    };
    if let Ok(mut kernel) = cad_brep_kernel().lock() {
        ensure_object_solid_handle(&mut kernel, &mut object);
    }
    let _ = pane;
    object
}

fn engagement_submit_line(envelope: &mut CadPlayEnvelope, pane: CadPaneId) -> bool {
    let input = envelope.runtime.engagement_input.trim();
    if input.is_empty() {
        envelope.runtime.engagement_step = "Idle".into();
        return false;
    }
    let model_definition_id = pane.model_definition_id();
    if let Some((event_kind, payload)) = parse_repl_line(input) {
        if let Some(session) = envelope.runtime.engagement_session.as_mut() {
            if apply_event(session, &event_kind, payload.as_ref()) {
                envelope.runtime.engagement_step = session.state.clone();
                let ready = can_commit(session);
                let session_snapshot = if ready { Some(session.clone()) } else { None };
                if let Some(session_snapshot) = session_snapshot {
                    let label_count = cad_pane_objects(&envelope.document, pane).len();
                    if let Ok(mut kernel) = cad_brep_kernel().lock() {
                        if let Some(object) = commit_object(
                            &mut kernel,
                            &session_snapshot,
                            label_count,
                            |prefix| next_cad_id(prefix),
                        ) {
                            let id = object.id.clone();
                            if dispatch_cad_ops(
                                envelope,
                                vec![CadOp::AddObject { pane, object }],
                            ) {
                                envelope.runtime.selected_object_ids = vec![id];
                                envelope.runtime.engagement_input.clear();
                                envelope.runtime.engagement_session = None;
                                envelope.runtime.engagement_step = "Idle".into();
                                return true;
                            }
                        }
                    }
                }
                return true;
            }
        }
        if let Some(entry) = resolve_interaction_key(&event_kind, model_definition_id) {
            envelope.runtime.engagement_session = start_session(entry.id, pane);
            if let Some(session) = envelope.runtime.engagement_session.as_mut() {
                let _ = apply_event(session, "start", None);
            }
            envelope.runtime.engagement_step = envelope
                .runtime
                .engagement_session
                .as_ref()
                .map(|session| session.state.clone())
                .unwrap_or_else(|| "Idle".into());
            envelope.runtime.engagement_input.clear();
            return true;
        }
        if let Some(session) = envelope.runtime.engagement_session.as_mut() {
            for transition in keyed_transitions(session) {
                if transition.key.eq_ignore_ascii_case(input) || transition.event_kind.eq_ignore_ascii_case(input) {
                    if apply_event(session, &transition.event_kind, None) {
                        envelope.runtime.engagement_step = session.state.clone();
                        envelope.runtime.engagement_input.clear();
                        return true;
                    }
                }
            }
        }
    }
    envelope.runtime.engagement_step = format!("Unknown: {input}");
    false
}

//#region 🔖CadApp
struct CadApp;

impl PluginApp for CadApp {
    fn app_id(&self) -> &str {
        CAD_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("cad envelope json")
    }

    fn handle_command_patch_ops(
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
                    let document = default_document();
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
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                if dispatch_cad_ops(
                    &mut envelope,
                    vec![CadOp::TranslateObjects {
                        object_ids: ids,
                        dx,
                        dy,
                        dz,
                    }],
                ) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                if dispatch_cad_ops(
                    &mut envelope,
                    vec![CadOp::RotateObjects {
                        object_ids: ids,
                        ax,
                        ay,
                        az,
                        angle,
                    }],
                ) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_object_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                if dispatch_cad_ops(
                    &mut envelope,
                    vec![CadOp::ScaleObjects {
                        object_ids: ids,
                        sx,
                        sy,
                        sz,
                    }],
                ) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "addObject" => {
                let typology = args
                    .and_then(|value| value.get("typology"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("spatial.shape.primitive.box");
                let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                    .unwrap_or(CadPaneId::Shape);
                let object = make_object_for_typology(typology, cad_pane_objects(&envelope.document, pane).len(), pane);
                let id = object.id.clone();
                if dispatch_cad_ops(
                    &mut envelope,
                    vec![CadOp::AddObject { pane, object }],
                ) {
                    envelope.runtime.selected_object_ids = vec![id];
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchObject" | "patchSelection" => {
                let object_ids: Vec<String> = if command == "patchSelection" {
                    args.and_then(|value| value.get("objectIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_else(|| envelope.runtime.selected_object_ids.clone())
                } else {
                    args.and_then(|value| value.get("objectId"))
                        .and_then(|value| value.as_str())
                        .map(|id| vec![id.to_string()])
                        .unwrap_or_default()
                };
                let field = args
                    .and_then(|value| value.get("field"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                if patch_objects_in_envelope(&mut envelope, &object_ids, field, value) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "deleteObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if let Some(pane) = cad_find_object_pane(&envelope.document, object_id) {
                    if dispatch_cad_ops(
                        &mut envelope,
                        vec![CadOp::RemoveObject {
                            pane,
                            object_id: object_id.into(),
                        }],
                    ) {
                        envelope.runtime.selected_object_ids.retain(|id| id != object_id);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "duplicateObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let duplicate_target = cad_all_objects(&envelope.document)
                    .find(|(object, _)| object.id == object_id)
                    .map(|(object, pane)| (object.clone(), pane));
                if let Some((mut duplicate, pane)) = duplicate_target {
                    duplicate.id = next_cad_id("object");
                    duplicate.label = format!("{} copy", duplicate.label);
                    if dispatch_cad_ops(
                        &mut envelope,
                        vec![CadOp::AddObject {
                            pane,
                            object: duplicate.clone(),
                        }],
                    ) {
                        envelope.runtime.selected_object_ids = vec![duplicate.id];
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("solid");
                let id = next_cad_id("node");
                let label = format!("Node {}", envelope.document.nodes.len() + 1);
                let node = CadNode { id: id.clone(), label, kind: kind.into() };
                if dispatch_cad_ops(&mut envelope, vec![CadOp::AddNode { node }]) {
                    envelope.runtime.selected_node_ids = vec![id];
                    return vec![set_document_op(&envelope)];
                }
            }
            "renameNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if !node_id.is_empty() && !label.is_empty()
                    && dispatch_cad_ops(
                        &mut envelope,
                        vec![CadOp::RenameNode {
                            node_id: node_id.into(),
                            label: label.into(),
                        }],
                    )
                {
                    return vec![set_document_op(&envelope)];
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
            "focusModelDefinition" => {
                if let Some(model_definition_id) = args
                    .and_then(|value| value.get("modelDefinitionId"))
                    .and_then(|value| value.as_str())
                {
                    envelope.document.active_model_definition_id = model_definition_id.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "applyTransformation" => {
                let qid = args
                    .and_then(|value| value.get("qid"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if apply_transformation_to_envelope(&mut envelope, qid) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "saveSelected" => {
                envelope.runtime.pending_export = Some(export_spatial_json(&envelope, "selected"));
                envelope.runtime.pending_export_filename = Some("cad.selected.spatial.json".into());
                envelope.runtime.pending_export_mime = Some("application/json".into());
                let mut ops = vec![set_document_op(&envelope)];
                ops.extend(export_download_ops(&envelope));
                return ops;
            }
            "saveInPlay" => {
                if let Some((filename, step)) = export_step_modelspace(&envelope) {
                    envelope.runtime.pending_export = Some(Value::String(step));
                    envelope.runtime.pending_export_filename = Some(filename);
                    envelope.runtime.pending_export_mime = Some("application/step".into());
                } else {
                    envelope.runtime.pending_export = Some(export_spatial_json(&envelope, "modelspace"));
                    envelope.runtime.pending_export_filename = Some("cad.modelspace.spatial.json".into());
                    envelope.runtime.pending_export_mime = Some("application/json".into());
                }
                let mut ops = vec![set_document_op(&envelope)];
                ops.extend(export_download_ops(&envelope));
                return ops;
            }
            "saveCurrent" => {
                let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                    .unwrap_or(CadPaneId::Shape);
                if let Some((filename, step)) = export_step_for_pane(&envelope, pane) {
                    envelope.runtime.pending_export = Some(Value::String(step));
                    envelope.runtime.pending_export_filename = Some(filename);
                    envelope.runtime.pending_export_mime = Some("application/step".into());
                } else {
                    envelope.runtime.pending_export = Some(export_spatial_json(&envelope, "current"));
                    envelope.runtime.pending_export_filename = Some("cad.current.spatial.json".into());
                    envelope.runtime.pending_export_mime = Some("application/json".into());
                }
                let mut ops = vec![set_document_op(&envelope)];
                ops.extend(export_download_ops(&envelope));
                return ops;
            }
            "loadRawRequest" => {
                envelope.runtime.pending_export = None;
                envelope.runtime.pending_export_filename = None;
                envelope.runtime.pending_export_mime = None;
                return vec![json!({
                    "op": "requestFileOpen",
                    "accept": ".json,.spatial.json,.stp,.step",
                    "importCommand": "importSpatialJson",
                })
                .to_string()];
            }
            "importSpatialJson" => {
                let payload = args
                    .and_then(|value| value.get("payload").or_else(|| value.get("modelSpace")))
                    .cloned()
                    .or_else(|| args.cloned());
                if let Some(payload) = payload {
                    let unwrapped = unwrap_spatial_load_payload(&payload).unwrap_or(payload);
                    if let Some(scene) = scene_from_spatial_payload(&unwrapped) {
                        envelope.document = scene;
                        envelope.history = seed_cad_history(&envelope.document);
                        envelope.applied_edit_ids.clear();
                        envelope.redo_edit_ids.clear();
                        envelope.runtime.selected_object_ids.clear();
                        envelope.runtime.engagement_session = None;
                        return vec![set_document_op(&envelope)];
                    }
                    if let Ok(scene) = serde_json::from_value::<CadScene>(unwrapped) {
                        envelope.document = scene;
                        envelope.history = seed_cad_history(&envelope.document);
                        envelope.applied_edit_ids.clear();
                        envelope.redo_edit_ids.clear();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setReferenceSelection" => {
                envelope.runtime.selected_reference_model_definition_id = args
                    .and_then(|value| value.get("modelDefinitionId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                envelope.runtime.selected_reference_id = args
                    .and_then(|value| value.get("referenceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                envelope.runtime.selected_object_ids.clear();
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "referenceHover" => {
                envelope.runtime.hovered_object_id = args
                    .and_then(|value| value.get("referenceId"))
                    .and_then(|value| value.as_str())
                    .map(|id| format!("reference:{id}"));
                return vec![set_document_op(&envelope)];
            }
            "patchCadPlayReference" => {
                let model_definition_id = args
                    .and_then(|value| value.get("modelDefinitionId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let reference_id = args
                    .and_then(|value| value.get("referenceId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let field = args
                    .and_then(|value| value.get("field"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let patch = match field {
                    "hidden" => value.and_then(|entry| entry.as_bool()).map(|hidden| CadReferencePatch {
                        hidden: Some(hidden),
                        ..Default::default()
                    }),
                    "locked" => value.and_then(|entry| entry.as_bool()).map(|locked| CadReferencePatch {
                        locked: Some(locked),
                        ..Default::default()
                    }),
                    "widthWorld" => value.and_then(|entry| entry.as_f64()).map(|width_world| CadReferencePatch {
                        width_world: Some(width_world),
                        ..Default::default()
                    }),
                    "origin" => value.and_then(parse_vec3_value).map(|origin| CadReferencePatch {
                        origin: Some(origin),
                        ..Default::default()
                    }),
                    _ => None,
                };
                if let Some(patch) = patch {
                    if dispatch_cad_ops(
                        &mut envelope,
                        vec![CadOp::PatchReference {
                            model_definition_id: model_definition_id.into(),
                            reference_id: reference_id.into(),
                            patch,
                        }],
                    ) {
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "engagementInput" => {
                envelope.runtime.engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
                envelope.runtime.engagement_pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "engagementSubmit" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .unwrap_or(CadPaneId::Shape);
                if engagement_submit_line(&mut envelope, pane) {
                    return vec![set_document_op(&envelope)];
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementPossibleSelect" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .unwrap_or(CadPaneId::Shape);
                if let Some(possible_id) = args
                    .and_then(|value| value.get("possibleId"))
                    .and_then(|value| value.as_str())
                {
                    if let Some(session) = envelope.runtime.engagement_session.as_mut() {
                        if apply_event(session, possible_id, None) {
                            envelope.runtime.engagement_step = session.state.clone();
                        }
                    } else if let Some(entry) = interaction::interaction_by_id(possible_id) {
                        envelope.runtime.engagement_session = start_session(entry.id, pane);
                        if let Some(session) = envelope.runtime.engagement_session.as_mut() {
                            let _ = apply_event(session, "start", None);
                        }
                        envelope.runtime.engagement_step = envelope
                            .runtime
                            .engagement_session
                            .as_ref()
                            .map(|session| session.state.clone())
                            .unwrap_or_else(|| "Idle".into());
                    } else {
                        envelope.runtime.engagement_input = possible_id.into();
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementRepeatLast" | "engagementAbort" => {
                if command == "engagementAbort" {
                    envelope.runtime.engagement_input.clear();
                    envelope.runtime.engagement_session = None;
                }
                envelope.runtime.engagement_step = "Idle".into();
                return vec![set_document_op(&envelope)];
            }
            "engagementPointerDown" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .unwrap_or(CadPaneId::Shape);
                let point = args.and_then(|value| value.get("position"));
                if let Some(session) = envelope.runtime.engagement_session.as_mut() {
                    if apply_event(session, "pointer.down", point) {
                        envelope.runtime.engagement_step = session.state.clone();
                        if can_commit(session) {
                            let label_count = cad_pane_objects(&envelope.document, pane).len();
                            if let Ok(mut kernel) = cad_brep_kernel().lock() {
                                if let Some(object) =
                                    commit_object(&mut kernel, session, label_count, |prefix| next_cad_id(prefix))
                                {
                                    let id = object.id.clone();
                                    if dispatch_cad_ops(
                                        &mut envelope,
                                        vec![CadOp::AddObject { pane, object }],
                                    ) {
                                        envelope.runtime.selected_object_ids = vec![id];
                                        envelope.runtime.engagement_session = None;
                                        envelope.runtime.engagement_step = "Idle".into();
                                    }
                                }
                            }
                        }
                    }
                }
                let _ = pane;
                return vec![set_document_op(&envelope)];
            }
            "setPrimitiveSelection" => {
                if let Some(object_id) = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()) {
                    envelope.runtime.selected_object_ids = vec![object_id.into()];
                    envelope.runtime.selected_node_ids.clear();
                    return vec![set_document_op(&envelope)];
                }
            }
            "noop" | "worldPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn tools(&self, document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        build_cad_play_toolbar(&parse_envelope(document_json))
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            CAD_PLAY_BODY_SHAPE => build_world_scene_for_pane(&envelope, CadPaneId::Shape, CAD_PLAY_SURFACE_SHAPE),
            CAD_PLAY_BODY_BUILDING => {
                build_world_scene_for_pane(&envelope, CadPaneId::Building, CAD_PLAY_SURFACE_BUILDING)
            }
            CAD_PLAY_BODY_ENERGY => build_world_scene_for_pane(&envelope, CadPaneId::Energy, CAD_PLAY_SURFACE_ENERGY),
            CAD_PLAY_BODY_STRUCTURE_CLASSIC => build_world_scene_for_pane(
                &envelope,
                CadPaneId::StructureClassic,
                CAD_PLAY_SURFACE_STRUCTURE_CLASSIC,
            ),
            CAD_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            CAD_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            CAD_PLAY_BODY_PROPERTIES => build_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        HashMap::from([
            (
                CAD_PLAY_WINDOW_SHAPE.to_string(),
                cad_window_engagement(&envelope, CadPaneId::Shape),
            ),
            (
                CAD_PLAY_WINDOW_BUILDING.to_string(),
                cad_window_engagement(&envelope, CadPaneId::Building),
            ),
            (
                CAD_PLAY_WINDOW_ENERGY.to_string(),
                cad_window_engagement(&envelope, CadPaneId::Energy),
            ),
            (
                CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(),
                cad_window_engagement(&envelope, CadPaneId::StructureClassic),
            ),
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
            .window_kind(CAD_PLAY_WINDOW_SHAPE, "Shape", CAD_PLAY_BODY_SHAPE, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_BUILDING, "Building", CAD_PLAY_BODY_BUILDING, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_ENERGY, "Energy", CAD_PLAY_BODY_ENERGY, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", CAD_PLAY_BODY_STRUCTURE_CLASSIC, SurfaceKind::World3d)
            .default_layout(cad_quad_layout())
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                CAD_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                CAD_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
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
        let ops = app.handle_command_patch_ops(
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
            .any(|object| object.typology == "building.building.column")
            || envelope.document.building_objects.iter().any(|object| object.typology == "building.building.column"));
    }

    #[test]
    fn cad_document_schema_matches_domain() {
        let scene = empty_cad_projection();
        assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn undo_redo_round_trips_added_object() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let before_count = parse_envelope(&document).document.objects.len();
        let add_ops = app.handle_command_patch_ops(
            "addObject",
            Some(&json!({ "typology": "spatial.shape.primitive.box" })),
            &document,
            &ViewState::default(),
        );
        let after_add = apply_ops(&parse_envelope(&document), &add_ops);
        assert_eq!(after_add.document.objects.len(), before_count + 1);
        let after_add_json = serde_json::to_string(&after_add).unwrap();
        let undo_ops = app.handle_command_patch_ops("undo", None, &after_add_json, &ViewState::default());
        let after_undo = apply_ops(&after_add, &undo_ops);
        assert_eq!(after_undo.document.objects.len(), before_count);
    }

    #[test]
    fn toolbar_exposes_save_and_transfer_tools() {
        let app = CadApp;
        let tools = app.tools(&app.initial_document_json(), &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("cad.play.save.selected"));
        assert!(json.contains("cad.play.transfer.to"));
    }

    #[test]
    fn engagement_input_and_possible_engagements_present() {
        let app = CadApp;
        let engagements = app.window_engagements(&app.initial_document_json(), &ViewState::default());
        let shape = engagements.get(CAD_PLAY_WINDOW_SHAPE).expect("shape engagement");
        assert!(shape.input.is_some());
        assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    }

    #[test]
    fn forest_example_includes_reference_overlay() {
        let envelope = forest_play_envelope();
        let references = world_references_json(&envelope.document, CadPaneId::Shape).expect("references");
        assert!(references.contains("ref-concrete-forest"));
    }

    #[test]
    fn document_tree_includes_primitive_children() {
        let app = CadApp;
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, &app.initial_document_json(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-primitive:"));
        assert!(json.contains("hoverCommand"));
    }

    #[test]
    fn gumball_fields_present_when_selection_active() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
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
        let ops = app.handle_command_patch_ops(
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

        let add_ops = app.handle_command_patch_ops("addNode", Some(&json!({ "kind": "solid" })), &document, &ViewState::default());
        let after_add = apply_ops(&parse_envelope(&document), &add_ops);
        assert_eq!(after_add.document.nodes.len(), before_count + 1);
        let after_add_json = serde_json::to_string(&after_add).unwrap();

        let undo_ops = app.handle_command_patch_ops("undo", None, &after_add_json, &ViewState::default());
        assert!(!undo_ops.is_empty(), "undo should produce an op");
        let after_undo = apply_ops(&after_add, &undo_ops);
        assert_eq!(after_undo.document.nodes.len(), before_count);
        let after_undo_json = serde_json::to_string(&after_undo).unwrap();

        let redo_ops = app.handle_command_patch_ops("redo", None, &after_undo_json, &ViewState::default());
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

    #[test]
    fn derive_transformation_populates_energy_pane() {
        let mut app = CadApp;
        let mut envelope = parse_envelope(&app.initial_document_json());
        let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
        envelope.document.objects = vec![object];
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops(
            "applyTransformation",
            Some(&json!({ "qid": "spatial.shape.from_geometry" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&envelope, &ops);
        assert!(!next.document.energy_objects.is_empty());
        assert!(next
            .document
            .energy_objects
            .iter()
            .any(|object| object.typology.starts_with("energy.energy.")));
    }

    #[test]
    fn save_selected_emits_download_op() {
        let mut app = CadApp;
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.selected_object_ids = vec!["object-box-1".into()];
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("saveSelected", None, &document, &ViewState::default());
        assert!(ops.iter().any(|op| op.contains("downloadMediaExport")));
        assert!(ops.iter().any(|op| op.contains("activeModelDefinitionId")));
    }

    #[test]
    fn engagement_starts_box_interaction_session() {
        let mut app = CadApp;
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.engagement_input = "b".into();
        let ops = app.handle_command_patch_ops(
            "engagementSubmit",
            Some(&json!({ "pane": "shape" })),
            &serde_json::to_string(&envelope).unwrap(),
            &ViewState::default(),
        );
        let next = apply_ops(&envelope, &ops);
        assert!(next.runtime.engagement_session.is_some());
    }

    #[test]
    fn import_spatial_modelspace_round_trips() {
        let payload = json!({
            "schema": "spatial.modelspace",
            "revision": 1,
            "activeModelDefinitionId": "spatial.shape",
            "models": [{
                "id": "spatial.shape",
                "model": {
                    "schema": "spatial.model",
                    "revision": 1,
                    "objects": [{
                        "id": "object-imported",
                        "label": "Imported",
                        "typology": "spatial.shape.primitive.box",
                        "visible": true,
                        "locked": false,
                        "origin": [1.0, 2.0, 3.0],
                        "primitives": []
                    }]
                }
            }]
        });
        let scene = scene_from_spatial_payload(&payload).expect("scene");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].id, "object-imported");
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
