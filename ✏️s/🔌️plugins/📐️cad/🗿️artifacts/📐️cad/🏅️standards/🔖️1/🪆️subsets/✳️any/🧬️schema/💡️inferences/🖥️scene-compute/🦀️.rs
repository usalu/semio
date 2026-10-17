//! 🖥️ Scene compute of the cad inference: the derived scene, mesh and contribution compute over the snapshot. See [`super`].

use semio_s_artifact_cad_cad_model::{geometry::centroid_from_host_snapshot_primitives, geometry::import_geometry_handles, geometry::resolve_primitive_handle, geometry::tessellate_geometry_handle, geometry::tessellate_object_mesh, geometry::tessellate_object_mesh_from_host_snapshot, geometry::CadGeometry, geometry::CadObject, geometry::CadPrimitiveSlot};
#[cfg(test)] use semio_s_artifact_cad_cad_model::geometry::{objects_from_host_snapshot_model, parse_geometry};
use semio_s_artifact_cad_cad_model::{snapshot::CadCamera, snapshot::CadNode, snapshot::CadPaneId, snapshot::CadProjectionDsl, snapshot::CadReference, snapshot::CadSnapshot, snapshot::CAD_PLAY_DOCUMENT_SCHEMA};
#[cfg(test)] use crate::{CadWorkingScene};
use semio_framework::parse_contributions;
use semio_framework_plugin::{mesh_from_kind, MeshData, WorldProjectionConfig};
use semio_framework_3d_brep_engine::mesh_data_from_mesh_transfer;
use {semio_framework_3d_brep_engine::{Brep, BrepKernel, GeometryHandle}, semio_framework_3d_brep_representation::contract::MeshTransfer};
use std::sync::{OnceLock};
#[cfg(test)] use std::sync::{Arc};

pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

pub const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

/// 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.










pub const CAD_MODEL_DEFINITION_SHAPE: &str = "spatial.shape";

pub const CAD_MODEL_DEFINITION_BUILDING: &str = "aec.building";

pub const CAD_MODEL_DEFINITION_ENERGY: &str = "aec.building.energy";

pub const CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC: &str = "aec.building.structure.classic";

/// 🖼️ Served by the `/cad-assets` static-dir row in this plugin's `Cargo.toml` (root
/// `📚️examples/🖼️assets`); the file is the kind-only basename `🖼️.png` under its own
/// `🌲️concrete-forest-reference` folder.
pub const CAD_CONCRETE_FOREST_REFERENCE_URL: &str = "/cad-assets/🌲️concrete-forest-reference/🖼️.png";

pub const CAD_FOREST_REFERENCE_WIDTH_WORLD: f64 = 28.6;

pub const CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX: f64 = 1430.0;

pub const CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX: f64 = 692.0;

const CAD_FOREST_REFERENCE_BASE_ORIGIN_XY: [f64; 2] = [-24.0, -18.0];

pub const CAD_FOREST_REFERENCE_PLANE_Z: f64 = 0.01;

pub const CAD_FOREST_REFERENCE_Y_OFFSET_RATIO: f64 = 0.2;

/// 🌱 Fresh, doctrine-tier-(d) brep kernel: a `Brep::new()` local to the caller, never a
/// process-global session (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
/// wave G4 — this replaces the deleted `static HOST: OnceLock<BrepEngineHost>`, which was the
/// exact ambient-reach anti-pattern the ticket exists to remove even though it was write-once).
/// Every call site already builds, uses and drops its handles within the one call that owns
/// this kernel, so no cross-call registry was ever load-bearing.
pub fn cad_brep_kernel() -> Brep {
    Brep::new()
}

/// 📐️ Tessellates a typology's primitive sized from authored geometry (or a universal
/// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>, solid_handle: Option<&str>, centroid: Option<[f64; 3]>) -> MeshData {
    let mut kernel = cad_brep_kernel();
    if let Some(handle_id) = solid_handle {
        let handle = GeometryHandle(handle_id.into());
        if let Ok(mesh) = kernel.tessellate(&handle, 0.1) {
            if let Ok(data)=mesh_data_from_mesh_transfer(&mesh) {return data;}
        }
    }
    let Some(handle) = typology_local_solid(&mut kernel, typology, extent) else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    let mesh: MeshTransfer = match kernel.tessellate(&handle, 0.1) {
        Ok(mesh) => mesh,
        Err(_) => {
            kernel.dispose(&handle);
            return mesh_from_kind(typology_mesh_kind(typology));
        }
    };
    kernel.dispose(&handle);
    let Ok(mut mesh_data) = mesh_data_from_mesh_transfer(&mesh) else {return mesh_from_kind(typology_mesh_kind(typology));};
    if let Some(center) = centroid {
        translate_mesh_positions(&mut mesh_data, [center[0] as f32, center[1] as f32, center[2] as f32]);
    }
    mesh_data
}

