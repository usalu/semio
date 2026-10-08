//! ➡️ The math.vector widget computes: vectors, points and planes built from numbers, taken apart, combined and measured.
//!
//! Directions are never silently invented: the zero vector has no direction (`zero-vector`), three points on a line span no plane
//! (`plane-degenerate`), and a non-finite component is refused (`math-overflow`). A plane value always carries a unit normal.
//!
//! 🔗️ [Euclidean vector](https://en.wikipedia.org/wiki/Euclidean_vector) · [Plane](https://en.wikipedia.org/wiki/Plane_(geometry))

use super::math_arithmetic::finite;
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;

type Triple = [f64; 3];

const COLLINEAR_TOLERANCE: f64 = 1e-12;

//#region 🔖️Algebra
/// ➕️ Component-wise sum.
pub(crate) fn add(a: Triple, b: Triple) -> Triple {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ➖️ Component-wise difference `a - b`.
pub(crate) fn sub(a: Triple, b: Triple) -> Triple {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ✖️ Scales every component by `factor`.
pub(crate) fn scale(a: Triple, factor: f64) -> Triple {
    [a[0] * factor, a[1] * factor, a[2] * factor]
}

/// 🔵️ The dot product.
pub(crate) fn dot(a: Triple, b: Triple) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// ❌️ The cross product `a x b`.
pub(crate) fn cross(a: Triple, b: Triple) -> Triple {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 📏️ The Euclidean length, free of intermediate overflow.
pub(crate) fn length(a: Triple) -> f64 {
    a[0].hypot(a[1]).hypot(a[2])
}

/// 🧭️ The unit vector of `a`, or `None` for the zero vector.
pub(crate) fn unit(a: Triple) -> Option<Triple> {
    let size = length(a);
    (size > 0.0 && size.is_finite()).then(|| scale(a, 1.0 / size))
}

/// 📐️ The angle between two non-zero vectors in `0..=pi`, stable for nearly parallel and nearly opposite vectors.
pub(crate) fn angle_between(a: Triple, b: Triple) -> Option<f64> {
    let (a, b) = (unit(a)?, unit(b)?);
    Some(length(cross(a, b)).atan2(dot(a, b)))
}
//#endregion 🔖️Algebra

//#region 🔖️Faults
/// 🚫️ The zero vector has no direction, attributed to the offending port.
pub(crate) fn zero_vector(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.zero-vector", "The vector has length zero and therefore no direction.", "Der Vektor hat die Länge null und damit keine Richtung.").at(port)
}

/// 🛡️ The triple itself, or `math-overflow` when a component is not finite.
pub(crate) fn finite_triple(value: Triple) -> Result<Triple, WidgetFault> {
    for component in value {
        finite(component)?;
    }
    Ok(value)
}
//#endregion 🔖️Faults

//#region 🔖️Computes
fn vector_from_components(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector([inputs.number("x")?, inputs.number("y")?, inputs.number("z")?]))])))())
}

fn point_from_components(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("point", GeometryValue::Point([inputs.number("x")?, inputs.number("y")?, inputs.number("z")?]))])))())
}

/// ✂️ The three components of a triple as the `x`, `y` and `z` outputs.
fn components(triple: Triple) -> Outputs {
    outputs([("x", GeometryValue::Number(triple[0])), ("y", GeometryValue::Number(triple[1])), ("z", GeometryValue::Number(triple[2]))])
}

fn vector_components(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(components(inputs.vector("vector")?)))())
}

fn point_components(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(components(inputs.point("point")?)))())
}

fn vector_add(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(finite_triple(add(inputs.vector("a")?, inputs.vector("b")?))?))])))())
}

fn vector_subtract(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(finite_triple(sub(inputs.vector("a")?, inputs.vector("b")?))?))])))())
}

fn vector_scale(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(finite_triple(scale(inputs.vector("vector")?, inputs.number("factor")?))?))])))())
}

fn vector_length(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("length", GeometryValue::Number(finite(length(inputs.vector("vector")?))?))])))())
}

fn vector_normalize(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(unit(inputs.vector("vector")?).ok_or_else(|| zero_vector("vector"))?))])))())
}

