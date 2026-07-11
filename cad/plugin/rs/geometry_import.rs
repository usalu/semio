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
    kernel: &mut BrepkitKernel,
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
        if let Ok(handle) = kernel.polyline_wire_sync(&points) {
            handles.insert(wire.id.clone(), handle.0.clone());
        }
    }

    for face in &geometry.faces {
        if let Some(wire_id) = face.wire_ids.first() {
            if let Some(wire_handle) = handles.get(wire_id) {
                let wire = GeometryHandle(wire_handle.clone());
                if let Ok(handle) = kernel.planar_face_from_wire_sync(&wire) {
                    handles.insert(face.id.clone(), handle.0.clone());
                    continue;
                }
                if let Ok(handle) = kernel.face_from_wire_sync(&wire) {
                    handles.insert(face.id.clone(), handle.0.clone());
                    continue;
                }
            }
        }
        let points = face_boundary_points(face, &wires, &edges, &vertices);
        if points.len() < 3 {
            continue;
        }
        if let Ok(handle) = kernel.planar_face_from_points_sync(&points) {
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
        if let Ok(solid) = kernel.sew_faces_sync(&face_handles, 0.01) {
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
        if let Ok(built) = kernel.sew_faces_sync(&face_handles, 0.01) {
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
    kernel: &mut BrepkitKernel,
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

fn curve_mesh_from_wire(kernel: &mut BrepkitKernel, wire: &GeometryHandle) -> Option<MeshData> {
    let profile_wire = kernel.regular_polygon_wire_sync(0.08, 8).ok()?;
    let profile_face = kernel.planar_face_from_wire_sync(&profile_wire).ok()?;
    let solid = kernel.sweep_sync(&profile_face, wire).ok()?;
    let mesh = block_on(kernel.tessellate(&solid, 0.1)).ok()?;
    let _ = kernel.dispose_sync(&solid);
    let _ = kernel.dispose_sync(&profile_face);
    let _ = kernel.dispose_sync(&profile_wire);
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
    kernel: &mut BrepkitKernel,
    id: impl Into<String>,
    label: impl Into<String>,
    typology: impl Into<String>,
    mesh: &MeshData,
) -> CadObject {
    let extent = mesh_extent(mesh);
    let solid_handle = if mesh.indices.len() >= 3 {
        kernel.import_obj_sync(&mesh_to_obj_text(mesh), 0.01).ok().map(|handle| handle.0)
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
//#endregion 🔖MeshImport

pub fn object_label_from_id(object_id: &str) -> String {
    object_id
        .split('-')
        .last()
        .map(str::to_string)
        .unwrap_or_else(|| object_id.to_string())
}

pub fn objects_from_fixture_model(
    kernel: &mut BrepkitKernel,
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
