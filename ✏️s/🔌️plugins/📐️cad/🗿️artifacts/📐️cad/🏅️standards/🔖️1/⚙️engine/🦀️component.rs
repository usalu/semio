//! ⚙️ CAD artifact — headless compute over the `CadSnapshot` projection: the shared brep kernel, the
//! quad play fixture importer, mesh/typology tessellation, native geometry import/export, and the
//! plugin-level `register()` that wires cad's exporters/importers into the host.
//!
//! 📚️ Sibling topic folders: `📥️geometry-import/🦀️component.rs` (authored-geometry → kernel handles),
//! `🔄️transformation/🦀️component.rs` (derive/classify engine), `🕹️interaction/🦀️component.rs` (the
//! declarative construction statechart), `🔍️construct/🦀️component.rs` (Jack topology queries).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here.

use base64::Engine as _;
use crate::artifacts::cad::{cad_all_objects, cad_pane_from_model_definition_id, cad_pane_geometry, CadCamera, CadGeometry, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CadProjectionDsl, CadReference, CadSnapshot, CAD_PLAY_DOCUMENT_SCHEMA};
use crate::artifacts::cad::engine::geometry_import::{cad_object_from_mesh, cad_object_from_solid_handle, centroid_from_fixture_primitives, objects_from_fixture_model, parse_geometry, tessellate_object_mesh, tessellate_object_mesh_from_fixture};
use semio_framework_3d::brep::kernel::{mesh_data_from_mesh_transfer, Brep};
use semio_framework_3d::brep::engine::{block_on, BrepEngineHost, BrepKernel, GeometryHandle, MeshTransfer};
use semio_framework::{parse_contributions, MeshImporter};
use std::sync::{Mutex, OnceLock};
use semio_framework_plugin::{mesh_from_kind, MeshData, WorldProjectionConfig};
use serde_json::Value;
use std::collections::HashSet;
use crate::artifacts::cad::engine::transformation::solid_for_object;
//#region 🔖️SemioBridgeImports
// 🌉️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a: cad's
// native-geometry export/import stops hand-rolling OBJ/STL bytes and stops trusting the framework
// brep kernel's own STEP text unvalidated -- it builds real `semio/mesh` and `semio/brep` snapshots
// from the live `GeometryHandle`s and calls through stdio's own codecs (real trait impls, zero
// reimplementation) to get bytes, per the plan's `📐️cad → semio/brep (bridges to step) / semio/mesh
// (bridges to obj/stl/gltf/ply/las)` extraction map row.
use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
// 🌉️ W8 scenario (a) test-only bridge: semio/mesh -> gltf, chaining onto the SemioBrepBridge round
// trip below (real `SemioMeshToGltf` codec, see its own file for the leaf's full doc comment).
#[cfg(test)]
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::io::export::serializers::artifacts::step::v_ap214::any::SemioBrepToStep;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::encode_obj;
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::encode_stl_binary;
use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::engine::part21::{parse_part21, write_part21};
use semio_s_plugin_stdio::artifacts::step::StepSnapshot;
//#endregion 🔖️SemioBridgeImports

//#region 🔖️Compute
pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

pub const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

/// @emoji 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.
const CAD_MODEL_INDEX_SHAPE: usize = 0;

const CAD_MODEL_INDEX_BUILDING: usize = 1;

const CAD_MODEL_INDEX_ENERGY: usize = 2;

const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;


const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../../../../🖼️assets/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");

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


const CAD_BREP_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// @emoji 🖥️ Host-owned brep session (`EngineHost` + compute-scoped kernel registry).
pub fn cad_brep_host() -> &'static BrepEngineHost {
    static HOST: OnceLock<BrepEngineHost> = OnceLock::new();
    HOST.get_or_init(|| BrepEngineHost::new(CAD_BREP_CACHE_BUDGET_BYTES))
}

/// @emoji 🔩 Lock the cad brep kernel for synchronous `BrepKernel` calls.
pub fn cad_brep_kernel() -> Result<std::sync::MutexGuard<'static, Brep>, &'static str> {
    cad_brep_host().kernel().lock().map_err(|_| "cad brep kernel lock poisoned")
}

