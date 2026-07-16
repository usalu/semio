//! 📏 CAD plugin — spatial model play app bundled as a hot-swappable WASM component.

pub mod geometry_import {
    //! 📐 Fixture geometry import — builds kernel handles from authored spatial.model geometry.

    use cad_document::{CadEdge, CadFace, CadGeometry, CadObject, CadPrimitiveSlot, CadShell, CadSolid, CadWire};
    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, Vec3};
    use semio_framework_core::mesh_from_indexed;
    use semio_framework_plugin::MeshData;
    use serde_json::Value;
    use std::collections::HashMap;

    //#region 🔖Parse
    pub fn parse_geometry(value: Option<&Value>) -> CadGeometry {
        value
            .and_then(|entry| serde_json::from_value(entry.clone()).ok())
            .unwrap_or_default()
    }

    fn vertex_map(geometry: &CadGeometry) -> HashMap<String, [f64; 3]> {
        geometry
            .vertices
            .iter()
            .map(|vertex| (vertex.id.clone(), vertex.position))
            .collect()
    }

    fn edge_map(geometry: &CadGeometry) -> HashMap<String, &CadEdge> {
        geometry.edges.iter().map(|edge| (edge.id.clone(), edge)).collect()
    }

    fn wire_map(geometry: &CadGeometry) -> HashMap<String, &CadWire> {
        geometry.wires.iter().map(|wire| (wire.id.clone(), wire)).collect()
    }

    fn face_map(geometry: &CadGeometry) -> HashMap<String, &CadFace> {
        geometry.faces.iter().map(|face| (face.id.clone(), face)).collect()
    }

    fn shell_map(geometry: &CadGeometry) -> HashMap<String, &CadShell> {
        geometry.shells.iter().map(|shell| (shell.id.clone(), shell)).collect()
    }

    fn solid_map(geometry: &CadGeometry) -> HashMap<String, &CadSolid> {
        geometry.solids.iter().map(|solid| (solid.id.clone(), solid)).collect()
    }
    //#endregion 🔖Parse

    //#region 🔖KernelBuild
    fn to_vec3(point: [f64; 3]) -> Vec3 {
        [point[0], point[1], point[2]]
    }

    fn wire_points(
        wire: &CadWire,
        edges: &HashMap<String, &CadEdge>,
        vertices: &HashMap<String, [f64; 3]>,
    ) -> Vec<Vec3> {
        let mut points = Vec::new();
        for edge_id in &wire.edge_ids {
            let Some(edge) = edges.get(edge_id) else {
                continue;
            };
            if edge.vertex_ids.len() < 2 {
                continue;
            };
            let Some(start) = vertices.get(&edge.vertex_ids[0]) else {
                continue;
            };
            let Some(end) = vertices.get(&edge.vertex_ids[1]) else {
                continue;
            };
            if points.is_empty() {
                points.push(to_vec3(*start));
            }
            points.push(to_vec3(*end));
        }
        points
    }

    fn face_boundary_points(
        face: &CadFace,
        wires: &HashMap<String, &CadWire>,
        edges: &HashMap<String, &CadEdge>,
        vertices: &HashMap<String, [f64; 3]>,
    ) -> Vec<Vec3> {
        face.wire_ids
            .iter()
            .find_map(|wire_id| wires.get(wire_id))
            .map(|wire| wire_points(wire, edges, vertices))
            .unwrap_or_default()
    }

    pub fn import_geometry_handles(
        kernel: &mut dyn BrepKernel,
        geometry: &CadGeometry,
    ) -> HashMap<String, String> {
        let vertices = vertex_map(geometry);
        let edges = edge_map(geometry);
        let wires = wire_map(geometry);
        let faces = face_map(geometry);
        let shells = shell_map(geometry);
        let _solids = solid_map(geometry);
        let mut handles: HashMap<String, String> = HashMap::new();

        for wire in &geometry.wires {
            let points = wire_points(wire, &edges, &vertices);
            if points.len() < 2 {
                continue;
            }
            if let Ok(handle) = kernel_3d_engine::block_on(kernel.polyline_wire(&points)) {
                handles.insert(wire.id.clone(), handle.0.clone());
            }
        }

        for face in &geometry.faces {
            if let Some(wire_id) = face.wire_ids.first() {
                if let Some(wire_handle) = handles.get(wire_id) {
                    let wire = GeometryHandle(wire_handle.clone());
                    if let Ok(handle) = kernel_3d_engine::block_on(kernel.planar_face_from_wire(&wire)) {
                        handles.insert(face.id.clone(), handle.0.clone());
                        continue;
                    }
                    if let Ok(handle) = kernel_3d_engine::block_on(kernel.face_from_wire(&wire)) {
                        handles.insert(face.id.clone(), handle.0.clone());
                        continue;
                    }
                }
            }
            let points = face_boundary_points(face, &wires, &edges, &vertices);
            if points.len() < 3 {
                continue;
            }
            if let Ok(handle) = kernel_3d_engine::block_on(kernel.planar_face_from_points(&points)) {
                handles.insert(face.id.clone(), handle.0.clone());
            }
        }

        for shell in &geometry.shells {
            let face_handles: Vec<GeometryHandle> = shell
                .face_ids
                .iter()
                .filter_map(|face_id| faces.get(face_id))
                .filter_map(|face| handles.get(&face.id).cloned().map(GeometryHandle))
                .collect();
            if face_handles.len() < 1 {
                continue;
            }
            if let Ok(solid) = kernel_3d_engine::block_on(kernel.sew_faces(&face_handles, 0.01)) {
                handles.insert(shell.id.clone(), solid.0.clone());
            }
        }

        for solid in &geometry.solids {
            let shell_handles: Vec<GeometryHandle> = solid
                .shell_ids
                .iter()
                .filter_map(|shell_id| handles.get(shell_id).cloned().map(GeometryHandle))
                .collect();
            if shell_handles.len() == 1 {
                handles.insert(solid.id.clone(), shell_handles[0].0.clone());
                continue;
            }
            let face_handles: Vec<GeometryHandle> = solid
                .shell_ids
                .iter()
                .filter_map(|shell_id| shells.get(shell_id))
                .flat_map(|shell| shell.face_ids.iter())
                .filter_map(|face_id| handles.get(face_id).cloned().map(GeometryHandle))
                .collect();
            if face_handles.is_empty() {
                continue;
            }
            if let Ok(built) = kernel_3d_engine::block_on(kernel.sew_faces(&face_handles, 0.01)) {
                handles.insert(solid.id.clone(), built.0.clone());
            }
        }

        handles
    }

    pub fn resolve_primitive_handle(
        primitives: &[CadPrimitiveSlot],
        handles: &HashMap<String, String>,
    ) -> Option<(String, String)> {
        for primitive in primitives {
            if let Some(handle) = handles.get(&primitive.primitive_id) {
                return Some((handle.clone(), primitive.kind.clone()));
            }
        }
        None
    }

    /// Collects the world-space vertex positions reachable from a solid/shell/face/wire/edge id by
    /// descending the authored topology graph (fixture vertex positions are already world-absolute).
    fn primitive_vertex_positions(geometry: &CadGeometry, primitive_id: &str) -> Vec<[f64; 3]> {
        let vertices = vertex_map(geometry);
        let edges = edge_map(geometry);
        let wires = wire_map(geometry);
        let faces = face_map(geometry);
        let shells = shell_map(geometry);
        let solids = solid_map(geometry);

        let wire_ids: Vec<&String> = if let Some(solid) = solids.get(primitive_id) {
            solid
                .shell_ids
                .iter()
                .filter_map(|shell_id| shells.get(shell_id))
                .flat_map(|shell| shell.face_ids.iter())
                .filter_map(|face_id| faces.get(face_id))
                .flat_map(|face| face.wire_ids.iter())
                .collect()
        } else if let Some(shell) = shells.get(primitive_id) {
            shell
                .face_ids
                .iter()
                .filter_map(|face_id| faces.get(face_id))
                .flat_map(|face| face.wire_ids.iter())
                .collect()
        } else if let Some(face) = faces.get(primitive_id) {
            face.wire_ids.iter().collect()
        } else {
            Vec::new()
        };

        let edge_ids: Vec<&String> = if let Some(wire) = wires.get(primitive_id) {
            wire.edge_ids.iter().collect()
        } else {
            wire_ids
                .into_iter()
                .filter_map(|wire_id| wires.get(wire_id))
                .flat_map(|wire| wire.edge_ids.iter())
                .collect()
        };

        let vertex_ids: Vec<&String> = if let Some(edge) = edges.get(primitive_id) {
            edge.vertex_ids.iter().collect()
        } else {
            edge_ids
                .into_iter()
                .filter_map(|edge_id| edges.get(edge_id))
                .flat_map(|edge| edge.vertex_ids.iter())
                .collect()
        };

        vertex_ids.into_iter().filter_map(|id| vertices.get(id).copied()).collect()
    }

    fn extent_from_positions(positions: &[[f64; 3]]) -> Option<[f64; 3]> {
        if positions.is_empty() {
            return None;
        }
        let mut min = [f64::MAX; 3];
        let mut max = [f64::MIN; 3];
        for position in positions {
            for axis in 0..3 {
                min[axis] = min[axis].min(position[axis]);
                max[axis] = max[axis].max(position[axis]);
            }
        }
        Some([max[0] - min[0], max[1] - min[1], max[2] - min[2]])
    }

    /// Derives an object's world-space bounding extent from its authored geometry, trying each
    /// primitive slot in order (mirrors `resolve_primitive_handle`'s slot priority).
    pub fn extent_from_fixture_primitives(geometry: &CadGeometry, primitives: &[CadPrimitiveSlot]) -> Option<[f64; 3]> {
        primitives
            .iter()
            .find_map(|primitive| extent_from_positions(&primitive_vertex_positions(geometry, &primitive.primitive_id)))
    }

    pub fn tessellate_geometry_handle(
        kernel: &mut dyn BrepKernel,
        handle_id: &str,
        kind: &str,
    ) -> Option<MeshData> {
        let handle = GeometryHandle(handle_id.into());
        if kind == "curve" {
            return curve_mesh_from_wire(kernel, &handle);
        }
        if let Ok(mesh) = block_on(kernel.tessellate(&handle, 0.1)) {
            return Some(mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index));
        }
        None
    }

    fn curve_mesh_from_wire(kernel: &mut dyn BrepKernel, wire: &GeometryHandle) -> Option<MeshData> {
        let profile_wire = kernel_3d_engine::block_on(kernel.regular_polygon_wire(0.08, 8)).ok()?;
        let profile_face = kernel_3d_engine::block_on(kernel.planar_face_from_wire(&profile_wire)).ok()?;
        let solid = kernel_3d_engine::block_on(kernel.sweep(&profile_face, wire)).ok()?;
        let mesh = block_on(kernel.tessellate(&solid, 0.1)).ok()?;
        let _ = kernel_3d_engine::block_on(kernel.dispose(&solid));
        let _ = kernel_3d_engine::block_on(kernel.dispose(&profile_face));
        let _ = kernel_3d_engine::block_on(kernel.dispose(&profile_wire));
        Some(mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index))
    }

    //#region 🔖MeshImport
    /// 📦 Serializes triangle mesh data to a minimal `v`/`f` OBJ payload the kernel's OBJ reader
    /// (which only needs vertex positions and triangle faces) can round-trip into a solid.
    fn mesh_to_obj_text(mesh: &MeshData) -> String {
        let mut text = String::new();
        for vertex in mesh.positions.chunks_exact(3) {
            text.push_str(&format!("v {} {} {}\n", vertex[0], vertex[1], vertex[2]));
        }
        for triangle in mesh.indices.chunks_exact(3) {
            text.push_str(&format!("f {} {} {}\n", triangle[0] + 1, triangle[1] + 1, triangle[2] + 1));
        }
        text
    }

    fn mesh_extent(mesh: &MeshData) -> Option<[f64; 3]> {
        if mesh.positions.is_empty() {
            return None;
        }
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for vertex in mesh.positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(vertex[axis]);
                max[axis] = max[axis].max(vertex[axis]);
            }
        }
        Some([(max[0] - min[0]) as f64, (max[1] - min[1]) as f64, (max[2] - min[2]) as f64])
    }

    /// 🧱 Builds a `CadObject` from an arbitrary triangle mesh (e.g. a tessellated DWG layer) by
    /// importing it into the brep kernel as a solid via the OBJ reader, so it plays through the same
    /// `solidHandle`/`primitives` path fixture geometry uses. Falls back to an extent-only object
    /// with no primitives (rendered via the typology bounding-box mesh fallback) when the mesh has
    /// no triangles or the kernel is unable to import it.
    pub fn cad_object_from_mesh(
        kernel: &mut dyn BrepKernel,
        id: impl Into<String>,
        label: impl Into<String>,
        typology: impl Into<String>,
        mesh: &MeshData,
    ) -> CadObject {
        let extent = mesh_extent(mesh);
        let solid_handle = if mesh.indices.len() >= 3 {
            kernel_3d_engine::block_on(kernel.import_obj(&mesh_to_obj_text(mesh), 0.01)).ok().map(|handle| handle.0)
        } else {
            None
        };
        let primitives = solid_handle
            .clone()
            .map(|primitive_id| vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() }])
            .unwrap_or_default();
        CadObject {
            id: id.into(),
            label: label.into(),
            typology: typology.into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent,
            solid_handle,
            primitives,
        }
    }
    /// 🧊 Builds a `CadObject` around a solid `GeometryHandle` already resident in `kernel` (e.g. from
    /// a native OBJ/STL/STEP import), tessellating once just to derive a display `extent` — the
    /// handle itself is kept verbatim rather than being round-tripped through a mesh reimport.
    pub fn cad_object_from_solid_handle(
        kernel: &mut dyn BrepKernel,
        id: impl Into<String>,
        label: impl Into<String>,
        typology: impl Into<String>,
        handle: GeometryHandle,
    ) -> CadObject {
        let extent = block_on(kernel.tessellate(&handle, 0.1))
            .ok()
            .and_then(|mesh| mesh_extent(&mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index)));
        let handle_id = handle.0.clone();
        CadObject {
            id: id.into(),
            label: label.into(),
            typology: typology.into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent,
            solid_handle: Some(handle_id.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: handle_id, kind: "solid".into() }],
        }
    }
    //#endregion 🔖MeshImport

    pub fn object_label_from_id(object_id: &str) -> String {
        object_id
            .split('-')
            .last()
            .map(str::to_string)
            .unwrap_or_else(|| object_id.to_string())
    }

    pub fn objects_from_fixture_model(
        kernel: &mut dyn BrepKernel,
        objects_value: &[Value],
        geometry: &CadGeometry,
    ) -> Vec<CadObject> {
        let handles = import_geometry_handles(kernel, geometry);
        objects_value
            .iter()
            .filter_map(|entry| {
                let object_id = entry.get("id")?.as_str()?;
                let typology = entry
                    .get("typology")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string();
                let primitives = primitives_from_json(entry);
                let (solid_handle, _primary_kind) = resolve_primitive_handle(&primitives, &handles)
                    .map(|(handle, kind)| (Some(handle), kind))
                    .unwrap_or((None, String::new()));
                let extent = extent_from_fixture_primitives(geometry, &primitives);
                Some(CadObject {
                    id: object_id.into(),
                    label: object_label_from_id(object_id),
                    typology,
                    visible: true,
                    locked: false,
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: None,
                    extent,
                    solid_handle,
                    primitives,
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
    //#endregion 🔖KernelBuild

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn forest_shape_geometry_imports_solid_handle() {
            let source = include_str!("../../asset/play/hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
            let objects = root
                .pointer("/models/0/model/objects")
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            assert_eq!(imported.len(), 1);
            assert!(imported[0].solid_handle.is_some());
            let mesh = tessellate_geometry_handle(
                &mut kernel,
                imported[0].solid_handle.as_ref().expect("handle"),
                "solid",
            );
            assert!(mesh.is_some());
            assert!(mesh.unwrap().positions.len() > 12);
        }

        #[test]
        fn forest_structure_curve_wires_tessellate_as_tubes() {
            let source = include_str!("../../asset/play/hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/3/model/geometry"));
            let objects = root
                .pointer("/models/3/model/objects")
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            assert!(!imported.is_empty());
            let curve_object = imported
                .iter()
                .find(|object| object.primitives.iter().any(|primitive| primitive.kind == "curve"))
                .expect("curve object");
            let handle = curve_object.solid_handle.as_ref().expect("curve handle");
            let mesh = tessellate_geometry_handle(&mut kernel, handle, "curve").expect("curve mesh");
            assert!(mesh.positions.len() > 36);
            assert!(mesh.indices.len() > 12);
        }
    }
}
mod interaction {
    //! 🎮 CAD interaction statechart — a generic interpreter over `spatial.interaction` JSON assets
    //! (`cad/asset/modelDefinition/*/interaction/*.json`, mirroring `cad/schema/json/interaction.json`),
    //! plus a small commit-action runner mapping each spec's `commit.operation.action` onto real
    //! `kernel_3d_brepkit` calls. Four "building.building.*" ids have no JSON asset (aec.building has
    //! no interaction directory) and keep a bespoke hand-written statechart (`legacy_*` functions)
    //! identical to the pre-engine behavior.

    use cad_document::{
        evaluate_expr, CadObject, CadPaneId, CadPrimitiveSlot, DisplayItemSpec, Effect, ExprEnv, ExprPathRoot,
        ExprPathSegment, ExprPathTarget, InteractionSpec,
    };
    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::BrepKernel;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::sync::OnceLock;

    //#region 🔖Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CadEngagementSession {
        pub interaction_id: String,
        pub state: String,
        pub context: HashMap<String, Value>,
        pub pane: CadPaneId,
        #[serde(default)]
        pub last_response: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct KeyedTransition {
        pub key: String,
        pub label: String,
        pub event_kind: String,
    }

    #[derive(Clone, Debug)]
    pub struct InteractionCatalogEntry {
        pub id: String,
        pub label: String,
        pub key: String,
        pub model_definition_id: String,
        pub produces_typology: String,
    }
    //#endregion 🔖Types

    //#region 🔖Registry
    /// `(modelDefinitionId, raw JSON)` for every `interaction/*.json` asset embedded at build time.
    /// `aec.building` has no interaction assets of its own — see `LEGACY_BUILDING_INTERACTION_IDS`.
    const RAW_INTERACTION_ASSETS: &[(&str, &str)] = &[
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/arc.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/area.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/booleanDifference.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/booleanIntersection.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/booleanUnion.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/box.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/chamfer.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/circle.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/constructCurve.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/constructSurface.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/controlPointCurve.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/copy.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/createAnchor.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/cylinder.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/explode.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/extrudeCrv.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/extrudeWire.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/fillet.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/interpolateCurve.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/join.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/length.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/line.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/loft.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/mirror.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/move.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/networkSrf.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/offsetSurface.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/plane.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/polyline.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/rotate.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/scale1d.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/scale3d.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/sphere.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/split.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/sweep1.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/sweep2.json")),
        ("spatial.shape", include_str!("../../asset/modelDefinition/spatial.shape/interaction/trim.json")),
        ("aec.building.energy", include_str!("../../asset/modelDefinition/aec.building.energy/interaction/constructBasePlate.json")),
        ("aec.building.energy", include_str!("../../asset/modelDefinition/aec.building.energy/interaction/constructExternalWall.json")),
        ("aec.building.energy", include_str!("../../asset/modelDefinition/aec.building.energy/interaction/constructHull.json")),
        ("aec.building.energy", include_str!("../../asset/modelDefinition/aec.building.energy/interaction/constructRoof.json")),
        ("aec.building.energy", include_str!("../../asset/modelDefinition/aec.building.energy/interaction/constructWindows.json")),
        (
            "aec.building.structure.classic",
            include_str!("../../asset/modelDefinition/aec.building.structure.classic/interaction/constructOneWayReinforcedConcreteSlab.json"),
        ),
        (
            "aec.building.structure.classic",
            include_str!("../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteColumn.json"),
        ),
        (
            "aec.building.structure.classic",
            include_str!("../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteExternalWall.json"),
        ),
        (
            "aec.building.structure.classic",
            include_str!("../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteInternalWall.json"),
        ),
        (
            "aec.building.structure.fem.line",
            include_str!("../../asset/modelDefinition/aec.building.structure.fem.line/interaction/constructLineElement.json"),
        ),
        (
            "aec.building.structure.fem.solid",
            include_str!("../../asset/modelDefinition/aec.building.structure.fem.solid/interaction/constructSolidElement.json"),
        ),
        (
            "aec.building.structure.fem.surface",
            include_str!("../../asset/modelDefinition/aec.building.structure.fem.surface/interaction/constructSurfaceElement.json"),
        ),
    ];

    const LEGACY_BUILDING_INTERACTION_IDS: &[&str] = &[
        "building.building.constructWall",
        "building.building.constructBeam",
        "building.building.constructColumn",
        "building.building.constructSlab",
    ];

    fn is_legacy_building_id(id: &str) -> bool {
        LEGACY_BUILDING_INTERACTION_IDS.contains(&id)
    }

    static PARSED_SPECS: OnceLock<Vec<(&'static str, InteractionSpec)>> = OnceLock::new();

    fn parsed_specs() -> &'static [(&'static str, InteractionSpec)] {
        PARSED_SPECS.get_or_init(|| {
            RAW_INTERACTION_ASSETS
                .iter()
                .filter_map(|(model_def, raw)| serde_json::from_str::<InteractionSpec>(raw).ok().map(|spec| (*model_def, spec)))
                .collect()
        })
    }

    fn spec_by_id(id: &str) -> Option<&'static InteractionSpec> {
        parsed_specs().iter().find(|(_, spec)| spec.id == id).map(|(_, spec)| spec)
    }

    static CATALOG: OnceLock<Vec<InteractionCatalogEntry>> = OnceLock::new();

    fn catalog() -> &'static [InteractionCatalogEntry] {
        CATALOG.get_or_init(|| {
            let mut entries = vec![
                InteractionCatalogEntry {
                    id: "building.building.constructWall".to_string(),
                    label: "Wall".to_string(),
                    key: "w".to_string(),
                    model_definition_id: "aec.building".to_string(),
                    produces_typology: "building.building.wall".to_string(),
                },
                InteractionCatalogEntry {
                    id: "building.building.constructBeam".to_string(),
                    label: "Beam".to_string(),
                    key: "m".to_string(),
                    model_definition_id: "aec.building".to_string(),
                    produces_typology: "building.building.beam".to_string(),
                },
                InteractionCatalogEntry {
                    id: "building.building.constructColumn".to_string(),
                    label: "Column".to_string(),
                    key: "c".to_string(),
                    model_definition_id: "aec.building".to_string(),
                    produces_typology: "building.building.column".to_string(),
                },
                InteractionCatalogEntry {
                    id: "building.building.constructSlab".to_string(),
                    label: "Slab".to_string(),
                    key: "l".to_string(),
                    model_definition_id: "aec.building".to_string(),
                    produces_typology: "building.building.slab".to_string(),
                },
            ];
            for (model_def, spec) in parsed_specs() {
                entries.push(InteractionCatalogEntry {
                    id: spec.id.clone(),
                    label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),
                    key: spec.key.clone().unwrap_or_default(),
                    model_definition_id: (*model_def).to_string(),
                    produces_typology: spec.produces.typology.clone().unwrap_or_default(),
                });
            }
            entries
        })
    }
    //#endregion 🔖Registry

    //#region 🔖Catalog
    pub fn list_interactions_for_model_definition(model_definition_id: &str) -> Vec<&'static InteractionCatalogEntry> {
        catalog().iter().filter(|entry| entry.model_definition_id == model_definition_id).collect()
    }

    pub fn resolve_interaction_key(input: &str, model_definition_id: &str) -> Option<&'static InteractionCatalogEntry> {
        let trimmed = input.trim().to_lowercase();
        catalog().iter().find(|entry| {
            entry.model_definition_id == model_definition_id
                && (entry.key == trimmed
                    || entry.id.eq_ignore_ascii_case(&trimmed)
                    || entry.id.to_lowercase().ends_with(&format!(".{trimmed}")))
        })
    }

    pub fn interaction_by_id(id: &str) -> Option<&'static InteractionCatalogEntry> {
        catalog().iter().find(|entry| entry.id == id)
    }
    //#endregion 🔖Catalog

    //#region 🔖Statechart
    fn vec3_json(point: [f64; 3]) -> Value {
        json!([point[0], point[1], point[2]])
    }

    fn parse_vec3(value: &Value) -> Option<[f64; 3]> {
        let array = value.as_array()?;
        if array.len() < 3 {
            return None;
        }
        Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
    }

    fn context_point(session: &CadEngagementSession, field: &str) -> Option<[f64; 3]> {
        session.context.get(field).and_then(parse_vec3)
    }

    pub fn start_session(interaction_id: &str, pane: CadPaneId) -> Option<CadEngagementSession> {
        if is_legacy_building_id(interaction_id) {
            return Some(CadEngagementSession {
                interaction_id: interaction_id.to_string(),
                state: "idle".to_string(),
                context: HashMap::new(),
                pane,
                last_response: None,
            });
        }
        let spec = spec_by_id(interaction_id)?;
        Some(CadEngagementSession {
            interaction_id: spec.id.clone(),
            state: spec.machine.initial.clone(),
            context: HashMap::new(),
            pane,
            last_response: None,
        })
    }

    pub fn keyed_transitions(session: &CadEngagementSession) -> Vec<KeyedTransition> {
        if is_legacy_building_id(&session.interaction_id) {
            return legacy_keyed_transitions(session);
        }
        let Some(spec) = spec_by_id(&session.interaction_id) else {
            return Vec::new();
        };
        let Some(state) = spec.state(&session.state) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for handler in &state.on {
            for transition in &handler.transitions {
                if let Some(key) = &transition.key {
                    out.push(KeyedTransition {
                        key: key.clone(),
                        label: transition.label.clone().unwrap_or_else(|| handler.event.clone()),
                        event_kind: handler.event.clone(),
                    });
                }
            }
        }
        out
    }

    pub fn can_commit(session: &CadEngagementSession) -> bool {
        if is_legacy_building_id(&session.interaction_id) {
            return session.state == "ready";
        }
        let Some(spec) = spec_by_id(&session.interaction_id) else {
            return false;
        };
        if !spec.commit.from_states.iter().any(|state| state == &session.state) {
            return false;
        }
        match &spec.commit.when {
            None => true,
            Some(guard_name) => {
                let env = ExprEnv { context: &session.context, event: None };
                spec.guard(guard_name, &env)
            }
        }
    }

    fn context_target_field(target: &ExprPathTarget) -> Option<&str> {
        if target.root != ExprPathRoot::Context {
            return None;
        }
        match target.segments.as_slice() {
            [ExprPathSegment::Field { name }] => Some(name.as_str()),
            _ => None,
        }
    }

    /// Wraps a raw event payload into the shape the JSON specs' `event.*` path expressions expect:
    /// `pointer.down`/`pointer.move` read `event.point`, `set.*` events read `event.value`. Callers
    /// (both `lib.rs`'s command handlers and this module's own tests) pass raw values (a `[x,y,z]`
    /// array, a bare number) for brevity — already-wrapped objects pass through unchanged.
    fn normalize_event_payload(event_kind: &str, payload: Option<&Value>) -> Option<Value> {
        let payload = payload?;
        if payload.is_object() {
            return Some(payload.clone());
        }
        if event_kind == "pointer.down" || event_kind == "pointer.move" {
            return Some(json!({ "point": payload }));
        }
        if event_kind.starts_with("set.") {
            return Some(json!({ "value": payload }));
        }
        Some(payload.clone())
    }

    /// Executes an `action` effect by name.
    ///
    /// `command.addPoint` (used by sphere/circle/etc.) records a named point into a
    /// `context[field][key]` map. `box.aabbFromDiagonalCorners` (box's default diagonal-mode second
    /// click) derives `context.origin`/`context.corner` — the axis-aligned min/max of `context.diagA`
    /// and `event.point` — which `hasValidBox` and the commit params then read.
    ///
    /// The remaining `box.*` rubber-band helpers and selection-driven actions (used only by box's
    /// advanced cube/3-point/center sub-modes and by selection-based utilities) are a documented
    /// follow-up; they no-op here rather than error.
    fn run_named_action_effect(
        context: &mut HashMap<String, Value>,
        payload: Option<&Value>,
        action: &str,
        params: &HashMap<String, Value>,
    ) {
        match action {
            "command.addPoint" => {
                let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("points").to_string();
                let key = params.get("key").and_then(|value| value.as_str()).map(str::to_string);
                let point = params.get("point").cloned().unwrap_or(Value::Null);
                let entry = context.entry(field).or_insert_with(|| json!({}));
                if !entry.is_object() {
                    *entry = json!({});
                }
                if let (Some(key), Some(object)) = (key, entry.as_object_mut()) {
                    object.insert(key, point);
                }
            }
            "box.aabbFromDiagonalCorners" => {
                let diag_a = context.get("diagA").and_then(parse_vec3);
                let second = payload.and_then(|value| value.get("point")).and_then(parse_vec3);
                if let (Some(a), Some(b)) = (diag_a, second) {
                    let origin = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
                    let corner = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
                    context.insert("origin".into(), vec3_json(origin));
                    context.insert("corner".into(), vec3_json(corner));
                }
            }
            _ => {}
        }
    }

    fn apply_effect(session: &mut CadEngagementSession, payload: Option<&Value>, effect: &Effect, raised: &mut Vec<String>) {
        let empty_vars = HashMap::new();
        match effect {
            Effect::Assign { target, value } => {
                if let Some(field) = context_target_field(target) {
                    let env = ExprEnv { context: &session.context, event: payload };
                    let evaluated = evaluate_expr(value, &env, &empty_vars);
                    session.context.insert(field.to_string(), evaluated);
                }
            }
            Effect::Clear { target } => {
                if let Some(field) = context_target_field(target) {
                    session.context.remove(field);
                }
            }
            Effect::Append { target, value } => {
                if let Some(field) = context_target_field(target) {
                    let env = ExprEnv { context: &session.context, event: payload };
                    let evaluated = evaluate_expr(value, &env, &empty_vars);
                    let entry = session.context.entry(field.to_string()).or_insert_with(|| json!([]));
                    if let Some(array) = entry.as_array_mut() {
                        array.push(evaluated);
                    } else {
                        *entry = json!([evaluated]);
                    }
                }
            }
            Effect::Raise { event } => raised.push(event.clone()),
            Effect::Action { action, params, .. } => {
                let env = ExprEnv { context: &session.context, event: payload };
                let evaluated: HashMap<String, Value> =
                    params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
                run_named_action_effect(&mut session.context, payload, action, &evaluated);
            }
            // Emit/OpenTransaction/CommitTransaction/RollbackTransaction/RequestPreview/KernelQuery/
            // ResolveEditable/SetDiagnostic/ClearDiagnostic/InteractionCall are not yet interpreted —
            // InteractionCall (nested sub-interaction composition) is a documented follow-up used only
            // by the curve-drawing sub-flow (`mode.curve`); the primary `mode.2points` flow doesn't
            // depend on it. The others have no observable effect on committed geometry.
            _ => {}
        }
    }

    fn apply_event_generic(session: &mut CadEngagementSession, event_kind: &str, raw_payload: Option<&Value>, depth: u8) -> bool {
        if depth > 8 {
            return false;
        }
        let Some(spec) = spec_by_id(&session.interaction_id) else {
            return false;
        };
        let Some(state) = spec.state(&session.state) else {
            return false;
        };
        let Some(handler) = state.on.iter().find(|handler| handler.event == event_kind) else {
            return false;
        };
        let normalized = normalize_event_payload(event_kind, raw_payload);
        let payload = normalized.as_ref();
        let chosen = handler.transitions.iter().find(|transition| match &transition.guard {
            None => true,
            Some(name) => {
                let env = ExprEnv { context: &session.context, event: payload };
                spec.guard(name, &env)
            }
        });
        let Some(transition) = chosen else {
            return false;
        };
        let mut raised = Vec::new();
        for effect in &transition.effects {
            apply_effect(session, payload, effect, &mut raised);
        }
        if let Some(target) = &transition.target {
            session.state = target.clone();
        }
        session.last_response = Some("OK".into());
        for raised_event in raised {
            apply_event_generic(session, &raised_event, None, depth + 1);
        }
        true
    }

    fn legacy_keyed_transitions(session: &CadEngagementSession) -> Vec<KeyedTransition> {
        if session.state == "idle" {
            return vec![KeyedTransition { key: "s".into(), label: "Start".into(), event_kind: "start".into() }];
        }
        Vec::new()
    }

    fn legacy_apply_event(session: &mut CadEngagementSession, event_kind: &str, payload: Option<&Value>) -> bool {
        let is_column = session.interaction_id == "building.building.constructColumn";
        let changed = match (session.state.as_str(), event_kind) {
            ("idle", "start") => {
                session.state = if is_column { "column_base" } else { "footprint_first" }.into();
                true
            }
            ("footprint_first", "pointer.down") => {
                if let Some(point) = payload.and_then(parse_vec3) {
                    session.context.insert("cornerA".into(), vec3_json(point));
                    session.state = "footprint_second".into();
                    true
                } else {
                    false
                }
            }
            ("footprint_second", "pointer.down") => {
                if let Some(point) = payload.and_then(parse_vec3) {
                    session.context.insert("cornerB".into(), vec3_json(point));
                    session.state = "slab_height".into();
                    true
                } else {
                    false
                }
            }
            ("slab_height", "set.height") => {
                if let Some(height) = payload.and_then(|value| value.as_f64()) {
                    session.context.insert("height".into(), json!(height));
                    session.state = "ready".into();
                    true
                } else {
                    false
                }
            }
            ("column_base", "pointer.down") => {
                if let Some(point) = payload.and_then(parse_vec3) {
                    session.context.insert("base".into(), vec3_json(point));
                    session.state = "column_height".into();
                    true
                } else {
                    false
                }
            }
            ("column_height", "set.height") => {
                if let Some(height) = payload.and_then(|value| value.as_f64()) {
                    session.context.insert("height".into(), json!(height));
                    session.state = "ready".into();
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        if changed {
            session.last_response = Some("OK".into());
        }
        changed
    }

    pub fn apply_event(session: &mut CadEngagementSession, event_kind: &str, payload: Option<&Value>) -> bool {
        if is_legacy_building_id(&session.interaction_id) {
            return legacy_apply_event(session, event_kind, payload);
        }
        apply_event_generic(session, event_kind, payload, 0)
    }

    /// States where a numeric-only line commits the pending height (premigration `tryCommitNumericEntry`).
    const NUMERIC_ENTRY_STATES: &[&str] =
        &["first_corner_height", "two_points_height", "slab_height", "column_height", "radius", "curve_height"];

    fn strip_prefix_ignore_case<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
        if text.len() < prefix.len() {
            return None;
        }
        let (head, tail) = text.split_at(prefix.len());
        head.eq_ignore_ascii_case(prefix).then_some(tail)
    }

    /// Parses a REPL command line into an `(event_kind, payload)` pair.
    ///
    /// `current_state` is the active engagement session's state (if any) — required to disambiguate a
    /// bare numeric line (e.g. `"3.5"`) as a height commit only while a numeric-entry state is active,
    /// mirroring premigration's `trySubmitLine` numeric-entry step.
    pub fn parse_repl_line(line: &str, current_state: Option<&str>) -> Option<(String, Option<Value>)> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }
        // Legacy raw forms (still used by the wgpu renderer's REPL, which does not PascalCase drafts).
        if let Some(rest) = trimmed.strip_prefix("set.height ") {
            return rest.trim().parse::<f64>().ok().map(|height| ("set.height".into(), Some(json!(height))));
        }
        if let Some(rest) = trimmed.strip_prefix("dist ") {
            return rest.trim().parse::<f64>().ok().map(|distance| ("set.distance".into(), Some(json!(distance))));
        }
        // Normalized forms: the React shell's engagement input PascalCases every draft (no separators),
        // so `set.height 3.5` arrives as `SetHeight3.5` (framework/renderer/react `Engagement.applyDraft`
        // via `normalizeEngagementCommandText`).
        if let Some(rest) = strip_prefix_ignore_case(trimmed, "SetHeight") {
            if let Ok(height) = rest.parse::<f64>() {
                return Some(("set.height".into(), Some(json!(height))));
            }
        }
        if let Some(rest) = strip_prefix_ignore_case(trimmed, "Dist") {
            if let Ok(distance) = rest.parse::<f64>() {
                return Some(("set.distance".into(), Some(json!(distance))));
            }
        }
        // Bare numeric entry commits height while a numeric-entry state is active.
        if current_state.is_some_and(|state| NUMERIC_ENTRY_STATES.contains(&state)) {
            if let Ok(height) = trimmed.parse::<f64>() {
                return Some(("set.height".into(), Some(json!(height))));
            }
        }
        Some((trimmed.into(), None))
    }
    //#endregion 🔖Statechart

    //#region 🔖CommitRunner
    fn commit_primitive_box(
        kernel: &mut dyn BrepKernel,
        params: &HashMap<String, Value>,
        label_count: usize,
        next_id: impl Fn(&str) -> String,
    ) -> Option<CadObject> {
        let corner_a = params.get("cornerA").and_then(parse_vec3)?;
        let corner_b = params.get("cornerB").and_then(parse_vec3)?;
        let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(1.0);
        let width = (corner_b[0] - corner_a[0]).abs().max(0.05);
        let depth = (corner_b[1] - corner_a[1]).abs().max(0.05);
        let solid = kernel_3d_engine::block_on(kernel.box_prim(width, depth, height.max(0.05))).ok()?;
        Some(CadObject {
            id: next_id("object"),
            label: format!("Box {}", label_count + 1),
            typology: "spatial.shape.primitive.box".into(),
            visible: true,
            locked: false,
            origin: corner_a,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([width, depth, height.max(0.05)]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        })
    }

    /// Generic commit for the "2 points + height" family shared by every `aec.building.energy`,
    /// `aec.building.structure.classic`, and `aec.building.structure.fem.*` construction interaction
    /// (`commit.operation.action` ending in `From2PointsAndHeight`/`FromSurface`) — differentiated only
    /// by the `typology` commit param.
    fn commit_from_2_points_and_height(
        kernel: &mut dyn BrepKernel,
        params: &HashMap<String, Value>,
        label: &str,
        label_count: usize,
        next_id: impl Fn(&str) -> String,
    ) -> Option<CadObject> {
        let typology = params.get("typology").and_then(|value| value.as_str()).unwrap_or("").to_string();
        let lower = typology.to_lowercase();
        let point_a = params.get("pointA").and_then(parse_vec3)?;
        let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);

        if lower.contains("column") {
            let radius = 0.25;
            let solid = kernel_3d_engine::block_on(kernel.cylinder_prim(radius, height.max(0.05))).ok()?;
            return Some(CadObject {
                id: next_id("object"),
                label: format!("{label} {}", label_count + 1),
                typology,
                visible: true,
                locked: false,
                origin: point_a,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
            });
        }

        let point_b = params.get("pointB").and_then(parse_vec3)?;
        let span = ((point_b[0] - point_a[0]).powi(2) + (point_b[1] - point_a[1]).powi(2)).sqrt().max(0.5);
        let (width, depth, solid_height) = if lower.contains("wall") {
            (span, 0.2, height.max(0.05))
        } else if lower.contains("windows") {
            (span, 0.05, height.max(0.05))
        } else {
            // slab / baseplate / roof / hull / fem elements: flat footprint extruded by `height`.
            let w = (point_b[0] - point_a[0]).abs().max(0.5);
            let d = (point_b[1] - point_a[1]).abs().max(0.5);
            (w, d, height.max(0.05))
        };
        let solid = kernel_3d_engine::block_on(kernel.box_prim(width, depth, solid_height)).ok()?;
        Some(CadObject {
            id: next_id("object"),
            label: format!("{label} {}", label_count + 1),
            typology,
            visible: true,
            locked: false,
            origin: point_a,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([width, depth, solid_height]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        })
    }

    /// `command.finish` dispatches by the commit's `resultKind` param, reading whatever context fields
    /// that interaction's machine populated (`points.<key>`, `radius`, ...). Only `sphere` is
    /// implemented so far; other result kinds (cylinder/circle/plane/curve/boolean/...) are a
    /// documented follow-up — this returns `None` for them, matching the pre-engine fallback behavior
    /// for any not-yet-implemented interaction.
    fn commit_command_finish(
        kernel: &mut dyn BrepKernel,
        params: &HashMap<String, Value>,
        context: &HashMap<String, Value>,
        label_count: usize,
        next_id: impl Fn(&str) -> String,
    ) -> Option<CadObject> {
        let result_kind = params.get("resultKind").and_then(|value| value.as_str())?;
        match result_kind {
            "sphere" => {
                let points = context.get("points")?.as_object()?;
                let center = points.get("center").and_then(parse_vec3)?;
                let radius = if let Some(radius) = context.get("radius").and_then(|value| value.as_f64()) {
                    radius
                } else {
                    let radius_point = points.get("radiusPoint").and_then(parse_vec3)?;
                    ((radius_point[0] - center[0]).powi(2)
                        + (radius_point[1] - center[1]).powi(2)
                        + (radius_point[2] - center[2]).powi(2))
                    .sqrt()
                }
                .max(0.05);
                let solid = kernel_3d_engine::block_on(kernel.sphere_prim(radius)).ok()?;
                Some(CadObject {
                    id: next_id("object"),
                    label: format!("Sphere {}", label_count + 1),
                    typology: "spatial.shape.solid.sphere".into(),
                    visible: true,
                    locked: false,
                    origin: center,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: None,
                    extent: Some([radius * 2.0, radius * 2.0, radius * 2.0]),
                    solid_handle: Some(solid.0.clone()),
                    primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
                })
            }
            _ => None,
        }
    }

    fn legacy_commit_object(
        kernel: &mut dyn BrepKernel,
        session: &CadEngagementSession,
        label_count: usize,
        next_id: impl Fn(&str) -> String,
    ) -> Option<CadObject> {
        let entry = interaction_by_id(&session.interaction_id)?;
        if session.interaction_id == "building.building.constructColumn" {
            let base = context_point(session, "base")?;
            let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);
            let radius = 0.25;
            let solid = kernel_3d_engine::block_on(kernel.cylinder_prim(radius, height.max(0.05))).ok()?;
            return Some(CadObject {
                id: next_id("object"),
                label: format!("{} {}", entry.label, label_count + 1),
                typology: entry.produces_typology.clone(),
                visible: true,
                locked: false,
                origin: base,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
            });
        }
        let corner_a = context_point(session, "cornerA")?;
        let corner_b = context_point(session, "cornerB")?;
        let id = session.interaction_id.as_str();
        let default_height = if id.contains("Slab") { 0.25 } else if id.contains("Beam") { 0.4 } else { 3.0 };
        let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(default_height);
        let span = ((corner_b[0] - corner_a[0]).powi(2) + (corner_b[1] - corner_a[1]).powi(2)).sqrt().max(0.5);
        let width = (corner_b[0] - corner_a[0]).abs().max(0.5);
        let depth = (corner_b[1] - corner_a[1]).abs().max(0.5);
        let (solid_width, solid_depth, solid_height) = if id.contains("Beam") {
            (span, 0.3, 0.3)
        } else if id.contains("Wall") {
            (span, 0.2, height.max(0.05))
        } else {
            (width, depth, height.max(0.05))
        };
        let solid = kernel_3d_engine::block_on(kernel.box_prim(solid_width, solid_depth, solid_height)).ok()?;
        Some(CadObject {
            id: next_id("object"),
            label: format!("{} {}", entry.label, label_count + 1),
            typology: entry.produces_typology.clone(),
            visible: true,
            locked: false,
            origin: corner_a,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([solid_width, solid_depth, solid_height]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        })
    }

    pub fn commit_object(
        kernel: &mut dyn BrepKernel,
        session: &CadEngagementSession,
        label_count: usize,
        next_id: impl Fn(&str) -> String,
    ) -> Option<CadObject> {
        if is_legacy_building_id(&session.interaction_id) {
            return legacy_commit_object(kernel, session, label_count, next_id);
        }
        let spec = spec_by_id(&session.interaction_id)?;
        let env = ExprEnv { context: &session.context, event: None };
        let empty_vars = HashMap::new();
        let params: HashMap<String, Value> = spec
            .commit
            .operation
            .params
            .iter()
            .map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars)))
            .collect();
        let action = spec.commit.operation.action.as_str();
        let label = spec.label.clone().unwrap_or_else(|| spec.id.clone());
        if action == "primitive.createBoxFromCorners" {
            return commit_primitive_box(kernel, &params, label_count, next_id);
        }
        if action.ends_with("From2PointsAndHeight") || action.ends_with("FromSurface") {
            return commit_from_2_points_and_height(kernel, &params, &label, label_count, next_id);
        }
        if action == "command.finish" {
            return commit_command_finish(kernel, &params, &session.context, label_count, next_id);
        }
        None
    }
    //#endregion 🔖CommitRunner

    //#region 🔖Preview
    fn preview_two_point_footprint(session: &CadEngagementSession, include_segment: bool) -> Vec<Value> {
        let mut items = Vec::new();
        if let Some(corner_a) = context_point(session, "cornerA") {
            items.push(json!({ "kind": "point", "role": "cornerA", "position": corner_a }));
        }
        if include_segment {
            if let (Some(corner_a), Some(corner_b)) = (context_point(session, "cornerA"), context_point(session, "cornerB")) {
                items.push(json!({ "kind": "segment", "role": "footprint", "from": corner_a, "to": corner_b }));
            }
        }
        items
    }

    fn legacy_preview_display_items(session: &CadEngagementSession) -> Vec<Value> {
        if session.interaction_id == "building.building.constructColumn" {
            return match session.state.as_str() {
                "column_height" | "ready" => {
                    let mut items = Vec::new();
                    if let Some(base) = context_point(session, "base") {
                        items.push(json!({ "kind": "point", "role": "base", "position": base }));
                    }
                    items
                }
                _ => Vec::new(),
            };
        }
        match session.state.as_str() {
            "footprint_first" => preview_two_point_footprint(session, false),
            "footprint_second" | "slab_height" | "ready" => preview_two_point_footprint(session, true),
            _ => Vec::new(),
        }
    }

    fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv, vars: &HashMap<String, Value>) -> Option<Value> {
        match item {
            DisplayItemSpec::Point { role, position, .. } => {
                let position = evaluate_expr(position, env, vars);
                if position.is_null() {
                    return None;
                }
                Some(json!({ "kind": "point", "role": role, "position": position }))
            }
            DisplayItemSpec::Label { role, text, position, .. } => {
                let position = evaluate_expr(position, env, vars);
                Some(json!({ "kind": "label", "role": role, "text": text, "position": position }))
            }
            DisplayItemSpec::Segment { role, from, to, .. } => {
                let from = evaluate_expr(from, env, vars);
                let to = evaluate_expr(to, env, vars);
                if from.is_null() || to.is_null() {
                    return None;
                }
                Some(json!({ "kind": "segment", "role": role, "from": from, "to": to }))
            }
            DisplayItemSpec::LinearHandle { role, axis, origin, .. } => {
                let origin = evaluate_expr(origin, env, vars);
                if origin.is_null() {
                    return None;
                }
                Some(json!({ "kind": "linear-handle", "role": role, "axis": axis, "origin": origin }))
            }
            DisplayItemSpec::BoxPreview { role, corner_a, corner_b, height, .. } => {
                let corner_a = evaluate_expr(corner_a, env, vars);
                let corner_b = evaluate_expr(corner_b, env, vars);
                if corner_a.is_null() || corner_b.is_null() {
                    return None;
                }
                let height = evaluate_expr(height, env, vars);
                Some(json!({ "kind": "box-preview", "role": role, "cornerA": corner_a, "cornerB": corner_b, "height": height }))
            }
            DisplayItemSpec::EntityHighlight { role, geometry_entity_kind, entity_id, .. } => {
                let entity_id = evaluate_expr(entity_id, env, vars);
                if entity_id.is_null() {
                    return None;
                }
                Some(json!({ "kind": "entity-highlight", "role": role, "geometryEntityKind": geometry_entity_kind, "entityId": entity_id }))
            }
            DisplayItemSpec::Curve { role, .. } => Some(json!({ "kind": "curve", "role": role })),
            DisplayItemSpec::Mesh { role, .. } => Some(json!({ "kind": "mesh", "role": role })),
            DisplayItemSpec::Preview { role, preview_kind, params, .. } => {
                let evaluated_params: serde_json::Map<String, Value> =
                    params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, env, vars))).collect();
                Some(json!({ "kind": "preview", "role": role, "previewKind": preview_kind, "params": evaluated_params }))
            }
        }
    }

    pub fn preview_display_items(session: &CadEngagementSession) -> Vec<Value> {
        if is_legacy_building_id(&session.interaction_id) {
            return legacy_preview_display_items(session);
        }
        let Some(spec) = spec_by_id(&session.interaction_id) else {
            return Vec::new();
        };
        let Some(display_state) = spec.display.states.iter().find(|state| state.state == session.state) else {
            return Vec::new();
        };
        let env = ExprEnv { context: &session.context, event: None };
        let empty_vars = HashMap::new();
        display_state.items.iter().filter_map(|item| display_item_to_json(item, &env, &empty_vars)).collect()
    }
    //#endregion 🔖Preview

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn catalog_includes_json_driven_and_legacy_building_entries() {
            assert!(interaction_by_id("primitive.box").is_some());
            assert!(interaction_by_id("solid.sphere").is_some());
            assert!(interaction_by_id("energy.energy.constructExternalWall").is_some());
            assert!(interaction_by_id("structure.structure.constructReinforcedConcreteColumn").is_some());
            assert!(interaction_by_id("building.building.constructWall").is_some());
            assert_eq!(list_interactions_for_model_definition("spatial.shape").len(), 37);
        }

        #[test]
        fn box_interaction_commits_after_height() {
            let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "mode.diagonal", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([2.0, 3.0, 0.0]))));
            assert!(apply_event(&mut session, "set.height", Some(&json!(2.5))));
            assert!(apply_event(&mut session, "confirm", None));
            assert!(can_commit(&session));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            assert!(object.is_some());
            assert_eq!(object.unwrap().typology, "spatial.shape.primitive.box");
        }

        #[test]
        fn box_interaction_default_mode_is_point_and_requires_length_prompt() {
            // box.json's default `boxMode` (set by the `start` transition) is "point", not "diagonal" —
            // a plain pointer.down after start does NOT reach diagonal_rubber.
            let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
            assert_eq!(session.state, "first_corner_other_or_length");
        }

        #[test]
        fn sphere_interaction_commits_via_command_finish() {
            let mut session = start_session("solid.sphere", CadPaneId::Shape).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [2.0, 0.0, 0.0] }))));
            assert!(can_commit(&session));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            let object = object.expect("sphere commits");
            assert_eq!(object.typology, "spatial.shape.solid.sphere");
            assert_eq!(object.origin, [0.0, 0.0, 0.0]);
            assert_eq!(object.extent, Some([4.0, 4.0, 4.0]));
        }

        #[test]
        fn external_wall_interaction_commits_via_generic_from_2_points_and_height() {
            let mut session = start_session("energy.energy.constructExternalWall", CadPaneId::Energy).expect("session");
            assert!(apply_event(&mut session, "mode.2points", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [4.0, 0.0, 0.0] }))));
            assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 3.0 }))));
            assert!(can_commit(&session));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            let object = object.expect("wall commits");
            assert_eq!(object.typology, "energy.energy.externalwall");
        }

        #[test]
        fn reinforced_concrete_column_interaction_commits_as_cylinder() {
            let mut session =
                start_session("structure.structure.constructReinforcedConcreteColumn", CadPaneId::StructureClassic)
                    .expect("session");
            assert!(apply_event(&mut session, "mode.2points", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [1.0, 1.0, 0.0] }))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [1.5, 1.0, 0.0] }))));
            assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 3.0 }))));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            let object = object.expect("column commits");
            assert_eq!(object.typology, "structure.structure.reinforcedconcretecolumn");
            assert_eq!(object.origin, [1.0, 1.0, 0.0]);
        }

        #[test]
        fn slab_interaction_commits() {
            let mut session = start_session(
                "structure.structure.constructOneWayReinforcedConcreteSlab",
                CadPaneId::StructureClassic,
            )
            .expect("session");
            assert!(apply_event(&mut session, "mode.2points", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [4.0, 5.0, 0.0] }))));
            assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 0.3 }))));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            assert!(object.is_some());
        }

        #[test]
        fn slab_preview_shows_footprint_point() {
            let mut session = start_session(
                "structure.structure.constructOneWayReinforcedConcreteSlab",
                CadPaneId::StructureClassic,
            )
            .expect("session");
            assert!(apply_event(&mut session, "mode.2points", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
            let items = preview_display_items(&session);
            assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
        }

        #[test]
        fn legacy_column_preview_shows_base_point() {
            let mut session = start_session("building.building.constructColumn", CadPaneId::Building).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([1.0, 2.0, 0.0]))));
            let items = preview_display_items(&session);
            assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
        }

        #[test]
        fn legacy_wall_interaction_still_commits() {
            let mut session = start_session("building.building.constructWall", CadPaneId::Building).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([4.0, 0.0, 0.0]))));
            assert!(apply_event(&mut session, "set.height", Some(&json!(3.0))));
            assert!(can_commit(&session));
            let mut kernel = BrepkitKernel::new();
            let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
            assert!(object.is_some());
            assert_eq!(object.unwrap().typology, "building.building.wall");
        }

        #[test]
        fn parse_repl_line_accepts_legacy_raw_forms() {
            assert_eq!(parse_repl_line("set.height 2.5", None), Some(("set.height".into(), Some(json!(2.5)))));
            assert_eq!(parse_repl_line("dist 12", None), Some(("set.distance".into(), Some(json!(12.0)))));
        }

        #[test]
        fn parse_repl_line_accepts_shell_normalized_forms() {
            // The React shell PascalCases every draft (framework/renderer/react `normalizeEngagementCommandText`),
            // so `set.height 3.5` arrives as `SetHeight3.5` with no separators.
            assert_eq!(parse_repl_line("SetHeight3.5", None), Some(("set.height".into(), Some(json!(3.5)))));
            assert_eq!(parse_repl_line("setheight0.25", None), Some(("set.height".into(), Some(json!(0.25)))));
            assert_eq!(parse_repl_line("Dist12.75", None), Some(("set.distance".into(), Some(json!(12.75)))));
        }

        #[test]
        fn parse_repl_line_commits_bare_number_only_in_numeric_entry_state() {
            // Bare numeric entry (premigration `tryCommitNumericEntry`) only applies while a
            // numeric-entry state (e.g. box's first_corner_height) is active.
            assert_eq!(parse_repl_line("3.5", Some("first_corner_height")), Some(("set.height".into(), Some(json!(3.5)))));
            assert_eq!(parse_repl_line("2", Some("column_height")), Some(("set.height".into(), Some(json!(2.0)))));
            // Outside a numeric-entry state, a bare number is treated as an (unresolvable) interaction key.
            assert_eq!(parse_repl_line("3.5", None), Some(("3.5".into(), None)));
            assert_eq!(parse_repl_line("3.5", Some("idle")), Some(("3.5".into(), None)));
        }

        #[test]
        fn box_interaction_commits_via_shell_normalized_repl_line() {
            let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
            assert!(apply_event(&mut session, "start", None));
            assert!(apply_event(&mut session, "mode.diagonal", None));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
            assert!(apply_event(&mut session, "pointer.down", Some(&json!([2.0, 3.0, 0.0]))));
            let (event_kind, payload) = parse_repl_line("SetHeight2.5", Some(&session.state)).expect("parsed line");
            assert!(apply_event(&mut session, &event_kind, payload.as_ref()));
            assert!(apply_event(&mut session, "confirm", None));
            assert!(can_commit(&session));
        }
    }
}
mod transformation {
    //! 🔄 CAD derive-transformation engine — ports premigration `runDeriveTransformation` onto `kernel_3d_brepkit`.

