//! 〰️ `brep.curve` computes: lines, arcs, circles, ellipses, splines, polygons and helices built in a fresh kernel session (polylines at value level) and exported as shape values.
//!
//! Besides the ten computes this module owns the value-level views the profile and path inputs of the surface and solid computes share: a polyline built directly as a wire value and a curve or an edge read as a one-edge wire.

use super::super::prelude::*;
use super::super::value::FAULT_PREFIX;
use semio_framework_3d::brep::engine::{Brep, BrepError, GeometryHandle, ShapeRoot, ShapeValue, ShapeWire};
use semio_framework_3d::brep::operations::euler::{make_edge, make_vertex};
use semio_framework_3d::brep::operations::primitives::make_polyline_wire;
use semio_framework_3d::brep::representation::curve::Curve3;
use semio_framework_3d::brep::representation::tolerance::Tol;
use semio_framework_3d::brep::representation::vector::matrix::Frame3;
use semio_framework_3d::brep::representation::topology::history::OpRecorder;
use semio_framework_3d::brep::representation::topology::Body;
use semio_framework_3d::brep::representation::vector::{Pnt3, Vec3 as KernelVec3};
use std::f64::consts::TAU;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

//#region 🔖️Vocabulary
/// 🚫️ A domain refusal `generation3d.geometry.<code>` with English and German text.
pub fn refusal(code: &str, en: impl Into<String>, de: impl Into<String>) -> WidgetFault {
    WidgetFault::new(format!("{FAULT_PREFIX}{code}"), en, de)
}

/// 📏️ The length of a vector.
pub fn norm(vector: [f64; 3]) -> f64 {
    vector.iter().map(|axis| axis * axis).sum::<f64>().sqrt()
}

/// ➖️ The difference of two points.
pub fn minus(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ✖️ The cross product.
pub fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// ⚫️ The dot product.
pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 🧭️ The unit vector, or a refusal at `port` for a zero or non-finite vector.
pub fn direction(vector: [f64; 3], port: &str, en: &str, de: &str) -> Result<[f64; 3], WidgetFault> {
    let length = norm(vector);
    if !length.is_finite() || length <= 1e-12 {
        return Err(refusal("degenerate-direction", format!("The {en} must not be the zero vector."), format!("{de} darf nicht der Nullvektor sein.")).at(port));
    }
    Ok([vector[0] / length, vector[1] / length, vector[2] / length])
}

fn positive(value: f64, port: &str, en: &str, de: &str) -> Result<f64, WidgetFault> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(refusal("degenerate-size", format!("The {en} must be greater than zero."), format!("{de} muss größer als null sein.")).at(port))
    }
}

/// 📍️ The points of a point-list port.
pub fn points(inputs: &WidgetInputs, port: &str) -> Result<Vec<[f64; 3]>, WidgetFault> {
    inputs
        .list(port)?
        .iter()
        .map(|item| match item {
            GeometryValue::Point(point) | GeometryValue::Vector(point) => Ok(*point),
            _ => Err(WidgetFault::new(format!("{FAULT_PREFIX}input-type"), "The list holds an item that is not a point.", "Die Liste enthält ein Element, das kein Punkt ist.").at(port)),
        })
        .collect()
}

/// 🔁️ Whether two points coincide at kernel resolution.
pub fn coincide(a: [f64; 3], b: [f64; 3]) -> bool {
    norm(minus(a, b)) <= 1e-9
}

/// 🛡️ Starts the job a compute builds, or a finished job holding the refusal that stopped it from being built.
pub fn guarded(kind: &Kind, build: impl FnOnce() -> Result<Box<dyn WidgetJob>, WidgetFault>) -> Box<dyn WidgetJob> {
    build().unwrap_or_else(|fault| failed(fault, kind.quality))
}

/// 🛡️ Runs a cheap compute, turning a panic of the kernel on degenerate geometry into a `generation3d.geometry.kernel` fault.
pub fn unwound(compute: impl FnOnce() -> Result<Outputs, WidgetFault>) -> Result<Outputs, WidgetFault> {
    catch_unwind(AssertUnwindSafe(compute)).unwrap_or_else(|_| Err(refusal("kernel", "The geometry kernel stopped unexpectedly on this input.", "Der Geometriekern wurde bei dieser Eingabe unerwartet beendet.")))
}

