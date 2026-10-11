//! 🏳️ `brep.surface` computes: planes, faces from points and wires, free-form patches and face offsets.
//!
//! This module also owns the planar fit every profile of the solid computes shares: a closed wire is filled with a face whose normal is the wire's own orientation normal, whatever plane the wire lies in, so extrusions, sweeps and lofts of the face see a consistently oriented boundary.

use super::brep_curve::{coincide, cross, direction, dot, guarded, minus, norm, points, refusal};
use super::phased_job::{Pipeline, Work};
use super::super::prelude::*;
use semio_framework_3d::brep::engine::{BrepOperation, GeometryHandle, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::representation::curve::Curve3;
use std::f64::consts::PI;

//#region 🔖️Fit
/// 🧭️ The plane of a closed outline: its mean point and the unit normal turned the way the outline winds counter-clockwise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutlinePlane {
    pub centroid: [f64; 3],
    pub normal: [f64; 3],
}

/// 🏷️ What an outline is called in a refusal.
#[derive(Clone, Copy)]
pub struct Subject {
    degenerate: &'static str,
    planar: &'static str,
    en: &'static str,
    de: &'static str,
}

/// 🧵️ A wire outline.
pub const WIRE: Subject = Subject { degenerate: "wire-degenerate", planar: "wire-not-planar", en: "wire", de: "Kantenzug" };
/// 📍️ A point outline.
pub const POINTS: Subject = Subject { degenerate: "points-degenerate", planar: "points-not-planar", en: "points", de: "Punkte" };

/// 📐️ Fits the plane of the outline through `corners` (Newell's method) and refuses an outline of no area or one that leaves its plane.
pub fn fit_plane(corners: &[[f64; 3]], subject: Subject) -> Result<OutlinePlane, WidgetFault> {
    let count = corners.len();
    let mut normal = [0.0; 3];
    let mut centroid = [0.0; 3];
    for (index, current) in corners.iter().enumerate() {
        let next = corners[(index + 1) % count];
        normal[0] += (current[1] - next[1]) * (current[2] + next[2]);
        normal[1] += (current[2] - next[2]) * (current[0] + next[0]);
        normal[2] += (current[0] - next[0]) * (current[1] + next[1]);
        for axis in 0..3 {
            centroid[axis] += current[axis] / count as f64;
        }
    }
    let extent = corners.iter().map(|corner| norm(minus(*corner, centroid))).fold(0.0, f64::max);
    let area_twice = norm(normal);
    if !area_twice.is_finite() || area_twice <= 1e-12 * extent.max(1.0).powi(2) {
        return Err(refusal(subject.degenerate, format!("The {} enclose no area; they must not be collinear.", subject.en), format!("{} umschließen keine Fläche; sie dürfen nicht auf einer Geraden liegen.", subject.de)));
    }
    let unit = [normal[0] / area_twice, normal[1] / area_twice, normal[2] / area_twice];
    if corners.iter().any(|corner| dot(minus(*corner, centroid), unit).abs() > 1e-6 * extent.max(1.0)) {
        return Err(refusal(subject.planar, format!("The {} do not lie in one plane.", subject.en), format!("{} liegen nicht in einer Ebene.", subject.de)));
    }
    Ok(OutlinePlane { centroid, normal: unit })
}

/// 🧵️ The corners of a closed wire value in winding order, curved members sampled; the shape must be a wire.
pub fn wire_corners(shape: &ShapeValue) -> Result<Vec<[f64; 3]>, WidgetFault> {
    let ShapeRoot::Wire(wire) = &shape.root else {
        return Err(refusal("shape-kind", "This input takes a wire.", "Dieser Eingang erwartet einen Kantenzug."));
    };
    if !wire.closed {
        return Err(refusal("wire-open", "The wire is not closed; close it so that it bounds a face.", "Der Kantenzug ist nicht geschlossen; schließe ihn, damit er eine Fläche begrenzt."));
    }
    let broken = || refusal("wire-degenerate", "The wire refers to an edge its shape does not hold.", "Der Kantenzug verweist auf eine Kante, die seine Form nicht enthält.");
    let mut corners = Vec::new();
    for (id, forward) in &wire.members {
        let edge = shape.body.edges.get(*id).ok_or_else(broken)?;
        let curve = shape.body.curves3.get(edge.curve).ok_or_else(broken)?;
        let (from, to) = if *forward { edge.range } else { (edge.range.1, edge.range.0) };
        let samples = if matches!(curve, Curve3::Line { .. }) { 1 } else { 32 };
        for step in 0..samples {
            let at = curve.eval(from + (to - from) * step as f64 / samples as f64);
            corners.push([at.x, at.y, at.z]);
        }
    }
    if corners.len() < 3 {
        return Err(refusal("wire-degenerate", "The wire needs at least three corners.", "Der Kantenzug braucht mindestens drei Ecken."));
    }
    Ok(corners)
}