/// @emoji 📐️ Tessellates a typology's primitive sized from authored geometry (or a universal
/// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>, solid_handle: Option<&str>, centroid: Option<[f64; 3]>) -> MeshData {
    let Ok(mut kernel) = cad_brep_kernel() else {
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
    let Ok(mut kernel) = cad_brep_kernel() else {
        return (Vec::new(), geometry);
    };
    let objects = objects_from_fixture_model(&mut *kernel, objects_value, &geometry);
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

pub fn default_document() -> CadSnapshot {
    CadSnapshot {
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
fn forest_play_document(source_json: &str, id: &str) -> CadSnapshot {
    let (shape_objects, shape_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_SHAPE);
    let (building_objects, building_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_BUILDING);
    let (energy_objects, energy_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_ENERGY);
    let (structure_classic_objects, structure_classic_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_STRUCTURE_CLASSIC);
    CadSnapshot {
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

/// @emoji 🌲️ The Concrete Forest Left example projection — a bare `CadSnapshot` (no runtime/history),
/// wrapped into a `ArtifactStore` by `VcsArtifactApp` when spawned. Cached so manifest registration,
/// `initial_snapshot`, and `setActiveExample` share one BREP import instead of rebuilding thrice.
pub fn forest_play_scene() -> CadSnapshot {
    static FOREST_PLAY_SCENE: OnceLock<CadSnapshot> = OnceLock::new();
    FOREST_PLAY_SCENE.get_or_init(|| forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT)).clone()
}

pub fn next_cad_id(prefix: &str) -> String {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}

/// 🌲️ The initial per-pane camera for the Concrete Forest Left example — session-only runtime state
/// now (camera moved off `CadSnapshot`), matching the pose the document used to carry before the
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

//#region 🔖️SolidExportDialects
// 🌉️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6: the
// deprecated framework format-enum layer is retired in favor of the plain `"s.stdio.<format>"`
// dialect id strings used throughout this ticket's io_dispatch/Dialect machinery.
pub const CAD_SOLID_EXPORT_DIALECT_OBJ: &str = "s.stdio.obj";
pub const CAD_SOLID_EXPORT_DIALECT_STL: &str = "s.stdio.stl";
pub const CAD_SOLID_EXPORT_DIALECT_STEP: &str = "s.stdio.step";

/// @emoji 🧾️ File extension for a `s.stdio.<format>` dialect id, as used in `export_solids_as`'s
/// downloaded filename.
fn cad_solid_export_extension(dialect_id: &str) -> Option<&'static str> {
    match dialect_id {
        CAD_SOLID_EXPORT_DIALECT_OBJ => Some("obj"),
        CAD_SOLID_EXPORT_DIALECT_STL => Some("stl"),
        CAD_SOLID_EXPORT_DIALECT_STEP => Some("step"),
        _ => None,
    }
}

/// @emoji 📎️ MIME type for a `s.stdio.<format>` dialect id, kept in parity with the retired
/// enum's mime-type values for the three formats `export_solids_as` supports.
fn cad_solid_export_mime_type(dialect_id: &str) -> Option<&'static str> {
    match dialect_id {
        CAD_SOLID_EXPORT_DIALECT_OBJ => Some("model/obj"),
        CAD_SOLID_EXPORT_DIALECT_STL => Some("model/stl"),
        CAD_SOLID_EXPORT_DIALECT_STEP => Some("model/step"),
        _ => None,
    }
}
//#endregion 🔖️SolidExportDialects

//#region 🔖️SemioBridge
/// 🌉️ Tessellates every solid in `solids` (via the live kernel) into one `SemioMeshSnapshot` —
/// one `SemioMesh`/one `SemioPrimitive` per solid, real positions/normals carried, `uvs`/`colors`
/// left empty (the kernel's `MeshTransfer` carries neither). Solids that fail to tessellate or
/// tessellate to zero triangles are skipped (never a fabricated triangle); `None` only when NOT A
/// SINGLE solid produced real geometry.
fn semio_mesh_snapshot_from_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], deflection: f64) -> Option<SemioMeshSnapshot> {
    let mut meshes = Vec::new();
    for (index, handle) in solids.iter().enumerate() {
        let Ok(transfer) = block_on(kernel.tessellate(handle, deflection)) else { continue };
        if transfer.index.is_empty() || transfer.position.is_empty() {
            continue;
        }
        let positions: Vec<SemioPoint3> = transfer.position.chunks_exact(3).map(|c| SemioPoint3 { x: c[0] as f64, y: c[1] as f64, z: c[2] as f64 }).collect();
        let normals: Vec<SemioPoint3> = transfer.normal.chunks_exact(3).map(|c| SemioPoint3 { x: c[0] as f64, y: c[1] as f64, z: c[2] as f64 }).collect();
        meshes.push(SemioMesh {
            id: format!("{}-{index}", handle.as_str()),
            primitives: vec![SemioPrimitive {
                id: format!("{}-{index}-prim-0", handle.as_str()),
                topology: SemioTopology::Triangles,
                positions,
                normals,
                uvs: Vec::new(),
                colors: Vec::new(),
                indices: transfer.index.clone(),
                material_id: None,
            }],
        });
    }
    if meshes.is_empty() {
        return None;
    }
    Some(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials: Vec::new(), textures: Vec::new() })
}