    use cad_document::{CadObject, CadPrimitiveSlot};
    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::{BrepKernel, GeometryHandle, Vec3};
    use std::collections::HashMap;

    //#region 🔖ClassifyRules
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DominantAxis {
        X,
        Y,
        Z,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum ZBand {
        Min,
        Max,
        Mid,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ClassifyRule {
        pub role: &'static str,
        pub typology: &'static str,
        pub dominant_axis: Option<DominantAxis>,
        pub min_dominant_normal: Option<f64>,
        pub min_axis_normal: Option<f64>,
        pub z_band: Option<ZBand>,
        pub fallback: bool,
    }

    const FROM_GEOMETRY_CLASSIFY_RULES: &[ClassifyRule] = &[
        ClassifyRule {
            role: "roof",
            typology: "energy.energy.roof",
            dominant_axis: Some(DominantAxis::Z),
            min_dominant_normal: Some(0.75),
            min_axis_normal: None,
            z_band: Some(ZBand::Max),
            fallback: false,
        },
        ClassifyRule {
            role: "baseplate",
            typology: "energy.energy.baseplate",
            dominant_axis: Some(DominantAxis::Z),
            min_dominant_normal: Some(0.75),
            min_axis_normal: None,
            z_band: Some(ZBand::Min),
            fallback: false,
        },
        ClassifyRule {
            role: "slab",
            typology: "energy.energy.hull",
            dominant_axis: Some(DominantAxis::Z),
            min_dominant_normal: Some(0.75),
            min_axis_normal: None,
            z_band: None,
            fallback: false,
        },
        ClassifyRule {
            role: "externalwall",
            typology: "energy.energy.externalwall",
            dominant_axis: None,
            min_dominant_normal: None,
            min_axis_normal: Some(0.5),
            z_band: None,
            fallback: false,
        },
        ClassifyRule {
            role: "slab",
            typology: "energy.energy.hull",
            dominant_axis: None,
            min_dominant_normal: None,
            min_axis_normal: None,
            z_band: None,
            fallback: true,
        },
    ];

    const ENERGY_TYPOLOGIES: &[&str] = &[
        "energy.energy.hull",
        "energy.energy.baseplate",
        "energy.energy.roof",
        "energy.energy.externalwall",
        "energy.energy.windows",
    ];
    //#endregion 🔖ClassifyRules

    //#region 🔖FaceAnalytics
    /// @emoji 📍 Face centroid via surface midpoint sampling (premigration `faceCentroid` equivalent).
    pub fn face_centroid_sync(kernel: &dyn BrepKernel, face: &GeometryHandle) -> Option<Vec3> {
        kernel_3d_engine::block_on(kernel.surface_point(face, 0.5, 0.5)).ok()
    }

    /// @emoji 🧭 Face outward normal at the surface midpoint.
    pub fn face_normal_sync(kernel: &dyn BrepKernel, face: &GeometryHandle) -> Option<Vec3> {
        kernel_3d_engine::block_on(kernel.surface_normal(face, 0.5, 0.5)).ok()
    }

    /// @emoji 🗂️ Groups coplanar faces by dominant axis, sign, and quantized centroid (premigration `facePlaneGroupKey`).
    pub fn face_plane_group_key(normal: Vec3, centroid: Vec3) -> String {
        let [nx, ny, nz] = normal;
        let abs = [nx.abs(), ny.abs(), nz.abs()];
        let (dominant, sign) = if abs[0] >= abs[1] && abs[0] >= abs[2] {
            ("x", nx.signum())
        } else if abs[1] >= abs[2] {
            ("y", ny.signum())
        } else {
            ("z", nz.signum())
        };
        let q = |v: f64| (v * 1000.0).round() / 1000.0;
        format!(
            "{dominant}:{sign}:{}:{}:{}",
            q(centroid[0]),
            q(centroid[1]),
            q(centroid[2])
        )
    }

    fn dominant_axis_of(normal: Vec3) -> DominantAxis {
        let [nx, ny, nz] = normal;
        let abs = [nx.abs(), ny.abs(), nz.abs()];
        if abs[0] >= abs[1] && abs[0] >= abs[2] {
            DominantAxis::X
        } else if abs[1] >= abs[2] {
            DominantAxis::Y
        } else {
            DominantAxis::Z
        }
    }

    fn axis_normal_component(normal: Vec3, axis: DominantAxis) -> f64 {
        match axis {
            DominantAxis::X => normal[0].abs(),
            DominantAxis::Y => normal[1].abs(),
            DominantAxis::Z => normal[2].abs(),
        }
    }

    fn classify_rule_matches(
        rule: &ClassifyRule,
        normal: Vec3,
        centroid_z: f64,
        z_min: f64,
        z_max: f64,
        z_tol: f64,
    ) -> bool {
        if rule.fallback {
            return true;
        }
        if let Some(min_axis) = rule.min_axis_normal {
            let dominant = dominant_axis_of(normal);
            if axis_normal_component(normal, dominant) < min_axis {
                return false;
            }
            if rule.dominant_axis.is_some() {
                return false;
            }
            return true;
        }
        if let Some(axis) = rule.dominant_axis {
            if dominant_axis_of(normal) != axis {
                return false;
            }
            if let Some(min_dom) = rule.min_dominant_normal {
                if axis_normal_component(normal, axis) < min_dom {
                    return false;
                }
            }
            if let Some(band) = rule.z_band {
                return match band {
                    ZBand::Min => (centroid_z - z_min).abs() <= z_tol,
                    ZBand::Max => (centroid_z - z_max).abs() <= z_tol,
                    ZBand::Mid => {
                        let mid = (z_min + z_max) * 0.5;
                        (centroid_z - mid).abs() <= z_tol
                    }
                };
            }
            return true;
        }
        false
    }
    //#endregion 🔖FaceAnalytics

    //#region 🔖SolidConstruction
    /// @emoji 📦 Builds or reuses a kernel solid for a CAD object.
    pub fn solid_for_object(kernel: &mut dyn BrepKernel, object: &CadObject) -> Option<GeometryHandle> {
        if let Some(handle) = object.solid_handle.as_ref() {
            if kernel_3d_engine::block_on(kernel.kind(&GeometryHandle(handle.clone()))).is_ok() {
                return Some(GeometryHandle(handle.clone()));
            }
        }
        let [ex, ey, ez] = object.extent.unwrap_or([1.0, 1.0, 1.0]);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        let is_cylindrical = object.typology.contains("column");
        let handle = if is_cylindrical {
            kernel_3d_engine::block_on(kernel.cylinder_prim(width.max(depth) * 0.5, height)).ok()
        } else {
            kernel_3d_engine::block_on(kernel.box_prim(width, depth, height)).ok()
        }?;
        Some(handle)
    }

    /// @emoji 📦 Builds a kernel solid sized from extent without mutating the object.
    pub fn build_solid_for_typology(
        kernel: &mut dyn BrepKernel,
        typology: &str,
        extent: [f64; 3],
    ) -> Option<GeometryHandle> {
        let [ex, ey, ez] = extent;
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        if typology.contains("column") {
            kernel_3d_engine::block_on(kernel.cylinder_prim(width.max(depth) * 0.5, height)).ok()
        } else {
            kernel_3d_engine::block_on(kernel.box_prim(width, depth, height)).ok()
        }
    }

    fn fuse_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle]) -> Option<GeometryHandle> {
        if solids.is_empty() {
            return None;
        }
        let mut current = solids[0].clone();
        for solid in solids.iter().skip(1) {
            current = kernel_3d_engine::block_on(kernel.fuse(&current, solid)).ok()?;
        }
        Some(current)
    }
    //#endregion 🔖SolidConstruction

