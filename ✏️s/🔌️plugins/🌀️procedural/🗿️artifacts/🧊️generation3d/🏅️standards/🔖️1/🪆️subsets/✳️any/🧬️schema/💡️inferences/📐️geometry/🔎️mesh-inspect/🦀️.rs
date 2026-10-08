//! 🔎️ `mesh.inspect` computes: the position of a vertex, the ends and length of an edge and the corners, normal and center of a face, read without changing the mesh.
//!
//! Each reads one element, so each is a single cheap unit.

use super::mesh_support::{picked_one, Element, KernelResult};
use super::super::prelude::*;
use semio_framework_3d::mesh::{EdgeId, FaceId, MeshKernelError, VertexId};

fn axes(point: semio_framework_3d::mesh::Vec3) -> [f64; 3] {
    point.0.map(f64::from)
}

fn vertex_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let mesh = inputs.mesh("mesh")?;
    let id = picked_one(inputs, "vertex", mesh, Element::Vertex)?;
    Ok(outputs([("position", GeometryValue::Point(axes(mesh.vertex_position(VertexId(id)).kernel()?)))]))
}

fn edge_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let mesh = inputs.mesh("mesh")?;
    let id = picked_one(inputs, "edge", mesh, Element::Edge)?;
    let (from, to) = mesh.edge_endpoints(EdgeId(id)).kernel()?;
    let (start, end) = (axes(mesh.vertex_position(from).kernel()?), axes(mesh.vertex_position(to).kernel()?));
    let length = (0..3).map(|axis| (end[axis] - start[axis]).powi(2)).sum::<f64>().sqrt();
    Ok(outputs([("start", GeometryValue::Point(start)), ("end", GeometryValue::Point(end)), ("length", GeometryValue::Number(length))]))
}

fn face_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let mesh = inputs.mesh("mesh")?;
    let id = picked_one(inputs, "face", mesh, Element::Face)?;
    let corners = mesh.face_vertex_ids(FaceId(id)).kernel()?;
    let normal = axes(mesh.face_normal(FaceId(id)).kernel()?);
    if normal.iter().all(|axis| *axis == 0.0) {
        return Err(super::mesh_support::mesh_fault(&MeshKernelError::DegenerateOperation).at("face"));
    }
    let mut center = [0.0f64; 3];
    for corner in &corners {
        let point = axes(mesh.vertex_position(*corner).kernel()?);
        for axis in 0..3 {
            center[axis] += point[axis] / corners.len() as f64;
        }
    }
    let indices = corners.iter().map(|corner| GeometryValue::Integer(i64::from(corner.0))).collect();
    Ok(outputs([("vertices", GeometryValue::List(indices)), ("normal", GeometryValue::Vector(normal)), ("center", GeometryValue::Point(center))]))
}

fn vertex(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, vertex_outputs(&inputs))
}

fn edge(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, edge_outputs(&inputs))
}

fn face(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, face_outputs(&inputs))
}

/// 🗃️ The `mesh.inspect` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.inspect.vertex", start: vertex }, ComputeEntry { id: "mesh.inspect.edge", start: edge }, ComputeEntry { id: "mesh.inspect.face", start: face }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
