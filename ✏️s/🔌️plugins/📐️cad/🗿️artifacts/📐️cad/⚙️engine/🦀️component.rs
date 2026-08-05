//! ⚙️ CAD artifact — headless compute over the `CadScene` projection: the shared brep kernel, the
//! quad play fixture importer, mesh/typology tessellation, native geometry import/export, and the
//! plugin-level `register()` that wires cad's exporters/importers into the host.
//!
//! 📚️ Sibling topic files: `🦀️geometry_import.rs` (authored-geometry → kernel handles),
//! `🦀️transformation.rs` (derive/classify engine), `🦀️interaction.rs` (the declarative
//! construction statechart), `🦀️construct.rs` (Jack topology queries).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here.

use base64::Engine as _;
use crate::artifacts::cad::{cad_all_objects, cad_pane_from_model_definition_id, cad_pane_geometry, CadCamera, CadGeometry, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CadProjectionDsl, CadReference, CadScene, CAD_PLAY_DOCUMENT_SCHEMA};
use crate::artifacts::cad::engine::geometry_import::{cad_object_from_mesh, cad_object_from_solid_handle, centroid_from_fixture_primitives, objects_from_fixture_model, parse_geometry, tessellate_object_mesh, tessellate_object_mesh_from_fixture};
use kernel_3d_brepkit::{mesh_data_from_mesh_transfer, BrepkitKernel};
use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, MeshTransfer};
use semio_framework_core::MeshImporter;
use semio_framework_plugin::{mesh_from_kind, MeshData, OsMediaFormat, WorldProjectionConfig};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use crate::artifacts::cad::engine::transformation::solid_for_object;

//#region 🔖️Compute
pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

/// @emoji 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.
const CAD_MODEL_INDEX_SHAPE: usize = 0;

const CAD_MODEL_INDEX_BUILDING: usize = 1;

const CAD_MODEL_INDEX_ENERGY: usize = 2;

const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../../🖼️assets/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");

pub const CAD_MODEL_DEFINITION_SHAPE: &str = "spatial.shape";

pub const CAD_MODEL_DEFINITION_BUILDING: &str = "aec.building";

pub const CAD_MODEL_DEFINITION_ENERGY: &str = "aec.building.energy";

pub const CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC: &str = "aec.building.structure.classic";

const CAD_CONCRETE_FOREST_REFERENCE_URL: &str = "/cad-fixture/🖼️concrete-forest-reference.png";

pub const CAD_FOREST_REFERENCE_WIDTH_WORLD: f64 = 28.6;

pub const CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX: f64 = 1430.0;

pub const CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX: f64 = 692.0;

const CAD_FOREST_REFERENCE_BASE_ORIGIN_XY: [f64; 2] = [-24.0, -18.0];

pub const CAD_FOREST_REFERENCE_PLANE_Z: f64 = 0.01;

pub const CAD_FOREST_REFERENCE_Y_OFFSET_RATIO: f64 = 0.2;

static CAD_BREP_KERNEL: OnceLock<Mutex<Box<dyn BrepKernel + Send + Sync>>> = OnceLock::new();

/// @emoji 📦️ Universal fallback extent for typologies with no authored geometry to measure.
pub const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

pub fn cad_brep_kernel() -> &'static Mutex<Box<dyn BrepKernel + Send + Sync>> {
    CAD_BREP_KERNEL.get_or_init(|| Mutex::new(Box::new(BrepkitKernel::new())))
}

/// @emoji 📐️ Tessellates a typology's primitive sized from authored geometry (or a universal
/// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>, solid_handle: Option<&str>, centroid: Option<[f64; 3]>) -> MeshData {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    if let Some(handle_id) = solid_handle {
        let handle = GeometryHandle(handle_id.into());
        if let Ok(mesh) = block_on(kernel.tessellate(&handle, 0.1)) {
            return mesh_data_from_mesh_transfer(&mesh);
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
            block_on(kernel.dispose(&handle));
            return mesh_from_kind(typology_mesh_kind(typology));
        }
    };
    block_on(kernel.dispose(&handle));
    let mut mesh_data = mesh_data_from_mesh_transfer(&mesh);
    if let Some(center) = centroid {
        translate_mesh_positions(&mut mesh_data, [center[0] as f32, center[1] as f32, center[2] as f32]);
    }
    mesh_data
}