    //#region 🔖DeriveEngine
    struct FaceMeta {
        handle: GeometryHandle,
        normal: Vec3,
        centroid: Vec3,
    }

    fn next_object_id(prefix: &str, index: usize) -> String {
        format!("{prefix}-{index}")
    }

    /// @emoji 🔄 Derives energy objects from shape-pane solids via fuse + face classification.
    pub fn run_derive_from_geometry(
        kernel: &mut dyn BrepKernel,
        source_objects: &[CadObject],
        id_seed: &str,
    ) -> Vec<CadObject> {
        let solids: Vec<GeometryHandle> = source_objects
            .iter()
            .filter_map(|object| solid_for_object(kernel, object))
            .collect();
        if solids.is_empty() {
            return Vec::new();
        }
        let hull = match fuse_solids(kernel, &solids) {
            Some(hull) => hull,
            None => return Vec::new(),
        };
        let topology = match kernel_3d_engine::block_on(kernel.deconstruct(&hull)) {
            Ok(topology) => topology,
            Err(_) => return Vec::new(),
        };
        let mut face_meta: Vec<FaceMeta> = topology
            .faces
            .iter()
            .filter_map(|face| {
                let normal = face_normal_sync(kernel, face)?;
                let centroid = face_centroid_sync(kernel, face)?;
                Some(FaceMeta {
                    handle: face.clone(),
                    normal,
                    centroid,
                })
            })
            .collect();
        if face_meta.is_empty() {
            return vec![CadObject {
                id: next_object_id(id_seed, 0),
                label: "Hull".into(),
                typology: "energy.energy.hull".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: Some(hull.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: hull.0.clone(),
                    kind: "solid".into(),
                }],
            }];
        }
        let z_min = face_meta.iter().map(|face| face.centroid[2]).fold(f64::INFINITY, f64::min);
        let z_max = face_meta.iter().map(|face| face.centroid[2]).fold(f64::NEG_INFINITY, f64::max);
        let z_span = (z_max - z_min).max(0.001);
        let z_tol = (z_span * 0.02).max(0.001);
        let mut objects = Vec::new();
        let hull_id = next_object_id(id_seed, 0);
        objects.push(CadObject {
            id: hull_id.clone(),
            label: "Hull".into(),
            typology: "energy.energy.hull".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: Some(hull.0.clone()),
            primitives: vec![CadPrimitiveSlot {
                slot: "solid".into(),
                primitive_id: hull.0.clone(),
                kind: "solid".into(),
            }],
        });
        let mut grouped: HashMap<String, Vec<&FaceMeta>> = HashMap::new();
        for face in &face_meta {
            let rule = FROM_GEOMETRY_CLASSIFY_RULES
                .iter()
                .find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol))
                .unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
            if rule.role == "slab" && rule.fallback {
                continue;
            }
            let key = format!("{}:{}", rule.typology, face_plane_group_key(face.normal, face.centroid));
            grouped.entry(key).or_default().push(face);
        }
        let mut index = 1usize;
        for (_key, faces) in grouped {
            let face = faces[0];
            let rule = FROM_GEOMETRY_CLASSIFY_RULES
                .iter()
                .find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol))
                .unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
            let label = rule.role.replace("externalwall", "External Wall").replace("slab", "Slab");
            objects.push(CadObject {
                id: next_object_id(id_seed, index),
                label: label.into(),
                typology: rule.typology.into(),
                visible: true,
                locked: false,
                origin: face.centroid,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: Some(face.handle.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "surface".into(),
                    primitive_id: face.handle.0.clone(),
                    kind: "surface".into(),
                }],
            });
            index += 1;
        }
        if !objects.iter().any(|object| object.typology == "energy.energy.windows") {
            objects.push(CadObject {
                id: next_object_id(id_seed, index),
                label: "Windows".into(),
                typology: "energy.energy.windows".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: None,
                primitives: Vec::new(),
            });
        }
        objects
    }

    const BUILDING_TO_STRUCTURE: &[(&str, &str)] = &[
        ("building.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
        ("building.building.column", "structure.structure.reinforcedconcretecolumn"),
        ("building.building.beam", "structure.structure.reinforcedconcretebeam"),
        ("building.building.wall", "structure.structure.reinforcedconcreteinternalwall"),
        ("aec.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
        ("aec.building.column", "structure.structure.reinforcedconcretecolumn"),
    ];

    /// @emoji 🔄 Maps building typologies to structure-classic equivalents (premigration `from_building` applier).
    pub fn apply_from_building(source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        source_objects
            .iter()
            .filter_map(|object| {
                BUILDING_TO_STRUCTURE
                    .iter()
                    .find(|(from, _)| *from == object.typology.as_str())
                    .map(|(_, to)| (*to, object))
            })
            .map(|(mapped, object)| {
                let index = counts.entry(mapped).or_insert(0);
                let object_id = format!("{id_seed}-{mapped}-{index}");
                *index += 1;
                CadObject {
                    id: object_id,
                    label: object.label.clone(),
                    typology: mapped.into(),
                    visible: object.visible,
                    locked: object.locked,
                    origin: object.origin,
                    orientation: object.orientation,
                    scale: object.scale,
                    mesh_url: object.mesh_url.clone(),
                    extent: object.extent,
                    solid_handle: object.solid_handle.clone(),
                    primitives: object.primitives.clone(),
                }
            })
            .collect()
    }

    /// @emoji 🔄 Filters source objects to whitelisted typologies (premigration `applyTransformationFallback`).
    pub fn apply_typology_fallback(source_objects: &[CadObject], typologies: &[&str], id_seed: &str) -> Vec<CadObject> {
        source_objects
            .iter()
            .enumerate()
            .filter(|(_, object)| typologies.contains(&object.typology.as_str()))
            .map(|(index, object)| CadObject {
                id: format!("{id_seed}-{index}"),
                label: object.label.clone(),
                typology: object.typology.clone(),
                visible: object.visible,
                locked: object.locked,
                origin: object.origin,
                orientation: object.orientation,
                scale: object.scale,
                mesh_url: object.mesh_url.clone(),
                extent: object.extent,
                solid_handle: object.solid_handle.clone(),
                primitives: object.primitives.clone(),
            })
            .collect()
    }

    pub fn energy_typologies() -> &'static [&'static str] {
        ENERGY_TYPOLOGIES
    }
    //#endregion 🔖DeriveEngine

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn derive_from_geometry_classifies_box() {
            let mut kernel = BrepkitKernel::new();
            let solid = kernel_3d_engine::block_on(kernel.box_prim(2.0, 2.0, 3.0)).expect("box");
            let source = vec![CadObject {
                id: "object-box".into(),
                label: "Box".into(),
                typology: "spatial.shape.primitive.box".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([2.0, 2.0, 3.0]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: solid.0.clone(),
                    kind: "solid".into(),
                }],
            }];
            let derived = run_derive_from_geometry(&mut kernel, &source, "energy");
            assert!(derived.iter().any(|object| object.typology == "energy.energy.hull"));
            assert!(derived.iter().any(|object| object.typology == "energy.energy.roof" || object.typology == "energy.energy.baseplate"));
            assert!(derived.iter().any(|object| object.typology == "energy.energy.externalwall"));
            assert!(derived.iter().any(|object| object.typology == "energy.energy.windows"));
        }

        #[test]
        fn face_plane_group_key_is_stable() {
            let key = face_plane_group_key([0.0, 0.0, 1.0], [1.0, 2.0, 3.0]);
            assert!(key.starts_with("z:1:"));
        }
    }
}

