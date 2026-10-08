//! 🧊️ `brep.primitive` computes: kernel primitives built in a fresh session and exported as shape values.

use super::super::prelude::*;

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

/// 🗃️ The `brep.primitive` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "brep.primitive.box", start: box_ }, ComputeEntry { id: "brep.primitive.sphere", start: sphere }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
