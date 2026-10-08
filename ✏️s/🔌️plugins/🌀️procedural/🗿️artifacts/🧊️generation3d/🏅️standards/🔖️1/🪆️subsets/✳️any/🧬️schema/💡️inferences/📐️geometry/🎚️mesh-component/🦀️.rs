//! 🎚️ `mesh.component` computes: translate, rotate, scale, proportional move and grid snap of selected vertices, edges or faces as stepped kernel mesh jobs.
//!
//! Selections are mesh element ids validated against the input mesh; edge ids are half-edge handles. Shared vertices of a selection move once.

use super::mesh_support::{mode_element, owned_mesh, picked, point3, modeling, scalar, vector3, Element, KernelResult};
use super::super::prelude::*;
use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};

type Components = (Vec<VertexId>, Vec<EdgeId>, Vec<FaceId>);

fn components(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Components, WidgetFault> {
    let element = mode_element(inputs, "mode")?;
    let ids = picked(inputs, port, mesh, element)?;
    Ok(match element {
        Element::Vertex => (ids.into_iter().map(VertexId).collect(), Vec::new(), Vec::new()),
        Element::Edge => (Vec::new(), ids.into_iter().map(EdgeId).collect(), Vec::new()),
        Element::Face => (Vec::new(), Vec::new(), ids.into_iter().map(FaceId).collect()),
    })
}

fn pivot(inputs: &WidgetInputs) -> Result<Option<semio_framework_3d::mesh::Vec3>, WidgetFault> {
    match inputs.text("pivot")? {
        "selection" => Ok(None),
        "point" => point3(inputs, "center").map(Some),
        other => Err(WidgetFault::new("generation3d.geometry.input-option", format!("The pivot \u{201c}{other}\u{201d} is not selection or point."), format!("Der Drehpunkt \u{201c}{other}\u{201d} ist weder selection noch point.")).at("pivot")),
    }
}

fn translate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let (vertices, edges, faces) = components(&inputs, &mesh, "selection")?;
        mesh.move_components_job_owned(vertices, edges, faces, vector3(&inputs, "offset")?).kernel()
    })
}

fn rotate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let (vertices, edges, faces) = components(&inputs, &mesh, "selection")?;
        mesh.rotate_components_job_owned(vertices, edges, faces, vector3(&inputs, "axis")?, scalar(&inputs, "angle")?, pivot(&inputs)?).kernel()
    })
}

fn scale(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let (vertices, edges, faces) = components(&inputs, &mesh, "selection")?;
        mesh.scale_components_job_owned(vertices, edges, faces, vector3(&inputs, "factor")?, pivot(&inputs)?).kernel()
    })
}

fn move_vertices(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let vertices = picked(&inputs, "vertices", &mesh, Element::Vertex)?.into_iter().map(VertexId).collect();
        mesh.move_components_job_owned(vertices, Vec::new(), Vec::new(), vector3(&inputs, "offset")?).kernel()
    })
}

fn move_proportional(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let vertices = picked(&inputs, "vertices", &mesh, Element::Vertex)?.into_iter().map(VertexId).collect();
        mesh.move_proportional_job_owned(vertices, vector3(&inputs, "offset")?, point3(&inputs, "center")?, scalar(&inputs, "radius")?).kernel()
    })
}

fn snap_to_grid(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let vertices = picked(&inputs, "vertices", &mesh, Element::Vertex)?.into_iter().map(VertexId).collect();
        mesh.snap_vertices_job_owned(vertices, scalar(&inputs, "grid")?).kernel()
    })
}

/// 🗃️ The `mesh.component` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.component.moveVertices", start: move_vertices },
    ComputeEntry { id: "mesh.component.translate", start: translate },
    ComputeEntry { id: "mesh.component.rotate", start: rotate },
    ComputeEntry { id: "mesh.component.scale", start: scale },
    ComputeEntry { id: "mesh.component.moveProportional", start: move_proportional },
    ComputeEntry { id: "mesh.component.snapToGrid", start: snap_to_grid },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
