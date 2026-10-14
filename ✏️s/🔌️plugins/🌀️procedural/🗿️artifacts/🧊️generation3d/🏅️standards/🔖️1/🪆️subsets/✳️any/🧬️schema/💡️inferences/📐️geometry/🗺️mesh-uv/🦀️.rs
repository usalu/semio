//! 🗺️ `mesh.uv` computes: seam marks and the least squares conformal unwrap along them, as stepped kernel surface jobs.
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
use semio_framework_3d::mesh::EdgeId;

fn mark_seams(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let mesh = owned_mesh(&inputs, "mesh")?;
        let edges: Vec<EdgeId> = picked(&inputs, "edges", &mesh, Element::Edge)?.into_iter().map(EdgeId).collect();
        mesh.set_uv_seams_job_owned(&edges, inputs.boolean("seam")?).kernel()
    })
}

fn unwrap(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.unwrap_uv_job_owned().kernel())
}

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("mesh-uv", include_str!("../../../🗂️catalogue/🔣️mesh-uv.json"));
/// 🗃️ The `mesh.uv` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.uv.markSeams", start: mark_seams }, ComputeEntry { id: "mesh.uv.unwrap", start: unwrap }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
