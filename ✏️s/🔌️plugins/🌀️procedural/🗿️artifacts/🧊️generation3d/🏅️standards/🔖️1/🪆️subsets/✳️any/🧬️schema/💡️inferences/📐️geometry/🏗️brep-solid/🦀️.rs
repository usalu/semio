//! 🏗️ `brep.solid` computes: extrusions, revolve, loft, sweep, pipe and helical sweep as stepped lane J operation jobs over a fresh kernel session.
//!
//! Profiles arrive as faces or closed wires (a wire is filled with the planar face of its own orientation); paths arrive as wires, edges or curves (an edge or a curve is read as a one-edge wire). Every refusal that the kernel would only report as an opaque failure is stated first, in English and German, at the port it concerns.

use super::brep_curve::{direction, dot, guarded, norm, path_wire, refusal};
use super::brep_surface::{fit_plane, profile_face, wire_corners, WIRE};
use super::phased_job::Pipeline;
use super::super::prelude::*;
use semio_framework_3d::brep::engine::BrepOperation;
use std::f64::consts::TAU;

fn extrude_wire(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let wire = inputs.shape("wire")?.clone();
        let vector = inputs.vector("vector")?;
        let along = direction(vector, "vector", "extrusion vector", "Der Extrusionsvektor")?;
        let outline = wire_corners(&wire).and_then(|corners| fit_plane(&corners, WIRE)).map_err(|fault| fault.at("wire"))?;
        if dot(outline.normal, along).abs() <= 1e-9 {
            return Err(refusal("extrusion-in-plane", "The extrusion vector lies in the plane of the wire and would sweep no volume.", "Der Extrusionsvektor liegt in der Ebene des Kantenzugs und würde kein Volumen überstreichen.").at("vector"));
        }
        Ok(Pipeline::new(kind).import(&wire).operation(move |work| Ok(BrepOperation::ExtrudeWire { wire: work.handle(0)?, vector })).exported("shape"))
    })
}

fn extrude_face(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let face = inputs.shape("face")?.clone();
        let vector = inputs.vector("vector")?;
        let along = direction(vector, "vector", "extrusion vector", "Der Extrusionsvektor")?;
        let distance = norm(vector);
        Ok(Pipeline::new(kind).import(&face).operation(move |work| Ok(BrepOperation::Extrude { face: work.handle(0)?, direction: along, distance })).exported("shape"))
    })
}

fn revolve(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let face = inputs.shape("face")?.clone();
        let axis_origin = inputs.point("axisOrigin")?;
        let axis_direction = direction(inputs.vector("axisDirection")?, "axisDirection", "axis direction", "Die Achsenrichtung")?;
        let angle = inputs.number("angle")?;
        if !angle.is_finite() || angle <= 0.0 || angle > TAU + 1e-9 {
            return Err(refusal("revolve-angle", "The revolution angle must be greater than zero and at most one full turn.", "Der Rotationswinkel muss größer als null und höchstens eine volle Drehung sein.").at("angle"));
        }
        Ok(Pipeline::new(kind).import(&face).operation(move |work| Ok(BrepOperation::Revolve { face: work.handle(0)?, axis_origin, axis_direction, angle: angle.min(TAU) }))
            .finish(|work| {
                let solid = work.result(0)?;
                let volume = work.session.brep().volume_sync(&solid).map_err(|error| kernel_fault(&error))?;
                if volume.abs() <= 1e-9 {
                    return Err(refusal("revolve-degenerate", "The profile touches or crosses the axis and sweeps no volume.", "Das Profil berührt oder schneidet die Achse und überstreicht kein Volumen.").at("face"));
                }
                Ok(outputs([("shape", work.export(&solid)?)]))
            }))
    })
}

fn loft(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let profiles: Vec<_> = inputs.shapes("profiles")?.into_iter().cloned().collect();
        let smooth = inputs.boolean("smooth")?;
        if profiles.len() < 2 {
            return Err(refusal("too-few-profiles", "A loft needs at least two profiles.", "Ein Ausformen braucht mindestens zwei Profile.").at("profiles"));
        }
        let count = profiles.len();
        let pipeline = profiles.iter().fold(Pipeline::new(kind), |pipeline, profile| pipeline.import(profile));
        Ok(pipeline
            .operation(move |work| {
                let faces = (0..count).map(|index| profile_face(work, index, "profiles")).collect::<Result<Vec<_>, _>>()?;
                Ok(BrepOperation::Loft { profiles: faces, smooth })
            })
            .exported("shape"))
    })
}

fn swept(kind: &Kind, inputs: &WidgetInputs) -> Result<Box<dyn WidgetJob>, WidgetFault> {
    let profile = inputs.shape("profile")?.clone();
    let path = path_wire(inputs.shape("path")?, "path")?;
    let guide = inputs.get("guide").map(|_| inputs.shape("guide").and_then(|guide| path_wire(guide, "guide"))).transpose()?;
    let has_guide = guide.is_some();
    let mut pipeline = Pipeline::new(kind).import(&profile).import(&path);
    if let Some(guide) = &guide {
        pipeline = pipeline.import(guide);
    }
    Ok(pipeline
        .operation(move |work| {
            let profile = profile_face(work, 0, "profile")?;
            let path = work.handle(1)?;
            if has_guide {
                Ok(BrepOperation::Pipe { profile, path, guide: Some(work.handle(2)?) })
            } else {
                Ok(BrepOperation::Sweep { profile, path })
            }
        })
        .exported("shape"))
}

fn sweep(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || swept(kind, &inputs))
}

fn pipe(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || swept(kind, &inputs))
}

fn helical_sweep(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let profile = inputs.shape("profile")?.clone();
        let axis_origin = inputs.point("axisOrigin")?;
        let axis_direction = direction(inputs.vector("axisDirection")?, "axisDirection", "axis direction", "Die Achsenrichtung")?;
        let (radius, pitch, turns) = (inputs.number("radius")?, inputs.number("pitch")?, inputs.number("turns")?);
        for (port, value, en, de) in [("radius", radius, "radius", "Der Radius"), ("pitch", pitch, "pitch", "Die Steigung"), ("turns", turns, "number of turns", "Die Windungszahl")] {
            if !value.is_finite() || value <= 0.0 {
                return Err(refusal("degenerate-size", format!("The {en} must be greater than zero."), format!("{de} muss größer als null sein.")).at(port));
            }
        }
        if turns > 1000.0 {
            return Err(refusal("degenerate-size", "A helical sweep has at most 1000 turns.", "Ein schraubenförmiges Austragen hat höchstens 1000 Windungen.").at("turns"));
        }
        Ok(Pipeline::new(kind)
            .import(&profile)
            .operation(move |work| {
                let profile = profile_face(work, 0, "profile")?;
                Ok(BrepOperation::HelicalSweep { profile, axis_origin, axis_direction, radius, pitch, turns })
            })
            .exported("shape"))
    })
}

/// 🗃️ The `brep.solid` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.solid.extrudeWire", start: extrude_wire },
    ComputeEntry { id: "brep.solid.extrudeFace", start: extrude_face },
    ComputeEntry { id: "brep.solid.revolve", start: revolve },
    ComputeEntry { id: "brep.solid.loft", start: loft },
    ComputeEntry { id: "brep.solid.sweep", start: sweep },
    ComputeEntry { id: "brep.solid.pipe", start: pipe },
    ComputeEntry { id: "brep.solid.helicalSweep", start: helical_sweep },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