/// 🧊️ A typology's primitive as a local kernel solid, sized from authored geometry (or the universal fallback
/// extent) — the one size rule [`typology_brep_mesh`] tessellates and [`pane_world_solids`] exports.
fn typology_local_solid(kernel: &mut Brep, typology: &str, extent: Option<[f64; 3]>) -> Option<GeometryHandle> {
    let [ex, ey, ez] = extent.unwrap_or(CAD_DEFAULT_TYPOLOGY_EXTENT);
    let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
    match typology_mesh_kind(typology) {
        "cylinder" => kernel.cylinder_prim(width.max(depth) * 0.5, height),
        "sphere" => kernel.sphere_prim(width.max(depth).max(height) * 0.5),
        _ => kernel.box_prim(width, depth, height),
    }
    .ok()
}

/// 🌍️ A pane's visible objects as WORLD-space kernel solids, built the way the viewport builds their meshes
/// ([`object_mesh_data`]): the host-snapshot primitive an object names (moved onto the primitives' centroid, as
/// [`align_mesh_to_host_snapshot_centroid`] moves the mesh), else its typology primitive — then placed by its
/// instance's node transform (`world_instances_json`). What a STEP/OBJ/STL export writes is what the pane shows.
pub fn pane_world_solids(kernel: &mut Brep, objects: &[CadObject], geometry: Option<&CadGeometry>) -> Vec<GeometryHandle> {
    let handles = geometry.map(|geometry| import_geometry_handles(kernel, geometry)).unwrap_or_default();
    objects
        .iter()
        .filter(|object| object.visible)
        .filter_map(|object| {
            if let Some(geometry) = geometry.filter(|geometry| object.solid_handle.as_ref().is_some_and(|id| geometry.owned_meshes.contains_key(id))) {
                let snapshot = geometry.owned_breps.get(object.solid_handle.as_ref()?)?;
                let body = semio_s_artifact_stdio_semio_brep::schema::snapshot::body::body_from_snapshot(snapshot).ok()?;
                let local = kernel.merge_representation(body);
                return Some(local.into_iter().filter_map(|solid| place_object_solid(kernel, solid, object)).collect::<Vec<_>>());
            }
            let local = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| host_snapshot_local_solid(kernel, object, geometry, &handles)).or_else(|| typology_local_solid(kernel, &object.typology, object.extent))?;
            place_object_solid(kernel, local, object).map(|solid| vec![solid])
        })
        .flatten()
        .collect()
}

/// 🧲️ The imported host-snapshot primitive an object names, moved onto the primitives' own centroid when it
/// is more than 5 cm away — the kernel twin of [`align_mesh_to_host_snapshot_centroid`].
fn host_snapshot_local_solid(kernel: &mut Brep, object: &CadObject, geometry: &CadGeometry, handles: &std::collections::HashMap<String, String>) -> Option<GeometryHandle> {
    let (handle_id, kind) = resolve_primitive_handle(&object.primitives, handles)?;
    let handle = GeometryHandle(handle_id.clone());
    let Some(target) = centroid_from_host_snapshot_primitives(geometry, &object.primitives) else {
        return Some(handle);
    };
    let current = tessellate_geometry_handle(kernel, &handle_id, &kind).as_ref().and_then(mesh_centroid)?;
    let delta = [target[0] - f64::from(current[0]), target[1] - f64::from(current[1]), target[2] - f64::from(current[2])];
    if delta[0].abs() + delta[1].abs() + delta[2].abs() > 0.05 {
        kernel.translate(&handle, delta).ok()
    } else {
        Some(handle)
    }
}