use cad_document::{
    cad_all_objects, cad_find_object_pane, cad_pane_from_model_definition_id, cad_pane_objects,
    cad_pane_camera,
    CadCamera, CadGeometry, CadNode, CadObject,
    CadObjectPatch, CadOp, CadPaneId, CadPrimitiveSlot, CadReference, CadReferencePatch, CadScene,
    CAD_DOCUMENT_SCHEMA, CAD_PLAY_DOCUMENT_SCHEMA,
};
use geometry_import::{
    cad_object_from_mesh, cad_object_from_solid_handle, objects_from_fixture_model, parse_geometry, tessellate_geometry_handle,
};
use semio_framework_plugin::{PanelGroup,
    apply_world3d_sun_action, build_world_3d_scene, merge_world_selection_ids, mesh_from_kind,
    ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_mixed_vec3, ui_inspector_all_equal, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App,
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, AppLabelsOverlay, DocumentApp, DocumentView, MeshData, UtilityCategory, UtilityDefinition, UiFieldNode,
    UiInspectorFieldGroup, UiInputNode, UiNode, UiSelectItem, UiSelectNode, UiTreeItemAction, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput,
    WindowEngagementPossible, WindowEngagementStatus, SET_ACTIVE_UTILITY_ACTION_ID,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, MeshImporter,
};
use semio_framework_core::kernel::HostEffect;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use interaction::{
    apply_event, can_commit, commit_object, keyed_transitions, list_interactions_for_model_definition,
    parse_repl_line, preview_display_items, resolve_interaction_key, start_session, CadEngagementSession,
};
use transformation::{
    apply_from_building, apply_typology_fallback, run_derive_from_geometry, solid_for_object,
};

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
        source_model_definition_id: CAD_MODEL_DEFINITION_SHAPE,
        target_model_definition_id: CAD_MODEL_DEFINITION_ENERGY,
        mode: TransformationMode::DeriveFromGeometry,
    },
    CadTransformationSpec {
        id: "from_building",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::FromBuilding,
    },
    CadTransformationSpec {
        id: "classic",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::TypologyFallback,
    },
];
//#endregion 🔖Constants

//#region 🔖BrepMeshes
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, MeshTransfer};
use base64::Engine;
use semio_framework_core::mesh_from_indexed;
use ui_wgpu::SurfaceKind;
use std::sync::{Mutex, OnceLock};

static CAD_BREP_KERNEL: OnceLock<Mutex<Box<dyn BrepKernel + Send + Sync>>> = OnceLock::new();

/// @emoji 📦 Universal fallback extent for typologies with no authored geometry to measure.
const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

fn cad_brep_kernel() -> &'static Mutex<Box<dyn BrepKernel + Send + Sync>> {
    CAD_BREP_KERNEL.get_or_init(|| Mutex::new(Box::new(BrepkitKernel::new())))
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

/// @emoji 🗃️ Reads one pane's objects and geometry from the shared quad fixture.
fn cad_document_pane_bundle(source_json: &str, model_index: usize) -> (Vec<CadObject>, CadGeometry) {
    let Ok(root) = serde_json::from_str::<Value>(source_json) else {
        return (Vec::new(), CadGeometry::default());
    };
    let geometry = parse_geometry(root.pointer(&format!("/models/{model_index}/model/geometry")));
    let Some(objects_value) = root
        .pointer(&format!("/models/{model_index}/model/objects"))
        .and_then(|value| value.as_array())
    else {
        return (Vec::new(), geometry);
    };
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return (Vec::new(), geometry);
    };
    let objects = objects_from_fixture_model(&mut **kernel, objects_value, &geometry);
    (objects, geometry)
}

fn cad_document_pane_objects(source_json: &str, model_index: usize) -> Vec<CadObject> {
    cad_document_pane_bundle(source_json, model_index).0
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
    selected_primitive_id: Option<String>,
    #[serde(default)]
    selected_primitive_kind: Option<String>,
    #[serde(default)]
    engagement_pane: Option<String>,
    #[serde(default)]
    engagement_session: Option<CadEngagementSession>,
    #[serde(default)]
    last_finalized_interaction_id: Option<String>,
    #[serde(default)]
    sun: WorldSunConfig,
}

fn default_selection_method() -> String {
    "rectangle".into()
}

/// @emoji 🕹️ The default active transform utility when the host has not yet set one — mirrors the
/// framework toolbar's first-declared utility (`move`), read from `ViewState::active_utility_id`.
const CAD_DEFAULT_UTILITY_ID: &str = "move";

impl Default for CadPlayRuntime {
    fn default() -> Self {
        Self {
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            engagement_pane: None,
            engagement_session: None,
            last_finalized_interaction_id: None,
            sun: WorldSunConfig::default(),
        }
    }
}

/// @emoji 🎛️ Ephemeral read/render view assembled per call from the store's materialized
/// `CadScene` projection and the app's `CadPlayRuntime` view-state. Replaces the old persisted play
/// envelope: its embedded history/undo stacks are now owned by the wrapping `VcsDocumentApp`'s
/// `DocumentVcsStore`, and its runtime view-state lives directly on the `CadApp` struct.
struct CadPlayView {
    document: CadScene,
    runtime: CadPlayRuntime,
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
    let default_cam = CadCamera {
        position: [12.0, -12.0, 8.0],
        target: [0.0, 0.0, 0.0],
        zoom: 1.0,
        fov: 50.0,
    };
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        camera: default_cam.clone(),
        camera_building: default_cam.clone(),
        camera_energy: default_cam.clone(),
        camera_structure_classic: default_cam.clone(),
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
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        shape_geometry: None,
        building_geometry: None,
        energy_geometry: None,
        structure_classic_geometry: None,
        references_by_model_definition_id: HashMap::new(),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

/// @emoji 🪟 Builds the quad play document: shape/building/energy/structure-classic panes each
/// sourced from their own model definition inside the shared fixture JSON.
fn forest_play_document(source_json: &str, id: &str) -> CadScene {
    let (shape_objects, shape_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_SHAPE);
    if shape_objects.is_empty() {
        return default_document();
    }
    let (building_objects, building_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_BUILDING);
    let (energy_objects, energy_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_ENERGY);
    let (structure_classic_objects, structure_classic_geometry) =
        cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_STRUCTURE_CLASSIC);
    let default_cam = CadCamera {
        position: [12.0, -12.0, 8.0],
        target: [5.4, 2.34, 1.5],
        zoom: 1.0,
        fov: 50.0,
    };
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: id.into(),
        camera: default_cam.clone(),
        camera_building: default_cam.clone(),
        camera_energy: default_cam.clone(),
        camera_structure_classic: default_cam.clone(),
        objects: shape_objects,
        nodes: vec![CadNode {
            id: "node-root".into(),
            label: "Concrete Forest Left".into(),
            kind: "group".into(),
        }],
        building_objects,
        energy_objects,
        structure_classic_objects,
        shape_geometry: Some(shape_geometry),
        building_geometry: Some(building_geometry),
        energy_geometry: Some(energy_geometry),
        structure_classic_geometry: Some(structure_classic_geometry),
        references_by_model_definition_id: forest_references_for_model_definitions(),
        active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
    }
}

/// @emoji 🌲 The Concrete Forest Left example projection — a bare `CadScene` (no runtime/history),
/// wrapped into a `DocumentVcsStore` by `VcsDocumentApp` when spawned.
fn forest_play_scene() -> CadScene {
    forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT)
}

fn next_cad_id(prefix: &str) -> String {
    let next = CAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn cad_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: CAD_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn camera_json(camera: &CadCamera) -> String {
    ui_wgpu::world3d_camera_json(camera.position, camera.target, camera.fov)
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

fn cad_pane_id_from_surface_id(surface_id: &str) -> CadPaneId {
    let suffix = surface_id.split('/').last().unwrap_or(surface_id);
    cad_pane_id_from_suffix(suffix)
}

fn cad_pane_suffix(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shape",
        CadPaneId::Building => "building",
        CadPaneId::Energy => "energy",
        CadPaneId::StructureClassic => "structure-classic",
    }
}

fn ensure_object_solid_handle(kernel: &mut dyn BrepKernel, object: &mut CadObject) {
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

/// @emoji 🔁 Derives the target-pane objects for transformation `qid` and returns the operations
/// that both replace the target pane and refocus onto the target model definition — dispatched by
/// the caller through the store (no direct mutation).
fn apply_transformation_ops(document: &CadScene, qid: &str) -> Vec<CadOp> {
    let Some((model_definition_id, transformation_id)) = qid.rsplit_once('.') else {
        return Vec::new();
    };
    let Some(spec) = CAD_TRANSFORMATION_SPECS.iter().find(|entry| {
        entry.source_model_definition_id == model_definition_id && entry.id == transformation_id
    }) else {
        return Vec::new();
    };
    let Some(source_pane) = cad_pane_from_model_definition_id(spec.source_model_definition_id) else {
        return Vec::new();
    };
    let Some(target_pane) = cad_pane_from_model_definition_id(spec.target_model_definition_id) else {
        return Vec::new();
    };
    let objects = {
        let source_objects: Vec<CadObject> = cad_pane_objects(document, source_pane)
            .iter()
            .cloned()
            .collect();
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return Vec::new();
        };
        let mut prepared = source_objects;
        for object in &mut prepared {
            ensure_object_solid_handle(&mut **kernel, object);
        }
        match spec.mode {
            TransformationMode::DeriveFromGeometry => {
                run_derive_from_geometry(&mut **kernel, &prepared, "derived-energy")
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
    vec![
        CadOp::SetPaneObjects {
            pane: target_pane,
            objects,
        },
        CadOp::SetActiveModelDefinition {
            model_definition_id: spec.target_model_definition_id.into(),
        },
    ]
}

/// @emoji 📤 A native-geometry export ready to be wrapped into a `HostEffect::DownloadMediaExport`.
struct CadSolidExport {
    filename: String,
    data: Value,
    mime_type: String,
    encoding: Option<String>,
}

fn collect_pane_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView, pane: CadPaneId) -> Vec<GeometryHandle> {
    cad_pane_objects(&envelope.document, pane)
        .iter()
        .filter_map(|object| {
            let mut next = object.clone();
            solid_for_object(kernel, &mut next)
        })
        .collect()
}

fn collect_modelspace_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView) -> Vec<GeometryHandle> {
    CadPaneId::all()
        .into_iter()
        .flat_map(|pane| collect_pane_solids(kernel, envelope, pane))
        .collect()
}

/// @emoji 📤 Encodes `solids` through the kernel's native OBJ/STL/STEP codec for `format`; STL is
/// base64-wrapped since it is a binary format, OBJ/STEP stay UTF-8 text.
fn export_solids_as(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], format: semio_framework_plugin::OsMediaFormat, stem: &str) -> Option<CadSolidExport> {
    use semio_framework_plugin::OsMediaFormat;
    let filename = format!("{stem}.{}", format.as_str());
    let mime_type = format.mime_type().to_string();
    match format {
        OsMediaFormat::Obj => {
            let text = kernel_3d_engine::block_on(kernel.export_obj(solids, 0.1)).ok()?;
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        OsMediaFormat::Stl => {
            let bytes = kernel_3d_engine::block_on(kernel.export_stl(solids, 0.1)).ok()?;
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            Some(CadSolidExport { filename, data: Value::String(encoded), mime_type, encoding: Some("base64".into()) })
        }
        OsMediaFormat::Step => {
            let text = kernel_3d_engine::block_on(kernel.export_step(solids)).ok()?;
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        _ => None,
    }
}

fn export_solid_for_pane(envelope: &CadPlayView, pane: CadPaneId, format: semio_framework_plugin::OsMediaFormat) -> Option<CadSolidExport> {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return None;
    };
    let solids = collect_pane_solids(&mut **kernel, envelope, pane);
    if solids.is_empty() {
        return None;
    }
    let stem = format!("cad-{}", pane.model_definition_id().replace('.', "-"));
    export_solids_as(&mut **kernel, &solids, format, &stem)
}

fn export_solid_modelspace(envelope: &CadPlayView, format: semio_framework_plugin::OsMediaFormat) -> Option<CadSolidExport> {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return None;
    };
    let solids = collect_modelspace_solids(&mut **kernel, envelope);
    if solids.is_empty() {
        return None;
    }
    export_solids_as(&mut **kernel, &solids, format, "cad.modelspace")
}

/// @emoji ⬇️ Converts a staged native-geometry export into a download host effect emitted directly
/// to the shell (no document mutation, no pending-export runtime slot).
fn cad_solid_export_effect(export: CadSolidExport) -> HostEffect {
    let data = match export.data {
        Value::String(text) => text,
        other => serde_json::to_string(&other).unwrap_or_default(),
    };
    HostEffect::DownloadMediaExport {
        filename: export.filename,
        mime_type: export.mime_type,
        data,
        encoding: export.encoding,
    }
}

/// @emoji ⬇️ Wraps a spatial-JSON export document into a download host effect.
fn cad_spatial_export_effect(value: Value, filename: &str) -> HostEffect {
    HostEffect::DownloadMediaExport {
        filename: filename.into(),
        mime_type: "application/json".into(),
        data: serde_json::to_string(&value).unwrap_or_default(),
        encoding: None,
    }
}

/// @emoji 📦 Decodes a `requestFileOpen` payload (a `data:` URL when `readAs: "dataUrl"` was
/// requested, otherwise a raw string) into bytes.
fn cad_file_bytes_from_payload(payload: &Value) -> Option<Vec<u8>> {
    let raw = payload.as_str()?;
    if raw.starts_with("data:") {
        let (_, encoded) = raw.split_once(',')?;
        base64::engine::general_purpose::STANDARD.decode(encoded).ok()
    } else {
        Some(raw.as_bytes().to_vec())
    }
}

/// @emoji 📦 Decodes a `requestFileOpen` payload into UTF-8 text; see `cad_file_bytes_from_payload`.
fn cad_file_text_from_payload(payload: &Value) -> Option<String> {
    String::from_utf8(cad_file_bytes_from_payload(payload)?).ok()
}

/// @emoji 🧊 Imports a STEP payload into the shared kernel and wraps the first solid it contains
/// (STEP files may hold more than one shape) as a new `CadObject`.
fn import_step_object(text: &str) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = kernel_3d_engine::block_on(kernel.import_step(text)).ok()?.into_iter().next()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-step"), "Imported STEP", "spatial.shape.imported", handle))
}