/// 🧭️ The local X axis of a plane given by its unit normal: the world X axis projected into the plane, the world Y axis when the normal lies along X.
pub fn local_x(normal: [f64; 3]) -> [f64; 3] {
    let reference = if normal[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let along = dot(reference, normal);
    let projected = [reference[0] - normal[0] * along, reference[1] - normal[1] * along, reference[2] - normal[2] * along];
    let length = norm(projected);
    [projected[0] / length, projected[1] / length, projected[2] / length]
}

/// 🔁️ The angle, counter-clockwise about `normal`, from the kernel's own frame axis of that normal to [`local_x`].
fn frame_phase(normal: [f64; 3]) -> Result<f64, WidgetFault> {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), KernelVec3::new(normal[0], normal[1], normal[2])).ok_or_else(|| refusal("degenerate-direction", "The normal must not be the zero vector.", "Die Normale darf nicht der Nullvektor sein.").at("normal"))?;
    let (x, y) = ([frame.x.x, frame.x.y, frame.x.z], [frame.y.x, frame.y.y, frame.y.z]);
    let wanted = local_x(normal);
    Ok(dot(wanted, y).atan2(dot(wanted, x)))
}

fn export_shape(session: &KernelSession, handle: &GeometryHandle) -> Result<Outputs, WidgetFault> {
    Ok(outputs([("shape", GeometryValue::shape(session.export(handle)?))]))
}

fn build(make: impl FnOnce(&mut Brep) -> Result<GeometryHandle, BrepError>) -> Result<Outputs, WidgetFault> {
    let mut session = KernelSession::new();
    let handle = make(session.brep()).map_err(|error| kernel_fault(&error))?;
    export_shape(&session, &handle)
}
//#endregion 🔖️Vocabulary

//#region 🔖️Wires
fn kernel_error(error: impl std::fmt::Display) -> WidgetFault {
    kernel_fault(&BrepError::InvalidInput(error.to_string()))
}

/// 🧵️ The wire value through `points`; a closed wire joins the last point back to the first.
pub fn polyline_value(points: &[[f64; 3]], closed: bool) -> Result<ShapeValue, WidgetFault> {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let corners: Vec<Pnt3> = points.iter().map(|point| Pnt3::new(point[0], point[1], point[2])).collect();
    let wire = make_polyline_wire(&mut body, &corners, closed, &mut rec).map_err(kernel_error)?;
    let label = body.new_label();
    Ok(ShapeValue { root: ShapeRoot::Wire(ShapeWire { members: wire.members, vertices: wire.vertices, closed: wire.closed, label }), body })
}

/// 〰️ A curve as a one-edge wire over its natural range: a line spans its segment `[0, 1]`, a periodic curve one period, a spline its knot domain.
fn curve_wire(curve: &Curve3) -> Result<ShapeValue, WidgetFault> {
    let range = match curve {
        Curve3::Line { .. } => (0.0, 1.0),
        Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, TAU),
        Curve3::Nurbs { knots, .. } => knots.domain(),
    };
    let (first, last) = (curve.eval(range.0), curve.eval(range.1));
    if ![first.x, first.y, first.z, last.x, last.y, last.z].iter().all(|axis| axis.is_finite()) {
        return Err(refusal("degenerate-path", "The curve has no finite end points.", "Die Kurve hat keine endlichen Endpunkte."));
    }
    let closed = first.distance(last) <= 1e-9;
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let curve_id = body.curves3.insert(curve.clone());
    let start = make_vertex(&mut body, first, Tol::DEFAULT, &mut rec);
    let end = if closed { start } else { make_vertex(&mut body, last, Tol::DEFAULT, &mut rec) };
    let edge = make_edge(&mut body, curve_id, range, start, end, Tol::DEFAULT, &mut rec);
    let label = body.new_label();
    let vertices = if closed { vec![start] } else { vec![start, end] };
    Ok(ShapeValue { root: ShapeRoot::Wire(ShapeWire { members: vec![(edge, true)], vertices, closed, label }), body })
}