fn vector_dot(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("result", GeometryValue::Number(finite(dot(inputs.vector("a")?, inputs.vector("b")?))?))])))())
}

fn vector_cross(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(finite_triple(cross(inputs.vector("a")?, inputs.vector("b")?))?))])))())
}

fn vector_angle(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (a, b) = (inputs.vector("a")?, inputs.vector("b")?);
            if unit(a).is_none() {
                return Err(zero_vector("a"));
            }
            Ok(outputs([("angle", GeometryValue::Number(angle_between(a, b).ok_or_else(|| zero_vector("b"))?))]))
        })(),
    )
}

fn point_offset(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("point", GeometryValue::Point(finite_triple(add(inputs.point("point")?, inputs.vector("offset")?))?))])))())
}

fn point_distance(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("distance", GeometryValue::Number(finite(length(sub(inputs.point("a")?, inputs.point("b")?)))?))])))())
}

fn point_interpolate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (a, b, t) = (inputs.point("a")?, inputs.point("b")?, inputs.number("t")?);
            Ok(outputs([("point", GeometryValue::Point(finite_triple(add(a, scale(sub(b, a), t)))?))]))
        })(),
    )
}

fn vector_between(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("vector", GeometryValue::Vector(finite_triple(sub(inputs.point("to")?, inputs.point("from")?))?))])))())
}

fn plane_from_point_normal(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (origin, normal) = (inputs.point("origin")?, inputs.vector("normal")?);
            let normal = unit(normal).ok_or_else(|| zero_vector("normal"))?;
            Ok(outputs([("plane", GeometryValue::Plane(PlaneValue { origin, normal }))]))
        })(),
    )
}

fn plane_from_points(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (a, b, c) = (inputs.point("a")?, inputs.point("b")?, inputs.point("c")?);
            let (ab, ac) = (sub(b, a), sub(c, a));
            let spanned = cross(ab, ac);
            let degenerate = || WidgetFault::new("generation3d.geometry.plane-degenerate", "The three points lie on one line and span no plane.", "Die drei Punkte liegen auf einer Geraden und spannen keine Ebene auf.").at("c");
            let size = length(spanned);
            if size <= COLLINEAR_TOLERANCE * length(ab) * length(ac) || !size.is_finite() {
                return Err(degenerate());
            }
            Ok(outputs([("plane", GeometryValue::Plane(PlaneValue { origin: a, normal: scale(spanned, 1.0 / size) }))]))
        })(),
    )
}

fn plane_components(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let plane = inputs.plane("plane")?;
            Ok(outputs([("origin", GeometryValue::Point(plane.origin)), ("normal", GeometryValue::Vector(plane.normal))]))
        })(),
    )
}
//#endregion 🔖️Computes

/// 🗃️ Every math.vector catalogue kind and the compute that starts it.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "math.vectorFromComponents", start: vector_from_components },
    ComputeEntry { id: "math.pointFromComponents", start: point_from_components },
    ComputeEntry { id: "math.vectorComponents", start: vector_components },
    ComputeEntry { id: "math.pointComponents", start: point_components },
    ComputeEntry { id: "math.vectorAdd", start: vector_add },
    ComputeEntry { id: "math.vectorSubtract", start: vector_subtract },
    ComputeEntry { id: "math.vectorScale", start: vector_scale },
    ComputeEntry { id: "math.vectorLength", start: vector_length },
    ComputeEntry { id: "math.vectorNormalize", start: vector_normalize },
    ComputeEntry { id: "math.vectorDot", start: vector_dot },
    ComputeEntry { id: "math.vectorCross", start: vector_cross },
    ComputeEntry { id: "math.vectorAngle", start: vector_angle },
    ComputeEntry { id: "math.pointOffset", start: point_offset },
    ComputeEntry { id: "math.pointDistance", start: point_distance },
    ComputeEntry { id: "math.pointInterpolate", start: point_interpolate },
    ComputeEntry { id: "math.vectorBetween", start: vector_between },
    ComputeEntry { id: "math.planeFromPointNormal", start: plane_from_point_normal },
    ComputeEntry { id: "math.planeFromPoints", start: plane_from_points },
    ComputeEntry { id: "math.planeComponents", start: plane_components },
];

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
