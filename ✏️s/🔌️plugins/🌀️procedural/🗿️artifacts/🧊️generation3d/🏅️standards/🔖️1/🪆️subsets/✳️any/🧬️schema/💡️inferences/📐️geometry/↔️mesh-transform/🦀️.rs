//! ↔️ `mesh.transform` computes: translate, rotate, scale, affine matrix and mirror as stepped kernel mesh jobs.
#![allow(clippy::result_large_err)]

use semio_framework_3d::mesh::MeshModelingOps;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_mesh_support::modeling;
use semio_s_artifact_procedural_3d_mesh_support::owned_mesh;
use semio_s_artifact_procedural_3d_mesh_support::scalar;
use semio_s_artifact_procedural_3d_mesh_support::vector3;
use semio_s_artifact_procedural_3d_mesh_support::KernelResult;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_framework_3d::mesh::MirrorAxis;

fn translate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.translate_job_owned(vector3(&inputs, "offset")?).kernel())
}

fn rotate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || owned_mesh(&inputs, "mesh")?.rotate_job_owned(vector3(&inputs, "axis")?, scalar(&inputs, "angle")?).kernel())
}

fn scale(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let factor = vector3(&inputs, "factor")?;
        let mirrored = factor.0.iter().filter(|axis| **axis < 0.0).count() % 2 == 1;
        owned_mesh(&inputs, "mesh")?.scale_job_owned(factor, mirrored).kernel()
    })
}

fn matrix(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let numbers = inputs.numbers("matrix")?;
        let columns: [f64; 16] = numbers.try_into().map_err(|rest: Vec<f64>| {
            WidgetFault::new("generation3d.geometry.input-items", format!("A 4x4 matrix needs 16 numbers, but {} are given.", rest.len()), format!("Eine 4x4-Matrix braucht 16 Zahlen, es sind aber {} gegeben.", rest.len())).at("matrix")
        })?;
        owned_mesh(&inputs, "mesh")?.affine_transform_job_owned(columns).kernel()
    })
}

fn mirror(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let axis = match inputs.text("axis")? {
            "x" => MirrorAxis::X,
            "y" => MirrorAxis::Y,
            "z" => MirrorAxis::Z,
            other => return Err(WidgetFault::new("generation3d.geometry.input-option", format!("The axis \u{201c}{other}\u{201d} is not x, y or z."), format!("Die Achse \u{201c}{other}\u{201d} ist weder x, y noch z.")).at("axis")),
        };
        owned_mesh(&inputs, "mesh")?.mirror_job_owned(axis, scalar(&inputs, "tolerance")?).kernel()
    })
}

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("mesh-transform", include_str!("../../../🗂️catalogue/🔣️mesh-transform.json"));
/// 🗃️ The `mesh.transform` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.transform.translate", start: translate },
    ComputeEntry { id: "mesh.transform.rotate", start: rotate },
    ComputeEntry { id: "mesh.transform.scale", start: scale },
    ComputeEntry { id: "mesh.transform.matrix", start: matrix },
    ComputeEntry { id: "mesh.transform.mirror", start: mirror },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
