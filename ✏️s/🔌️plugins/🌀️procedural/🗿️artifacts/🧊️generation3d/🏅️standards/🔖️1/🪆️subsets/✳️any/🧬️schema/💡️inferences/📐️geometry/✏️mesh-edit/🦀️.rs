//! ✏️ `mesh.edit` computes: bevel, dissolve, merge, loop cut, knife, extrude, inset, subdivide, flip, delete and triangulate as stepped kernel mesh jobs.
//!
//! Selections are mesh element ids validated against the input mesh; edge ids are half-edge handles.

use super::mesh_support::{count, modeling, owned_mesh, picked, picked_one, point3, scalar, Element, KernelResult};
use super::super::prelude::*;
use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId, WeldMode};

fn edges(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<EdgeId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Edge)?.into_iter().map(EdgeId).collect())
}

fn vertices(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<VertexId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Vertex)?.into_iter().map(VertexId).collect())
}

fn faces(inputs: &WidgetInputs, mesh: &semio_framework_3d::mesh::HalfedgeMesh, port: &str) -> Result<Vec<FaceId>, WidgetFault> {
    Ok(picked(inputs, port, mesh, Element::Face)?.into_iter().map(FaceId).collect())
}

fn bevel(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = edges(&inputs, &mesh, "edges")?;
        mesh.bevel_job_owned(&selected, scalar(&inputs, "amount")?, count(&inputs, "segments")?).kernel()
    })
}

fn dissolve_edges(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = edges(&inputs, &mesh, "edges")?;
        mesh.dissolve_edges_job_owned(&selected).kernel()
    })
}

fn dissolve_vertices(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = vertices(&inputs, &mesh, "vertices")?;
        mesh.dissolve_vertices_job_owned(&selected).kernel()
    })
}

fn merge_vertices(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = vertices(&inputs, &mesh, "vertices")?;
        let mode = match inputs.text("mode")? {
            "first" => WeldMode::First,
            "center" => WeldMode::Center,
            "distance" => WeldMode::ByDistance,
            other => return Err(WidgetFault::new("generation3d.geometry.input-option", format!("The merge mode \u{201c}{other}\u{201d} is not first, center or distance."), format!("Der Verschmelzmodus \u{201c}{other}\u{201d} ist weder first, center noch distance.")).at("mode")),
        };
        mesh.merge_vertices_job_owned(&selected, mode, scalar(&inputs, "tolerance")?).kernel()
    })
}

fn loop_cut(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = edges(&inputs, &mesh, "edges")?;
        mesh.loop_cut_job_owned(&selected, count(&inputs, "cuts")?).kernel()
    })
}

fn knife_cut(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let face = picked_one(&inputs, "face", &mesh, Element::Face)?;
        mesh.knife_cut_job_owned(FaceId(face), point3(&inputs, "start")?, point3(&inputs, "end")?).kernel()
    })
}

fn extrude(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = faces(&inputs, &mesh, "faces")?;
        mesh.extrude_faces_job_owned(&selected, scalar(&inputs, "distance")?).kernel()
    })
}

fn inset(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = faces(&inputs, &mesh, "faces")?;
        mesh.inset_faces_job_owned(&selected, scalar(&inputs, "amount")?).kernel()
    })
}

fn subdivide(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = faces(&inputs, &mesh, "faces")?;
        mesh.subdivide_faces_job_owned(&selected).kernel()
    })
}

fn flip(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = faces(&inputs, &mesh, "faces")?;
        mesh.flip_faces_job_owned(&selected).kernel()
    })
}

fn delete_faces(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let selected = faces(&inputs, &mesh, "faces")?;
        mesh.delete_faces_job_owned(&selected).kernel()
    })
}

fn triangulate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.triangulate_job_owned().kernel())
}

/// 🗃️ The `mesh.edit` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.edit.bevel", start: bevel },
    ComputeEntry { id: "mesh.edit.dissolveEdges", start: dissolve_edges },
    ComputeEntry { id: "mesh.edit.dissolveVertices", start: dissolve_vertices },
    ComputeEntry { id: "mesh.edit.mergeVertices", start: merge_vertices },
    ComputeEntry { id: "mesh.edit.loopCut", start: loop_cut },
    ComputeEntry { id: "mesh.edit.knifeCut", start: knife_cut },
    ComputeEntry { id: "mesh.edit.extrude", start: extrude },
    ComputeEntry { id: "mesh.edit.inset", start: inset },
    ComputeEntry { id: "mesh.edit.subdivide", start: subdivide },
    ComputeEntry { id: "mesh.edit.flip", start: flip },
    ComputeEntry { id: "mesh.edit.deleteFaces", start: delete_faces },
    ComputeEntry { id: "mesh.edit.triangulate", start: triangulate },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