/// @emoji 🧊 Imports an OBJ payload into the shared kernel as a new `CadObject`.
fn import_obj_object(text: &str) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = kernel_3d_engine::block_on(kernel.import_obj(text, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-obj"), "Imported OBJ", "spatial.shape.imported", handle))
}

/// @emoji 🧊 Imports an STL payload into the shared kernel as a new `CadObject`.
fn import_stl_object(bytes: &[u8]) -> Option<CadObject> {
    let mut kernel = cad_brep_kernel().lock().ok()?;
    let handle = kernel_3d_engine::block_on(kernel.import_stl(bytes, 0.01)).ok()?;
    Some(cad_object_from_solid_handle(&mut **kernel, next_cad_id("object-stl"), "Imported STL", "spatial.shape.imported", handle))
}

/// @emoji 🧊 Imports a GLB payload by decoding it to a tessellated mesh (via the shared
/// `MeshImporter` codec) and re-importing that mesh into the kernel as a solid, matching the
/// DWG-derived import path (`cad_object_from_mesh`) since GLB carries no exact B-Rep to preserve.
fn import_glb_object(bytes: &[u8]) -> Option<CadObject> {
    let mesh = semio_framework_plugin::GlbImporter.import(bytes).ok()?;
    let mut kernel = cad_brep_kernel().lock().ok()?;
    Some(cad_object_from_mesh(&mut **kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", &mesh))
}

/// @emoji 🗂️ Routes a `requestFileOpen` payload to the matching native-geometry import by the
/// picked file's extension; returns `None` for anything else so the caller can fall back to the
/// spatial-JSON document path.
fn import_cad_object_by_extension(name: &str, payload: &Value) -> Option<CadObject> {
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

fn export_spatial_json(envelope: &CadPlayView, mode: &str) -> Value {
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

//#endregion 🔖PaneHelpers

fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
    object.mesh_url.as_ref().filter(|url| !url.is_empty()).cloned()
}

fn primary_primitive_kind(object: &CadObject) -> &str {
    object
        .primitives
        .first()
        .map(|primitive| primitive.kind.as_str())
        .unwrap_or("solid")
}

fn object_mesh_data(object: &CadObject) -> MeshData {
    if let Some(handle) = object.solid_handle.as_deref() {
        if let Ok(mut kernel) = cad_brep_kernel().lock() {
            if let Some(mesh) = tessellate_geometry_handle(&mut **kernel, handle, primary_primitive_kind(object)) {
                return mesh;
            }
        }
    }
    typology_brep_mesh(
        &object.typology,
        object.extent,
        object.solid_handle.as_deref(),
    )
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
                .unwrap_or_else(|| object.id.clone());
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
    let meshes: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let data = object_mesh_data(object);
            json!({ "id": object.id, "data": data })
        })
        .collect();
    if meshes.is_empty() {
        let data = mesh_from_kind(CAD_FALLBACK_MESH_KIND);
        return serde_json::to_string(&[json!({ "id": CAD_FALLBACK_MESH_KIND, "data": data })])
            .unwrap_or_else(|_| "[]".into());
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_selection_json(document: &CadScene, runtime: &CadPlayRuntime, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_object_ids,
        runtime.hovered_object_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("transformTool".into(), json!(active_utility));
        object.insert("gumballActive".into(), json!(gumball_active(runtime)));
        object.insert(
            "engagementSessionActive".into(),
            json!(runtime.engagement_session.is_some()),
        );
        object.insert("showEdges".into(), json!(true));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("granularity".into(), json!("mesh"));
        if let Some(reference_id) = runtime.selected_reference_id.as_deref() {
            object.insert("referenceSelectedId".into(), json!(reference_id));
        }
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

fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, surface_id: &str, active_utility: &str) -> UiNode {
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
            camera_json(cad_pane_camera(&envelope.document, pane)),
            world_meshes_json(objects),
            world_instances_json(objects, &envelope.runtime),
            world_selection_json(&envelope.document, &envelope.runtime, active_utility),
            None,
            None,
            None,
            world_references_json(&envelope.document, pane),
            None,
            None,
            preview,
            None,
            Some(world3d_chunking_json(256.0, 8000.0)),
            None,
            Some(world3d_environment_json(&envelope.runtime.sun)),
        ),
    )
}

/// @emoji 🧵 Tessellates a representative mesh for the OS mesh-exporter boundary — the document's
/// first object across panes, or the default box typology for an empty scene (no runtime selection
/// exists at this boundary).
fn export_mesh_from_scene(document: &CadScene) -> MeshData {
    let first = cad_all_objects(document).next();
    let typology = first
        .map(|(object, _)| object.typology.as_str())
        .unwrap_or("spatial.shape.primitive.box");
    let extent = first.and_then(|(object, _)| object.extent);
    let solid_handle = first.and_then(|(object, _)| object.solid_handle.as_deref());
    typology_brep_mesh(typology, extent, solid_handle)
}

//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the CAD app; one field per label makes every terminology×locale combination compile-checked.
struct CadLabels {
    // entity nouns — remapped under the "reuse" terminology
    object: &'static str,
    objects: &'static str,
    primitive: &'static str,
    // model-definition pane / document-tree section names
    pane_shape: &'static str,
    pane_building: &'static str,
    pane_energy: &'static str,
    pane_structure_classic: &'static str,
    references: &'static str,
    nodes: &'static str,
    // catalogue
    typologies: &'static str,
    typology_box: &'static str,
    typology_slab: &'static str,
    typology_column: &'static str,
    typology_beam: &'static str,
    typology_wall: &'static str,
    typology_external_wall: &'static str,
    // inspector group titles
    reference: &'static str,
    node: &'static str,
    // tree item actions
    hide: &'static str,
    show: &'static str,
    lock: &'static str,
    unlock: &'static str,
    duplicate: &'static str,
    delete: &'static str,
}

const CAD_LABELS_NATIVE_EN: CadLabels = CadLabels {
    object: "Object",
    objects: "Objects",
    primitive: "Primitive",
    pane_shape: "Shape",
    pane_building: "Building",
    pane_energy: "Energy",
    pane_structure_classic: "Structure Classic",
    references: "References",
    nodes: "Nodes",
    typologies: "Typologies",
    typology_box: "Box",
    typology_slab: "Slab",
    typology_column: "Column",
    typology_beam: "Beam",
    typology_wall: "Wall",
    typology_external_wall: "External Wall",
    reference: "Reference",
    node: "Node",
    hide: "Hide",
    show: "Show",
    lock: "Lock",
    unlock: "Unlock",
    duplicate: "Duplicate",
    delete: "Delete",
};

const CAD_LABELS_NATIVE_DE: CadLabels = CadLabels {
    object: "Objekt",
    objects: "Objekte",
    primitive: "Primitiv",
    pane_shape: "Form",
    pane_building: "Gebäude",
    pane_energy: "Energie",
    pane_structure_classic: "Struktur Klassisch",
    references: "Referenzen",
    nodes: "Knoten",
    typologies: "Typologien",
    typology_box: "Box",
    typology_slab: "Platte",
    typology_column: "Stütze",
    typology_beam: "Balken",
    typology_wall: "Wand",
    typology_external_wall: "Außenwand",
    reference: "Referenz",
    node: "Knoten",
    hide: "Ausblenden",
    show: "Anzeigen",
    lock: "Sperren",
    unlock: "Entsperren",
    duplicate: "Duplizieren",
    delete: "Löschen",
};

const CAD_LABELS_REUSE_EN: CadLabels = CadLabels {
    object: "Building component",
    objects: "Building components",
    primitive: "Component part",
    ..CAD_LABELS_NATIVE_EN
};

const CAD_LABELS_REUSE_DE: CadLabels = CadLabels {
    object: "Baukomponente",
    objects: "Baukomponenten",
    primitive: "Bauteil",
    ..CAD_LABELS_NATIVE_DE
};

/// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native.
fn cad_labels(view_state: &ViewState) -> &'static CadLabels {
    let terminology = view_state.terminology.as_deref().unwrap_or("native");
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    match (terminology, is_de) {
        ("reuse", true) => &CAD_LABELS_REUSE_DE,
        ("reuse", false) => &CAD_LABELS_REUSE_EN,
        (_, true) => &CAD_LABELS_NATIVE_DE,
        (_, false) => &CAD_LABELS_NATIVE_EN,
    }
}

/// 🗣️ Resolves a typology catalog entry's display label from its stable id; unknown ids fall back to the catalog's native English text.
fn typology_label(typology: &'static str, labels: &CadLabels) -> &'static str {
    match typology {
        "spatial.shape.primitive.box" => labels.typology_box,
        "building.building.slab" | "structure.structure.onewayreinforcedconcreteslab" => labels.typology_slab,
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" => labels.typology_column,
        "building.building.beam" => labels.typology_beam,
        "building.building.wall" => labels.typology_wall,
        "energy.energy.externalwall" => labels.typology_external_wall,
        _ => TYPOLOGY_CATALOG.iter().find(|entry| entry.typology == typology).map(|entry| entry.label).unwrap_or(typology),
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn object_tree_item(id_suffix: &str, object: &CadObject, labels: &CadLabels) -> UiTreeItemNode {
    let primitive_items: Vec<UiTreeItemNode> = object
        .primitives
        .iter()
        .map(|primitive| {
            let mut item = tree_item_with_action(
                format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id),
                format!("{}: {}", primitive.slot, primitive.primitive_id),
                Some("hexagon"),
                cad_action(
                    "setPrimitiveSelection",
                    Some(json!({
                        "objectId": object.id,
                        "primitiveId": primitive.primitive_id,
                        "kind": primitive.kind,
                    })),
                ),
            );
            item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
            item.unhover_action = Some(cad_action("worldHover", None));
            item
        })
        .collect();
    let mut item = tree_item_with_action(
        format!("cad-object:{id_suffix}:{}", object.id),
        object.label.clone(),
        Some("box"),
        cad_action("setSelection", Some(json!({ "objectIds": [object.id] }))),
    );
    item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
    item.unhover_action = Some(cad_action("worldHover", None));
    item.is_hidden = Some(!object.visible);
    item.draggable = Some(!object.locked);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if object.visible { "eye-off" } else { "eye" }.into(),
            label: Some(if object.visible { labels.hide } else { labels.show }.into()),
            action: cad_action(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "hidden", "value": object.visible })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: if object.locked { "unlock" } else { "lock" }.into(),
            label: Some(if object.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "locked", "value": !object.locked })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "copy".into(),
            label: Some(labels.duplicate.into()),
            action: cad_action("duplicateObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "trash-2".into(),
            label: Some(labels.delete.into()),
            action: cad_action("deleteObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
    ]);
    if !primitive_items.is_empty() {
        item.items = Some(primitive_items);
        item.default_open = Some(false);
    }
    item
}

fn reference_tree_item(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiTreeItemNode {
    let mut item = tree_item_with_action(
        format!("cad-reference:{model_definition_id}:{}", reference.id),
        reference.id.clone(),
        Some("image"),
        cad_action(
            "setReferenceSelection",
            Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
        ),
    );
    item.description = Some(reference.source_url.clone());
    item.hover_action = Some(cad_action(
        "referenceHover",
        Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
    ));
    item.unhover_action = Some(cad_action("referenceHover", None));
    item.is_hidden = Some(reference.hidden);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if reference.hidden { "eye" } else { "eye-off" }.into(),
            label: Some(if reference.hidden { labels.show } else { labels.hide }.into()),
            action: cad_action(
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
            label: Some(if reference.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action(
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

fn tree_item_with_action(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    action: ActionDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
        loading: None,
    }
}

fn pane_document_section(label: &str, id_suffix: &str, objects: &[CadObject], labels: &CadLabels) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: format!("cad-play-document.{id_suffix}"),
        label: Some(label.into()),
        default_open: Some(true),
        items: objects.iter().map(|object| object_tree_item(id_suffix, object, labels)).collect(),
        loading: None,
    }
}

fn references_section(model_definition_id: &str, references: &[CadReference], labels: &CadLabels) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: format!("cad-play-document.references.{model_definition_id}"),
        label: Some(labels.references.into()),
        default_open: Some(false),
        items: if references.is_empty() {
            vec![tree_item_with_action(
                format!("cad-play-document.references.{model_definition_id}.empty"),
                "(none)",
                None,
                cad_action("noop", None),
            )]
        } else {
            references
                .iter()
                .map(|reference| reference_tree_item(model_definition_id, reference, labels))
                .collect()
        },
        loading: None,
    }
}

fn document_tree_selected_ids(document: &CadScene, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    if let (Some(model_definition_id), Some(reference_id)) = (
        runtime.selected_reference_model_definition_id.as_deref(),
        runtime.selected_reference_id.as_deref(),
    ) {
        return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
    }
    if let (Some(object_id), Some(primitive_id)) = (
        runtime.selected_object_ids.first(),
        runtime.selected_primitive_id.as_deref(),
    ) {
        if let Some(pane) = cad_find_object_pane(document, object_id) {
            return Some(vec![format!(
                "cad-primitive:{}:{object_id}:{primitive_id}",
                cad_pane_suffix(pane)
            )]);
        }
    }
    let selected: Vec<String> = runtime
        .selected_object_ids
        .iter()
        .filter_map(|object_id| {
            cad_find_object_pane(document, object_id)
                .map(|pane| format!("cad-object:{}:{object_id}", cad_pane_suffix(pane)))
        })
        .collect();
    if selected.is_empty() {
        None
    } else {
        Some(selected)
    }
}

fn document_tree_highlighted_ids(document: &CadScene, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    let hovered = runtime.hovered_object_id.as_deref()?;
    if let Some(reference_id) = hovered.strip_prefix("reference:") {
        for pane in CadPaneId::all() {
            let model_definition_id = pane.model_definition_id();
            if document
                .references_by_model_definition_id
                .get(model_definition_id)
                .is_some_and(|rows| rows.iter().any(|row| row.id == reference_id))
            {
                return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
            }
        }
        return None;
    }
    cad_find_object_pane(document, hovered).map(|pane| {
        vec![format!("cad-object:{}:{hovered}", cad_pane_suffix(pane))]
    })
}

fn build_document_tree(envelope: &CadPlayView, labels: &CadLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = envelope
        .document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                format!("cad-node:{}", node.id),
                node.label.clone(),
                Some("git-branch"),
                cad_action("setNodeSelection", Some(json!({ "nodeIds": [node.id] }))),
            )
        })
        .collect();
    let mut sections = vec![
        pane_document_section(labels.pane_shape, "shape", &envelope.document.objects, labels),
        references_section(
            CAD_MODEL_DEFINITION_SHAPE,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_SHAPE)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
            labels,
        ),
        pane_document_section(labels.pane_building, "building", &envelope.document.building_objects, labels),
        references_section(
            CAD_MODEL_DEFINITION_BUILDING,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_BUILDING)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
            labels,
        ),
        pane_document_section(labels.pane_energy, "energy", &envelope.document.energy_objects, labels),
        references_section(
            CAD_MODEL_DEFINITION_ENERGY,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_ENERGY)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
            labels,
        ),
        pane_document_section(
            labels.pane_structure_classic,
            "structure-classic",
            &envelope.document.structure_classic_objects,
            labels,
        ),
        references_section(
            CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
            envelope
                .document
                .references_by_model_definition_id
                .get(CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC)
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]),
            labels,
        ),
        UiTreeSectionNode {
            id: "cad-play-document.nodes".into(),
            label: Some(labels.nodes.into()),
            default_open: Some(true),
            items: node_items,
            loading: None,
        },
    ];
    let _ = &mut sections;
    UiNode::Tree(UiTreeNode {
        sections,
        selected_ids: document_tree_selected_ids(&envelope.document, &envelope.runtime),
        highlighted_ids: document_tree_highlighted_ids(&envelope.document, &envelope.runtime),
        selection_change: None,
        drop_action: None,
        loading: None,
    })
}

fn build_catalogue_tree(labels: &CadLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = TYPOLOGY_CATALOG
        .iter()
        .map(|entry| {
            tree_item_with_action(
                format!("cad-play-catalogue.{}", entry.typology),
                typology_label(entry.typology, labels),
                Some(entry.icon),
                cad_action("addObject", Some(json!({ "typology": entry.typology }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "cad-play-catalogue.typologies".into(),
            label: Some(labels.typologies.into()),
            default_open: Some(true),
            items,
            loading: None,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        loading: None,
    })
}

fn build_properties_panel(envelope: &CadPlayView, labels: &CadLabels, active_utility: &str) -> UiNode {
    if let (Some(object_id), Some(primitive_id)) = (
        envelope.runtime.selected_object_ids.first(),
        envelope.runtime.selected_primitive_id.as_deref(),
    ) {
        if let Some((object, _)) = cad_all_objects(&envelope.document).find(|(object, _)| object.id == *object_id) {
            let kind = envelope
                .runtime
                .selected_primitive_kind
                .as_deref()
                .or_else(|| {
                    object
                        .primitives
                        .iter()
                        .find(|primitive| primitive.primitive_id == primitive_id)
                        .map(|primitive| primitive.kind.as_str())
                })
                .unwrap_or("primitive");
            return ui_inspector_groups_to_tree(&[primitive_inspector_group(
                object,
                labels,
                primitive_id,
                kind,
            )]);
        }
    }
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
            return ui_inspector_groups_to_tree(&[object_inspector_group(&selected, labels)]);
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
                labels,
            )]);
        }
    }
    if let Some(node_id) = envelope.runtime.selected_node_ids.first() {
        if let Some(node) = envelope.document.nodes.iter().find(|entry| &entry.id == node_id) {
            return ui_inspector_groups_to_tree(&[node_inspector_group(node, labels)]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.document.schema)),
        ui_text(format!("Utility: {active_utility}")),
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
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform {
                mixed.value.to_string()
            } else {
                String::new()
            },
            placeholder: if mixed.uniform { None } else { Some("—".into()) },
            commit: None,
            on_change: cad_action(
                "patchSelection",
                Some(json!({ "objectIds": object_ids, "field": field })),
            ),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
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
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: if mixed.uniform {
                format!("[{}, {}, {}]", value[0], value[1], value[2])
            } else {
                String::new()
            },
            placeholder: if mixed.uniform { None } else { Some("—".into()) },
            commit: None,
            on_change: cad_action(
                "patchSelection",
                Some(json!({ "objectIds": object_ids, "field": field })),
            ),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn object_inspector_group(objects: &[&CadObject], term_labels: &CadLabels) -> UiInspectorFieldGroup {
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
    let orientations: Vec<[f64; 4]> = objects
        .iter()
        .map(|object| object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))
        .collect();
    let label_mixed = ui_inspector_mixed_text(&labels);
    let typology_mixed = ui_inspector_mixed_text(&typologies);
    let hidden_mixed = ui_inspector_mixed_toggle(&hidden);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: if objects.len() == 1 {
            term_labels.object.into()
        } else {
            format!("{} {}", objects.len(), term_labels.objects)
        },
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: "Label".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: label_mixed.value.clone(),
                    placeholder: label_mixed.placeholder.clone(),
                    commit: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "label" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.typology".into(),
                label: "Typology".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "cad-play-inspector.object.typology.select".into(),
                    value: typology_mixed.value.clone(),
                    items: TYPOLOGY_CATALOG
                        .iter()
                        .map(|entry| UiSelectItem {
                            value: entry.typology.into(),
                            label: typology_label(entry.typology, term_labels).into(),
                        })
                        .collect(),
                    placeholder: typology_mixed.placeholder.clone(),
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "typology" })),
                    ),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.hidden".into(),
                label: "Hidden".into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.hidden.toggle".into(),
                    icon_id: "eye-off".into(),
                    pressed: hidden_mixed.pressed,
                    text: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "hidden" })),
                    ),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.locked".into(),
                label: "Locked".into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.pressed,
                    text: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "locked" })),
                    ),
                })),
                description: None,
                required: None,
                error: None,
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
            inspector_quat_field(
                "cad-play-inspector.object.orientation",
                "Rotation",
                &orientations,
                &object_ids,
            ),
        ],
    }
}