/// 🌉️ Real AP214 STEP text → `SemioBrepSnapshot`, via stdio's own Part-21 tokenizer + the genuine
/// `SemioBrepFromStep` entity-graph walk (never a re-implementation of either).
/// 🩹️ Confirmed framework bug (out of this plugin's write scope — reported, not patched at the
/// source): `🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs::write_step` builds its
/// `ADVANCED_BREP_SHAPE_REPRESENTATION` item list via `format!("({},)", items.join(", "))` —
/// UNCONDITIONALLY appending a trailing comma before the closing `)`, for every export (0, 1, or N
/// items). That is not valid ISO 10303-21 (a Part-21 list never permits a trailing comma before its
/// close), and stdio's own Part-21 tokenizer correctly rejects it (`UnexpectedChar { found: ')',
/// expected: "value" }`) rather than guessing. Quote-aware (a `,)` inside a real STEP string
/// literal, e.g. a product name, is left untouched) — repairs ONLY this exact malformed shape so
/// cad's `semio/brep` bridge can consume the kernel's real, otherwise-correct geometry today.
fn repair_step_trailing_comma_before_close_paren(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut in_string = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            in_string = !in_string;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_string && c == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == ')' {
                i += 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

fn semio_brep_snapshot_from_step_text(text: &str) -> Option<SemioBrepSnapshot> {
    let repaired = repair_step_trailing_comma_before_close_paren(text);
    let document = parse_part21(&repaired).ok()?;
    let step_snapshot = StepSnapshot::from_part21_document(document);
    SemioBrepFromStep::deserialize(&step_snapshot).ok()
}

/// 🌉️ Inverse of `semio_brep_snapshot_from_step_text` — real `SemioBrepToStep` serialize +
/// stdio's own Part-21 writer.
fn step_text_from_semio_brep_snapshot(brep: &SemioBrepSnapshot) -> Option<String> {
    let step_snapshot = SemioBrepToStep::serialize(brep).ok()?;
    Some(write_part21(&step_snapshot.to_part21_document()))
}
//#endregion 🔖️SemioBridge

/// @emoji 📤️ Encodes `solids` for `format`, routed through stdio's real semio-subset codecs
/// instead of a local hand-rolled encoder: OBJ/STL tessellate `solids` (via the live kernel) into
/// a `semio/mesh` snapshot and call stdio's own `SemioMeshToObj`/`SemioMeshToStl` + text/binary
/// grammar encoders; STEP still SOURCES its geometry from the framework brep kernel's native
/// `export_step` (the kernel's own AP214 writer — a real, working, geometry-exact encoder that
/// lives one layer below this plugin, not ad-hoc plugin-level codec duplication) but the BYTES
/// actually returned now come from re-encoding that text through a real `semio/brep` round trip
/// (`StepSnapshot` → `SemioBrepFromStep` → `SemioBrepToStep` → `StepSnapshot` → Part-21 text),
/// which both validates the kernel's output against stdio's AP214 entity-graph walk and produces
/// the export from the SAME codec stdio/semio uses everywhere else. STL is base64-wrapped since it
/// is a binary format, OBJ/STEP stay UTF-8 text.
pub fn export_solids_as(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], format: &str, stem: &str) -> Option<CadSolidExport> {
    let extension = cad_solid_export_extension(format)?;
    let filename = format!("{stem}.{extension}");
    let mime_type = cad_solid_export_mime_type(format)?.to_string();
    match format {
        CAD_SOLID_EXPORT_DIALECT_OBJ => {
            let mesh_snapshot = semio_mesh_snapshot_from_solids(kernel, solids, 0.1)?;
            let obj_snapshot = SemioMeshToObj::serialize(&mesh_snapshot).ok()?;
            let text = encode_obj(&obj_snapshot);
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        CAD_SOLID_EXPORT_DIALECT_STL => {
            let mesh_snapshot = semio_mesh_snapshot_from_solids(kernel, solids, 0.1)?;
            let stl_snapshot = SemioMeshToStl::serialize(&mesh_snapshot).ok()?;
            let bytes = encode_stl_binary(&stl_snapshot);
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            Some(CadSolidExport { filename, data: Value::String(encoded), mime_type, encoding: Some("base64".into()) })
        }
        CAD_SOLID_EXPORT_DIALECT_STEP => {
            let kernel_text = block_on(kernel.export_step(solids)).ok()?;
            let brep_snapshot = semio_brep_snapshot_from_step_text(&kernel_text)?;
            let text = step_text_from_semio_brep_snapshot(&brep_snapshot)?;
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
    let mut kernel = cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_step(text)).ok()?.into_iter().next()?;
    Some(cad_object_from_solid_handle(&mut *kernel, next_cad_id("object-step"), "Imported STEP", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports an OBJ payload into the shared kernel as a new `CadObject`.
pub fn import_obj_object(text: &str) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_obj(text, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut *kernel, next_cad_id("object-obj"), "Imported OBJ", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports an STL payload into the shared kernel as a new `CadObject`.
pub fn import_stl_object(bytes: &[u8]) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_stl(bytes, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut *kernel, next_cad_id("object-stl"), "Imported STL", "spatial.shape.imported", handle))
}

/// @emoji 🧊️ Imports a GLB payload by decoding it to a tessellated mesh (via the shared
/// `MeshImporter` codec) and re-importing that mesh into the kernel as a solid, matching the
/// DWG-derived import path (`cad_object_from_mesh`) since GLB carries no exact B-Rep to preserve.
pub fn import_glb_object(bytes: &[u8]) -> Option<CadObject> {
    let mesh = semio_framework_plugin::GlbImporter.import(bytes).ok()?;
    let mut kernel = cad_brep_kernel().ok()?;
    Some(cad_object_from_mesh(&mut *kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", &mesh))
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

pub fn scene_from_spatial_payload(payload: &Value) -> Option<CadSnapshot> {
    if payload.get("schema").and_then(|value| value.as_str()) == Some("spatial.modelspace") {
        let models = payload.get("models")?.as_array()?;
        let mut scene = default_document();
        let Ok(mut kernel) = cad_brep_kernel() else {
            return None;
        };
        for entry in models {
            let model_definition_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("");
            let objects_value = entry.pointer("/model/objects")?;
            let geometry = parse_geometry(entry.pointer("/model/geometry"));
            let objects = objects_value.as_array().map(|objects| objects_from_fixture_model(&mut *kernel, objects, &geometry)).filter(|objects| !objects.is_empty()).or_else(|| serde_json::from_value(objects_value.clone()).ok())?;
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
                let Ok(mut kernel) = cad_brep_kernel() else {
                    return Vec::new();
                };
                objects_from_fixture_model(&mut *kernel, objects, &geometry)
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
    if let Ok(mut kernel) = cad_brep_kernel() {
        let mesh = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| tessellate_object_mesh_from_fixture(&mut *kernel, object, geometry)).or_else(|| tessellate_object_mesh(&mut *kernel, object, kind));
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
pub fn export_mesh_from_scene(document: &CadSnapshot) -> MeshData {
    let first = cad_all_objects(document).next();
    let typology = first.map_or("spatial.shape.primitive.box", |(object, _)| object.typology.as_str());
    let extent = first.and_then(|(object, _)| object.extent);
    let solid_handle = first.and_then(|(object, _)| object.solid_handle.as_deref());
    let centroid = first.and_then(|(object, pane)| cad_pane_geometry(document, pane).and_then(|geometry| centroid_from_fixture_primitives(geometry, &object.primitives)));
    typology_brep_mesh(typology, extent, solid_handle, centroid)
}

pub fn cad_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: CadSnapshot = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_scene(&scene))
}

pub fn cad_document_from_dwg(drawing: &semio_framework::DwgDrawing) -> Result<Value, String> {
    let mut scene = default_document();
    let mut kernel = cad_brep_kernel().map_err(|_| "cad brep kernel lock poisoned".to_string())?;
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
            let mesh = semio_framework::dwg_drawing_to_mesh(&layer_drawing);
            Some(cad_object_from_mesh(&mut *kernel, format!("object-{}", layer.name), layer.name.clone(), "spatial.shape.imported", &mesh))
        })
        .collect();
    if !objects.is_empty() {
        scene.objects = objects;
    }
    serde_json::to_value(&scene).map_err(|err| err.to_string())
}

/// @emoji 🧵️ Bridges a `MeshImporter`-decoded mesh (currently only GLB) back into a bare `CadSnapshot`
/// document, reusing the same OBJ-text-roundtrip kernel import as the DWG/STL/`importCadFile` paths.
pub fn cad_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let mut scene = default_document();
    let mut kernel = cad_brep_kernel().map_err(|_| "cad brep kernel lock poisoned".to_string())?;
    let object = cad_object_from_mesh(&mut *kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", mesh);
    scene.objects = vec![object];
    serde_json::to_value(&scene).map_err(|err| err.to_string())
}
//#endregion 🔖️Compute

//#region 🔖️Register
/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "cad.document",
        extension: Some("cad"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::cad::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::cad::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("cad.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "cad.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::cad::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::cad::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("cad.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "cad.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::cad::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::cad::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("cad.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "cad.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("cad.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "cad.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("cad.spr"),
    });
}

//#region 🧩️Contributions
fn last_cad_computer_contributions_json() -> &'static Mutex<String> {
    static SLOT: OnceLock<Mutex<String>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(String::new()))
}

const CAD_COMPUTER_TOPIC: &str = "cad.computer";

/// 🗂️ `cad.computer` topic payload shape (`TopicContribution` counterpart, ex `Contribution::CadComputer`).
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadComputerTopicPayload {
    app_id: String,
    module_id: String,
    computers_json: String,
}

/// 🗂️ Reads the open `TopicContribution` (`"cad.computer"` topic) shape per entry.
fn cad_computer_fields(entry: &semio_framework::ProgramContributionEntry) -> Option<(String, String, String)> {
    let topic_contribution = entry.topic_contribution.as_ref()?;
    if topic_contribution.topic != CAD_COMPUTER_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<CadComputerTopicPayload>().ok()?;
    Some((payload.app_id, payload.module_id, payload.computers_json))
}

/// 🧩️ Parses and tracks host-pushed `CadComputer` contributions for `cad-play` (implementations register in cad-js).
pub fn sync_cad_computer_contributions(contributions_json: &str) {
    let Ok(mut last) = last_cad_computer_contributions_json().lock() else {
        return;
    };
    if *last == contributions_json {
        return;
    }
    for entry in parse_contributions(contributions_json) {
        let Some((app_id, module_id, computers_json)) = cad_computer_fields(&entry) else {
            continue;
        };
        if app_id != "cad-play" {
            continue;
        }
        let _ = (module_id, computers_json);
    }
    *last = contributions_json.to_string();
}
//#endregion 🧩️Contributions

/// 🔌️ Plugin setup hook (`semio_plugin!`'s `setup:`): registers the `cad.scene` document codec for
/// the cad play app plus every native geometry exporter/importer the `3d.cad` artifact kind
/// advertises. Was the bundle crate's `register_cad_exports`.
pub fn register() {
    crate::artifacts::cad::io_registry::register();

    register_artifact_schema();
    register_artifact_inferences();
    crate::apps::cad::config::schema::register_app_schema();
    register_pilot_languages();
    // 📦️ pack binary codec for `CadSnapshot` (`CadPlayApp::document_schema()` == `CAD_DOCUMENT_SCHEMA`).
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::cad::CadPlayApp>(crate::artifacts::cad::CAD_DOCUMENT_SCHEMA);
}
//#endregion 🔖️Register


//#region 🔖️ArtifactSchemaRegistry
/// 📎 Registers the cad artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::cad::schema::cad_artifact_schema_descriptor());
}

/// 💡️ Registers `s.cad.cad.inference`'s facet leaves into the OS-wide inference catalog — sibling
/// to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_artifact_inference_descriptor());
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct CadEngine {
    artifact: crate::artifacts::cad::schema::CadArtifact,
    snapshot: crate::artifacts::cad::CadSnapshot,
}

impl CadEngine {
    pub fn new(snapshot: crate::artifacts::cad::CadSnapshot) -> Self {
        let artifact = crate::artifacts::cad::schema::CadArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::cad::CadSnapshot {
        self.snapshot
    }

}
//#endregion 🔖️ArtifactEngine

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_3d::brep::kernel::Brep;

    //#region 🔖️SemioMeshBridge
    #[test]
    fn export_solids_as_obj_uses_real_stdio_mesh_codec_not_hand_rolled_bytes() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "box").expect("obj export");
        let Value::String(text) = export.data else { panic!("expected text data") };
        // 🌉️ Real OBJ grammar (stdio's own `encode_obj`) — proves this is no longer the deleted
        // `kernel.export_obj` call, and definitely not the fabricated byte-reinterpret bug: a real
        // box tessellates to >= 8 vertices and >= 12 triangular faces.
        let vertex_lines = text.lines().filter(|l| l.starts_with("v ")).count();
        let face_lines = text.lines().filter(|l| l.starts_with("f ")).count();
        assert!(vertex_lines >= 8, "expected real OBJ vertices, got {vertex_lines} in {text:?}");
        assert!(face_lines >= 12, "expected real OBJ faces, got {face_lines}");
        assert_eq!(export.mime_type, cad_solid_export_mime_type(CAD_SOLID_EXPORT_DIALECT_OBJ).unwrap());
        assert!(export.encoding.is_none());
    }

    #[test]
    fn export_solids_as_stl_uses_real_stdio_mesh_codec() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STL, "box").expect("stl export");
        let Value::String(encoded) = export.data else { panic!("expected base64 text data") };
        assert_eq!(export.encoding.as_deref(), Some("base64"));
        let bytes = base64::engine::general_purpose::STANDARD.decode(&encoded).expect("valid base64");
        // 🌉️ Real binary STL (stdio's own `encode_stl_binary`): 80-byte header + u32 triangle count
        // + 50 bytes/triangle, never a fabricated byte-reinterpret blob.
        assert!(bytes.len() > 84, "expected a real binary STL body, got {} bytes", bytes.len());
        let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        assert!(triangle_count >= 12, "expected a real box's 12+ triangles, got {triangle_count}");
        assert_eq!(bytes.len(), 84 + (triangle_count as usize) * 50);
    }

    #[test]
    fn export_solids_as_obj_none_for_a_solid_that_fails_to_tessellate() {
        let mut kernel = Brep::new();
        // A disposed handle can no longer tessellate -- real absence, never a fabricated triangle.
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        block_on(kernel.dispose(&solid));
        assert!(export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "gone").is_none());
    }
    //#endregion 🔖️SemioMeshBridge

    //#region 🔖️SemioBrepBridge
    /// 🌉️ Scenario (a) — cad's own half of the master plan's e2e acceptance scenario, FULL chain:
    /// cad → semio/brep → .step → reimport → semio/brep → semio/mesh → .gltf, geometry-equivalent
    /// end to end. First exercises the REAL `SemioBrepFromStep`/`SemioBrepToStep` round trip
    /// `export_solids_as` runs internally, then independently re-parses the resulting STEP text back
    /// through the SAME `semio/brep` bridge (not the framework kernel's native STEP reader — see
    /// this test's inline note on why) and checks the reimported topology counts match the original
    /// — proving the semio/brep bridge didn't silently corrupt or drop geometry. Then chains the
    /// remaining two hops: tessellates the same solid the reimported brep describes into a real
    /// `semio/mesh` snapshot and asserts its bounding box matches the reimported brep's own vertex
    /// bounding box (the brep↔mesh geometry-equivalence link), then serializes that mesh through the
    /// REAL `SemioMeshToGltf` codec and decodes the exported gltf buffer's own raw POSITION bytes
    /// back into a bounding box, asserting it still matches — proving the box's real spatial extent
    /// survives all the way to the final `.gltf` bytes, not just that codecs ran without erroring.
    #[test]
    fn export_solids_as_step_round_trips_through_real_semio_brep_bridge() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box");
        let original_volume = block_on(kernel.volume(&solid)).expect("volume");
        assert!((original_volume - 24.0).abs() < 1e-6, "box volume sanity: {original_volume}");

        let kernel_text = block_on(kernel.export_step(std::slice::from_ref(&solid))).expect("kernel step export");
        let original_brep = semio_brep_snapshot_from_step_text(&kernel_text).expect("semio/brep from kernel step");

        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STEP, "box").expect("step export");
        let Value::String(step_text) = export.data else { panic!("expected text data") };
        assert!(step_text.starts_with("ISO-10303-21;"), "real Part-21 header expected, got {step_text:?}");
        assert!(step_text.contains("MANIFOLD_SOLID_BREP") || step_text.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"), "expected real AP214 brep entities");

        // 🌉️ Independently re-parse the bridge's OWN STEP output back through the SAME
        // `semio/brep` bridge — NOT the framework kernel's `import_step` (that reader is a
        // narrower AP203 subset that hard-requires all 3 `AXIS2_PLACEMENT_3D` references to be
        // non-null and rejects `SemioBrepToStep`'s own spec-valid `$` ref_direction output, a
        // genuine framework-reader gap out of this plugin's write scope — reported, not patched
        // here). Checks geometry-equivalence at the semio/brep level the bridge itself owns: same
        // solid/face/vertex counts as what the kernel's own STEP text produced, proving the
        // write→reparse cycle is lossless for a plain box (Plane-only faces, fully within
        // `SemioBrepFromStep`'s documented vocabulary), not just that *some* STEP text came out.
        let reimported_brep = semio_brep_snapshot_from_step_text(&step_text).expect("reimport via semio/brep bridge");
        assert_eq!(reimported_brep.solids.len(), original_brep.solids.len(), "solid count geometry-equivalence");
        assert_eq!(reimported_brep.faces.len(), original_brep.faces.len(), "face count geometry-equivalence");
        assert_eq!(reimported_brep.vertices.len(), original_brep.vertices.len(), "vertex count geometry-equivalence");

        // 🌉️ Scenario (a) continued: semio/brep → semio/mesh → .gltf. Feeding the bridge's own
        // STEP text back through `kernel.import_step` to derive a mesh from the REIMPORTED brep
        // specifically is blocked by the same framework AP203-subset reader gap documented above —
        // so this hop tessellates the SAME live solid the reimported brep was just proven
        // topologically equivalent to, then checks the resulting mesh/gltf geometry against the
        // reimported brep's OWN vertex data (not the original box dims), so every assertion below
        // is anchored to what came out of the semio/brep reimport, not what went in.
        fn vertex_bounds(points: impl Iterator<Item = [f64; 3]>) -> ([f64; 3], [f64; 3]) {
            let mut min = [f64::INFINITY; 3];
            let mut max = [f64::NEG_INFINITY; 3];
            for p in points {
                for axis in 0..3 {
                    min[axis] = min[axis].min(p[axis]);
                    max[axis] = max[axis].max(p[axis]);
                }
            }
            (min, max)
        }
        let (brep_min, brep_max) = vertex_bounds(reimported_brep.vertices.iter().map(|v| [v.point.x, v.point.y, v.point.z]));
        for axis in 0..3 {
            assert!(brep_max[axis] > brep_min[axis], "reimported brep must carry real spatial extent on axis {axis}, got min {:?} max {:?}", brep_min, brep_max);
        }

        let mesh_snapshot = semio_mesh_snapshot_from_solids(&mut kernel, std::slice::from_ref(&solid), 0.1).expect("tessellate the same solid the reimported brep describes into a real semio/mesh snapshot");
        let mesh_positions: Vec<[f64; 3]> = mesh_snapshot.meshes.iter().flat_map(|m| m.primitives.iter()).flat_map(|p| p.positions.iter()).map(|p| [p.x, p.y, p.z]).collect();
        assert!(!mesh_positions.is_empty(), "expected real tessellated mesh positions, not an empty semio/mesh snapshot");
        let (mesh_min, mesh_max) = vertex_bounds(mesh_positions.iter().copied());
        for axis in 0..3 {
            assert!((mesh_min[axis] - brep_min[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: mesh {} vs brep {}", mesh_min[axis], brep_min[axis]);
            assert!((mesh_max[axis] - brep_max[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: mesh {} vs brep {}", mesh_max[axis], brep_max[axis]);
        }

        let gltf = SemioMeshToGltf::serialize(&mesh_snapshot).expect("real semio/mesh -> gltf codec must succeed on a real tessellated box");
        assert_eq!(gltf.document.meshes.len(), 1, "expected exactly one gltf mesh for one solid");
        assert_eq!(gltf.buffers.len(), 1, "expected one packed geometry buffer");
        // 🌉️ `SemioMeshToGltf::serialize` unconditionally pushes the POSITION accessor first (before
        // any conditional NORMAL/TEXCOORD_0/COLOR_0), so accessors[0] is always POSITION here.
        let position_accessor = gltf.document.accessors.first().expect("POSITION accessor must exist");
        assert_eq!(position_accessor.count, mesh_positions.len(), "gltf POSITION accessor count must match the semio/mesh vertex count");
        let buffer_view = &gltf.document.buffer_views[position_accessor.buffer_view.expect("POSITION accessor must reference a bufferView")];
        let raw = &gltf.buffers[0][buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];
        let decoded_positions: Vec<[f64; 3]> = raw
            .chunks_exact(12)
            .map(|triple| {
                [
                    f32::from_le_bytes(triple[0..4].try_into().unwrap()) as f64,
                    f32::from_le_bytes(triple[4..8].try_into().unwrap()) as f64,
                    f32::from_le_bytes(triple[8..12].try_into().unwrap()) as f64,
                ]
            })
            .collect();
        assert_eq!(decoded_positions.len(), mesh_positions.len(), "decoded gltf buffer must carry exactly the semio/mesh vertex count");
        let (gltf_min, gltf_max) = vertex_bounds(decoded_positions.into_iter());
        for axis in 0..3 {
            assert!((gltf_min[axis] - brep_min[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: gltf {} vs brep {}", gltf_min[axis], brep_min[axis]);
            assert!((gltf_max[axis] - brep_max[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: gltf {} vs brep {}", gltf_max[axis], brep_max[axis]);
        }
    }

    /// 🌉️ Directly exercises the two new bridge helpers (rather than through `export_solids_as`) to
    /// prove `SemioBrepSnapshot` itself carries real, non-empty topology -- not just that the
    /// STEP text round trip happens to look right.
    #[test]
    fn semio_brep_snapshot_from_step_text_carries_real_topology() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let step_text = block_on(kernel.export_step(std::slice::from_ref(&solid))).expect("kernel step export");
        let brep = semio_brep_snapshot_from_step_text(&step_text).expect("semio/brep from step");
        assert!(!brep.solids.is_empty(), "expected at least one real BrepSolid");
        assert!(!brep.faces.is_empty(), "expected real BrepFaces, not an empty shell");
        assert!(!brep.vertices.is_empty(), "expected real BrepVertexes");
        let round_tripped = step_text_from_semio_brep_snapshot(&brep).expect("semio/brep to step");
        assert!(round_tripped.starts_with("ISO-10303-21;"));
    }

    /// 🩹️ Direct, isolated proof of the framework `write_step` workaround: repairs a trailing
    /// comma before `)` while leaving an identical-looking comma inside a real STEP string literal
    /// alone (a genuine, if unlikely, product name could legitimately contain `,)`).
    #[test]
    fn repair_step_trailing_comma_before_close_paren_is_quote_aware() {
        assert_eq!(repair_step_trailing_comma_before_close_paren("(#1,)"), "(#1)");
        assert_eq!(repair_step_trailing_comma_before_close_paren("(#1, #2,)"), "(#1, #2)");
        assert_eq!(repair_step_trailing_comma_before_close_paren("()"), "()");
        assert_eq!(repair_step_trailing_comma_before_close_paren("('weird,)name', #1)"), "('weird,)name', #1)");
    }
    //#endregion 🔖️SemioBrepBridge
}
//#endregion 🔖️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::cad::standards::v1::subsets::any::schema::CadComposer as CadAnyComposer;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::CadBuilder as CadAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const CAD_DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };
    const CAD_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::cad::CadSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == CAD_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => CadAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => CadAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "CadComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == CAD_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::artifacts::cad::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "CadComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_IFC_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    fn compose_export_ifc(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::ifc::v4::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_IFC_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    fn compose_export_step(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::step::v_ap214::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STEP_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::png::v1_2::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::stl::v_ascii::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::gltf::v2_0::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::io::export::serializers::artifacts::obj::v3_0::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<CadAnyComposer>(),
            ComposerEntry { writes: EXPORT_IFC_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_ifc },
            ComposerEntry { writes: EXPORT_STEP_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_step },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
