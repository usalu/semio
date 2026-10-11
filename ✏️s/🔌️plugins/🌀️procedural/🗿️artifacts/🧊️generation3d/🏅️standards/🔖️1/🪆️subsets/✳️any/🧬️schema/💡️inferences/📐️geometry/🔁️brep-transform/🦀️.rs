//! 🔁️ The `brep.transform` widgets: rigid, scaling and mirroring motions of any solid, compound, face, wire, curve, surface or vertex, and the three patterns as stepped lane J operation jobs.
//!
//! A motion never edits its input: the input value is imported into a fresh kernel session and the moved copy is exported, so the input widget's value is shared untouched.

use super::brep_curve::guarded;
use super::phased_job::{Pipeline, Work};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{Brep, BrepError, BrepOperation, GeometryHandle, GeometryKind, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::representation::vector::matrix::Affine3;
use semio_framework_3d::brep::representation::vector::{Pnt3, Vec3};

const ZERO_VECTOR: &str = "generation3d.geometry.vector-zero";
const COUNT_RANGE: &str = "generation3d.geometry.input-range";

//#region 🔖️Motion
/// 🧭️ One affine motion of the catalogue, spelled in world coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Motion {
    Translate([f64; 3]),
    Rotate { origin: [f64; 3], axis: [f64; 3], angle: f64 },
    Scale { factors: [f64; 3], center: [f64; 3] },
    Mirror { origin: [f64; 3], normal: [f64; 3] },
    Copy,
}

fn point(value: [f64; 3]) -> Pnt3 {
    Pnt3::new(value[0], value[1], value[2])
}

fn vector(value: [f64; 3]) -> Vec3 {
    Vec3::new(value[0], value[1], value[2])
}

impl Motion {
    fn on(self, brep: &mut Brep, handle: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        match self {
            Self::Translate(offset) => brep.translate_sync(handle, offset),
            Self::Rotate { origin, axis, angle } => brep.rotate_about_sync(handle, origin, axis, angle),
            Self::Scale { factors, center } => brep.scale_axes_sync(handle, factors, center),
            Self::Mirror { origin, normal } => brep.mirror_sync(handle, origin, normal),
            Self::Copy => brep.copy_shape_sync(handle),
        }
    }

    fn affine(self) -> Affine3 {
        match self {
            Self::Translate(offset) => Affine3::translation(vector(offset)),
            Self::Rotate { origin, axis, angle } => Affine3::rotation_about(point(origin), vector(axis), angle),
            Self::Scale { factors, center } => Affine3::scaling(point(center), vector(factors)),
            Self::Mirror { origin, normal } => Affine3::mirror(point(origin), vector(normal)),
            Self::Copy => Affine3::IDENTITY,
        }
    }
}

fn moved_handle(work: &mut Work, motion: Motion, root: &ShapeRoot, source: &ShapeValue) -> Result<GeometryHandle, WidgetFault> {
    let handle = work.handle(0)?;
    let brep = work.session.brep();
    let kernel = |error: BrepError| kernel_fault(&error);
    match root {
        ShapeRoot::Vertex(id) => {
            let position = source.body.vertices.get(*id).map(|vertex| vertex.position).ok_or_else(|| kernel(BrepError::InvalidInput("vertex does not resolve".into())))?;
            let moved = motion.affine().apply_point(position);
            brep.vertex_sync([moved.x, moved.y, moved.z]).map_err(kernel)
        }
        ShapeRoot::Compound { .. } => {
            let members = brep.explode_sync(&handle).map_err(kernel)?;
            let moved = members.iter().map(|member| motion.on(brep, member)).collect::<Result<Vec<_>, _>>().map_err(kernel)?;
            brep.compound_sync(&moved).map_err(kernel)
        }
        _ => motion.on(brep, &handle).map_err(kernel),
    }
}

fn moved(kind: &Kind, inputs: &WidgetInputs, motion: impl FnOnce(&WidgetInputs) -> Result<Motion, WidgetFault>) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let motion = motion(inputs)?;
        let source = shape.clone();
        Ok(Pipeline::new(kind).import(&shape).once(move |work| {
            let result = moved_handle(work, motion, &source.root, &source)?;
            work.groups = vec![vec![result]];
            Ok(())
        }).exported("shape"))
    })
}

