//! ⚙️ Cad app — headless compute (constitutional: engine).

pub mod geometry_import {
    //! 📐️ Fixture geometry import — builds kernel handles from authored spatial.model geometry.

    use cad_document::{CadEdge, CadFace, CadGeometry, CadObject, CadPrimitiveSlot, CadShell, CadSolid, CadWire};
    use kernel_3d_brepkit::mesh_data_from_mesh_transfer;
    use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, Vec3};
    use semio_framework_core::mesh_from_indexed;
    use semio_framework_plugin::MeshData;
    use serde_json::Value;
    use std::collections::HashMap;

    //#region 🔖️Parse
    pub fn parse_geometry(value: Option<&Value>) -> CadGeometry {
        value.and_then(|entry| serde_json::from_value(entry.clone()).ok()).unwrap_or_default()
    }

    fn vertex_map(geometry: &CadGeometry) -> HashMap<String, [f64; 3]> {
        geometry.vertices.iter().map(|vertex| (vertex.id.clone(), vertex.position)).collect()
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
    //#endregion 🔖️Parse

    //#region 🔖️KernelBuild
    fn to_vec3(point: [f64; 3]) -> Vec3 {
        [point[0], point[1], point[2]]
    }

    fn dedupe_consecutive_points(points: Vec<Vec3>) -> Vec<Vec3> {
        const EPS: f64 = 1e-9;
        let mut deduped: Vec<Vec3> = Vec::new();
        for point in points {
            if let Some(last) = deduped.last() {
                if (point[0] - last[0]).abs() < EPS && (point[1] - last[1]).abs() < EPS && (point[2] - last[2]).abs() < EPS {
                    continue;
                }
            }
            deduped.push(point);
        }
        deduped
    }

    fn wire_vertex_chain(wire: &CadWire, edges: &HashMap<String, &CadEdge>) -> Vec<String> {
        let Some(first_edge_id) = wire.edge_ids.first() else {
            return Vec::new();
        };
        let Some(first_edge) = edges.get(first_edge_id) else {
            return Vec::new();
        };
        if first_edge.vertex_ids.len() < 2 {
            return Vec::new();
        }
        let mut start = first_edge.vertex_ids[0].clone();
        let mut end = first_edge.vertex_ids[1].clone();
        if let Some(second_edge_id) = wire.edge_ids.get(1) {
            if let Some(second_edge) = edges.get(second_edge_id) {
                if second_edge.vertex_ids.len() >= 2 {
                    let shares_start = second_edge.vertex_ids[0] == start || second_edge.vertex_ids[1] == start;
                    let shares_end = second_edge.vertex_ids[0] == end || second_edge.vertex_ids[1] == end;
                    if shares_start && !shares_end {
                        std::mem::swap(&mut start, &mut end);
                    }
                }
            }
        }
        let mut chain = vec![start];
        let mut tip = end.clone();
        chain.push(tip.clone());
        for edge_id in wire.edge_ids.iter().skip(1) {
            let Some(edge) = edges.get(edge_id) else {
                continue;
            };
            if edge.vertex_ids.len() < 2 || edge.vertex_ids[0] == edge.vertex_ids[1] {
                continue;
            }
            let (vertex_a, vertex_b) = (&edge.vertex_ids[0], &edge.vertex_ids[1]);
            if vertex_a == &tip {
                chain.push(vertex_b.clone());
                tip = vertex_b.clone();
            } else if vertex_b == &tip {
                chain.push(vertex_a.clone());
                tip = vertex_a.clone();
            }
        }
        chain
    }

    fn wire_points(wire: &CadWire, edges: &HashMap<String, &CadEdge>, vertices: &HashMap<String, [f64; 3]>) -> Vec<Vec3> {
        dedupe_consecutive_points(wire_vertex_chain(wire, edges).iter().filter_map(|vertex_id| vertices.get(vertex_id).map(|position| to_vec3(*position))).collect())
    }

    fn face_boundary_points(face: &CadFace, wires: &HashMap<String, &CadWire>, edges: &HashMap<String, &CadEdge>, vertices: &HashMap<String, [f64; 3]>) -> Vec<Vec3> {
        face.wire_ids.iter().find_map(|wire_id| wires.get(wire_id)).map(|wire| wire_points(wire, edges, vertices)).unwrap_or_default()
    }

    pub fn import_geometry_handles(kernel: &mut dyn BrepKernel, geometry: &CadGeometry) -> HashMap<String, String> {
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
            if let Ok(handle) = block_on(kernel.polyline_wire(&points)) {
                handles.insert(wire.id.clone(), handle.0.clone());
            }
        }

        for face in &geometry.faces {
            if let Some(wire_id) = face.wire_ids.first() {
                if let Some(wire_handle) = handles.get(wire_id) {
                    let wire = GeometryHandle(wire_handle.clone());
                    if let Ok(handle) = block_on(kernel.planar_face_from_wire(&wire)) {
                        handles.insert(face.id.clone(), handle.0.clone());
                        continue;
                    }
                    if let Ok(handle) = block_on(kernel.face_from_wire(&wire)) {
                        handles.insert(face.id.clone(), handle.0.clone());
                        continue;
                    }
                }
            }
            let points = face_boundary_points(face, &wires, &edges, &vertices);
            if points.len() < 3 {
                continue;
            }
            if let Ok(handle) = block_on(kernel.planar_face_from_points(&points)) {
                handles.insert(face.id.clone(), handle.0.clone());
            }
        }

        for shell in &geometry.shells {
            let face_handles: Vec<GeometryHandle> = shell.face_ids.iter().filter_map(|face_id| faces.get(face_id)).filter_map(|face| handles.get(&face.id).cloned().map(GeometryHandle)).collect();
            if face_handles.len() < 1 {
                continue;
            }
            if let Ok(solid) = block_on(kernel.sew_faces(&face_handles, 0.01)) {
                handles.insert(shell.id.clone(), solid.0.clone());
            }
        }

        for solid in &geometry.solids {
            let shell_handles: Vec<GeometryHandle> = solid.shell_ids.iter().filter_map(|shell_id| handles.get(shell_id).cloned().map(GeometryHandle)).collect();
            if shell_handles.len() == 1 {
                handles.insert(solid.id.clone(), shell_handles[0].0.clone());
                continue;
            }
            let face_handles: Vec<GeometryHandle> = solid.shell_ids.iter().filter_map(|shell_id| shells.get(shell_id)).flat_map(|shell| shell.face_ids.iter()).filter_map(|face_id| handles.get(face_id).cloned().map(GeometryHandle)).collect();
            if face_handles.is_empty() {
                continue;
            }
            if let Ok(built) = block_on(kernel.sew_faces(&face_handles, 0.01)) {
                handles.insert(solid.id.clone(), built.0.clone());
            }
        }

        handles
    }

    pub fn resolve_primitive_handle(primitives: &[CadPrimitiveSlot], handles: &HashMap<String, String>) -> Option<(String, String)> {
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
            solid.shell_ids.iter().filter_map(|shell_id| shells.get(shell_id)).flat_map(|shell| shell.face_ids.iter()).filter_map(|face_id| faces.get(face_id)).flat_map(|face| face.wire_ids.iter()).collect()
        } else if let Some(shell) = shells.get(primitive_id) {
            shell.face_ids.iter().filter_map(|face_id| faces.get(face_id)).flat_map(|face| face.wire_ids.iter()).collect()
        } else if let Some(face) = faces.get(primitive_id) {
            face.wire_ids.iter().collect()
        } else {
            Vec::new()
        };

        let edge_ids: Vec<&String> = if let Some(wire) = wires.get(primitive_id) { wire.edge_ids.iter().collect() } else { wire_ids.into_iter().filter_map(|wire_id| wires.get(wire_id)).flat_map(|wire| wire.edge_ids.iter()).collect() };

        let vertex_ids: Vec<&String> = if let Some(edge) = edges.get(primitive_id) { edge.vertex_ids.iter().collect() } else { edge_ids.into_iter().filter_map(|edge_id| edges.get(edge_id)).flat_map(|edge| edge.vertex_ids.iter()).collect() };

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
        primitives.iter().find_map(|primitive| extent_from_positions(&primitive_vertex_positions(geometry, &primitive.primitive_id)))
    }

    fn centroid_from_positions(positions: &[[f64; 3]]) -> Option<[f64; 3]> {
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
        Some([(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5, (min[2] + max[2]) * 0.5])
    }

    /// 🎯️ World-space centroid of the first primitive slot that resolves against authored geometry.
    pub fn centroid_from_fixture_primitives(geometry: &CadGeometry, primitives: &[CadPrimitiveSlot]) -> Option<[f64; 3]> {
        primitives.iter().find_map(|primitive| centroid_from_positions(&primitive_vertex_positions(geometry, &primitive.primitive_id)))
    }

    /// 🧵️ Tessellates an object through a kernel handle when that handle is still resident.
    pub fn tessellate_object_mesh(kernel: &mut dyn BrepKernel, object: &CadObject, kind: &str) -> Option<MeshData> {
        let handle_id = object.solid_handle.as_deref()?;
        if block_on(kernel.kind(&GeometryHandle(handle_id.into()))).is_err() {
            return None;
        }
        tessellate_geometry_handle(kernel, handle_id, kind)
    }

    /// 🧵️ Re-imports fixture geometry and tessellates the object's primitive slots.
    pub fn tessellate_object_mesh_from_fixture(kernel: &mut dyn BrepKernel, object: &CadObject, geometry: &CadGeometry) -> Option<MeshData> {
        if object.primitives.is_empty() {
            return None;
        }
        let handles = import_geometry_handles(kernel, geometry);
        let (handle_id, kind) = resolve_primitive_handle(&object.primitives, &handles)?;
        tessellate_geometry_handle(kernel, &handle_id, &kind)
    }

    pub fn tessellate_geometry_handle(kernel: &mut dyn BrepKernel, handle_id: &str, kind: &str) -> Option<MeshData> {
        let handle = GeometryHandle(handle_id.into());
        if kind == "curve" {
            return curve_mesh_from_wire(kernel, &handle);
        }
        if let Ok(mesh) = block_on(kernel.tessellate(&handle, 0.1)) {
            let data = mesh_data_from_mesh_transfer(&mesh);
            if data.indices.len() < 3 {
                return None;
            }
            return Some(data);
        }
        None
    }

    fn curve_mesh_from_wire(kernel: &mut dyn BrepKernel, wire: &GeometryHandle) -> Option<MeshData> {
        let mesh = block_on(kernel.tessellate(wire, 0.1)).ok()?;
        Some(mesh_data_from_mesh_transfer(&mesh))
    }

    //#region 🔖️MeshImport
    /// 📦️ Serializes triangle mesh data to a minimal `v`/`f` OBJ payload the kernel's OBJ reader
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

    /// 🧱️ Builds a `CadObject` from an arbitrary triangle mesh (e.g. a tessellated DWG layer) by
    /// importing it into the brep kernel as a solid via the OBJ reader, so it plays through the same
    /// `solidHandle`/`primitives` path fixture geometry uses. Falls back to an extent-only object
    /// with no primitives (rendered via the typology bounding-box mesh fallback) when the mesh has
    /// no triangles or the kernel is unable to import it.
    pub fn cad_object_from_mesh(kernel: &mut dyn BrepKernel, id: impl Into<String>, label: impl Into<String>, typology: impl Into<String>, mesh: &MeshData) -> CadObject {
        let extent = mesh_extent(mesh);
        let solid_handle = if mesh.indices.len() >= 3 { block_on(kernel.import_obj(&mesh_to_obj_text(mesh), 0.01)).ok().map(|handle| handle.0) } else { None };
        let primitives = solid_handle.clone().map(|primitive_id| vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() }]).unwrap_or_default();
        CadObject { id: id.into(), label: label.into(), typology: typology.into(), visible: true, locked: false, origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, extent, solid_handle, primitives }
    }
    /// 🧊️ Builds a `CadObject` around a solid `GeometryHandle` already resident in `kernel` (e.g. from
    /// a native OBJ/STL/STEP import), tessellating once just to derive a display `extent` — the
    /// handle itself is kept verbatim rather than being round-tripped through a mesh reimport.
    pub fn cad_object_from_solid_handle(kernel: &mut dyn BrepKernel, id: impl Into<String>, label: impl Into<String>, typology: impl Into<String>, handle: GeometryHandle) -> CadObject {
        let extent = block_on(kernel.tessellate(&handle, 0.1)).ok().and_then(|mesh| mesh_extent(&mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index)));
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
    //#endregion 🔖️MeshImport

    pub fn object_label_from_id(object_id: &str) -> String {
        object_id.split('-').last().map(str::to_string).unwrap_or_else(|| object_id.to_string())
    }

    pub fn objects_from_fixture_model(kernel: &mut dyn BrepKernel, objects_value: &[Value], geometry: &CadGeometry) -> Vec<CadObject> {
        let handles = import_geometry_handles(kernel, geometry);
        objects_value
            .iter()
            .filter_map(|entry| {
                let object_id = entry.get("id")?.as_str()?;
                let typology = entry.get("typology").and_then(|value| value.as_str()).unwrap_or("").to_string();
                let primitives = primitives_from_json(entry);
                let (solid_handle, _primary_kind) = resolve_primitive_handle(&primitives, &handles).map(|(handle, kind)| (Some(handle), kind)).unwrap_or((None, String::new()));
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
            return map.iter().map(|(slot, value)| CadPrimitiveSlot { slot: slot.clone(), primitive_id: value.as_str().unwrap_or_default().into(), kind: slot.clone() }).collect();
        }
        if let Some(rows) = primitives.as_array() {
            return rows
                .iter()
                .filter_map(|row| {
                    let kind = row.get("kind")?.as_str()?;
                    let primitive_id = row.get("id")?.as_str()?;
                    let slot = row.get("slot").and_then(|value| value.as_str()).unwrap_or(kind);
                    Some(CadPrimitiveSlot { slot: slot.into(), primitive_id: primitive_id.into(), kind: kind.into() })
                })
                .collect();
        }
        Vec::new()
    }
    //#endregion 🔖️KernelBuild

    #[cfg(test)]
    mod tests {
        use super::*;
        use kernel_3d_brepkit::BrepkitKernel;

        fn mesh_triangle_area(mesh: &MeshData, triangle_index: usize) -> f32 {
            let i0 = mesh.indices[triangle_index * 3] as usize;
            let i1 = mesh.indices[triangle_index * 3 + 1] as usize;
            let i2 = mesh.indices[triangle_index * 3 + 2] as usize;
            let p0 = [mesh.positions[i0 * 3], mesh.positions[i0 * 3 + 1], mesh.positions[i0 * 3 + 2]];
            let p1 = [mesh.positions[i1 * 3], mesh.positions[i1 * 3 + 1], mesh.positions[i1 * 3 + 2]];
            let p2 = [mesh.positions[i2 * 3], mesh.positions[i2 * 3 + 1], mesh.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
        }

        #[test]
        fn forest_wire_chains_reversed_edges_by_vertex_id() {
            let source = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
            let edges = edge_map(&geometry);
            let wire = geometry.wires.iter().find(|wire| wire.id == "hexagonal-cut-concrete-forest-left-wire-103").expect("wire");
            let chain = wire_vertex_chain(wire, &edges);
            assert_eq!(
                chain,
                vec![
                    "hexagonal-cut-concrete-forest-left-vertex-84".to_string(),
                    "hexagonal-cut-concrete-forest-left-vertex-96".to_string(),
                    "hexagonal-cut-concrete-forest-left-vertex-94".to_string(),
                    "hexagonal-cut-concrete-forest-left-vertex-83".to_string(),
                    "hexagonal-cut-concrete-forest-left-vertex-84".to_string(),
                ]
            );
        }

        #[test]
        fn forest_shape_geometry_imports_solid_handle() {
            let source = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
            let objects = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            assert_eq!(imported.len(), 1);
            assert!(imported[0].solid_handle.is_some());
            let mesh = tessellate_geometry_handle(&mut kernel, imported[0].solid_handle.as_ref().expect("handle"), "solid").expect("mesh");
            assert!(mesh.positions.len() > 12);
            assert!(mesh.edge_positions.len() >= 6);
            assert_eq!(mesh.edge_positions.len() % 6, 0);
            for triangle_index in 0..mesh.triangle_count() {
                assert!(mesh_triangle_area(&mesh, triangle_index) > 1e-10, "triangle {triangle_index} must not be degenerate");
            }
        }

        #[test]
        fn forest_energy_surface_tessellates_at_authored_height() {
            let source = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/2/model/geometry"));
            let objects = root.pointer("/models/2/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            assert_eq!(imported.len(), 1);
            assert!(imported[0].solid_handle.is_some(), "energy face handle");
            let handle_id = imported[0].solid_handle.as_ref().expect("handle");
            let mesh = tessellate_geometry_handle(&mut kernel, handle_id, "surface").expect("surface mesh");
            let min_z = mesh.positions.chunks_exact(3).map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
            let max_z = mesh.positions.chunks_exact(3).map(|vertex| vertex[2]).fold(f32::NEG_INFINITY, f32::max);
            assert!(min_z > 2.5, "energy surface min z {min_z}");
            assert!(max_z < 3.5, "energy surface max z {max_z}");
        }

        #[test]
        fn forest_structure_surface_tessellates_at_authored_height() {
            let source = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/3/model/geometry"));
            let objects = root.pointer("/models/3/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            let slab = imported.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("surface object");
            let mesh = tessellate_geometry_handle(&mut kernel, slab.solid_handle.as_ref().expect("handle"), "surface").expect("surface mesh");
            let min_z = mesh.positions.chunks_exact(3).map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
            assert!(min_z > 2.5, "structure slab min z {min_z}");
        }

        #[test]
        fn forest_structure_curve_wires_tessellate_as_centerlines() {
            let source = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
            let root: Value = serde_json::from_str(source).expect("fixture");
            let geometry = parse_geometry(root.pointer("/models/3/model/geometry"));
            let objects = root.pointer("/models/3/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
            let mut kernel = BrepkitKernel::new();
            let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
            assert!(!imported.is_empty());
            let curve_object = imported.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "curve")).expect("curve object");
            let handle = curve_object.solid_handle.as_ref().expect("curve handle");
            let mesh = tessellate_geometry_handle(&mut kernel, handle, "curve").expect("curve mesh");
            assert!(mesh.edge_positions.len() >= 6);
            assert_eq!(mesh.edge_positions.len() % 6, 0);
            assert!(mesh.indices.is_empty());
        }
    }
}

pub mod transformation {
    //! 🔄️ CAD derive-transformation engine — ports premigration `runDeriveTransformation` onto `kernel_3d_brepkit`.

    use cad_document::{CadObject, CadPrimitiveSlot};

    use kernel_3d_engine::{BrepKernel, GeometryHandle, Vec3};
    use std::collections::HashMap;

    //#region 🔖️ClassifyRules
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
        ClassifyRule { role: "roof", typology: "energy.energy.roof", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: Some(ZBand::Max), fallback: false },
        ClassifyRule { role: "baseplate", typology: "energy.energy.baseplate", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: Some(ZBand::Min), fallback: false },
        ClassifyRule { role: "slab", typology: "energy.energy.hull", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: None, fallback: false },
        ClassifyRule { role: "externalwall", typology: "energy.energy.externalwall", dominant_axis: None, min_dominant_normal: None, min_axis_normal: Some(0.5), z_band: None, fallback: false },
        ClassifyRule { role: "slab", typology: "energy.energy.hull", dominant_axis: None, min_dominant_normal: None, min_axis_normal: None, z_band: None, fallback: true },
    ];

    const ENERGY_TYPOLOGIES: &[&str] = &["energy.energy.hull", "energy.energy.baseplate", "energy.energy.roof", "energy.energy.externalwall", "energy.energy.windows"];
    //#endregion 🔖️ClassifyRules

    //#region 🔖️FaceAnalytics
    /// @emoji 📍️ Face centroid via surface midpoint sampling (premigration `faceCentroid` equivalent).
    pub fn face_centroid_sync(kernel: &dyn BrepKernel, face: &GeometryHandle) -> Option<Vec3> {
        kernel_3d_engine::block_on(kernel.surface_point(face, 0.5, 0.5)).ok()
    }

    /// @emoji 🧭️ Face outward normal at the surface midpoint.
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
        format!("{dominant}:{sign}:{}:{}:{}", q(centroid[0]), q(centroid[1]), q(centroid[2]))
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

    fn classify_rule_matches(rule: &ClassifyRule, normal: Vec3, centroid_z: f64, z_min: f64, z_max: f64, z_tol: f64) -> bool {
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
    //#endregion 🔖️FaceAnalytics

    //#region 🔖️SolidConstruction
    /// @emoji 📦️ Builds or reuses a kernel solid for a CAD object.
    pub fn solid_for_object(kernel: &mut dyn BrepKernel, object: &CadObject) -> Option<GeometryHandle> {
        if let Some(handle) = object.solid_handle.as_ref() {
            if kernel_3d_engine::block_on(kernel.kind(&GeometryHandle(handle.clone()))).is_ok() {
                return Some(GeometryHandle(handle.clone()));
            }
        }
        let [ex, ey, ez] = object.extent.unwrap_or([1.0, 1.0, 1.0]);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        let is_cylindrical = object.typology.contains("column");
        let handle = if is_cylindrical { kernel_3d_engine::block_on(kernel.cylinder_prim(width.max(depth) * 0.5, height)).ok() } else { kernel_3d_engine::block_on(kernel.box_prim(width, depth, height)).ok() }?;
        Some(handle)
    }

    /// @emoji 📦️ Builds a kernel solid sized from extent without mutating the object.
    pub fn build_solid_for_typology(kernel: &mut dyn BrepKernel, typology: &str, extent: [f64; 3]) -> Option<GeometryHandle> {
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
    //#endregion 🔖️SolidConstruction

    //#region 🔖️DeriveEngine
    struct FaceMeta {
        handle: GeometryHandle,
        normal: Vec3,
        centroid: Vec3,
    }

    fn next_object_id(prefix: &str, index: usize) -> String {
        format!("{prefix}-{index}")
    }

    /// @emoji 🔄️ Derives energy objects from shape-pane solids via fuse + face classification.
    pub fn run_derive_from_geometry(kernel: &mut dyn BrepKernel, source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
        let solids: Vec<GeometryHandle> = source_objects.iter().filter_map(|object| solid_for_object(kernel, object)).collect();
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
        let face_meta: Vec<FaceMeta> = topology
            .faces
            .iter()
            .filter_map(|face| {
                let normal = face_normal_sync(kernel, face)?;
                let centroid = face_centroid_sync(kernel, face)?;
                Some(FaceMeta { handle: face.clone(), normal, centroid })
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
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: hull.0.clone(), kind: "solid".into() }],
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
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: hull.0.clone(), kind: "solid".into() }],
        });
        let mut grouped: HashMap<String, Vec<&FaceMeta>> = HashMap::new();
        for face in &face_meta {
            let rule = FROM_GEOMETRY_CLASSIFY_RULES.iter().find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol)).unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
            if rule.role == "slab" && rule.fallback {
                continue;
            }
            let key = format!("{}:{}", rule.typology, face_plane_group_key(face.normal, face.centroid));
            grouped.entry(key).or_default().push(face);
        }
        let mut index = 1usize;
        for (_key, faces) in grouped {
            let face = faces[0];
            let rule = FROM_GEOMETRY_CLASSIFY_RULES.iter().find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol)).unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
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
                primitives: vec![CadPrimitiveSlot { slot: "surface".into(), primitive_id: face.handle.0.clone(), kind: "surface".into() }],
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

    /// @emoji 🔄️ Maps building typologies to structure-classic equivalents (premigration `from_building` applier).
    pub fn apply_from_building(source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        source_objects
            .iter()
            .filter_map(|object| BUILDING_TO_STRUCTURE.iter().find(|(from, _)| *from == object.typology.as_str()).map(|(_, to)| (*to, object)))
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

    /// @emoji 🔄️ Filters source objects to whitelisted typologies (premigration `applyTransformationFallback`).
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
    //#endregion 🔖️DeriveEngine

    #[cfg(test)]
    mod tests {
        use super::*;
        use kernel_3d_brepkit::BrepkitKernel;

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
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0.clone(), kind: "solid".into() }],
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