/// 🔁️ The rotation about the origin that turns `normal` onto +Z, or `None` when it already is +Z.
fn alignment(normal: [f64; 3]) -> Option<([f64; 3], f64)> {
    let along = normal[2].clamp(-1.0, 1.0);
    if along >= 1.0 - 1e-12 {
        return None;
    }
    if along <= -1.0 + 1e-12 {
        return Some(([1.0, 0.0, 0.0], PI));
    }
    let axis = cross(normal, [0.0, 0.0, 1.0]);
    let length = norm(axis);
    Some(([axis[0] / length, axis[1] / length, axis[2] / length], along.acos()))
}

/// 🖼️ Fills the closed wire imported as input `index` with a planar face: the wire is turned flat onto the XY plane, filled there and turned back, so the face normal is the wire's orientation normal.
pub fn fill_wire(work: &mut Work, index: usize, port: &str) -> Result<GeometryHandle, WidgetFault> {
    let source = work.source(index)?;
    let plane = wire_corners(&source).and_then(|corners| fit_plane(&corners, WIRE)).map_err(|fault| fault.at(port))?;
    let wire = work.handle(index)?;
    let brep = work.session.brep();
    let face = match alignment(plane.normal) {
        None => brep.planar_face_from_wire_sync(&wire),
        Some((axis, angle)) => brep.rotate_sync(&wire, axis, angle).and_then(|flat| brep.planar_face_from_wire_sync(&flat)).and_then(|face| brep.rotate_sync(&face, axis, -angle)),
    };
    face.map_err(|error| kernel_fault(&error))
}

/// 🖼️ The face of input `index`: a face as it is, a wire filled.
pub fn profile_face(work: &mut Work, index: usize, port: &str) -> Result<GeometryHandle, WidgetFault> {
    let wire = matches!(work.source(index)?.root, ShapeRoot::Wire(_));
    if wire {
        fill_wire(work, index, port)
    } else {
        work.handle(index)
    }
}
//#endregion 🔖️Fit

//#region 🔖️Computes
fn export(session: &KernelSession, handle: &GeometryHandle) -> Result<Outputs, WidgetFault> {
    Ok(outputs([("shape", GeometryValue::shape(session.export(handle)?))]))
}

fn plane_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let plane = inputs.plane("plane")?;
    let normal = direction(plane.normal, "plane", "plane normal", "Die Ebenennormale")?;
    let mut session = KernelSession::new();
    let handle = session.brep().plane_surface_sync(plane.origin, normal).map_err(|error| kernel_fault(&error))?;
    export(&session, &handle)
}

fn planar_face_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let mut corners = points(inputs, "points")?;
    if corners.len() >= 4 && coincide(corners[0], corners[corners.len() - 1]) {
        corners.pop();
    }
    if corners.len() < 3 {
        return Err(refusal("too-few-points", "A face needs at least three corner points.", "Eine Fläche braucht mindestens drei Eckpunkte.").at("points"));
    }
    if (0..corners.len()).any(|index| coincide(corners[index], corners[(index + 1) % corners.len()])) {
        return Err(refusal("points-degenerate", "Two neighbouring corner points coincide; every edge needs a length.", "Zwei benachbarte Eckpunkte fallen zusammen; jede Kante braucht eine Länge.").at("points"));
    }
    fit_plane(&corners, POINTS).map_err(|fault| fault.at("points"))?;
    let mut session = KernelSession::new();
    let handle = session.brep().planar_face_from_points_sync(&corners).map_err(|error| kernel_fault(&error))?;
    export(&session, &handle)
}

