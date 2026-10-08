//! 🌗️ `mesh.shading` computes: smooth or flat marks and the vertex normals rebuilt from them, as stepped kernel surface jobs.

use super::mesh_support::{modeling, owned_mesh, picked, Element, KernelResult};
use super::super::prelude::*;
use semio_framework_3d::mesh::FaceId;

fn set_shading(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let faces: Vec<FaceId> = picked(&inputs, "faces", &mesh, Element::Face)?.into_iter().map(FaceId).collect();
        mesh.set_shading_job_owned(&faces, inputs.boolean("smooth")?).kernel()
    })
}

fn recompute_normals(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.recompute_normals_job_owned().kernel())
}

/// 🗃️ The `mesh.shading` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.shading.setShading", start: set_shading }, ComputeEntry { id: "mesh.shading.recomputeNormals", start: recompute_normals }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
