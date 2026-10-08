//! 🗺️ `mesh.uv` computes: seam marks and the least squares conformal unwrap along them, as stepped kernel surface jobs.

use super::mesh_support::{modeling, owned_mesh, picked, Element, KernelResult};
use super::super::prelude::*;
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

/// 🗃️ The `mesh.uv` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.uv.markSeams", start: mark_seams }, ComputeEntry { id: "mesh.uv.unwrap", start: unwrap }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