fn nurbs_grid_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let fitted = points(inputs, "points")?;
    let (rows, degree_u, degree_v) = (inputs.integer("rows")?, inputs.integer("degreeU")?, inputs.integer("degreeV")?);
    if rows < 2 || fitted.len() % rows as usize != 0 || fitted.len() / (rows as usize) < 2 {
        return Err(refusal("grid-shape", format!("The {} points do not form a grid of {rows} rows with at least two columns.", fitted.len()), format!("Die {} Punkte bilden kein Gitter aus {rows} Zeilen mit mindestens zwei Spalten.", fitted.len())).at("rows"));
    }
    let columns = fitted.len() / rows as usize;
    if degree_u < 1 || degree_u as usize >= columns {
        return Err(refusal("degree-points", format!("A degree of {degree_u} along the rows needs at least {} columns.", degree_u.max(1) + 1), format!("Ein Grad von {degree_u} entlang der Zeilen braucht mindestens {} Spalten.", degree_u.max(1) + 1)).at("degreeU"));
    }
    if degree_v < 1 || degree_v >= rows {
        return Err(refusal("degree-points", format!("A degree of {degree_v} along the columns needs at least {} rows.", degree_v.max(1) + 1), format!("Ein Grad von {degree_v} entlang der Spalten braucht mindestens {} Zeilen.", degree_v.max(1) + 1)).at("degreeV"));
    }
    let transposed: Vec<Vec<[f64; 3]>> = (0..columns).map(|column| (0..rows as usize).map(|row| fitted[row * columns + column]).collect()).collect();
    let mut session = KernelSession::new();
    let handle = session.brep().nurbs_surface_from_grid_sync(&transposed, degree_u as usize, degree_v as usize).map_err(|error| kernel_fault(&error))?;
    export(&session, &handle)
}

fn coons_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let boundary = ["bottom", "right", "top", "left"].iter().map(|port| points(inputs, port).map(|curve| (*port, curve))).collect::<Result<Vec<_>, _>>()?;
    if let Some((port, _)) = boundary.iter().find(|(_, curve)| curve.len() < 2 || curve.windows(2).any(|pair| coincide(pair[0], pair[1]))) {
        return Err(refusal("degenerate-curve", "A boundary needs at least two different points in a row.", "Ein Rand braucht mindestens zwei verschiedene aufeinanderfolgende Punkte.").at(*port));
    }
    let curves: Vec<Vec<[f64; 3]>> = boundary.into_iter().map(|(_, curve)| curve).collect();
    let mut session = KernelSession::new();
    let handle = session.brep().coons_patch_sync(&curves).map_err(|error| kernel_fault(&error))?;
    export(&session, &handle)
}

fn wire_face(kind: &Kind, inputs: &WidgetInputs, parallel_to_xy: bool) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let wire = inputs.shape("wire")?.clone();
        Ok(Pipeline::new(kind)
            .import(&wire)
            .once(move |work| {
                if parallel_to_xy {
                    let plane = wire_corners(&*work.source(0)?).and_then(|corners| fit_plane(&corners, WIRE)).map_err(|fault| fault.at("wire"))?;
                    if plane.normal[2].abs() < 1.0 - 1e-9 {
                        return Err(refusal("wire-not-xy-parallel", "The wire must lie in a plane parallel to the XY plane; use \u{201c}Face from wire\u{201d} for other planes.", "Der Kantenzug muss in einer zur XY-Ebene parallelen Ebene liegen; für andere Ebenen dient \u{201e}Fläche aus Kantenzug\u{201c}.").at("wire"));
                    }
                }
                let face = fill_wire(work, 0, "wire")?;
                work.groups = vec![vec![face]];
                Ok(())
            })
            .exported("shape"))
    })
}

fn planar_face_from_wire(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    wire_face(kind, &inputs, true)
}

fn face_from_wire(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    wire_face(kind, &inputs, false)
}

fn offset(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let face = inputs.shape("face")?.clone();
        let distance = inputs.number("distance")?;
        if !distance.is_finite() {
            return Err(refusal("degenerate-size", "The offset distance must be a finite number.", "Der Versatz muss eine endliche Zahl sein.").at("distance"));
        }
        Ok(Pipeline::new(kind).import(&face).operation(move |work| Ok(BrepOperation::OffsetFace { face: work.handle(0)?, distance })).exported("shape"))
    })
}

macro_rules! cheap {
    ($($name:ident => $compute:ident),+ $(,)?) => {
        $(fn $name(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
            finish(kind, $compute(&inputs))
        })+
    };
}

cheap! {
    plane => plane_outputs,
    planar_face => planar_face_outputs,
    nurbs_grid => nurbs_grid_outputs,
    coons => coons_outputs,
}

/// 🗃️ The `brep.surface` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.surface.plane", start: plane },
    ComputeEntry { id: "brep.surface.planarFace", start: planar_face },
    ComputeEntry { id: "brep.surface.planarFaceFromWire", start: planar_face_from_wire },
    ComputeEntry { id: "brep.surface.faceFromWire", start: face_from_wire },
    ComputeEntry { id: "brep.surface.nurbsGrid", start: nurbs_grid },
    ComputeEntry { id: "brep.surface.coons", start: coons },
    ComputeEntry { id: "brep.surface.offset", start: offset },
];
//#endregion 🔖️Computes

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
