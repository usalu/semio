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
                extent: None,
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