fn primitive_inspector_group(object: &CadObject, labels: &CadLabels, primitive_id: &str, kind: &str) -> UiInspectorFieldGroup {
    let slot = object
        .primitives
        .iter()
        .find(|primitive| primitive.primitive_id == primitive_id)
        .map(|primitive| primitive.slot.as_str())
        .unwrap_or("primitive");
    UiInspectorFieldGroup {
        id: "cad-play-inspector.primitive".into(),
        label: labels.primitive.into(),
        default_open: None,
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.primitive.object", labels.object, &object.label),
            ui_inspector_readonly_field("cad-play-inspector.primitive.slot", "Slot", slot),
            ui_inspector_readonly_field("cad-play-inspector.primitive.kind", "Kind", kind),
            ui_inspector_readonly_field("cad-play-inspector.primitive.id", "Id", primitive_id),
        ],
    }
}

fn inspector_quat_field(id: &str, label: &str, values: &[[f64; 4]], object_ids: &[String]) -> UiNode {
    let serialized: Vec<String> = values
        .iter()
        .map(|row| format!("[{}, {}, {}, {}]", row[0], row[1], row[2], row[3]))
        .collect();
    let uniform = ui_inspector_all_equal(&serialized);
    let value = values.first().copied().unwrap_or([0.0, 0.0, 0.0, 1.0]);
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: if uniform {
                format!("[{}, {}, {}, {}]", value[0], value[1], value[2], value[3])
            } else {
                String::new()
            },
            placeholder: if uniform { None } else { Some("—".into()) },
            commit: None,
            on_change: cad_action(
                "patchSelection",
                Some(json!({ "objectIds": object_ids, "field": "orientation" })),
            ),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn reference_inspector_group(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.reference".into(),
        label: labels.reference.into(),
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
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.reference.widthWorld.input".into(),
                    input_kind: "number".into(),
                    value: reference.width_world.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_action(
                        "patchCadPlayReference",
                        Some(json!({
                            "modelDefinitionId": model_definition_id,
                            "referenceId": reference.id,
                            "field": "widthWorld",
                        })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
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

fn node_inspector_group(node: &CadNode, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.node".into(),
        label: labels.node.into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.node.label".into(),
                label: "Label".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "cad-play-inspector.node.label.input".into(),
                    input_kind: "text".into(),
                    value: node.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_action(
                        "renameNode",
                        Some(json!({ "nodeId": node.id })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            ui_inspector_readonly_field("cad-play-inspector.node.kind", "Kind", &node.kind),
        ],
    }
}

fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId) -> WindowEngagement {
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
                    action: Some(cad_action(
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
                    id: entry.id.clone(),
                    label: entry.label.clone(),
                    detail: Some(entry.key.clone()),
                    action: Some(cad_action(
                        "engagementPossibleSelect",
                        Some(json!({ "pane": cad_pane_suffix(pane), "possibleId": entry.id.clone() })),
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
        session_active: Some(session_active),
        // 🧰 The move/rotate/scale transform switcher now lives in the framework toolbar (derived
        // from `UtilityDefinition`s + `ViewState::active_utility_id`); the engagement HUD no longer
        // duplicates it — utilities must have exactly one surface.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("engagement-input".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some("Action".into()),
            disabled: None,
            on_change: Some(cad_action(
                "engagementInput",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_submit: Some(cad_action(
                "engagementSubmit",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_repeat_last: Some(cad_action(
                "engagementRepeatLast",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_abort: Some(cad_action(
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
        "orientation" => value.and_then(parse_quat_value).map(|orientation| CadObjectPatch {
            orientation: Some(orientation),
            ..Default::default()
        }),
        _ => None,
    }
}

fn parse_quat_value(value: &Value) -> Option<[f64; 4]> {
    if let Some(array) = value.as_array() {
        if array.len() >= 4 {
            return Some([
                array[0].as_f64().unwrap_or(0.0),
                array[1].as_f64().unwrap_or(0.0),
                array[2].as_f64().unwrap_or(0.0),
                array[3].as_f64().unwrap_or(1.0),
            ]);
        }
    }
    if let Some(text) = value.as_str() {
        let trimmed = text.trim().trim_start_matches('[').trim_end_matches(']');
        let parts: Vec<f64> = trimmed
            .split(',')
            .filter_map(|part| part.trim().parse().ok())
            .collect();
        if parts.len() >= 4 {
            return Some([parts[0], parts[1], parts[2], parts[3]]);
        }
    }
    None
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

/// @emoji 🩹 Builds the `PatchObject` operations that apply `field=value` across `object_ids`.
fn patch_objects_ops(
    document: &CadScene,
    object_ids: &[String],
    field: &str,
    value: Option<&Value>,
) -> Vec<CadOp> {
    let patch = match object_patch_from_field(field, value) {
        Some(patch) => patch,
        None => return Vec::new(),
    };
    let mut operations = Vec::new();
    for object_id in object_ids {
        let Some(pane) = cad_find_object_pane(document, object_id) else {
            continue;
        };
        operations.push(CadOp::PatchObject {
            pane,
            object_id: object_id.clone(),
            patch: patch.clone(),
        });
    }
    operations
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
        mesh_url: None,
        extent,
        solid_handle: None,
        primitives: Vec::new(),
    };
    if let Ok(mut kernel) = cad_brep_kernel().lock() {
        ensure_object_solid_handle(&mut **kernel, &mut object);
    }
    let _ = pane;
    object
}

/// Commits `session` if it satisfies `can_commit`, returning the `AddObject` operation and clearing
/// the session runtime state. Returns the ops (empty when no commit happened) — used by both the
/// direct-event and keyed-transition REPL paths in `engagement_submit_ops` (a state reached via
/// either path can be commit-ready, e.g. box's explicit `confirm` step reachable via a keyed
/// transition).
fn try_commit_session_ops(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId, session: &CadEngagementSession) -> Vec<CadOp> {
    if !can_commit(session) {
        return Vec::new();
    }
    let label_count = cad_pane_objects(document, pane).len();
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return Vec::new();
    };
    let Some(object) = commit_object(&mut **kernel, session, label_count, |prefix| next_cad_id(prefix)) else {
        return Vec::new();
    };
    drop(kernel);
    let id = object.id.clone();
    let interaction_id = session.interaction_id.clone();
    runtime.selected_object_ids = vec![id];
    runtime.engagement_input.clear();
    runtime.last_finalized_interaction_id = Some(interaction_id);
    runtime.engagement_session = None;
    runtime.engagement_step = "Idle".into();
    vec![CadOp::AddObject { pane, object }]
}

/// @emoji ⌨️ Advances the engagement REPL for the current `engagement_input`, mutating runtime
/// session state and returning any commit operations produced.
fn engagement_submit_ops(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId) -> Vec<CadOp> {
    let input = runtime.engagement_input.trim().to_string();
    if input.is_empty() {
        runtime.engagement_step = "Idle".into();
        return Vec::new();
    }
    let model_definition_id = pane.model_definition_id();
    let current_state = runtime.engagement_session.as_ref().map(|session| session.state.clone());
    if let Some((event_kind, payload)) = parse_repl_line(&input, current_state.as_deref()) {
        // An active session's own events/keyed-transitions always take priority over starting an
        // unrelated interaction by key — otherwise a mid-flow keypress that happens to collide
        // with another interaction's top-level key (e.g. box's "d" for diagonal mode vs. length's
        // top-level key "d") would silently abandon the current session.
        if let Some(session) = runtime.engagement_session.as_mut() {
            if apply_event(session, &event_kind, payload.as_ref()) {
                runtime.engagement_step = session.state.clone();
                let session_snapshot = session.clone();
                return try_commit_session_ops(document, runtime, pane, &session_snapshot);
            }
            for transition in keyed_transitions(session) {
                if transition.key.eq_ignore_ascii_case(&input) || transition.event_kind.eq_ignore_ascii_case(&input) {
                    if apply_event(session, &transition.event_kind, None) {
                        runtime.engagement_step = session.state.clone();
                        runtime.engagement_input.clear();
                        let session_snapshot = session.clone();
                        return try_commit_session_ops(document, runtime, pane, &session_snapshot);
                    }
                }
            }
        } else if let Some(entry) = resolve_interaction_key(&event_kind, model_definition_id) {
            runtime.engagement_session = start_session(&entry.id, pane);
            if let Some(session) = runtime.engagement_session.as_mut() {
                let _ = apply_event(session, "start", None);
            }
            runtime.engagement_step = runtime
                .engagement_session
                .as_ref()
                .map(|session| session.state.clone())
                .unwrap_or_else(|| "Idle".into());
            runtime.engagement_input.clear();
            return Vec::new();
        }
    }
    runtime.engagement_step = format!("Unknown: {input}");
    Vec::new()
}

/// Starts a fresh engagement session for `interaction_id` in `pane` (used by
/// `engagementPossibleSelect`'s start-by-id path and `engagementRepeatLast`).
fn start_interaction_session(runtime: &mut CadPlayRuntime, pane: CadPaneId, interaction_id: &str) -> bool {
    let Some(entry) = interaction::interaction_by_id(interaction_id) else {
        return false;
    };
    runtime.engagement_session = start_session(&entry.id, pane);
    if let Some(session) = runtime.engagement_session.as_mut() {
        let _ = apply_event(session, "start", None);
    }
    runtime.engagement_step = runtime
        .engagement_session
        .as_ref()
        .map(|session| session.state.clone())
        .unwrap_or_else(|| "Idle".into());
    true
}

//#region 🔖CadApp
/// @emoji 📐 The CAD play app. Document content lives in the wrapping `VcsDocumentApp`'s
/// `DocumentVcsStore<CadScene, CadOp>`; only ephemeral view-state (selection, hover, engagement
/// session, transform utility, sun) lives here on `runtime`. History (undo/redo/checkpoint) is
/// intercepted and dispatched by the wrapper — no manual arms or keybindings.
#[derive(Default)]
struct CadApp {
    runtime: CadPlayRuntime,
}

impl DocumentApp for CadApp {
    type Projection = CadScene;
    type Op = CadOp;

    fn app_id(&self) -> &str {
        CAD_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        CAD_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> CadScene {
        default_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, CadScene>,
        _view_state: &ViewState,
    ) -> ActionEmit<CadOp> {
        let document = doc.projection;
        match action {
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let (scene, runtime) = if example_id.is_empty() || example_id == "empty" || example_id == "default" {
                    (default_document(), CadPlayRuntime::default())
                } else if example_id == CAD_EXAMPLE_FOREST_LEFT || example_id == "forest-left" {
                    (
                        forest_play_scene(),
                        CadPlayRuntime {
                            active_example_id: Some(CAD_EXAMPLE_FOREST_LEFT.into()),
                            ..CadPlayRuntime::default()
                        },
                    )
                } else {
                    return ActionEmit::default();
                };
                self.runtime = runtime;
                ActionEmit::ops(vec![CadOp::SetScene { scene: Box::new(scene) }])
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Switching the host-owned active utility (`ViewState::active_utility_id`) is a pure
                // View action: it never mutates the document. Clear any in-progress engagement
                // session / rubber-band scratch so a stale preview cannot leak across a utility switch.
                self.runtime.engagement_input.clear();
                self.runtime.engagement_session = None;
                self.runtime.engagement_step = "Idle".into();
                self.runtime.hovered_object_id = None;
                ActionEmit::default()
            }
            "setSelection" => {
                self.runtime.selected_object_ids = args
                    .and_then(|value| value.get("objectIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.runtime.selected_node_ids.clear();
                self.runtime.selected_primitive_id = None;
                self.runtime.selected_primitive_kind = None;
                self.runtime.selected_reference_model_definition_id = None;
                self.runtime.selected_reference_id = None;
                ActionEmit::default()
            }
            "setNodeSelection" => {
                self.runtime.selected_node_ids = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.runtime.selected_object_ids.clear();
                ActionEmit::default()
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value::<CadCamera>(camera.clone()) {
                        let pane = args
                            .and_then(|value| value.get("surfaceId"))
                            .and_then(|v| v.as_str())
                            .map(cad_pane_id_from_surface_id)
                            .unwrap_or(CadPaneId::Shape);
                        return ActionEmit {
                            ops: vec![CadOp::SetCamera { pane, camera: parsed }],
                            coalesce_key: Some(format!("camera:{}", pane.model_definition_id())),
                            ..Default::default()
                        };
                    }
                }
                ActionEmit::default()
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &self.runtime.selected_object_ids);
                if ids.is_empty() {
                    return ActionEmit::default();
                }
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                ActionEmit::amend(vec![CadOp::TranslateObjects { object_ids: ids, dx, dy, dz }], "gumball.translate")
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &self.runtime.selected_object_ids);
                if ids.is_empty() {
                    return ActionEmit::default();
                }
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                ActionEmit::amend(vec![CadOp::RotateObjects { object_ids: ids, ax, ay, az, angle }], "gumball.rotate")
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &self.runtime.selected_object_ids);
                if ids.is_empty() {
                    return ActionEmit::default();
                }
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                ActionEmit::amend(vec![CadOp::ScaleObjects { object_ids: ids, sx, sy, sz }], "gumball.scale")
            }
            "addObject" => {
                let typology = args
                    .and_then(|value| value.get("typology"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("spatial.shape.primitive.box");
                let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id)
                    .unwrap_or(CadPaneId::Shape);
                let object = make_object_for_typology(typology, cad_pane_objects(document, pane).len(), pane);
                self.runtime.selected_object_ids = vec![object.id.clone()];
                ActionEmit::ops(vec![CadOp::AddObject { pane, object }])
            }
            "patchObject" | "patchSelection" => {
                let object_ids: Vec<String> = if action == "patchSelection" {
                    args.and_then(|value| value.get("objectIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_else(|| self.runtime.selected_object_ids.clone())
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
                ActionEmit::ops(patch_objects_ops(document, &object_ids, field, value))
            }
            "deleteObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if let Some(pane) = cad_find_object_pane(document, object_id) {
                    self.runtime.selected_object_ids.retain(|id| id != object_id);
                    return ActionEmit::ops(vec![CadOp::RemoveObject { pane, object_id: object_id.into() }]);
                }
                ActionEmit::default()
            }
            "duplicateObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let duplicate_target = cad_all_objects(document)
                    .find(|(object, _)| object.id == object_id)
                    .map(|(object, pane)| (object.clone(), pane));
                if let Some((mut duplicate, pane)) = duplicate_target {
                    duplicate.id = next_cad_id("object");
                    duplicate.label = format!("{} copy", duplicate.label);
                    self.runtime.selected_object_ids = vec![duplicate.id.clone()];
                    return ActionEmit::ops(vec![CadOp::AddObject { pane, object: duplicate }]);
                }
                ActionEmit::default()
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("solid");
                let id = next_cad_id("node");
                let label = format!("Node {}", document.nodes.len() + 1);
                let node = CadNode { id: id.clone(), label, kind: kind.into() };
                self.runtime.selected_node_ids = vec![id];
                ActionEmit::ops(vec![CadOp::AddNode { node }])
            }
            "renameNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if node_id.is_empty() || label.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![CadOp::RenameNode { node_id: node_id.into(), label: label.into() }])
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.runtime.selected_object_ids =
                    merge_world_selection_ids(&self.runtime.selected_object_ids, &ids, merge);
                self.runtime.selected_node_ids.clear();
                self.runtime.selected_primitive_id = None;
                self.runtime.selected_primitive_kind = None;
                self.runtime.selected_reference_model_definition_id = None;
                self.runtime.selected_reference_id = None;
                ActionEmit::default()
            }
            "worldHover" => {
                self.runtime.hovered_object_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                ActionEmit::default()
            }
            "setHover" => {
                self.runtime.hovered_object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                ActionEmit::default()
            }
            "worldPick" => {
                let merge = args
                    .and_then(|value| value.get("merge"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("replace");
                if args
                    .and_then(|value| value.get("id"))
                    .map_or(true, |value| value.is_null())
                {
                    if merge == "replace" {
                        self.runtime.selected_object_ids.clear();
                        self.runtime.selected_primitive_id = None;
                        self.runtime.selected_primitive_kind = None;
                    }
                    return ActionEmit::default();
                }
                let index = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0) as usize;
                let pane = args
                    .and_then(|value| value.get("surfaceId"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_surface_id)
                    .or_else(|| {
                        args.and_then(|value| value.get("pane"))
                            .and_then(|value| value.as_str())
                            .map(cad_pane_id_from_suffix)
                    })
                    .unwrap_or(CadPaneId::Shape);
                if let Some(object) = cad_pane_objects(document, pane)
                    .iter()
                    .filter(|object| object.visible)
                    .nth(index)
                {
                    let id = object.id.clone();
                    self.runtime.selected_object_ids =
                        merge_world_selection_ids(&self.runtime.selected_object_ids, &[id], merge);
                    self.runtime.selected_node_ids.clear();
                    self.runtime.selected_primitive_id = None;
                    self.runtime.selected_primitive_kind = None;
                    self.runtime.selected_reference_model_definition_id = None;
                    self.runtime.selected_reference_id = None;
                }
                ActionEmit::default()
            }
            "setSelectionMethod" => {
                self.runtime.selection_method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle")
                    .into();
                ActionEmit::default()
            }
            "focusModelDefinition" => {
                if let Some(model_definition_id) = args
                    .and_then(|value| value.get("modelDefinitionId"))
                    .and_then(|value| value.as_str())
                {
                    return ActionEmit::ops(vec![CadOp::SetActiveModelDefinition {
                        model_definition_id: model_definition_id.into(),
                    }]);
                }
                ActionEmit::default()
            }
            "applyTransformation" => {
                let qid = args
                    .and_then(|value| value.get("qid"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                ActionEmit::ops(apply_transformation_ops(document, qid))
            }
            "saveSelected" => {
                let view = CadPlayView { document: document.clone(), runtime: self.runtime.clone() };
                ActionEmit::effect(cad_spatial_export_effect(
                    export_spatial_json(&view, "selected"),
                    "cad.selected.spatial.json",
                ))
            }
            "saveInPlay" => {
                let view = CadPlayView { document: document.clone(), runtime: self.runtime.clone() };
                let effect = match export_solid_modelspace(&view, semio_framework_plugin::OsMediaFormat::Step) {
                    Some(export) => cad_solid_export_effect(export),
                    None => cad_spatial_export_effect(export_spatial_json(&view, "modelspace"), "cad.modelspace.spatial.json"),
                };
                ActionEmit::effect(effect)
            }
            "saveCurrent" => {
                let format = match args.and_then(|value| value.get("format")).and_then(|value| value.as_str()) {
                    Some("obj") => semio_framework_plugin::OsMediaFormat::Obj,
                    Some("stl") => semio_framework_plugin::OsMediaFormat::Stl,
                    _ => semio_framework_plugin::OsMediaFormat::Step,
                };
                let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id)
                    .unwrap_or(CadPaneId::Shape);
                let view = CadPlayView { document: document.clone(), runtime: self.runtime.clone() };
                let effect = match export_solid_for_pane(&view, pane, format) {
                    Some(export) => cad_solid_export_effect(export),
                    None => cad_spatial_export_effect(export_spatial_json(&view, "current"), "cad.current.spatial.json"),
                };
                ActionEmit::effect(effect)
            }
            "loadRawRequest" => ActionEmit::effect(HostEffect::RequestFileOpen {
                accept: ".json,.spatial.json,.stp,.step,.obj,.stl,.glb".into(),
                read_as: Some("dataUrl".into()),
                import_action: "importCadFile".into(),
            }),
            "importCadFile" => {
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                let payload = args
                    .and_then(|value| value.get("payload").or_else(|| value.get("modelSpace")))
                    .cloned()
                    .or_else(|| args.cloned());
                let Some(payload) = payload else { return ActionEmit::default() };
                if let Some(object) = import_cad_object_by_extension(&name, &payload) {
                    self.runtime.selected_object_ids = vec![object.id.clone()];
                    return ActionEmit::ops(vec![CadOp::AddObject { pane: CadPaneId::Shape, object }]);
                }
                let payload = match payload {
                    Value::String(text) => serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text)),
                    other => other,
                };
                let unwrapped = unwrap_spatial_load_payload(&payload).unwrap_or(payload);
                let scene = scene_from_spatial_payload(&unwrapped)
                    .or_else(|| serde_json::from_value::<CadScene>(unwrapped).ok());
                if let Some(scene) = scene {
                    self.runtime.selected_object_ids.clear();
                    self.runtime.engagement_session = None;
                    return ActionEmit::ops(vec![CadOp::SetScene { scene: Box::new(scene) }]);
                }
                ActionEmit::default()
            }
            "setReferenceSelection" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .or_else(|| {
                        args.and_then(|value| value.get("modelDefinitionId"))
                            .and_then(|value| value.as_str())
                            .and_then(cad_pane_from_model_definition_id)
                    })
                    .unwrap_or(CadPaneId::Shape);
                self.runtime.selected_reference_model_definition_id =
                    Some(pane.model_definition_id().into());
                self.runtime.selected_reference_id = args
                    .and_then(|value| value.get("referenceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                self.runtime.selected_object_ids.clear();
                self.runtime.selected_node_ids.clear();
                self.runtime.selected_primitive_id = None;
                self.runtime.selected_primitive_kind = None;
                ActionEmit::default()
            }
            "referenceHover" => {
                self.runtime.hovered_object_id = args
                    .and_then(|value| value.get("referenceId"))
                    .and_then(|value| value.as_str())
                    .map(|id| format!("reference:{id}"));
                ActionEmit::default()
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
                match patch {
                    Some(patch) => ActionEmit::ops(vec![CadOp::PatchReference {
                        model_definition_id: model_definition_id.into(),
                        reference_id: reference_id.into(),
                        patch,
                    }]),
                    None => ActionEmit::default(),
                }
            }
            "engagementInput" => {
                self.runtime.engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
                self.runtime.engagement_pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                ActionEmit::default()
            }
            "engagementSubmit" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .unwrap_or(CadPaneId::Shape);
                ActionEmit::ops(engagement_submit_ops(document, &mut self.runtime, pane))
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
                    if let Some(session) = self.runtime.engagement_session.as_mut() {
                        if apply_event(session, possible_id, None) {
                            self.runtime.engagement_step = session.state.clone();
                        }
                    } else if !start_interaction_session(&mut self.runtime, pane, possible_id) {
                        self.runtime.engagement_input = possible_id.into();
                    }
                }
                ActionEmit::default()
            }
            "engagementRepeatLast" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .unwrap_or(CadPaneId::Shape);
                if self.runtime.engagement_session.is_none() {
                    if let Some(interaction_id) = self.runtime.last_finalized_interaction_id.clone() {
                        start_interaction_session(&mut self.runtime, pane, &interaction_id);
                        return ActionEmit::default();
                    }
                }
                self.runtime.engagement_step = "Idle".into();
                ActionEmit::default()
            }
            "engagementAbort" => {
                self.runtime.engagement_input.clear();
                self.runtime.engagement_session = None;
                self.runtime.engagement_step = "Idle".into();
                ActionEmit::default()
            }
            "worldPointerDown" | "engagementPointerDown" => {
                let pane = args
                    .and_then(|value| value.get("pane"))
                    .and_then(|value| value.as_str())
                    .map(cad_pane_id_from_suffix)
                    .or_else(|| {
                        args.and_then(|value| value.get("surfaceId"))
                            .and_then(|value| value.as_str())
                            .and_then(|surface_id| surface_id.rsplit('/').next())
                            .map(cad_pane_id_from_suffix)
                    })
                    .unwrap_or(CadPaneId::Shape);
                let point = args.and_then(|value| value.get("position"));
                if let Some(session) = self.runtime.engagement_session.as_mut() {
                    if apply_event(session, "pointer.down", point) {
                        self.runtime.engagement_step = session.state.clone();
                        let snapshot = session.clone();
                        return ActionEmit::ops(try_commit_session_ops(document, &mut self.runtime, pane, &snapshot));
                    }
                }
                ActionEmit::default()
            }
            "worldPointerMove" => {
                // Live rubber-band preview during an active engagement session: applies
                // `pointer.move` (updating the session's cursor/preview context) without ever
                // committing an object or touching VCS history.
                let point = args.and_then(|value| value.get("position"));
                if let Some(session) = self.runtime.engagement_session.as_mut() {
                    apply_event(session, "pointer.move", point);
                }
                ActionEmit::default()
            }
            "setPrimitiveSelection" => {
                if let Some(object_id) = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()) {
                    self.runtime.selected_object_ids = vec![object_id.into()];
                    self.runtime.selected_node_ids.clear();
                    self.runtime.selected_primitive_id = args
                        .and_then(|value| value.get("primitiveId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    self.runtime.selected_primitive_kind = args
                        .and_then(|value| value.get("kind"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    self.runtime.selected_reference_model_definition_id = None;
                    self.runtime.selected_reference_id = None;
                }
                ActionEmit::default()
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                apply_world3d_sun_action(&mut self.runtime.sun, action, args);
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, CadScene>, view_state: &ViewState) -> UiNode {
        let view = CadPlayView { document: doc.projection.clone(), runtime: self.runtime.clone() };
        let labels = cad_labels(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(CAD_DEFAULT_UTILITY_ID);
        match body_key {
            CAD_PLAY_BODY_SHAPE => build_world_scene_for_pane(&view, CadPaneId::Shape, CAD_PLAY_SURFACE_SHAPE, active_utility),
            CAD_PLAY_BODY_BUILDING => {
                build_world_scene_for_pane(&view, CadPaneId::Building, CAD_PLAY_SURFACE_BUILDING, active_utility)
            }
            CAD_PLAY_BODY_ENERGY => build_world_scene_for_pane(&view, CadPaneId::Energy, CAD_PLAY_SURFACE_ENERGY, active_utility),
            CAD_PLAY_BODY_STRUCTURE_CLASSIC => build_world_scene_for_pane(
                &view,
                CadPaneId::StructureClassic,
                CAD_PLAY_SURFACE_STRUCTURE_CLASSIC,
                active_utility,
            ),
            CAD_PLAY_BODY_DOCUMENT => build_document_tree(&view, labels),
            CAD_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            CAD_PLAY_BODY_PROPERTIES => build_properties_panel(&view, labels, active_utility),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, CadScene>, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView { document: doc.projection.clone(), runtime: self.runtime.clone() };
        HashMap::from([
            (
                CAD_PLAY_WINDOW_SHAPE.to_string(),
                cad_window_engagement(&view, CadPaneId::Shape),
            ),
            (
                CAD_PLAY_WINDOW_BUILDING.to_string(),
                cad_window_engagement(&view, CadPaneId::Building),
            ),
            (
                CAD_PLAY_WINDOW_ENERGY.to_string(),
                cad_window_engagement(&view, CadPaneId::Energy),
            ),
            (
                CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(),
                cad_window_engagement(&view, CadPaneId::StructureClassic),
            ),
        ])
    }

    fn window_measures(&self, _doc: &DocumentView<'_, CadScene>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let measures = vec![world3d_sun_measures("cad", &self.runtime.sun, cad_action)];
        HashMap::from([
            (CAD_PLAY_WINDOW_SHAPE.to_string(), measures.clone()),
            (CAD_PLAY_WINDOW_BUILDING.to_string(), measures.clone()),
            (CAD_PLAY_WINDOW_ENERGY.to_string(), measures.clone()),
            (CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(), measures),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = cad_labels(view_state);
        AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (CAD_PLAY_WINDOW_SHAPE.to_string(), labels.pane_shape.to_string()),
                (CAD_PLAY_WINDOW_BUILDING.to_string(), labels.pane_building.to_string()),
                (CAD_PLAY_WINDOW_ENERGY.to_string(), labels.pane_energy.to_string()),
                (CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(), labels.pane_structure_classic.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
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

/// @emoji 🧰 A cad transform-gumball utility: an exclusive member of the `transform` group rendered in
/// the framework toolbar (`UtilityCategory::Tools`). Switching it is a pure `setActiveUtility` View action
/// (`ViewState::active_utility_id`) — it gates the action panel while active (the default), since a
/// transform mode is a content-editing mode, not a passive viewing aid.
fn cad_transform_utility(id: &str, label: &str, icon: &str) -> UtilityDefinition {
    UtilityDefinition {
        group: Some("transform".into()),
        category: Some(UtilityCategory::Tools),
        ..UtilityDefinition::new(id, label, icon)
    }
}

/// @emoji 🧰 The transform-utility refs scoping the gumball to every world-3d pane uniformly.
fn cad_transform_utility_refs() -> Vec<semio_framework_plugin::UtilityRef> {
    vec!["move".into(), "rotate".into(), "scale".into()]
}

fn create_cad_app() -> App {
    App::from_builder(
        App::builder(CAD_PLAY_APP_ID, "CAD").document(["semio", "cad"])
            .icon_id("box")
            .terminology("reuse")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(CAD_PLAY_WINDOW_SHAPE, "Shape", CAD_PLAY_BODY_SHAPE, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_BUILDING, "Building", CAD_PLAY_BODY_BUILDING, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_ENERGY, "Energy", CAD_PLAY_BODY_ENERGY, SurfaceKind::World3d)
            .window_kind(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", CAD_PLAY_BODY_STRUCTURE_CLASSIC, SurfaceKind::World3d)
            .default_layout(cad_quad_layout())
            .operation("addObject", "Add Object")
            .operation("patchObject", "Patch Object")
            .operation("patchSelection", "Patch Selection")
            .operation("deleteObject", "Delete Object")
            .operation("duplicateObject", "Duplicate Object")
            .operation("addNode", "Add Node")
            .operation("renameNode", "Rename Node")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("applyTransformation", "Apply Transformation")
            .operation("importCadFile", "Import CAD File")
            .operation("patchCadPlayReference", "Patch Reference")
            .operation("engagementSubmit", "Engagement Submit")
            .operation("setCamera", "Set Camera")
            .operation("focusModelDefinition", "Focus Model Definition")
            .operation("setActiveExample", "Set Active Example")
            .view_action("setSelection", "Set Selection")
            .view_action("setNodeSelection", "Set Node Selection")
            .view_action("worldSelect", "World Select")
            .view_action("worldHover", "World Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldPick", "World Pick")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setReferenceSelection", "Set Reference Selection")
            .view_action("referenceHover", "Reference Hover")
            .view_action("engagementInput", "Engagement Input")
            .view_action("engagementPossibleSelect", "Engagement Possible Select")
            .view_action("engagementRepeatLast", "Engagement Repeat Last")
            .view_action("engagementAbort", "Engagement Abort")
            .view_action("worldPointerDown", "World Pointer Down")
            .view_action("worldPointerMove", "World Pointer Move")
            .view_action("engagementPointerDown", "Engagement Pointer Down")
            .view_action("setPrimitiveSelection", "Set Primitive Selection")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .shell_action("saveSelected", "Save Selected")
            .shell_action("saveInPlay", "Save In Play")
            .shell_action("saveCurrent", "Save Current")
            .shell_action("loadRawRequest", "Load Raw Request")
            .action_args("saveCurrent", vec![ActionArgDef::select("format", "Format", vec![
                ActionArgOption::new("step", "STEP"),
                ActionArgOption::new("obj", "OBJ"),
                ActionArgOption::new("stl", "STL"),
            ]).default_value("step")])
            .action_args("focusModelDefinition", vec![ActionArgDef::select("modelDefinitionId", "Model Definition", vec![
                ActionArgOption::new(CAD_MODEL_DEFINITION_SHAPE, "Shape"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_BUILDING, "Building"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_ENERGY, "Energy"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, "Structure Classic"),
            ]).required()])
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", "Example", vec![
                ActionArgOption::new("default", "Default"),
                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, "Hexagonal Cut Concrete Forest Left"),
            ]).default_value("default")])
            .utility(cad_transform_utility("move", "Move", "move"))
            .utility(cad_transform_utility("rotate", "Rotate", "rotate-cw"))
            .utility(cad_transform_utility("scale", "Scale", "maximize-2"))
            .window_kind_utilities(CAD_PLAY_WINDOW_SHAPE, cad_transform_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_BUILDING, cad_transform_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_ENERGY, cad_transform_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, cad_transform_utility_refs())
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
    .example("default", "Default", &serde_json::to_string(&default_document()).unwrap())
    .example(
        CAD_EXAMPLE_FOREST_LEFT,
        "Hexagonal Cut Concrete Forest Left",
        &serde_json::to_string(&forest_play_scene()).unwrap(),
    )
    .program("cad", "CAD", "model")
}

fn cad_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
    let scene: CadScene = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_scene(&scene))
}

fn cad_document_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<serde_json::Value, String> {
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

/// @emoji 🧵 Bridges a `MeshImporter`-decoded mesh (currently only GLB) back into a bare `CadScene`
/// document, reusing the same OBJ-text-roundtrip kernel import as the DWG/STL/`importCadFile` paths.
fn cad_document_from_mesh(mesh: &semio_framework_plugin::MeshData) -> Result<serde_json::Value, String> {
    let mut scene = default_document();
    let mut kernel = cad_brep_kernel().lock().map_err(|_| "cad brep kernel lock poisoned".to_string())?;
    let object = cad_object_from_mesh(&mut **kernel, next_cad_id("object-glb"), "Imported GLB", "spatial.shape.imported", mesh);
    scene.objects = vec![object];
    serde_json::to_value(&scene).map_err(|err| err.to_string())
}

fn register_cad_exports() {
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

semio_framework_plugin::semio_plugin! {
    id: "cad", label: "CAD", version: "0.1.0",
    setup: register_cad_exports,
    apps: [ create_cad_app => CadApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cad_document::empty_cad_projection;
    use semio_framework_plugin::{ActionMeta, HistoryView, PluginApp, VcsDocumentApp};
    use vcs::{Backbone, BackboneMessage, MemoryBackbone, Operation, OperationDiff};

    //#region 🔖Harness
    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<CadApp> {
        VcsDocumentApp::new(CadApp::default())
    }

    fn empty_history() -> HistoryView {
        HistoryView {
            columns: Vec::new(),
            can_undo: false,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
        }
    }

    /// 🕹️ Drives one action against a bare `CadApp` (unwrapped) so tests can inspect ephemeral
    /// runtime view-state and the emitted operations directly.
    fn drive(app: &mut CadApp, scene: &CadScene, action: &str, args: Option<Value>) -> ActionEmit<CadOp> {
        let history = empty_history();
        let doc = DocumentView { projection: scene, history: &history };
        app.handle_action(action, args.as_ref(), &doc, &ViewState::default())
    }

    /// 🧮 Folds a list of `CadOp`s onto a scene via the core `Operation`/`OperationDiff` impls —
    /// mirrors what the wrapping `VcsDocumentApp` store does when it dispatches the emitted ops.
    fn apply_ops(scene: &CadScene, ops: &[CadOp]) -> CadScene {
        let mut next = scene.clone();
        for op in ops {
            next = op.diff(&next).apply(&next);
        }
        next
    }

    fn view(scene: CadScene, runtime: CadPlayRuntime) -> CadPlayView {
        CadPlayView { document: scene, runtime }
    }
    //#endregion 🔖Harness

    //#region 🔖Fixtures
    #[test]
    fn forest_example_uses_per_object_brep_meshes() {
        let scene = forest_play_scene();
        let runtime = CadPlayRuntime::default();
        let json = world_instances_json(&scene.building_objects, &runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        let meshes = world_meshes_json(&scene.building_objects);
        assert!(meshes.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        assert!(!meshes.contains("hexagonal-cut-concrete-forest-left.glb"));
        assert!(scene.building_objects.len() > 5);
        assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
    }

    #[test]
    fn cad_document_from_dwg_creates_one_object_per_layer_with_geometry() {
        let mut drawing = semio_framework_core::DwgDrawing::default();
        let outline = drawing.ensure_layer("outline");
        let empty_layer = drawing.ensure_layer("empty");
        let _ = empty_layer;
        drawing.entities.push(semio_framework_core::DwgEntity {
            layer: outline,
            color: semio_framework_core::DwgColor::ByLayer,
            geometry: semio_framework_core::DwgGeometry::PolyfaceMesh {
                vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                faces: vec![[1, 2, 3, 4]],
            },
        });
        let value = cad_document_from_dwg(&drawing).expect("cad document from dwg");
        let scene: CadScene = serde_json::from_value(value).expect("valid cad scene");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].label, "outline");
    }

    #[test]
    fn cad_document_from_empty_dwg_falls_back_to_default_document() {
        let drawing = semio_framework_core::DwgDrawing::default();
        let value = cad_document_from_dwg(&drawing).expect("cad document from empty dwg");
        let scene: CadScene = serde_json::from_value(value).expect("valid cad scene");
        assert!(!scene.objects.is_empty());
    }

    #[test]
    fn quad_panes_each_populate_distinct_objects() {
        let scene = forest_play_scene();
        assert!(!scene.objects.is_empty(), "shape pane");
        assert!(!scene.building_objects.is_empty(), "building pane");
        assert!(!scene.energy_objects.is_empty(), "energy pane");
        assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
    }

    #[test]
    fn cad_document_schema_matches_domain() {
        let scene = empty_cad_projection();
        assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn default_example_and_forest_scene_parse_as_projections() {
        let default_json = serde_json::to_string(&default_document()).unwrap();
        let default_scene: CadScene = serde_json::from_str(&default_json).unwrap();
        assert!(!default_scene.objects.is_empty());
        let forest_json = serde_json::to_string(&forest_play_scene()).unwrap();
        let forest_scene: CadScene = serde_json::from_str(&forest_json).unwrap();
        assert!(!forest_scene.building_objects.is_empty());
    }
    //#endregion 🔖Fixtures

    //#region 🔖Render
    #[test]
    fn renders_world_scene_for_each_pane() {
        let app = CadApp::default();
        let scene = forest_play_scene();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        for body_key in [
            CAD_PLAY_BODY_SHAPE,
            CAD_PLAY_BODY_BUILDING,
            CAD_PLAY_BODY_ENERGY,
            CAD_PLAY_BODY_STRUCTURE_CLASSIC,
        ] {
            let node = app.render(body_key, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
        }
    }

    #[test]
    fn document_lists_objects_and_nodes() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-object:"));
        assert!(json.contains("cad-node:"));
    }

    #[test]
    fn document_tree_includes_primitive_children() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-primitive:"));
        assert!(json.contains("hoverAction"));
    }

    #[test]
    fn app_definition_declares_transform_utilities_and_no_actions_variant() {
        let definition = create_cad_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert!(utility_ids.contains(&"move"));
        assert!(utility_ids.contains(&"rotate"));
        assert!(utility_ids.contains(&"scale"));
        // 🧰 The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
        // cad must NOT also declare it as an Operation.
        let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
        assert_eq!(set_active_utility.kind, semio_framework_plugin::ActionKind::View);
        // 🚦 Transform utilities gate the action panel while active (the default) — cad declares no
        // passive `allows_actions_while_active` view utilities.
        assert!(definition.utilities.iter().all(|utility| !utility.allows_actions_while_active));
        // 🧭 Every world-3d pane scopes the three transform utilities.
        for window in &definition.window_kinds {
            let refs: Vec<&str> = window.utilities.iter().map(|utility_ref| utility_ref.as_str()).collect();
            assert_eq!(refs, vec!["move", "rotate", "scale"], "window {} utilities", window.id);
        }
    }

    #[test]
    fn engagement_input_and_possible_engagements_present() {
        let mut app = new_app();
        let engagements = app.window_engagements(&ViewState::default());
        let shape = engagements.get(CAD_PLAY_WINDOW_SHAPE).expect("shape engagement");
        assert!(shape.input.is_some());
        assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    }

    #[test]
    fn window_engagements_registered_for_all_four_panes() {
        let mut app = new_app();
        let engagements = app.window_engagements(&ViewState::default());
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
    fn forest_example_includes_reference_overlay() {
        let scene = forest_play_scene();
        let references = world_references_json(&scene, CadPaneId::Shape).expect("references");
        assert!(references.contains("ref-concrete-forest"));
    }

    #[test]
    fn typology_extent_derives_from_authored_geometry() {
        let scene = forest_play_scene();
        let column = scene
            .building_objects
            .iter()
            .find(|object| object.typology == "building.building.column")
            .expect("column object");
        let extent = column.extent.expect("column extent derived from geometry");
        assert!(extent[2] > 0.05, "authored column height should be measurable");
        assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
    }
    //#endregion 🔖Render

    //#region 🔖ViewState
    #[test]
    fn gumball_fields_present_when_selection_active() {
        let mut app = CadApp::default();
        let scene = default_document();
        drive(&mut app, &scene, "setSelection", Some(json!({ "objectIds": ["object-box-1"] })));
        let selection = world_selection_json(&scene, &app.runtime, "rotate");
        assert!(selection.contains("\"transformTool\":\"rotate\""), "gumball utility sourced from ViewState::active_utility_id");
        assert!(selection.contains("\"gumballActive\":true"));
        assert!(selection.contains("\"gumballTarget\""));
    }

    #[test]
    fn gumball_inactive_without_selection() {
        let selection = world_selection_json(&default_document(), &CadPlayRuntime::default(), CAD_DEFAULT_UTILITY_ID);
        assert!(selection.contains("\"gumballActive\":false"));
        assert!(!selection.contains("\"gumballTarget\""));
    }

    #[test]
    fn active_utility_flows_from_view_state_into_scene() {
        let app = CadApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let view_state = ViewState { active_utility_id: Some("scale".into()), ..ViewState::default() };
        let node = app.render(CAD_PLAY_BODY_SHAPE, &doc, &view_state);
        let json = serde_json::to_string(&node).unwrap();
        // The world selection blob is embedded as an escaped JSON string inside the scene node.
        assert!(json.contains(r#"transformTool\":\"scale"#), "render sources gumball utility from ViewState::active_utility_id");
    }

    #[test]
    fn engagement_hud_no_longer_carries_utility_switcher_options() {
        let mut app = new_app();
        let engagements = app.window_engagements(&ViewState::default());
        for engagement in engagements.values() {
            assert!(engagement.options.is_none(), "utility switching now lives in the framework toolbar, not the engagement HUD");
        }
    }

    #[test]
    fn switching_utility_emits_no_operations_and_no_history_entry() {
        // 🧰 The key regression guard: switching the host-owned active utility must be a pure View
        // action — zero operations, no projection mutation, and (proven below) no intervening
        // history entry. If the switch recorded an edit, the single undo would revert the switch
        // instead of the preceding addObject.
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.handle_action("addObject", Some(&json!({ "typology": "spatial.shape.primitive.box" })), &ViewState::default(), &meta("local"))
            .expect("add object");
        let projection_after_add = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        let view_state = ViewState { active_utility_id: Some("rotate".into()), ..ViewState::default() };
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "rotate" })), &view_state, &meta("local"))
            .expect("set active utility");
        assert!(result.operations.is_empty(), "utility switch must emit zero operations");
        let projection_after_switch = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        assert_eq!(projection_after_add, projection_after_switch, "utility switch must not mutate the projection");
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(
            app.projection().expect("projection").objects.len(),
            before,
            "a single undo reverts the addObject — proving the utility switch created no history entry"
        );
    }

    #[test]
    fn sun_measures_registered_for_all_four_panes_and_default_off() {
        let mut app = CadApp::default();
        assert!(!app.runtime.sun.enabled, "sun must be off by default");
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let measures = app.window_measures(&doc, &ViewState::default());
        for window_kind in [
            CAD_PLAY_WINDOW_SHAPE,
            CAD_PLAY_WINDOW_BUILDING,
            CAD_PLAY_WINDOW_ENERGY,
            CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
        ] {
            assert!(measures.contains_key(window_kind), "missing sun measures for {window_kind}");
        }
        drive(&mut app, &scene, "toggleSun", None);
        assert!(app.runtime.sun.enabled);
    }

    #[test]
    fn world_pick_selects_visible_object_by_index() {
        // The Shape pane's fixture object is a single hexagonal-cut solid (one object), so this
        // exercises worldPick-by-index against the Building pane, which has multiple objects.
        let mut app = CadApp::default();
        let scene = forest_play_scene();
        let building_visible: Vec<_> = scene.building_objects.iter().filter(|object| object.visible).collect();
        assert!(building_visible.len() > 1);
        let expected_id = building_visible[1].id.clone();
        drive(
            &mut app,
            &scene,
            "worldPick",
            Some(json!({ "surfaceId": "cad.play.scene3d/building", "id": 1, "merge": "replace" })),
        );
        assert_eq!(app.runtime.selected_object_ids, vec![expected_id]);
    }

    #[test]
    fn document_tree_reflects_viewport_selection() {
        let scene = forest_play_scene();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible shape object").id.clone();
        let runtime = CadPlayRuntime {
            selected_object_ids: vec![object_id.clone()],
            hovered_object_id: Some(object_id.clone()),
            ..CadPlayRuntime::default()
        };
        let selected = document_tree_selected_ids(&scene, &runtime).expect("selected");
        assert!(selected.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
        let highlighted = document_tree_highlighted_ids(&scene, &runtime).expect("highlighted");
        assert!(highlighted.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
    }
    //#endregion 🔖ViewState

    //#region 🔖Terminology
    #[test]
    fn multi_selection_inspector_shows_mixed_values() {
        let mut scene = default_document();
        let second = make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape);
        let second_id = second.id.clone();
        scene.objects.push(second);
        scene.objects[0].label = "Alpha".into();
        scene.objects[1].label = "Beta".into();
        scene.objects[0].orientation = Some([0.0, 0.0, 0.0, 1.0]);
        scene.objects[1].orientation = Some([0.0, 0.707, 0.0, 0.707]);
        let runtime = CadPlayRuntime {
            selected_object_ids: vec!["object-box-1".into(), second_id],
            ..CadPlayRuntime::default()
        };
        let panel = build_properties_panel(&view(scene, runtime), cad_labels(&ViewState::default()), CAD_DEFAULT_UTILITY_ID);
        let json = serde_json::to_string(&panel).unwrap();
        assert!(json.contains("Mixed"));
        assert!(json.contains("cad-play-inspector.object.orientation"));
    }

    fn selected_box_panel(view_state: &ViewState) -> String {
        let runtime = CadPlayRuntime {
            selected_object_ids: vec!["object-box-1".into()],
            ..CadPlayRuntime::default()
        };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(view_state), CAD_DEFAULT_UTILITY_ID);
        serde_json::to_string(&panel).unwrap()
    }

    #[test]
    fn cad_labels_resolve_native_by_default() {
        let json = selected_box_panel(&ViewState::default());
        assert!(json.contains("\"Object\""));
        assert!(!json.contains("Building component"));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_english() {
        let view_state = ViewState { terminology: Some("reuse".into()), locale: Some("en".into()), ..ViewState::default() };
        let json = selected_box_panel(&view_state);
        assert!(json.contains("Building component"));
        assert!(!json.contains("\"Object\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_german() {
        let view_state = ViewState { terminology: Some("reuse".into()), locale: Some("de".into()), ..ViewState::default() };
        assert!(selected_box_panel(&view_state).contains("Baukomponente"));
    }

    #[test]
    fn cad_labels_resolve_native_terminology_in_german() {
        let view_state = ViewState { terminology: Some("native".into()), locale: Some("de".into()), ..ViewState::default() };
        assert!(selected_box_panel(&view_state).contains("\"Objekt\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_for_primitive() {
        let runtime = CadPlayRuntime {
            selected_object_ids: vec!["object-box-1".into()],
            selected_primitive_id: Some("box-solid".into()),
            ..CadPlayRuntime::default()
        };
        let view_state = ViewState { terminology: Some("reuse".into()), locale: Some("de".into()), ..ViewState::default() };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(&view_state), CAD_DEFAULT_UTILITY_ID);
        assert!(serde_json::to_string(&panel).unwrap().contains("Bauteil"));
    }

    #[test]
    fn cad_labels_translate_document_tree_panes_in_german() {
        let app = CadApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, &doc, &view_state);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Form\""));
        assert!(json.contains("Gebäude"));
        assert!(json.contains("Energie"));
        assert!(json.contains("Struktur Klassisch"));
        assert!(json.contains("Referenzen"));
        assert!(json.contains("\"Knoten\""));
        assert!(!json.contains("\"Shape\""));
    }

    #[test]
    fn cad_labels_translate_catalogue_typologies_in_german() {
        let app = CadApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let node = app.render(CAD_PLAY_BODY_CATALOGUE, &doc, &view_state);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Typologien"));
        assert!(json.contains("Platte"));
        assert!(json.contains("Stütze"));
        assert!(json.contains("Balken"));
        assert!(json.contains("Wand"));
        assert!(json.contains("Außenwand"));
        assert!(!json.contains("\"Slab\""));
    }
    //#endregion 🔖Terminology

    //#region 🔖Operations
    #[test]
    fn add_object_action_appends_object_and_selects_it() {
        let mut app = CadApp::default();
        let scene = default_document();
        let emit = drive(&mut app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
        assert_eq!(emit.ops.len(), 1);
        let next = apply_ops(&scene, &emit.ops);
        assert!(
            next.objects.iter().any(|object| object.typology == "building.building.column")
                || next.building_objects.iter().any(|object| object.typology == "building.building.column")
        );
        assert_eq!(app.runtime.selected_object_ids.len(), 1);
    }

    #[test]
    fn add_object_through_wrapper_grows_projection() {
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.handle_action("addObject", Some(&json!({ "typology": "spatial.shape.primitive.box" })), &ViewState::default(), &meta("local"))
            .expect("add object");
        assert_eq!(app.projection().expect("projection").objects.len(), before + 1);
    }

    #[test]
    fn focus_model_definition_emits_document_op() {
        let mut app = new_app();
        app.handle_action("focusModelDefinition", Some(&json!({ "modelDefinitionId": "aec.building" })), &ViewState::default(), &meta("local"))
            .expect("focus model definition");
        assert_eq!(app.projection().expect("projection").active_model_definition_id, "aec.building");
    }

    #[test]
    fn derive_transformation_populates_energy_pane() {
        let mut app = CadApp::default();
        let mut scene = default_document();
        scene.objects = vec![make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape)];
        let emit = drive(&mut app, &scene, "applyTransformation", Some(json!({ "qid": "spatial.shape.from_geometry" })));
        assert!(!emit.ops.is_empty());
        let next = apply_ops(&scene, &emit.ops);
        assert!(!next.energy_objects.is_empty());
        assert!(next.energy_objects.iter().any(|object| object.typology.starts_with("energy.energy.")));
        assert_eq!(next.active_model_definition_id, "aec.building.energy");
    }

    #[test]
    fn forest_transformation_uses_live_shape_pane() {
        let mut app = CadApp::default();
        let mut scene = forest_play_scene();
        let fixture_energy_ids: Vec<String> = scene.energy_objects.iter().map(|object| object.id.clone()).collect();
        assert!(!fixture_energy_ids.is_empty(), "forest fixture should have energy objects");
        scene.energy_objects.clear();
        scene.objects.truncate(1);
        scene.objects[0].typology = "spatial.shape.primitive.box".into();
        scene.objects[0].label = "live-shape-only".into();
        let emit = drive(&mut app, &scene, "applyTransformation", Some(json!({ "qid": "spatial.shape.from_geometry" })));
        let next = apply_ops(&scene, &emit.ops);
        assert!(!next.energy_objects.is_empty());
        assert!(
            next.energy_objects.iter().all(|object| !fixture_energy_ids.contains(&object.id)),
            "live single-box derive should not repopulate the static forest energy fixture's original objects"
        );
    }

    #[test]
    fn save_selected_emits_download_effect() {
        let mut app = CadApp::default();
        let scene = default_document();
        app.runtime.selected_object_ids = vec!["object-box-1".into()];
        let emit = drive(&mut app, &scene, "saveSelected", None);
        assert!(emit.ops.is_empty(), "export must not mutate the document");
        assert_eq!(emit.effects.len(), 1);
        match &emit.effects[0] {
            HostEffect::DownloadMediaExport { filename, data, .. } => {
                assert_eq!(filename, "cad.selected.spatial.json");
                assert!(data.contains("activeModelDefinitionId"));
            }
            other => panic!("expected DownloadMediaExport, got {other:?}"),
        }
    }

    #[test]
    fn load_raw_request_emits_file_open_effect() {
        let mut app = CadApp::default();
        let emit = drive(&mut app, &default_document(), "loadRawRequest", None);
        match &emit.effects[0] {
            HostEffect::RequestFileOpen { import_action, read_as, .. } => {
                assert_eq!(import_action, "importCadFile");
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
    //#endregion 🔖Operations

    //#region 🔖Engagement
    #[test]
    fn engagement_starts_box_interaction_session() {
        let mut app = CadApp::default();
        let scene = default_document();
        app.runtime.engagement_input = "b".into();
        drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));
        assert!(app.runtime.engagement_session.is_some());
    }

    #[test]
    fn world_pointer_move_updates_live_preview_without_committing_or_emitting_ops() {
        let mut app = CadApp::default();
        let scene = default_document();
        app.runtime.engagement_input = "b".into();
        drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));

        let emit = drive(&mut app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })));
        assert!(emit.ops.is_empty(), "a pointer move must not emit any document operation");
        let session = app.runtime.engagement_session.as_ref().expect("session still active");
        assert_eq!(session.state, "first_corner", "pointer.move must not change state");
        assert_eq!(session.context.get("cursor"), Some(&json!([3.0, 4.0, 0.0])));
    }

    #[test]
    fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
        let mut app = CadApp::default();
        let mut scene = default_document();
        app.runtime.engagement_input = "b".into();
        drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));
        assert!(app.runtime.engagement_session.is_some());

        // box.json's default boxMode is "point" (length/width prompt); select diagonal mode (key
        // "d") to reach the classic two-corner-click flow.
        app.runtime.engagement_input = "d".into();
        drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));

        for position in [json!([0.0, 0.0, 0.0]), json!([2.0, 3.0, 0.0])] {
            let emit = drive(&mut app, &scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": position })));
            scene = apply_ops(&scene, &emit.ops);
        }

        app.runtime.engagement_input = "SetHeight2.5".into();
        drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));

        // box.json's `set.height` only records the height (state stays first_corner_height); an
        // explicit `confirm` (Enter) is needed to reach `ready`, box's commit.fromStates.
        app.runtime.engagement_input = "Confirm".into();
        let emit = drive(&mut app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })));
        scene = apply_ops(&scene, &emit.ops);
        assert!(app.runtime.engagement_session.is_none(), "box should have committed");
        assert_eq!(app.runtime.last_finalized_interaction_id.as_deref(), Some("primitive.box"));

        drive(&mut app, &scene, "engagementRepeatLast", Some(json!({ "pane": "shape" })));
        let session = app.runtime.engagement_session.as_ref().expect("repeat-last should start a session");
        assert_eq!(session.interaction_id, "primitive.box");
    }
    //#endregion 🔖Engagement

    //#region 🔖Import
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

    #[test]
    fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
        let mut app = CadApp::default();
        let scene = default_document();
        let file_text = json!({
            "schema": "spatial.model",
            "revision": 1,
            "modelDefinitionId": "spatial.shape",
            "objects": [{
                "id": "object-loaded",
                "label": "Loaded",
                "typology": "spatial.shape.primitive.box",
                "visible": true,
                "locked": false,
                "origin": [1.0, 2.0, 3.0],
                "primitives": []
            }]
        })
        .to_string();
        let emit = drive(
            &mut app,
            &scene,
            "importCadFile",
            Some(json!({ "payload": file_text, "name": "cad.spatial.json" })),
        );
        assert!(!emit.ops.is_empty(), "importCadFile must emit a SetScene op for a spatial JSON string payload");
        let next = apply_ops(&scene, &emit.ops);
        assert_eq!(next.objects.len(), 1);
        assert_eq!(next.objects[0].id, "object-loaded");
    }

    #[test]
    fn import_cad_file_action_imports_obj_by_extension() {
        let mut app = CadApp::default();
        let scene = default_document();
        let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        let obj_data_url = format!(
            "data:model/obj;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(obj_text)
        );
        let emit = drive(
            &mut app,
            &scene,
            "importCadFile",
            Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })),
        );
        assert!(!emit.ops.is_empty(), "importCadFile must emit an AddObject op for an OBJ payload");
        let next = apply_ops(&scene, &emit.ops);
        assert_eq!(next.objects.len(), scene.objects.len() + 1);
        assert!(next.objects.last().unwrap().solid_handle.is_some());
    }
    //#endregion 🔖Import

    //#region 🔖History
    #[test]
    fn undo_redo_round_trips_added_object_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.handle_action("addObject", Some(&json!({ "typology": "spatial.shape.primitive.box" })), &ViewState::default(), &meta("local"))
            .expect("add object");
        assert_eq!(app.projection().expect("projection").objects.len(), before + 1);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").objects.len(), before);
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").objects.len(), before + 1);
    }

    #[test]
    fn undo_redo_round_trips_added_node_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.handle_action("addNode", Some(&json!({ "kind": "solid" })), &ViewState::default(), &meta("local")).expect("add node");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
        let undo = app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("projection").nodes.len(), before);
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
    }

    #[test]
    fn coalesced_translate_drag_is_a_single_undo_step() {
        let mut app = new_app();
        app.handle_action("addObject", Some(&json!({ "typology": "spatial.shape.primitive.box" })), &ViewState::default(), &meta("local"))
            .expect("add object");
        let object_id = app.projection().expect("projection").objects.last().unwrap().id.clone();
        let origin_before = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        for _ in 0..3 {
            app.handle_action(
                "translateSelection",
                Some(&json!({ "ids": [object_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("translate tick");
        }
        let dragged = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        assert_eq!(dragged[0], origin_before[0] + 3.0, "three coalesced ticks accumulate");
        // One undo reverts the whole coalesced drag back to the pre-drag origin (not one tick).
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo drag");
        let after_undo = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        assert_eq!(after_undo, origin_before, "the coalesced drag undoes as one edit");
    }
    //#endregion 🔖History

    //#region 🔖Convergence
    /// 🧪 The definitional merge proof: two instances start from the SAME base projection, apply
    /// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging ops
    /// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
    /// `setDocument` snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        // A shared two-object base scene loaded identically into both instances.
        let mut base = default_document();
        base.objects = vec![
            make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape),
            make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape),
        ];
        let object_a = base.objects[0].id.clone();
        let object_b = base.objects[1].id.clone();
        let base_envelope = serde_json::to_string(&vcs::create_document_vcs_envelope::<CadScene, CadOp>(
            CAD_DOCUMENT_SCHEMA,
            "cad-play",
            base,
            None,
        ))
        .unwrap();

        let mut instance_a = new_app();
        let mut instance_b = new_app();
        instance_a.load_document(&base_envelope).expect("load a");
        instance_b.load_document(&base_envelope).expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A renames object A.
        instance_a
            .handle_action(
                "patchObject",
                Some(&json!({ "objectId": object_a, "field": "label", "value": "Renamed By A" })),
                &ViewState::default(),
                &meta("actor-a"),
            )
            .expect("a renames object a");

        // B renames object B — a disjoint edit that must survive alongside A's.
        instance_b
            .handle_action(
                "patchObject",
                Some(&json!({ "objectId": object_b, "field": "label", "value": "Renamed By B" })),
                &ViewState::default(),
                &meta("actor-b"),
            )
            .expect("b renames object b");

        // A neutral history command always pumps inbound ops before doing its own work.
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let scene_a = instance_a.projection().expect("projection a");
        let scene_b = instance_b.projection().expect("projection b");

        let label_a_in_a = scene_a.objects.iter().find(|object| object.id == object_a).unwrap().label.clone();
        let label_a_in_b = scene_b.objects.iter().find(|object| object.id == object_a).unwrap().label.clone();
        let label_b_in_a = scene_a.objects.iter().find(|object| object.id == object_b).unwrap().label.clone();
        let label_b_in_b = scene_b.objects.iter().find(|object| object.id == object_b).unwrap().label.clone();

        assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
        assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
        assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
        assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
    }

    #[test]
    fn ingest_operations_is_idempotent_for_cad() {
        let mut sender = new_app();
        let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender
            .handle_action("addNode", Some(&json!({ "kind": "solid" })), &ViewState::default(), &meta("local"))
            .expect("add node");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Ops { envelopes: ops } = message {
                envelopes.extend(ops);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied op to flow onto the channel");
        let operations_json = serde_json::to_string(&envelopes).expect("serialize envelopes");

        let mut receiver = new_app();
        let nodes_before = receiver.projection().expect("projection").nodes.len();
        receiver.ingest_operations(&operations_json).expect("ingest once");
        receiver.ingest_operations(&operations_json).expect("ingest twice");
        assert_eq!(
            receiver.projection().expect("projection").nodes.len(),
            nodes_before + 1,
            "feeding the same op twice must not double-apply"
        );
    }
    //#endregion 🔖Convergence
}
//#endregion 🧪Tests
