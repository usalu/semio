//! 🧊️ `brep.primitive` computes: kernel primitives built in a fresh session and exported as shape values.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_geometry_compute::finish;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::KernelSession;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::kernel_fault;
use semio_s_artifact_procedural_3d_geometry_value::outputs;
use semio_s_artifact_procedural_3d_geometry_value::GeometryValue;
use semio_s_artifact_procedural_3d_geometry_value::Outputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;

fn box_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (width, depth, height) = (inputs.number("width")?, inputs.number("depth")?, inputs.number("height")?);
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(width, depth, height).map_err(|error| kernel_fault(&error))?;
    Ok(outputs([("shape", GeometryValue::shape(session.export(&handle)?))]))
}

fn sphere_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let radius = inputs.number("radius")?;
    let mut session = KernelSession::new();
    let handle = session.brep().sphere_prim_sync(radius).map_err(|error| kernel_fault(&error))?;
    Ok(outputs([("shape", GeometryValue::shape(session.export(&handle)?))]))
}

fn box_(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, box_outputs(&inputs))
}

fn sphere(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, sphere_outputs(&inputs))
}

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("brep-primitive", include_str!("../../../🗂️catalogue/🔣️brep-primitive.json"));
/// 🗃️ The `brep.primitive` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "brep.primitive.box", start: box_ }, ComputeEntry { id: "brep.primitive.sphere", start: sphere }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
