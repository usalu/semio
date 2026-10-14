//! 🩹️ `mesh.repair` computes: weld, orient, fill holes, merge coplanar faces and decimate as stepped kernel mesh jobs.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_mesh_support::modeling;
use semio_s_artifact_procedural_3d_mesh_support::owned_mesh;
use semio_s_artifact_procedural_3d_mesh_support::scalar;
use semio_s_artifact_procedural_3d_mesh_support::KernelResult;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;

fn weld(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.weld_coincident_vertices_job_owned(scalar(&inputs, "tolerance")?).kernel())
}

fn orient(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.orient_faces_job_owned().kernel())
}

fn fill_holes(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.fill_holes_job_owned().kernel())
}

fn merge_coplanar(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.merge_coplanar_faces_job_owned().kernel())
}

fn decimate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.decimate_job_owned(scalar(&inputs, "ratio")?).kernel())
}

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("mesh-repair", include_str!("../../../🗂️catalogue/🔣️mesh-repair.json"));
/// 🗃️ The `mesh.repair` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.repair.weld", start: weld },
    ComputeEntry { id: "mesh.repair.orient", start: orient },
    ComputeEntry { id: "mesh.repair.fillHoles", start: fill_holes },
    ComputeEntry { id: "mesh.repair.mergeCoplanar", start: merge_coplanar },
    ComputeEntry { id: "mesh.repair.decimate", start: decimate },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