fn mesh_centroid(mesh: &MeshData) -> Option<[f32; 3]> {
    if mesh.positions.is_empty() {
        return None;
    }
    let count = mesh.positions.len() / 3;
    let mut sum = [0.0f32; 3];
    for vertex in mesh.positions.as_chunks::<3>().0 {
        sum[0] += vertex[0];
        sum[1] += vertex[1];
        sum[2] += vertex[2];
    }
    let n = count as f32;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}

/// @emoji 📐️ Shifts a tessellated mesh onto the authored fixture primitive centroid when kernel output drifts.
pub fn align_mesh_to_fixture_centroid(mesh: &mut MeshData, geometry: &CadGeometry, primitives: &[CadPrimitiveSlot]) {
    let Some(target) = centroid_from_fixture_primitives(geometry, primitives) else {
        return;
    };
    let Some(current) = mesh_centroid(mesh) else {
        return;
    };
    let delta = [(target[0] as f32) - current[0], (target[1] as f32) - current[1], (target[2] as f32) - current[2]];
    if delta[0].abs() + delta[1].abs() + delta[2].abs() > 0.05 {
        translate_mesh_positions(mesh, delta);
    }
}

/// @emoji 🖼️ Centers the concrete-forest reference and moves it forward from the authored base corner.
fn forest_reference_origin(reference_z: f64) -> [f64; 3] {
    let height_world = CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX;
    [CAD_FOREST_REFERENCE_BASE_ORIGIN_XY[0] + CAD_FOREST_REFERENCE_WIDTH_WORLD * 0.5, CAD_FOREST_REFERENCE_BASE_ORIGIN_XY[1] + height_world * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO), reference_z]
}

fn translate_mesh_positions(mesh: &mut MeshData, offset: [f32; 3]) {
    for vertex in mesh.positions.as_chunks_mut::<3>().0 {
        vertex[0] += offset[0];
        vertex[1] += offset[1];
        vertex[2] += offset[2];
    }
    for segment in mesh.edge_positions.as_chunks_mut::<6>().0 {
        segment[0] += offset[0];
        segment[1] += offset[1];
        segment[2] += offset[2];
        segment[3] += offset[0];
        segment[4] += offset[1];
        segment[5] += offset[2];
    }
}

/// @emoji 🗃️ Reads one pane's objects and geometry from the shared quad fixture.
fn cad_document_pane_bundle(source_json: &str, model_index: usize) -> (Vec<CadObject>, CadGeometry) {
    let Ok(root) = serde_json::from_str::<Value>(source_json) else {
        return (Vec::new(), CadGeometry::default());
    };
    let geometry = parse_geometry(root.pointer(&format!("/models/{model_index}/model/geometry")));
    let Some(objects_value) = root.pointer(&format!("/models/{model_index}/model/objects")).and_then(|value| value.as_array()) else {
        return (Vec::new(), geometry);
    };
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return (Vec::new(), geometry);
    };
    let objects = objects_from_fixture_model(&mut **kernel, objects_value, &geometry);
    (objects, geometry)
}

fn forest_references_for_model_definitions(reference_z: f64) -> std::collections::BTreeMap<String, Vec<CadReference>> {
    CadPaneId::all()
        .into_iter()
        .map(|pane| {
            (
                pane.model_definition_id().into(),
                vec![CadReference {
                    id: "ref-concrete-forest".into(),
                    source_url: CAD_CONCRETE_FOREST_REFERENCE_URL.into(),
                    media_kind: "image".into(),
                    origin: forest_reference_origin(reference_z),
                    orientation: None,
                    scale: None,
                    width_world: CAD_FOREST_REFERENCE_WIDTH_WORLD,
                    hidden: false,
                    locked: true,
                    opacity: Some(1.0),
                }],
            )
        })
        .collect()
}

pub fn typology_mesh_kind(typology: &str) -> &'static str {
    match typology {
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" | "aec.building.column" => "cylinder",
        _ => "box",
    }
}

pub fn default_document() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        objects: vec![CadObject {
            id: "object-box-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.primitive.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([1.0, 1.0, 1.0]),
            solid_handle: None,
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "box-solid".into(), kind: "solid".into() }],
        }],
        nodes: vec![CadNode { id: "node-root".into(), label: "Model".into(), kind: "group".into() }, CadNode { id: "node-box".into(), label: "Box".into(), kind: "solid".into() }],
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        shape_geometry: None,
        building_geometry: None,
        energy_geometry: None,
        structure_classic_geometry: None,
        references_by_model_definition_id: std::collections::BTreeMap::new(),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