/// 🛤️ A path input read as a wire value: a wire is itself, an edge and a curve become one-edge wires.
pub fn path_wire(shape: &Arc<ShapeValue>, port: &str) -> Result<Arc<ShapeValue>, WidgetFault> {
    match &shape.root {
        ShapeRoot::Wire(_) => Ok(shape.clone()),
        ShapeRoot::Curve { curve, .. } => curve_wire(curve).map(Arc::new).map_err(|fault| fault.at(port)),
        ShapeRoot::Edge(id) => {
            let edge = shape.body.edges.get(*id).ok_or_else(|| refusal("degenerate-path", "The edge does not exist on its shape.", "Die Kante existiert an ihrer Form nicht.").at(port))?;
            let (start, end) = (edge.v0, edge.v1);
            let mut body = shape.body.clone();
            let label = body.new_label();
            let closed = start == end;
            let vertices = if closed { vec![start] } else { vec![start, end] };
            Ok(Arc::new(ShapeValue { root: ShapeRoot::Wire(ShapeWire { members: vec![(*id, true)], vertices, closed, label }), body }))
        }
        _ => Err(refusal("shape-kind", "This input takes a curve, an edge or a wire.", "Dieser Eingang erwartet eine Kurve, eine Kante oder einen Kantenzug.").at(port)),
    }
}
//#endregion 🔖️Wires

//#region 🔖️Computes
fn line_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (start, end) = (inputs.point("start")?, inputs.point("end")?);
    if coincide(start, end) {
        return Err(refusal("degenerate-curve", "The line needs two different end points.", "Die Linie braucht zwei verschiedene Endpunkte.").at("end"));
    }
    build(|brep| brep.line_curve_sync(start, end))
}

fn circle_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (center, normal) = (inputs.point("center")?, direction(inputs.vector("normal")?, "normal", "normal", "Die Normale")?);
    let radius = positive(inputs.number("radius")?, "radius", "radius", "Der Radius")?;
    build(|brep| brep.circle_curve_sync(center, normal, radius))
}

fn arc_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (center, normal) = (inputs.point("center")?, direction(inputs.vector("normal")?, "normal", "normal", "Die Normale")?);
    let radius = positive(inputs.number("radius")?, "radius", "radius", "Der Radius")?;
    let (start, mut end) = (inputs.number("startAngle")?, inputs.number("endAngle")?);
    let span = end - start;
    if !span.is_finite() || span.abs() <= 1e-12 {
        return Err(refusal("degenerate-curve", "The arc needs two different angles.", "Der Bogen braucht zwei verschiedene Winkel.").at("endAngle"));
    }
    if span.abs() > TAU + 1e-9 {
        return Err(refusal("arc-span", "An arc spans at most one full turn.", "Ein Bogen überspannt höchstens eine volle Drehung.").at("endAngle"));
    }
    if span < 0.0 {
        end += TAU * (-span / TAU).ceil();
    }
    let phase = frame_phase(normal)?;
    build(|brep| brep.arc_curve_sync(center, normal, radius, start + phase, end + phase))
}

fn ellipse_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (center, normal) = (inputs.point("center")?, direction(inputs.vector("normal")?, "normal", "normal", "Die Normale")?);
    let major = positive(inputs.number("semiMajor")?, "semiMajor", "major semi-axis", "Die große Halbachse")?;
    let minor = positive(inputs.number("semiMinor")?, "semiMinor", "minor semi-axis", "Die kleine Halbachse")?;
    let phase = frame_phase(normal)?;
    build(|brep| {
        let ellipse = brep.ellipse_curve_sync(center, normal, major, minor)?;
        if phase.abs() > 1e-12 {
            brep.rotate_about_sync(&ellipse, center, normal, phase)
        } else {
            Ok(ellipse)
        }
    })
}

fn polyline_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let mut corners = points(inputs, "points")?;
    if corners.len() < 2 {
        return Err(refusal("too-few-points", "A polyline needs at least two points.", "Ein Linienzug braucht mindestens zwei Punkte.").at("points"));
    }
    let closed = corners.len() >= 4 && coincide(corners[0], corners[corners.len() - 1]);
    if closed {
        corners.pop();
    }
    if corners.windows(2).any(|pair| coincide(pair[0], pair[1])) || (closed && coincide(corners[0], corners[corners.len() - 1])) {
        return Err(refusal("degenerate-curve", "Two neighbouring points coincide; every segment needs a length.", "Zwei benachbarte Punkte fallen zusammen; jedes Segment braucht eine Länge.").at("points"));
    }
    Ok(outputs([("shape", GeometryValue::shape(polyline_value(&corners, closed)?))]))
}

fn rectangle_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let width = positive(inputs.number("width")?, "width", "width", "Die Breite")?;
    let height = positive(inputs.number("height")?, "height", "height", "Die Höhe")?;
    build(|brep| brep.rectangle_wire_sync(width, height))
}