/// 📍️ An object's node transform applied to its local solid: scale about the local origin, rotation by its
/// unit quaternion `[x, y, z, w]`, translation to `origin` — the order the viewport composes an instance.
fn place_object_solid(kernel: &mut Brep, local: GeometryHandle, object: &CadObject) -> Option<GeometryHandle> {
    let scale = object.scale.unwrap_or([1.0, 1.0, 1.0]);
    let scaled = if scale == [1.0, 1.0, 1.0] { local } else { kernel.scale_axes(&local, scale, [0.0, 0.0, 0.0]).ok()? };
    let [x, y, z, w] = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let sine = (x * x + y * y + z * z).sqrt();
    let rotated = if sine <= f64::EPSILON { scaled } else { kernel.rotate(&scaled, [x / sine, y / sine, z / sine], 2.0 * sine.atan2(w)).ok()? };
    kernel.translate(&rotated, object.origin).ok()
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

/// 📐️ Shifts a tessellated mesh onto the authored fixture primitive centroid when kernel output drifts.
pub(crate) fn align_mesh_to_host_snapshot_centroid(mesh: &mut MeshData, geometry: &CadGeometry, primitives: &[CadPrimitiveSlot]) {
    let Some(target) = centroid_from_host_snapshot_primitives(geometry, primitives) else {
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

/// 🖼️ Centers the concrete-forest reference and moves it forward from the authored base corner.
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

/// 🗃️ Reads one pane's objects and geometry from the shared quad fixture.


/// 🌲️ `cad_document_pane_bundle`, scoped to the Concrete Forest Left fixture and keyed by
/// `CadPaneId` rather than a raw fixture index — the real, non-stub object+geometry source
/// `crate::editor::cad::forest_working_scene` (the app layer's `CadWorkingScene` test/render
/// fixture) builds each pane from.


fn forest_references_for_model_definitions(reference_z: f64) -> semio_s_artifact_cad_cad_model::CadReferenceIndex {
    let mut output = semio_s_artifact_cad_cad_model::CadReferenceIndex::new();
    for pane in CadPaneId::all() {
        output.insert(pane.model_definition_id().into(), vec![CadReference {
            id: "ref-concrete-forest".into(), source_url: CAD_CONCRETE_FOREST_REFERENCE_URL.into(),
            media_kind: "image".into(), origin: forest_reference_origin(reference_z),
            orientation: None, scale: None, width_world: CAD_FOREST_REFERENCE_WIDTH_WORLD,
            hidden: false, locked: true, opacity: Some(1.0),
        }]);
    }
    output
}

/// 🧊️ The kernel primitive a typology's persisted `extent` rebuilds: cylinders and spheres are
/// CENTRED on the object origin in x/y (the kernel's `cylinder_prim`/`sphere_prim` axis), boxes
/// grow from the origin corner (`box_prim`).
pub fn typology_mesh_kind(typology: &str) -> &'static str {
    match typology {
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" | "aec.building.column" | "spatial.shape.solid.cylinder" => "cylinder",
        "spatial.shape.solid.sphere" => "sphere",
        _ => "box",
    }
}

/// 🆕️ The document a new CAD artifact starts as: no objects, and every pane composing its EMPTY
/// `s.stdio.semio@v1/model` child (`crate::cad_empty_pane_child`), so the first object gesture, construction or
/// import in any pane has a child lane to land on (design §20.15).
pub fn default_document() -> CadSnapshot {
    CadSnapshot {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        shape_model: Some(semio_s_artifact_cad_cad_model::snapshot::cad_empty_pane_child(CadPaneId::Shape)),
        building_model: Some(semio_s_artifact_cad_cad_model::snapshot::cad_empty_pane_child(CadPaneId::Building)),
        energy_model: Some(semio_s_artifact_cad_cad_model::snapshot::cad_empty_pane_child(CadPaneId::Energy)),
        structure_classic_model: Some(semio_s_artifact_cad_cad_model::snapshot::cad_empty_pane_child(CadPaneId::StructureClassic)),
        drawings: Vec::new(),
        breps: Vec::new(),
        nodes: vec![CadNode { id: "node-root".into(), label: "Model".into(), kind: "group".into() }, CadNode { id: "node-box".into(), label: "Box".into(), kind: "solid".into() }],
        references_by_model_definition_id: semio_s_artifact_cad_cad_model::CadReferenceIndex::new(),
    }
}

/// 🌲️ The Concrete Forest Left example's genesis scene for `pane` — the pane's objects and the fixture's raw wire/vertex
/// `CadGeometry` (which `SemioModelSnapshot` has no field for), imported once through the real importer
/// (`cad_document_pane_bundle`) and shared by the example document and the genesis catalogue
/// (`crate::cad_bundled_pane_scene`).


/// 📟️ Builds the quad play document: every pane composes its STABLE named model child
/// (`crate::cad_named_pane_child`), whose content is born from the genesis catalogue its id names — the Concrete
/// Forest Left fixture's pane scene ([`forest_pane_scene`]) — and lives in the child store from then on (design
/// §20.15). The handle is two strings and never a digest of the content it was born with.
fn forest_play_document() -> CadSnapshot {
    let child = |pane: CadPaneId| Some(semio_s_artifact_cad_cad_model::snapshot::cad_named_pane_child(CAD_EXAMPLE_FOREST_LEFT, pane));
    CadSnapshot {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: CAD_EXAMPLE_FOREST_LEFT.into(),
        shape_model: child(CadPaneId::Shape),
        building_model: child(CadPaneId::Building),
        energy_model: child(CadPaneId::Energy),
        structure_classic_model: child(CadPaneId::StructureClassic),
        drawings: Vec::new(),
        breps: Vec::new(),
        nodes: vec![CadNode { id: "node-root".into(), label: "Concrete Forest Left".into(), kind: "group".into() }],
        references_by_model_definition_id: forest_references_for_model_definitions(CAD_FOREST_REFERENCE_PLANE_Z),
    }
}

/// 🌲️ The Concrete Forest Left example projection — a bare `CadSnapshot` (no runtime/history),
/// wrapped into a `ArtifactStore` by `VcsArtifactApp` when spawned. Cached so manifest registration,
/// `initial_snapshot`, and `setActiveExample` share one BREP import instead of rebuilding thrice.
pub fn forest_play_scene() -> CadSnapshot {
    static FOREST_PLAY_SCENE: OnceLock<CadSnapshot> = OnceLock::new();
    FOREST_PLAY_SCENE.get_or_init(forest_play_document).clone()
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

pub fn ensure_object_solid_handle(kernel: &mut Brep, object: &mut CadObject) {
    if object.solid_handle.is_some() {
        return;
    }
    if let Some(handle) = super::solid_for_object(kernel, object) {
        let primitive_id = handle.0;
        object.solid_handle = Some(primitive_id.clone());
        if object.primitives.is_empty() {
            object.primitives.push(CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() });
        }
    }
}

pub fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
    object.mesh_url.as_ref().filter(|url| !url.is_empty()).cloned()
}

pub(crate) fn primary_primitive_kind(object: &CadObject) -> &str {
    object.primitives.first().map_or("solid", |primitive| primitive.kind.as_str())
}

pub fn object_mesh_data(object: &CadObject, geometry: Option<&CadGeometry>) -> MeshData {
    if let Some(mesh) = object.solid_handle.as_ref().and_then(|id| geometry?.owned_meshes.get(id)) {
        return (**mesh).clone();
    }
    let kind = primary_primitive_kind(object);
    {
        let mut kernel = cad_brep_kernel();
        let mesh = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| tessellate_object_mesh_from_host_snapshot(&mut kernel, object, geometry)).or_else(|| tessellate_object_mesh(&mut kernel, object, kind));
        if let Some(mut mesh) = mesh {
            if let Some(geometry) = geometry {
                align_mesh_to_host_snapshot_centroid(&mut mesh, geometry, &object.primitives);
            }
            return mesh;
        }
    }
    let centroid = geometry.and_then(|geometry| centroid_from_host_snapshot_primitives(geometry, &object.primitives));
    typology_brep_mesh(&object.typology, object.extent, object.solid_handle.as_deref(), centroid)
}

