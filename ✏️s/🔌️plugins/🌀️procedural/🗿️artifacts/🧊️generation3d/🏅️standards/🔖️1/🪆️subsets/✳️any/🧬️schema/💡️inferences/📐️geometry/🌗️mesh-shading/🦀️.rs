//! 🌗️ `mesh.shading` computes: smooth or flat marks and the vertex normals rebuilt from them, as stepped kernel surface jobs.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_mesh_support::modeling;
use semio_s_artifact_procedural_3d_mesh_support::owned_mesh;
use semio_s_artifact_procedural_3d_mesh_support::picked;
use semio_s_artifact_procedural_3d_mesh_support::Element;
use semio_s_artifact_procedural_3d_mesh_support::KernelResult;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
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

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("mesh-shading", include_str!("../../../🗂️catalogue/🔣️mesh-shading.json"));
/// 🗃️ The `mesh.shading` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.shading.setShading", start: set_shading }, ComputeEntry { id: "mesh.shading.recomputeNormals", start: recompute_normals }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