/// @emoji 📟️ Builds the quad play document: shape/building/energy/structure-classic panes each
/// sourced from their own model definition inside the shared fixture JSON. Empty panes stay empty —
/// never collapse to `default_document` (that single-box placeholder was the cut-concrete bug).
fn forest_play_document(source_json: &str, id: &str) -> CadScene {
    let (shape_objects, shape_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_SHAPE);
    let (building_objects, building_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_BUILDING);
    let (energy_objects, energy_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_ENERGY);
    let (structure_classic_objects, structure_classic_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_STRUCTURE_CLASSIC);
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: id.into(),
        objects: shape_objects,
        nodes: vec![CadNode { id: "node-root".into(), label: "Concrete Forest Left".into(), kind: "group".into() }],
        building_objects,
        energy_objects,
        structure_classic_objects,
        shape_geometry: Some(shape_geometry),
        building_geometry: Some(building_geometry),
        energy_geometry: Some(energy_geometry),
        structure_classic_geometry: Some(structure_classic_geometry),
        references_by_model_definition_id: forest_references_for_model_definitions(CAD_FOREST_REFERENCE_PLANE_Z),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

/// @emoji 🌲️ The Concrete Forest Left example projection — a bare `CadScene` (no runtime/history),
/// wrapped into a `DocumentStore` by `VcsDocumentApp` when spawned. Cached so manifest registration,
/// `initial_projection`, and `setActiveExample` share one BREP import instead of rebuilding thrice.
pub fn forest_play_scene() -> CadScene {
    static FOREST_PLAY_SCENE: OnceLock<CadScene> = OnceLock::new();
    FOREST_PLAY_SCENE.get_or_init(|| forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT)).clone()
}

pub fn next_cad_id(prefix: &str) -> String {
    let next = CAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

/// 🌲️ The initial per-pane camera for the Concrete Forest Left example — session-only runtime state
/// now (camera moved off `CadScene`), matching the pose the document used to carry before the
/// camera-as-View-action refactor.
pub fn forest_play_camera() -> CadCamera {
    CadCamera { position: [12.0, -12.0, 8.0], target: [5.4, 2.34, 1.5], zoom: 1.0, fov: 50.0, projection: CadProjectionDsl::default() }
}

/// 📐️ Converts `camera.projection`'s local DSL twin into the shared taxonomy config — field-for-field,
/// since `CadProjectionDsl` mirrors `WorldProjectionConfig` exactly (see its doc comment in `cad/rs`).
pub fn cad_camera_projection_config(camera: &CadCamera) -> WorldProjectionConfig {
    let p = &camera.projection;
    WorldProjectionConfig {
        kind: p.kind.clone(),
        orthographic_view: p.orthographic_view.clone(),
        axonometric_variant: p.axonometric_variant.clone(),
        axonometric_angle_a: p.axonometric_angle_a,
        axonometric_angle_b: p.axonometric_angle_b,
        axonometric_quadrant: p.axonometric_quadrant.clone(),
        oblique_variant: p.oblique_variant.clone(),
        oblique_angle: p.oblique_angle,
        oblique_depth: p.oblique_depth,
        one_point_axis: p.one_point_axis.clone(),
        fov: p.fov,
        two_point_shift: p.two_point_shift,
        curvilinear_fov: p.curvilinear_fov,
        curvilinear_strength: p.curvilinear_strength,
        curvilinear_mapping: p.curvilinear_mapping.clone(),
    }
}

/// 📐️ Writes a taxonomy config back into `camera.projection`'s local DSL twin slot.
pub fn cad_camera_set_projection_config(camera: &mut CadCamera, config: &WorldProjectionConfig) {
    camera.projection = CadProjectionDsl {
        kind: config.kind.clone(),
        orthographic_view: config.orthographic_view.clone(),
        axonometric_variant: config.axonometric_variant.clone(),
        axonometric_angle_a: config.axonometric_angle_a,
        axonometric_angle_b: config.axonometric_angle_b,
        axonometric_quadrant: config.axonometric_quadrant.clone(),
        oblique_variant: config.oblique_variant.clone(),
        oblique_angle: config.oblique_angle,
        oblique_depth: config.oblique_depth,
        one_point_axis: config.one_point_axis.clone(),
        fov: config.fov,
        two_point_shift: config.two_point_shift,
        curvilinear_fov: config.curvilinear_fov,
        curvilinear_strength: config.curvilinear_strength,
        curvilinear_mapping: config.curvilinear_mapping.clone(),
    };
}

/// 📐️ Distance from `camera.position` to `camera.target`, defaulting to the historic orbit radius when degenerate.
pub fn cad_camera_distance(camera: &CadCamera) -> f64 {
    let [dx, dy, dz] = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
    if distance > 1e-3 {
        distance
    } else {
        20.0
    }
}

pub fn ensure_object_solid_handle(kernel: &mut dyn BrepKernel, object: &mut CadObject) {
    if object.solid_handle.is_some() {
        return;
    }
    if let Some(handle) = solid_for_object(kernel, object) {
        let primitive_id = handle.0;
        object.solid_handle = Some(primitive_id.clone());
        if object.primitives.is_empty() {
            object.primitives.push(CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() });
        }
    }
}

