//! 🩹️ `mesh.repair` computes: weld, orient, fill holes, merge coplanar faces and decimate as stepped kernel mesh jobs.

use super::mesh_support::{modeling, owned_mesh, scalar, KernelResult};
use super::super::prelude::*;

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