pub fn object_scale_json(object: &CadObject) -> [f64; 3] {
    object.scale.unwrap_or([1.0, 1.0, 1.0])
}

/// 🧵️ Tessellates a representative mesh for the OS mesh-exporter boundary — the document's
/// first object across panes, or the default box typology for an empty scene (no runtime selection
/// exists at this boundary).
/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: this used to scan the
/// document's inline object list for a representative mesh. `CadSnapshot` no longer carries
/// inline objects (only composed model child HANDLES, unresolved at this boundary) — falls back
/// to the default box typology unconditionally. Documented reduced-fidelity gap, not silently
/// wrong: `document`'s model-child handles are available via `crate::
/// cad_pane_model` for a caller that has ALSO resolved the child content and wants to do better.
pub fn export_mesh_from_scene(document: &CadSnapshot) -> MeshData {
    let _ = document;
    typology_brep_mesh("spatial.shape.primitive.box", None, None, None)
}

//#region 🧩️Contributions
const CAD_COMPUTER_TOPIC: &str = "cad.computer";

/// 🗂️ `cad.computer` topic payload shape (`TopicContribution` counterpart, ex `Contribution::CadComputer`).
#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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

/// 🧩️ One accepted `cad.computer` pack: the contributing module and its own computer roster,
/// still as the JSON the contributor authored (cad-js owns that vocabulary, not this crate).
#[derive(Clone, Debug, PartialEq)]
pub struct CadComputerContribution {
    pub plugin_id: String,
    pub module_id: String,
    pub computers_json: String,
}

/// 🧩️ Validates host-pushed `CadComputer` contributions for `cad-play` (implementations register
/// in cad-js) and RETURNS the packs that were accepted — an entry on another topic, or addressed
/// to another app, is skipped rather than refused, so a mixed host payload still installs cad's
/// own share. Returning the accepted packs (instead of `()`) is what makes acceptance observable:
/// `CadConfigMutation::SetContributions`'s diff calls this for its side effect, and the laws call
/// it for its answer.
pub fn validate_cad_computer_contributions(contributions_json: &str) -> Vec<CadComputerContribution> {
    parse_contributions(contributions_json)
        .into_iter()
        .filter_map(|entry| {
            let (app_id, module_id, computers_json) = cad_computer_fields(&entry)?;
            (app_id == "cad-play").then(|| CadComputerContribution { plugin_id: entry.plugin_id.clone(), module_id, computers_json })
        })
        .collect()
}
//#endregion 🧩️Contributions