fn translate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |inputs| Ok(Motion::Translate(inputs.vector("offset")?)))
}

fn rotate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |inputs| Ok(Motion::Rotate { origin: [0.0; 3], axis: inputs.vector("axis")?, angle: inputs.number("angle")? }))
}

fn rotate_about(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |inputs| Ok(Motion::Rotate { origin: inputs.point("origin")?, axis: inputs.vector("axis")?, angle: inputs.number("angle")? }))
}

fn scale(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |inputs| Ok(Motion::Scale { factors: inputs.vector("factor")?, center: inputs.point("center")? }))
}

fn mirror(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |inputs| {
        let plane = inputs.plane("plane")?;
        Ok(Motion::Mirror { origin: plane.origin, normal: plane.normal })
    })
}

fn copy(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    moved(kind, &inputs, |_| Ok(Motion::Copy))
}
//#endregion 🔖️Motion

//#region 🔖️Pattern
fn direction(inputs: &WidgetInputs, port: &str) -> Result<[f64; 3], WidgetFault> {
    let axes = inputs.vector(port)?;
    let length = (axes[0] * axes[0] + axes[1] * axes[1] + axes[2] * axes[2]).sqrt();
    if !length.is_finite() || length <= 1e-12 {
        return Err(WidgetFault::new(ZERO_VECTOR, "The direction must not be the zero vector.", "Die Richtung darf nicht der Nullvektor sein.").at(port));
    }
    Ok([axes[0] / length, axes[1] / length, axes[2] / length])
}

fn count(inputs: &WidgetInputs, port: &str) -> Result<usize, WidgetFault> {
    usize::try_from(inputs.integer(port)?).ok().filter(|count| *count >= 1).ok_or_else(|| WidgetFault::new(COUNT_RANGE, "The count must be at least one.", "Die Anzahl muss mindestens eins sein.").at(port))
}

fn pattern<P: Send + 'static>(kind: &Kind, inputs: &WidgetInputs, read: impl FnOnce(&WidgetInputs) -> Result<P, WidgetFault>, plan: impl FnOnce(GeometryHandle, P) -> BrepOperation + Send + 'static) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let parameters = read(inputs)?;
        Ok(Pipeline::new(kind).import(&shape).operation(move |work| Ok(plan(work.handle(0)?, parameters))).exported("shape"))
    })
}

fn linear_pattern(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pattern(kind, &inputs, |inputs| Ok((direction(inputs, "direction")?, inputs.number("spacing")?, count(inputs, "count")?)), |shape, (direction, spacing, count)| BrepOperation::LinearPattern { shape, direction, spacing, count })
}

fn circular_pattern(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pattern(kind, &inputs, |inputs| Ok((direction(inputs, "axis")?, count(inputs, "count")?)), |shape, (axis, count)| BrepOperation::CircularPattern { shape, axis, count })
}

fn grid_pattern(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pattern(
        kind,
        &inputs,
        |inputs| Ok((direction(inputs, "dirX")?, direction(inputs, "dirY")?, inputs.number("spacingX")?, inputs.number("spacingY")?, count(inputs, "countX")?, count(inputs, "countY")?)),
        |shape, (dir_x, dir_y, spacing_x, spacing_y, count_x, count_y)| BrepOperation::GridPattern { shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y },
    )
}
//#endregion 🔖️Pattern

/// 🗃️ Every `brep.transform` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.transform.translate", start: translate },
    ComputeEntry { id: "brep.transform.rotate", start: rotate },
    ComputeEntry { id: "brep.transform.rotateAbout", start: rotate_about },
    ComputeEntry { id: "brep.transform.scale", start: scale },
    ComputeEntry { id: "brep.transform.mirror", start: mirror },
    ComputeEntry { id: "brep.transform.copy", start: copy },
    ComputeEntry { id: "brep.transform.linearPattern", start: linear_pattern },
    ComputeEntry { id: "brep.transform.circularPattern", start: circular_pattern },
    ComputeEntry { id: "brep.transform.gridPattern", start: grid_pattern },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