fn polygon_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let radius = positive(inputs.number("radius")?, "radius", "radius", "Der Radius")?;
    let sides = inputs.integer("sides")?;
    if !(3..=256).contains(&sides) {
        return Err(refusal("degenerate-size", "A regular polygon has between 3 and 256 sides.", "Ein regelmäßiges Vieleck hat zwischen 3 und 256 Seiten.").at("sides"));
    }
    build(|brep| brep.regular_polygon_wire_sync(radius, sides as usize))
}

fn distinct_points(inputs: &WidgetInputs, minimum: usize) -> Result<Vec<[f64; 3]>, WidgetFault> {
    let fitted = points(inputs, "points")?;
    if fitted.len() < minimum {
        return Err(refusal("too-few-points", format!("This curve needs at least {minimum} points."), format!("Diese Kurve braucht mindestens {minimum} Punkte.")).at("points"));
    }
    if fitted.windows(2).any(|pair| coincide(pair[0], pair[1])) {
        return Err(refusal("degenerate-curve", "Two neighbouring points coincide; the spline cannot be fitted.", "Zwei benachbarte Punkte fallen zusammen; der Spline kann nicht angepasst werden.").at("points"));
    }
    Ok(fitted)
}

fn interpolate_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let degree = inputs.integer("degree")?;
    let fitted = distinct_points(inputs, 2)?;
    if degree < 1 || degree as usize >= fitted.len() {
        return Err(refusal("degree-points", format!("A spline of degree {degree} needs at least {} points.", degree.max(1) + 1), format!("Ein Spline vom Grad {degree} braucht mindestens {} Punkte.", degree.max(1) + 1)).at("degree"));
    }
    build(|brep| brep.interpolate_curve_sync(&fitted, degree as usize))
}

fn approximate_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (degree, controls) = (inputs.integer("degree")?, inputs.integer("controlPoints")?);
    let fitted = distinct_points(inputs, 2)?;
    if degree < 1 || controls <= degree || controls as usize > fitted.len() {
        return Err(refusal("degree-points", format!("The spline needs more control points than its degree {degree} and at most as many as the {} points.", fitted.len()), format!("Der Spline braucht mehr Kontrollpunkte als sein Grad {degree} und höchstens so viele wie die {} Punkte.", fitted.len())).at("controlPoints"));
    }
    build(|brep| brep.approximate_curve_sync(&fitted, degree as usize, controls as usize))
}

fn helix_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (origin, axis) = (inputs.point("origin")?, direction(inputs.vector("axis")?, "axis", "axis", "Die Achse")?);
    let radius = positive(inputs.number("radius")?, "radius", "radius", "Der Radius")?;
    let pitch = positive(inputs.number("pitch")?, "pitch", "pitch", "Die Steigung")?;
    let turns = positive(inputs.number("turns")?, "turns", "number of turns", "Die Windungszahl")?;
    if turns > 1000.0 {
        return Err(refusal("degenerate-size", "A helix has at most 1000 turns.", "Eine Schraubenlinie hat höchstens 1000 Windungen.").at("turns"));
    }
    build(|brep| brep.helix_curve_sync(origin, axis, radius, pitch, turns))
}

macro_rules! cheap {
    ($($name:ident => $compute:ident),+ $(,)?) => {
        $(fn $name(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
            finish(kind, unwound(|| $compute(&inputs)))
        })+
    };
}

cheap! {
    line => line_outputs,
    circle => circle_outputs,
    arc => arc_outputs,
    ellipse => ellipse_outputs,
    polyline => polyline_outputs,
    rectangle => rectangle_outputs,
    polygon => polygon_outputs,
    interpolate => interpolate_outputs,
    approximate => approximate_outputs,
    helix => helix_outputs,
}

/// 🗃️ The `brep.curve` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.curve.line", start: line },
    ComputeEntry { id: "brep.curve.circle", start: circle },
    ComputeEntry { id: "brep.curve.arc", start: arc },
    ComputeEntry { id: "brep.curve.ellipse", start: ellipse },
    ComputeEntry { id: "brep.curve.polyline", start: polyline },
    ComputeEntry { id: "brep.curve.rectangle", start: rectangle },
    ComputeEntry { id: "brep.curve.polygon", start: polygon },
    ComputeEntry { id: "brep.curve.interpolate", start: interpolate },
    ComputeEntry { id: "brep.curve.approximate", start: approximate },
    ComputeEntry { id: "brep.curve.helix", start: helix },
];
//#endregion 🔖️Computes

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