/// @emoji 📤️ A native-geometry export ready to be wrapped into a `HostEffect::DownloadMediaExport`.
pub struct CadSolidExport {
    pub filename: String,
    pub data: Value,
    pub mime_type: String,
    pub encoding: Option<String>,
}

/// @emoji 📤️ Encodes `solids` through the kernel's native OBJ/STL/STEP codec for `format`; STL is
/// base64-wrapped since it is a binary format, OBJ/STEP stay UTF-8 text.
pub fn export_solids_as(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], format: OsMediaFormat, stem: &str) -> Option<CadSolidExport> {
    let filename = format!("{stem}.{}", format.as_str());
    let mime_type = format.mime_type().to_string();
    match format {
        OsMediaFormat::Obj => {
            let text = block_on(kernel.export_obj(solids, 0.1)).ok()?;
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        OsMediaFormat::Stl => {
            let bytes = block_on(kernel.export_stl(solids, 0.1)).ok()?;
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            Some(CadSolidExport { filename, data: Value::String(encoded), mime_type, encoding: Some("base64".into()) })
        }
        OsMediaFormat::Step => {
            let text = block_on(kernel.export_step(solids)).ok()?;
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        _ => None,
    }
}

/// @emoji 📦️ Decodes a `requestFileOpen` payload (a `data:` URL when `readAs: "dataUrl"` was
/// requested, otherwise a raw string) into bytes.
pub fn cad_file_bytes_from_payload(payload: &Value) -> Option<Vec<u8>> {
    let raw = payload.as_str()?;
    if raw.starts_with("data:") {
        let (_, encoded) = raw.split_once(',')?;
        base64::engine::general_purpose::STANDARD.decode(encoded).ok()
    } else {
        Some(raw.as_bytes().to_vec())
    }
}

/// @emoji 📦️ Decodes a `requestFileOpen` payload into UTF-8 text; see `cad_file_bytes_from_payload`.
pub fn cad_file_text_from_payload(payload: &Value) -> Option<String> {
    String::from_utf8(cad_file_bytes_from_payload(payload)?).ok()
}

/// @emoji 🧊️ Imports a STEP payload into the shared kernel and wraps the first solid it contains
/// (STEP files may hold more than one shape) as a new `CadObject`.
pub fn import_step_object(text: &str) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = block_on(kernel.import_step(text)).ok()?.into_iter().next()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-step"), "Imported STEP", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports an OBJ payload into the shared kernel as a new `CadObject`.
pub fn import_obj_object(text: &str) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = block_on(kernel.import_obj(text, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-obj"), "Imported OBJ", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports an STL payload into the shared kernel as a new `CadObject`.
pub fn import_stl_object(bytes: &[u8]) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = block_on(kernel.import_stl(bytes, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-stl"), "Imported STL", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports a GLB payload by decoding it to a tessellated mesh (via the shared
/// `MeshImporter` codec) and re-importing that mesh into the kernel as a solid, matching the
/// DWG-derived import path (`cad_object_from_mesh`) since GLB carries no exact B-Rep to preserve.
pub fn import_glb_object(bytes: &[u8]) -> Option<CadObject> {
    let mesh = semio_framework_plugin::GlbImporter.import(bytes).ok()?;
    let mut kernel = cad_brep_kernel().lock().ok()?;
    Some(cad_object_from_mesh(&mut **kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", &mesh))
}

/// @emoji 🗂️ Routes a `requestFileOpen` payload to the matching native-geometry import by the
/// picked file's extension; returns `None` for anything else so the caller can fall back to the
/// spatial-JSON document path.
pub fn import_cad_object_by_extension(name: &str, payload: &Value) -> Option<CadObject> {
    if name.ends_with(".stp") || name.ends_with(".step") {
        return import_step_object(&cad_file_text_from_payload(payload)?);
    }
    if name.ends_with(".obj") {
        return import_obj_object(&cad_file_text_from_payload(payload)?);
    }
    if name.ends_with(".stl") {
        return import_stl_object(&cad_file_bytes_from_payload(payload)?);
    }
    if name.ends_with(".glb") {
        return import_glb_object(&cad_file_bytes_from_payload(payload)?);
    }
    None
}

pub fn unwrap_spatial_load_payload(raw: &Value) -> Option<Value> {
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

pub fn scene_from_spatial_payload(payload: &Value) -> Option<CadScene> {
    if payload.get("schema").and_then(|value| value.as_str()) == Some("spatial.modelspace") {
        let models = payload.get("models")?.as_array()?;
        let mut scene = default_document();
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return None;
        };
        for entry in models {
            let model_definition_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("");
            let objects_value = entry.pointer("/model/objects")?;
            let geometry = parse_geometry(entry.pointer("/model/geometry"));
            let objects = objects_value.as_array().map(|objects| objects_from_fixture_model(&mut **kernel, objects, &geometry)).filter(|objects| !objects.is_empty()).or_else(|| serde_json::from_value(objects_value.clone()).ok())?;
            match model_definition_id {
                CAD_MODEL_DEFINITION_SHAPE => {
                    scene.objects = objects;
                    scene.shape_geometry = Some(geometry);
                }
                CAD_MODEL_DEFINITION_BUILDING => {
                    scene.building_objects = objects;
                    scene.building_geometry = Some(geometry);
                }
                CAD_MODEL_DEFINITION_ENERGY => {
                    scene.energy_objects = objects;
                    scene.energy_geometry = Some(geometry);
                }
                CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC => {
                    scene.structure_classic_objects = objects;
                    scene.structure_classic_geometry = Some(geometry);
                }
                _ => {}
            }
        }
        if let Some(active) = payload.get("activeModelDefinitionId").and_then(|value| value.as_str()) {
            scene.active_model_definition_id = active.into();
        }
        return Some(scene);
    }
    if payload.get("schema").and_then(|value| value.as_str()) == Some("spatial.model") {
        let geometry = parse_geometry(payload.get("geometry"));
        let objects = payload
            .get("objects")
            .and_then(|value| value.as_array())
            .map(|objects| {
                let Ok(mut kernel) = cad_brep_kernel().lock() else {
                    return Vec::new();
                };
                objects_from_fixture_model(&mut **kernel, objects, &geometry)
            })
            .filter(|objects| !objects.is_empty())
            .or_else(|| serde_json::from_value(payload.get("objects")?.clone()).ok())?;
        let mut scene = default_document();
        let pane = payload.get("modelDefinitionId").and_then(|value| value.as_str()).and_then(cad_pane_from_model_definition_id).unwrap_or(CadPaneId::Shape);
        match pane {
            CadPaneId::Shape => {
                scene.objects = objects;
                scene.shape_geometry = Some(geometry);
            }
            CadPaneId::Building => {
                scene.building_objects = objects;
                scene.building_geometry = Some(geometry);
            }
            CadPaneId::Energy => {
                scene.energy_objects = objects;
                scene.energy_geometry = Some(geometry);
            }
            CadPaneId::StructureClassic => {
                scene.structure_classic_objects = objects;
                scene.structure_classic_geometry = Some(geometry);
            }
        }
        scene.active_model_definition_id = pane.model_definition_id().into();
        return Some(scene);
    }
    None
}

pub fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
    object.mesh_url.as_ref().filter(|url| !url.is_empty()).cloned()
}

pub fn primary_primitive_kind(object: &CadObject) -> &str {
    object.primitives.first().map_or("solid", |primitive| primitive.kind.as_str())
}

pub fn object_mesh_data(object: &CadObject, geometry: Option<&CadGeometry>) -> MeshData {
    let kind = primary_primitive_kind(object);
    if let Ok(mut kernel) = cad_brep_kernel().lock() {
        let mesh = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| tessellate_object_mesh_from_fixture(&mut **kernel, object, geometry)).or_else(|| tessellate_object_mesh(&mut **kernel, object, kind));
        if let Some(mut mesh) = mesh {
            if let Some(geometry) = geometry {
                align_mesh_to_fixture_centroid(&mut mesh, geometry, &object.primitives);
            }
            return mesh;
        }
    }
    let centroid = geometry.and_then(|geometry| centroid_from_fixture_primitives(geometry, &object.primitives));
    typology_brep_mesh(&object.typology, object.extent, object.solid_handle.as_deref(), centroid)
}

pub fn collect_mesh_urls(objects: &[CadObject]) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in objects {
        if let Some(url) = resolve_object_mesh_url(object) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

pub fn object_scale_json(object: &CadObject) -> [f64; 3] {
    object.scale.unwrap_or([1.0, 1.0, 1.0])
}

/// @emoji 🧵️ Tessellates a representative mesh for the OS mesh-exporter boundary — the document's
/// first object across panes, or the default box typology for an empty scene (no runtime selection
/// exists at this boundary).
pub fn export_mesh_from_scene(document: &CadScene) -> MeshData {
    let first = cad_all_objects(document).next();
    let typology = first.map_or("spatial.shape.primitive.box", |(object, _)| object.typology.as_str());
    let extent = first.and_then(|(object, _)| object.extent);
    let solid_handle = first.and_then(|(object, _)| object.solid_handle.as_deref());
    let centroid = first.and_then(|(object, pane)| cad_pane_geometry(document, pane).and_then(|geometry| centroid_from_fixture_primitives(geometry, &object.primitives)));
    typology_brep_mesh(typology, extent, solid_handle, centroid)
}

pub fn cad_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: CadScene = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_scene(&scene))
}

pub fn cad_document_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    let mut scene = default_document();
    let mut kernel = cad_brep_kernel().lock().map_err(|_| "cad brep kernel lock poisoned".to_string())?;
    let objects: Vec<CadObject> = drawing
        .layers
        .iter()
        .enumerate()
        .filter_map(|(layer_index, layer)| {
            let mut layer_drawing = drawing.clone();
            layer_drawing.entities.retain(|entity| entity.layer == layer_index);
            if layer_drawing.entities.is_empty() {
                return None;
            }
            let mesh = semio_framework_core::dwg_drawing_to_mesh(&layer_drawing);
            Some(cad_object_from_mesh(&mut **kernel, format!("object-{}", layer.name), layer.name.clone(), "spatial.shape.imported", &mesh))
        })
        .collect();
    if !objects.is_empty() {
        scene.objects = objects;
    }
    serde_json::to_value(&scene).map_err(|err| err.to_string())
}

/// @emoji 🧵️ Bridges a `MeshImporter`-decoded mesh (currently only GLB) back into a bare `CadScene`
/// document, reusing the same OBJ-text-roundtrip kernel import as the DWG/STL/`importCadFile` paths.
pub fn cad_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let mut scene = default_document();
    let mut kernel = cad_brep_kernel().lock().map_err(|_| "cad brep kernel lock poisoned".to_string())?;
    let object = cad_object_from_mesh(&mut **kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", mesh);
    scene.objects = vec![object];
    serde_json::to_value(&scene).map_err(|err| err.to_string())
}
//#endregion 🔖️Compute

//#region 🔖️Register
/// 🔌️ Plugin setup hook (`semio_plugin!`'s `setup:`): registers the `cad.scene` document codec for
/// the cad play app plus every native geometry exporter/importer the `3d.cad` artifact kind
/// advertises. Was the bundle crate's `register_cad_exports`.
pub fn register() {
    // 📦️ pack binary codec for `CadScene` (`CadPlayApp::document_schema()` == `CAD_DOCUMENT_SCHEMA`).
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::cad::CadPlayApp>(crate::artifacts::cad::CAD_DOCUMENT_SCHEMA);
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StlSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StepSolidExporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StlSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StepSolidImporter));
    semio_framework_os::register_mesh_exporter("3d.cad", "cad", cad_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_importer("3d.cad", cad_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.cad", "cad", cad_mesh_from_document);
    semio_framework_os::register_dwg_import_handler("3d.cad", cad_document_from_dwg);
}
//#endregion 🔖️Register