pub mod interaction {
    //! 🎮️ CAD interaction statechart — a generic interpreter over `spatial.interaction` JSON assets
    //! (`cad/asset/modelDefinition/*/interaction/*.json`, mirroring `cad/schema/json/🔣️inter🔣️action.json`),
    //! plus a small commit-action runner mapping each spec's `commit.operation.action` onto real
    //! `kernel_3d_brepkit` calls. Four "building.building.*" ids have no JSON asset (aec.building has
    //! no interaction directory) and keep a bespoke hand-written statechart (`legacy_*` functions)
    //! identical to the pre-engine behavior.

    use cad_document::{evaluate_expr, CadObject, CadPaneId, CadPrimitiveSlot, DisplayItemSpec, Effect, ExprEnv, ExprPathRoot, ExprPathSegment, ExprPathTarget, InteractionSpec};

    use kernel_3d_engine::BrepKernel;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::sync::OnceLock;

    //#region 🔖️Types
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
    //#endregion 🔖️Types

    //#region 🔖️Registry
    /// `(modelDefinitionId, raw JSON)` for every `interaction/*.json` asset embedded at build time.
    /// `aec.building` has no interaction assets of its own — see `LEGACY_BUILDING_INTERACTION_IDS`.
    const RAW_INTERACTION_ASSETS: &[(&str, &str)] = &[
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️arc.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️area.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️booleanDifference.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️booleanIntersection.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️booleanUnion.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️box.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️chamfer.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️circle.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️constructCurve.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️constructSurface.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️controlPointCurve.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️copy.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️createAnchor.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️cylinder.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️explode.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️extrudeCrv.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️extrudeWire.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️fillet.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️interpolateCurve.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️join.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️length.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️line.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️loft.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️mirror.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️move.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️networkSrf.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️offsetSurface.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️plane.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️polyline.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️rotate.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️scale1d.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️scale3d.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️sphere.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️split.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️sweep1.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️sweep2.json")),
        ("spatial.shape", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📐️spatial.shape/🎬️interaction/🔣️trim.json")),
        ("aec.building.energy", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🔥️aec.building.energy/🎬️interaction/🔣️constructBasePlate.json")),
        ("aec.building.energy", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🔥️aec.building.energy/🎬️interaction/🔣️constructExternalWall.json")),
        ("aec.building.energy", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🔥️aec.building.energy/🎬️interaction/🔣️constructHull.json")),
        ("aec.building.energy", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🔥️aec.building.energy/🎬️interaction/🔣️constructRoof.json")),
        ("aec.building.energy", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🔥️aec.building.energy/🎬️interaction/🔣️constructWindows.json")),
        ("aec.building.structure.classic", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🏛️aec.building.structure.classic/🎬️interaction/🔣️constructOneWayReinforcedConcreteSlab.json")),
        ("aec.building.structure.classic", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🏛️aec.building.structure.classic/🎬️interaction/🔣️constructReinforcedConcreteColumn.json")),
        ("aec.building.structure.classic", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🏛️aec.building.structure.classic/🎬️interaction/🔣️constructReinforcedConcreteExternalWall.json")),
        ("aec.building.structure.classic", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🏛️aec.building.structure.classic/🎬️interaction/🔣️constructReinforcedConcreteInternalWall.json")),
        ("aec.building.structure.fem.line", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/📏️aec.building.structure.fem.line/🎬️interaction/🔣️constructLineElement.json")),
        ("aec.building.structure.fem.solid", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🧊️aec.building.structure.fem.solid/🎬️interaction/🔣️constructSolidElement.json")),
        ("aec.building.structure.fem.surface", include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🏗️modelDefinition/🗺️aec.building.structure.fem.surface/🎬️interaction/🔣️constructSurfaceElement.json")),
    ];

    const LEGACY_BUILDING_INTERACTION_IDS: &[&str] = &["building.building.constructWall", "building.building.constructBeam", "building.building.constructColumn", "building.building.constructSlab"];

    fn is_legacy_building_id(id: &str) -> bool {
        LEGACY_BUILDING_INTERACTION_IDS.contains(&id)
    }

    static PARSED_SPECS: OnceLock<Vec<(&'static str, InteractionSpec)>> = OnceLock::new();

    fn parsed_specs() -> &'static [(&'static str, InteractionSpec)] {
        PARSED_SPECS.get_or_init(|| RAW_INTERACTION_ASSETS.iter().filter_map(|(model_def, raw)| serde_json::from_str::<InteractionSpec>(raw).ok().map(|spec| (*model_def, spec))).collect())
    }

    fn spec_by_id(id: &str) -> Option<&'static InteractionSpec> {
        parsed_specs().iter().find(|(_, spec)| spec.id == id).map(|(_, spec)| spec)
    }

    static CATALOG: OnceLock<Vec<InteractionCatalogEntry>> = OnceLock::new();

    fn catalog() -> &'static [InteractionCatalogEntry] {
        CATALOG.get_or_init(|| {
            let mut entries = vec![
                InteractionCatalogEntry { id: "building.building.constructWall".to_string(), label: "Wall".to_string(), key: "w".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.wall".to_string() },
                InteractionCatalogEntry { id: "building.building.constructBeam".to_string(), label: "Beam".to_string(), key: "m".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.beam".to_string() },
                InteractionCatalogEntry {
                    id: "building.building.constructColumn".to_string(),
                    label: "Column".to_string(),
                    key: "c".to_string(),
                    model_definition_id: "aec.building".to_string(),
                    produces_typology: "building.building.column".to_string(),
                },
                InteractionCatalogEntry { id: "building.building.constructSlab".to_string(), label: "Slab".to_string(), key: "l".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.slab".to_string() },
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
    //#endregion 🔖️Registry

    //#region 🔖️Catalog
    pub fn list_interactions_for_model_definition(model_definition_id: &str) -> Vec<&'static InteractionCatalogEntry> {
        catalog().iter().filter(|entry| entry.model_definition_id == model_definition_id).collect()
    }

    pub fn resolve_interaction_key(input: &str, model_definition_id: &str) -> Option<&'static InteractionCatalogEntry> {
        let trimmed = input.trim().to_lowercase();
        catalog().iter().find(|entry| entry.model_definition_id == model_definition_id && (entry.key == trimmed || entry.id.eq_ignore_ascii_case(&trimmed) || entry.id.to_lowercase().ends_with(&format!(".{trimmed}"))))
    }

    pub fn interaction_by_id(id: &str) -> Option<&'static InteractionCatalogEntry> {
        catalog().iter().find(|entry| entry.id == id)
    }
    //#endregion 🔖️Catalog

    //#region 🔖️Statechart
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
            return Some(CadEngagementSession { interaction_id: interaction_id.to_string(), state: "idle".to_string(), context: HashMap::new(), pane, last_response: None });
        }
        let spec = spec_by_id(interaction_id)?;
        Some(CadEngagementSession { interaction_id: spec.id.clone(), state: spec.machine.initial.clone(), context: HashMap::new(), pane, last_response: None })
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
                    out.push(KeyedTransition { key: key.clone(), label: transition.label.clone().unwrap_or_else(|| handler.event.clone()), event_kind: handler.event.clone() });
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
    /// follow-up; they no-operation here rather than error.
    fn run_named_action_effect(context: &mut HashMap<String, Value>, payload: Option<&Value>, action: &str, params: &HashMap<String, Value>) {
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
                let evaluated: HashMap<String, Value> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
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
    const NUMERIC_ENTRY_STATES: &[&str] = &["first_corner_height", "two_points_height", "slab_height", "column_height", "radius", "curve_height"];

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
    //#endregion 🔖️Statechart

    //#region 🔖️CommitRunner
    fn commit_primitive_box(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
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
    fn commit_from_2_points_and_height(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label: &str, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
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
    fn commit_command_finish(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, context: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
        let result_kind = params.get("resultKind").and_then(|value| value.as_str())?;
        match result_kind {
            "sphere" => {
                let points = context.get("points")?.as_object()?;
                let center = points.get("center").and_then(parse_vec3)?;
                let radius = if let Some(radius) = context.get("radius").and_then(|value| value.as_f64()) {
                    radius
                } else {
                    let radius_point = points.get("radiusPoint").and_then(parse_vec3)?;
                    ((radius_point[0] - center[0]).powi(2) + (radius_point[1] - center[1]).powi(2) + (radius_point[2] - center[2]).powi(2)).sqrt()
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

    fn legacy_commit_object(kernel: &mut dyn BrepKernel, session: &CadEngagementSession, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
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
        let default_height = if id.contains("Slab") {
            0.25
        } else if id.contains("Beam") {
            0.4
        } else {
            3.0
        };
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

    pub fn commit_object(kernel: &mut dyn BrepKernel, session: &CadEngagementSession, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
        if is_legacy_building_id(&session.interaction_id) {
            return legacy_commit_object(kernel, session, label_count, next_id);
        }
        let spec = spec_by_id(&session.interaction_id)?;
        let env = ExprEnv { context: &session.context, event: None };
        let empty_vars = HashMap::new();
        let params: HashMap<String, Value> = spec.commit.operation.params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
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
    //#endregion 🔖️CommitRunner

    //#region 🔖️Preview
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

    fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv<'_>, vars: &HashMap<String, Value>) -> Option<Value> {
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
                let evaluated_params: serde_json::Map<String, Value> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, env, vars))).collect();
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
    //#endregion 🔖️Preview

    #[cfg(test)]
    mod tests {
        use super::*;
        use kernel_3d_brepkit::BrepkitKernel;

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
            // 🔣️box.json's default `boxMode` (set by the `start` transition) is "point", not "diagonal" —
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
            let mut session = start_session("structure.structure.constructReinforcedConcreteColumn", CadPaneId::StructureClassic).expect("session");
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
            let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
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
            let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
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

use base64::Engine as _;
use cad_document::{cad_all_objects, cad_pane_from_model_definition_id, cad_pane_geometry, CadCamera, CadGeometry, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CadProjectionDsl, CadReference, CadScene, CAD_PLAY_DOCUMENT_SCHEMA};
use geometry_import::{cad_object_from_mesh, cad_object_from_solid_handle, centroid_from_fixture_primitives, objects_from_fixture_model, parse_geometry, tessellate_object_mesh, tessellate_object_mesh_from_fixture};
use kernel_3d_brepkit::{mesh_data_from_mesh_transfer, BrepkitKernel};
use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, MeshTransfer};
use semio_framework_core::MeshImporter;
use semio_framework_plugin::{mesh_from_kind, MeshData, OsMediaFormat, WorldProjectionConfig};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use transformation::solid_for_object;

//#region 🔖️Compute
pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

/// @emoji 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.
const CAD_MODEL_INDEX_SHAPE: usize = 0;

const CAD_MODEL_INDEX_BUILDING: usize = 1;

const CAD_MODEL_INDEX_ENERGY: usize = 2;

const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/📐️cad/🖼️asset/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");

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
            let _ = block_on(kernel.dispose(&handle));
            return mesh_from_kind(typology_mesh_kind(typology));
        }
    };
    let _ = block_on(kernel.dispose(&handle));
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
    for vertex in mesh.positions.chunks_exact(3) {
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
    for vertex in mesh.positions.chunks_exact_mut(3) {
        vertex[0] += offset[0];
        vertex[1] += offset[1];
        vertex[2] += offset[2];
    }
    for segment in mesh.edge_positions.chunks_exact_mut(6) {
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
        let primitive_id = handle.0.clone();
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
    object.primitives.first().map(|primitive| primitive.kind.as_str()).unwrap_or("solid")
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
    let typology = first.map(|(object, _)| object.typology.as_str()).unwrap_or("spatial.shape.primitive.box");
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

//#region 🔖️Construct
/// @emoji 🕸️ "Construct" — CAD's topology query capability — is NOT a new parser: it is Jack
/// (`mathematical_graph_dsl`, an already-complete Cypher-like language with its own parser,
/// executor, formatter, and LSP) applied to brep topology through this `QueryableGraph`
/// implementation. Every editable entity (`CadVertex`/`CadEdge`/`CadWire`/`CadFace`/`CadShell`/
/// `CadSolid`) becomes a Jack node labeled by its kind (`"Vertex"`/`"Edge"`/.../`"Solid"`);
/// `[:BOUNDED_BY]` moves down one topological dimension (Solid->Shell, Shell->Face, Face->Wire),
/// `[:CONTAINS]` reaches the entities that directly compose a boundary member (Wire->Edge,
/// Edge->Vertex) — exactly the relationship vocabulary `.🦑️repo/✍️notes/construct.md`'s TopoCypher
/// design calls for. A query like `MATCH (f:Face)-[:BOUNDED_BY]->(w:Wire)-[:CONTAINS]->(e:Edge)`
/// runs against this today via `mathematical_graph_dsl::run_query`, with zero new grammar.
pub mod construct {
    use cad_document::CadGeometry;
    use mathematical_graph_dsl::{QueryableEdge, QueryableGraph};
    use mathematical_graph_manifest::PropertyValue;
    use std::collections::BTreeSet;

    /// @emoji 🏷️ The Jack node-label vocabulary for brep entities — mirrors TopoCypher's
    /// `(:Vertex) (:Edge) (:Wire) (:Face) (:Shell) (:Solid)` (the "Cell"/"CellComplex"/"Cluster"
    /// labels from `construct.md` have no `CadGeometry` equivalent yet, so they're omitted rather
    /// than faked).
    const KIND_VERTEX: &str = "Vertex";
    const KIND_EDGE: &str = "Edge";
    const KIND_WIRE: &str = "Wire";
    const KIND_FACE: &str = "Face";
    const KIND_SHELL: &str = "Shell";
    const KIND_SOLID: &str = "Solid";

    const REL_BOUNDED_BY: &str = "BOUNDED_BY";
    const REL_CONTAINS: &str = "CONTAINS";

    /// @emoji 🕸️ One `CadGeometry` pane (e.g. `scene.shape_geometry`), exposed as a Jack
    /// `QueryableGraph` — read-only, matching `construct.md`'s explicit constraint that direct
    /// graph mutation is unsafe for a B-rep and must go through a validated command layer instead.
    pub struct CadTopologyGraph<'a> {
        geometry: &'a CadGeometry,
    }

    impl<'a> CadTopologyGraph<'a> {
        pub fn new(geometry: &'a CadGeometry) -> Self {
            Self { geometry }
        }
    }

    impl QueryableGraph for CadTopologyGraph<'_> {
        fn manifest(&self) -> Option<&mathematical_graph_manifest::GraphManifest> {
            // No compile-time schema for a dynamically-shaped brep pane — every query resolves
            // purely against `node_kind`/`node_property`, matching `EmptyGraph`'s precedent in
            // `mathematical_graph_dsl`'s own idiom-hooks completion path.
            None
        }

        fn node_ids(&self) -> Vec<String> {
            let g = self.geometry;
            g.vertices
                .iter()
                .map(|v| v.id.clone())
                .chain(g.edges.iter().map(|e| e.id.clone()))
                .chain(g.wires.iter().map(|w| w.id.clone()))
                .chain(g.faces.iter().map(|f| f.id.clone()))
                .chain(g.shells.iter().map(|s| s.id.clone()))
                .chain(g.solids.iter().map(|s| s.id.clone()))
                .collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            let g = self.geometry;
            if g.vertices.iter().any(|v| v.id == id) {
                return Some(KIND_VERTEX.to_string());
            }
            if g.edges.iter().any(|e| e.id == id) {
                return Some(KIND_EDGE.to_string());
            }
            if g.wires.iter().any(|w| w.id == id) {
                return Some(KIND_WIRE.to_string());
            }
            if g.faces.iter().any(|f| f.id == id) {
                return Some(KIND_FACE.to_string());
            }
            if g.shells.iter().any(|s| s.id == id) {
                return Some(KIND_SHELL.to_string());
            }
            if g.solids.iter().any(|s| s.id == id) {
                return Some(KIND_SOLID.to_string());
            }
            None
        }

        fn node_name(&self, id: &str) -> Option<String> {
            // Brep entities have no separate display name distinct from their id.
            self.node_kind(id).map(|_| id.to_string())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let g = self.geometry;
            match key {
                "position" => g.vertices.iter().find(|v| v.id == id).map(|v| PropertyValue::Array(v.position.iter().map(|c| PropertyValue::Number(*c)).collect())),
                "curveKind" => g.edges.iter().find(|e| e.id == id).map(|e| PropertyValue::String(e.curve.kind.clone())),
                "surfaceKind" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::String(f.surface.kind.clone())),
                "normal" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::Array(f.surface.normal.iter().map(|c| PropertyValue::Number(*c)).collect())),
                _ => None,
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            let g = self.geometry;
            let mut out = Vec::new();
            let mut next_id = 0usize;
            let mut push = |kind: &str, source_node_id: String, target_node_id: String| {
                next_id += 1;
                out.push(QueryableEdge { id: format!("{kind}-{next_id}"), kind: kind.to_string(), source_node_id, target_node_id, source_port: None, target_port: None, properties: mathematical_graph_manifest::PropertyBag::default() });
            };
            for solid in &g.solids {
                for shell_id in &solid.shell_ids {
                    push(REL_BOUNDED_BY, solid.id.clone(), shell_id.clone());
                }
            }
            for shell in &g.shells {
                for face_id in &shell.face_ids {
                    push(REL_BOUNDED_BY, shell.id.clone(), face_id.clone());
                }
            }
            for face in &g.faces {
                for wire_id in &face.wire_ids {
                    push(REL_BOUNDED_BY, face.id.clone(), wire_id.clone());
                }
            }
            for wire in &g.wires {
                for edge_id in &wire.edge_ids {
                    push(REL_CONTAINS, wire.id.clone(), edge_id.clone());
                }
            }
            for edge in &g.edges {
                for vertex_id in &edge.vertex_ids {
                    push(REL_CONTAINS, edge.id.clone(), vertex_id.clone());
                }
            }
            out
        }

        fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            // Not needed for querying — this graph is read directly off `CadGeometry`, never
            // round-tripped through Jack's own fixture JSON format.
            None
        }
    }

    /// @emoji 🔍️ Runs a Jack query against one `CadGeometry` pane and returns its JSON result —
    /// the single entry point `cad-ui`/an MCP tool calls for topology queries (`saved selections`,
    /// non-manifold-edge checks, adjacency lookups), reusing `mathematical_graph_dsl::run_query_json`
    /// unchanged.
    pub fn run_construct_query(geometry: &CadGeometry, source: &str) -> Result<String, mathematical_graph_dsl::GraphDslError> {
        let graph = CadTopologyGraph::new(geometry);
        mathematical_graph_dsl::run_query_json(&graph, source)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use cad_document::{CadEdge, CadEdgeCurve, CadFace, CadPlaneSurface, CadShell, CadSolid, CadVertex, CadWire};

        /// 📦️ A closed box: 8 vertices, 12 edges, 6 wires, 6 faces, 1 shell, 1 solid — enough
        /// topology to exercise BOUNDED_BY/CONTAINS traversal across every dimension.
        fn box_geometry() -> CadGeometry {
            let corners: [[f64; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
            let vertices: Vec<CadVertex> = corners.iter().enumerate().map(|(i, p)| CadVertex { id: format!("v{i}"), position: *p }).collect();
            let edge_pairs: [(usize, usize); 12] = [
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0), // bottom
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4), // top
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7), // verticals
            ];
            let edges: Vec<CadEdge> = edge_pairs.iter().enumerate().map(|(i, (a, b))| CadEdge { id: format!("e{i}"), vertex_ids: vec![format!("v{a}"), format!("v{b}")], curve: CadEdgeCurve { kind: "line".into() } }).collect();
            let face_wire_edges: [[usize; 4]; 6] = [
                [0, 1, 2, 3],   // bottom
                [4, 5, 6, 7],   // top
                [0, 9, 4, 8],   // front
                [2, 11, 6, 10], // back
                [3, 8, 7, 11],  // left
                [1, 10, 5, 9],  // right
            ];
            let wires: Vec<CadWire> = face_wire_edges.iter().enumerate().map(|(i, es)| CadWire { id: format!("w{i}"), edge_ids: es.iter().map(|e| format!("e{e}")).collect() }).collect();
            let faces: Vec<CadFace> = (0..6).map(|i| CadFace { id: format!("f{i}"), wire_ids: vec![format!("w{i}")], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }).collect();
            let shell = CadShell { id: "s0".into(), face_ids: (0..6).map(|i| format!("f{i}")).collect() };
            let solid = CadSolid { id: "sol0".into(), shell_ids: vec!["s0".into()] };
            CadGeometry { anchors: Vec::new(), vertices, edges, wires, faces, shells: vec![shell], solids: vec![solid] }
        }

        #[test]
        fn topology_graph_exposes_every_entity_as_a_labeled_node() {
            let geometry = box_geometry();
            let graph = CadTopologyGraph::new(&geometry);
            assert_eq!(graph.node_kind("v0").as_deref(), Some(KIND_VERTEX));
            assert_eq!(graph.node_kind("e0").as_deref(), Some(KIND_EDGE));
            assert_eq!(graph.node_kind("w0").as_deref(), Some(KIND_WIRE));
            assert_eq!(graph.node_kind("f0").as_deref(), Some(KIND_FACE));
            assert_eq!(graph.node_kind("s0").as_deref(), Some(KIND_SHELL));
            assert_eq!(graph.node_kind("sol0").as_deref(), Some(KIND_SOLID));
            assert_eq!(graph.node_kind("nonexistent"), None);
            assert_eq!(graph.node_ids().len(), 8 + 12 + 6 + 6 + 1 + 1);
        }

        #[test]
        fn topology_graph_bounded_by_and_contains_edges_traverse_every_dimension() {
            let geometry = box_geometry();
            let graph = CadTopologyGraph::new(&geometry);
            let rel_edges = graph.edges();
            // Solid -[:BOUNDED_BY]-> Shell -[:BOUNDED_BY]-> Face -[:BOUNDED_BY]-> Wire -[:CONTAINS]-> Edge -[:CONTAINS]-> Vertex
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "sol0" && e.target_node_id == "s0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "s0" && e.target_node_id == "f0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "f0" && e.target_node_id == "w0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "w0" && e.target_node_id == "e0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "e0" && e.target_node_id == "v0"));
        }

        /// 🕸️ Runs a REAL Jack query — `MATCH (f:Face)-[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name`
        /// — against `CadTopologyGraph`, proving Jack's existing parser/executor answers a genuine
        /// TopoCypher-shaped question with zero new grammar, exactly as `construct.md` envisioned.
        #[test]
        fn construct_query_finds_every_face_bounded_by_its_wire() {
            let geometry = box_geometry();
            let json = run_construct_query(&geometry, "MATCH (f:Face)--[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name").expect("construct query must run");
            let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON result");
            let rows = value["rows"].as_array().expect("rows array");
            assert_eq!(rows.len(), 6, "every one of the 6 faces must match exactly its own wire: {json}");
        }

        #[test]
        fn construct_query_filters_edges_by_curve_kind_property() {
            let geometry = box_geometry();
            let json = run_construct_query(&geometry, "MATCH (e:Edge) WHERE e.curveKind = 'line' RETURN e.name").expect("construct query must run");
            let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON result");
            let rows = value["rows"].as_array().expect("rows array");
            assert_eq!(rows.len(), 12, "all 12 box edges are line curves: {json}");
        }

        #[test]
        fn construct_query_rejects_malformed_syntax_with_a_real_parse_error() {
            let geometry = box_geometry();
            let error = run_construct_query(&geometry, "NOT A QUERY (((").unwrap_err();
            let _ = error; // exists and is Err — the exact message is Jack's own concern, not this adapter's.
        }
    }
}
//#endregion 🔖️Construct

//#region 🔖️Config
/// 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe: cad's real `DocumentApp::Config` — absorbs
/// every field that used to live in `cad_ui::CadPlayRuntime` (an app-struct `RefCell`, never VCS'd)
/// plus the `ViewState::{active_utility_id,locale}` reads `cad_ui` used to take off the host-pushed
/// `ViewState` (deleted by B1) — session-only view state now round-trips through the config
/// `DocumentStore` exactly like document content, with a real `backwards` per
/// `cad_document_op::CadConfigOperation`. `cad_ui::CadPlayApp` keeps its own ergonomic
/// `CadPlayRuntime` mirror (reusing foreign, non-`dsl`-shaped types like `SelectionSet`/
/// `WorldSunConfig` verbatim) and converts at the `DocumentApp::handle`/`render` boundary — see
/// `cad_ui::{cad_runtime_from_config, cad_config_from_runtime}`.
use serde::{Deserialize, Serialize};

/// @emoji 🎯️ Ephemeral World3d hover target — object + optional component (edge/face/vertex). Moved
/// out of `cad_ui` (was private there) so `CadConfig` can embed it as a `#[dsl(block)]` field; every
/// field stays optional so the whole record round-trips through a still-empty hover state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadHoverTarget {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub id: Option<u32>,
}

/// @emoji 🎯️ Which geometry kinds World3d may pick; edges stay enabled so B-rep lines hover/select.
/// Moved out of `cad_ui` alongside `CadHoverTarget`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

impl Default for CadSelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: true, face: false }
    }
}

fn default_component_selection_mode() -> String {
    "mesh".into()
}

/// @emoji 🧩️ Component-level selection for World3d edge/face/vertex overlays. Moved out of `cad_ui`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadComponentSelection {
    #[serde(default)]
    #[dsl(block)]
    pub targets: CadSelectionTargets,
    #[serde(default = "default_component_selection_mode")]
    pub mode: String,
    #[serde(default)]
    pub ids: Vec<u32>,
}

impl Default for CadComponentSelection {
    fn default() -> Self {
        Self { targets: CadSelectionTargets::default(), mode: default_component_selection_mode(), ids: Vec::new() }
    }
}

/// 🎛️ Per-pane handle groups exposed by the Dislocate gumball utility — was keyed by an arbitrary
/// host-pushed `ViewState.window_id` (`cad_ui::CadPlayRuntime::dislocate_options_by_window_id`); the
/// pure `DocumentApp::render`/`window_measures` surface has no per-window-instance parameter anymore
/// (only `body_key`, which already resolves 1:1 to one of the 4 fixed CAD panes), so `CadConfig` keys
/// this by PANE instead — one named field per pane, mirroring `camera`/`camera_building`/…
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 🌞️ Local `dsl::DslRecord`-able mirror of `semio_framework_plugin::WorldSunConfig` (foreign,
/// out-of-scope crate — cannot gain a `dsl` derive there). `cad_sun_config_from_world`/
/// `cad_sun_config_to_world` convert at the boundary; field-for-field identical otherwise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for CadSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}

pub fn cad_sun_config_from_world(sun: &semio_framework_plugin::WorldSunConfig) -> CadSunConfig {
    CadSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

pub fn cad_sun_config_to_world(sun: &CadSunConfig) -> semio_framework_plugin::WorldSunConfig {
    semio_framework_plugin::WorldSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

/// 🧮️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: cad's real `DocumentApp::Config` — see the region doc
/// comment above for the full absorption story. `selected_object_ids` is a plain `Vec<String>` (not
/// `semio_framework_plugin::SelectionSet`, which is foreign and has no `dsl` derive); `cad_ui` still
/// uses the richer `SelectionSet` internally and converts at the boundary.
/// `engagement_session_json` is the pre-serialized JSON of `cad_document_engine::interaction::
/// CadEngagementSession` — that type's `context: HashMap<String, Value>` field has no `dsl` shape
/// (arbitrary JSON), so it round-trips as an opaque string rather than a nested `#[dsl(block)]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "cadcfg")]
#[dsl(layout = "lines")]
pub struct CadConfig {
    /// 👁️ Was `CadPlayRuntime::selected_object_ids` (`SelectionSet`).
    pub selected_object_ids: Vec<String>,
    /// 👁️ Was `CadPlayRuntime::selected_node_ids`.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Marquee selection method (`"rectangle"`/…) — was `CadPlayRuntime::selection_method`.
    pub selection_method: String,
    /// 👁️ Was `CadPlayRuntime::hovered_object_id`.
    pub hovered_object_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::hovered_target`.
    #[dsl(block)]
    pub hovered_target: Option<CadHoverTarget>,
    /// 👁️ Was `CadPlayRuntime::active_object_id`.
    pub active_object_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::component_selection`.
    #[dsl(block)]
    pub component_selection: CadComponentSelection,
    /// 👁️ Was `CadPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 👁️ Was `CadPlayRuntime::engagement_step`.
    pub engagement_step: String,
    /// 👁️ Was `CadPlayRuntime::active_example_id`.
    pub active_example_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_model_definition_id`.
    pub selected_reference_model_definition_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_id`.
    pub selected_reference_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_primitive_id`.
    pub selected_primitive_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_primitive_kind`.
    pub selected_primitive_kind: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_pane`.
    pub engagement_pane: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_session` (`Option<CadEngagementSession>`) — see the struct
    /// doc comment for why this is an opaque JSON string here.
    pub engagement_session_json: Option<String>,
    /// 👁️ Was `CadPlayRuntime::last_finalized_interaction_id`.
    pub last_finalized_interaction_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::sun` (`WorldSunConfig`).
    #[dsl(block)]
    pub sun: CadSunConfig,
    /// 🎥️ Per-pane camera pose — was `CadPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_building`.
    #[dsl(block)]
    pub camera_building: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_energy`.
    #[dsl(block)]
    pub camera_energy: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_structure_classic`.
    #[dsl(block)]
    pub camera_structure_classic: CadCamera,
    /// 🎛️ Was `CadPlayRuntime::dislocate_options_by_window_id.get(CAD_PLAY_WINDOW_SHAPE)` — see
    /// `CadDislocateOptions`'s doc comment for the per-window-id → per-pane simplification.
    #[dsl(block)]
    pub dislocate_shape: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_building: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_energy: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_structure_classic: CadDislocateOptions,
    /// 🧰️ The active transform-gumball utility — was read off `view_state.active_utility_id`
    /// (host-pushed `ViewState`, deleted by B1).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🗣️ Terminology id (`"native"`/`"reuse"`) — was read off `view_state.terminology`.
    pub terminology: String,
}

impl Default for CadConfig {
    fn default() -> Self {
        Self {
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            selection_method: "rectangle".into(),
            hovered_object_id: None,
            hovered_target: None,
            active_object_id: None,
            component_selection: CadComponentSelection::default(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            engagement_pane: None,
            engagement_session_json: None,
            last_finalized_interaction_id: None,
            sun: CadSunConfig::default(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_shape: CadDislocateOptions::default(),
            dislocate_building: CadDislocateOptions::default(),
            dislocate_energy: CadDislocateOptions::default(),
            dislocate_structure_classic: CadDislocateOptions::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            terminology: "native".into(),
        }
    }
}

impl store::ConfigRecord for CadConfig {}

/// @emoji 🧮️ Whole-record diff for `cad_document_op::CadConfigOperation` (lives here, not in
/// `cad_document_op`, since `protocol::OperationDiff`/`CadConfig` are both foreign to that crate — the
/// orphan rule requires at least one local type). Mirrors `CadOperation::SetScene`'s "whole-document
/// replace" pattern: `apply` ignores `base` entirely.
impl protocol::OperationDiff<CadConfig> for CadConfig {
    fn apply(&self, _base: &CadConfig) -> CadConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (`3d.cad`,
/// `ThreeD×Brep`) plus the two workflow ports the port recipe adds: `geometry:in` (accepts geometry
/// from any upstream 3D producer — `MediaForm::Any` only ever legal on the accepting side) and
/// `brep:out` (this app's own `3d.cad` kind, `Many` multiplicity so several downstream consumers can
/// each pull an independent export).
pub fn cad_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "cad.scene".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "geometry:in".into(),
                label: "Geometry".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Any },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_core::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "brep:out".into(),
                label: "Brep".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
                kind_id: Some("3d.cad".into()),
                required: false,
                multiplicity: semio_framework_core::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Glb],
        import_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.cad".into(), name: "3D CAD".into(), dimension: "3d".into(), component_kind: "cad".into() },
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn cad_config_default_matches_the_existing_runtime_defaults() {
        let config = CadConfig::default();
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.engagement_step, "Idle");
        assert_eq!(config.active_utility_id, "move");
        assert_eq!(config.locale, "en-US");
        assert!(config.dislocate_shape.move_enabled);
        assert!(config.dislocate_shape.rotate_enabled);
    }

    #[test]
    fn cad_config_dsl_round_trips_a_populated_record() {
        let mut config = CadConfig::default();
        config.selected_object_ids = vec!["object-1".into(), "object-2".into()];
        config.hovered_target = Some(CadHoverTarget { object_id: Some("object-1".into()), mode: Some("edge".into()), id: Some(3) });
        config.component_selection.mode = "face".into();
        config.component_selection.ids = vec![1, 2, 3];
        config.engagement_session_json = Some("{\"interactionId\":\"box\"}".into());
        config.camera.position = [1.0, 2.0, 3.0];
        config.active_utility_id = "rotate".into();
        config.locale = "de-DE".into();
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <CadConfig as store::DocumentDsl>::parse_dsl(&text).expect("cad config dsl parses");
        assert_eq!(parsed, config);
    }

    #[test]
    fn cad_config_pack_round_trips() {
        let mut config = CadConfig::default();
        config.selected_node_ids = vec!["node-1".into()];
        config.dislocate_building.rotate_enabled = false;
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <CadConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, config);
    }

    #[test]
    fn cad_sun_config_round_trips_through_world_sun_config() {
        let world = semio_framework_plugin::WorldSunConfig { enabled: true, azimuth: 12.0, elevation: 34.0, intensity: 0.5, color: "#112233".into() };
        let cad_sun = cad_sun_config_from_world(&world);
        let back = cad_sun_config_to_world(&cad_sun);
        assert_eq!(back, world);
    }

    #[test]
    fn cad_io_declares_geometry_in_and_brep_out_ports() {
        let io = cad_io();
        let ports = io.all_ports();
        let geometry_in = ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(geometry_in.multiplicity, semio_framework_core::PortMultiplicity::Many);
        assert_eq!(geometry_in.media_type.form, semio_framework_plugin::MediaForm::Any);
        let brep_out = ports.iter().find(|port| port.id == "brep:out").expect("brep:out declared");
        assert_eq!(brep_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(brep_out.kind_id.as_deref(), Some("3d.cad"));
        assert_eq!(brep_out.multiplicity, semio_framework_core::PortMultiplicity::Many);
    }
}
//#endregion 🔖️Config
