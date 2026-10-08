//! ✂️ The `brep.intersect` widgets: planar section and split of solids as stepped lane J operation jobs, and curve/curve, curve/surface and surface/surface intersection of edges, wires, curves, faces and surfaces.
//!
//! Intersections of edges and wires keep only the hits inside the edge ranges; intersections with a face use its supporting surface. Hits closer than the tolerance are one point, so the joint of two wire members is reported once.

use super::brep_sources::{curve_input, surface_input, CurveSource, SurfaceSource};
use super::phased_job::{launch, Pipeline, Work};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{BrepOperation, GeometryKind, ShapeRoot, ShapeValue, ShapeWire};
use semio_framework_3d::brep::operations::euler::{make_edge, make_vertex};
use semio_framework_3d::brep::operations::intersect::{intersect_curve_curve, intersect_curve_surface, intersect_surface_surface};
use semio_framework_3d::brep::representation::curve::Curve3;
use semio_framework_3d::brep::representation::tolerance::Tol;
use semio_framework_3d::brep::representation::topology::history::OpRecorder;
use semio_framework_3d::brep::representation::topology::Body;
use std::sync::Arc;

const INTERSECT_FAILED: &str = "generation3d.geometry.intersect-failed";
const SHAPE_KIND: &str = "generation3d.geometry.shape-kind";
const UNBOUNDED: &str = "generation3d.geometry.domain-unbounded";

//#region 🔖️Plane
type PlaneOperation = fn(semio_framework_3d::brep::engine::GeometryHandle, [f64; 3], [f64; 3]) -> BrepOperation;

fn plane_operation(kind: &Kind, inputs: &WidgetInputs, operation: PlaneOperation, sides: bool) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let solid = inputs.shape("solid")?.clone();
        let plane = inputs.plane("plane")?;
        let compound = solid.kind() == GeometryKind::Compound;
        if sides && compound {
            return Err(WidgetFault::new(SHAPE_KIND, "A split takes one solid; explode the compound first.", "Eine Teilung erwartet einen Körper; zerlege die Verbundform zuerst.").at("solid"));
        }
        Ok(Pipeline::new(kind).import(&solid).operations(move |work| {
            let handle = work.handle(0)?;
            let members = if compound { work.session.brep().explode_sync(&handle).map_err(|error| kernel_fault(&error))? } else { vec![handle] };
            Ok(members.into_iter().map(|member| operation(member, plane.origin, plane.normal)).collect())
        }).export_groups().finish(move |work| {
            let mut values = work.values.first().cloned().unwrap_or_default().into_iter();
            if !sides {
                return Ok(outputs([("faces", GeometryValue::List(values.collect()))]));
            }
            match (values.next(), values.next()) {
                (Some(positive), Some(negative)) => Ok(outputs([("positive", positive), ("negative", negative)])),
                _ => Err(WidgetFault::new(INTERSECT_FAILED, "The split produced fewer than two solids.", "Die Teilung lieferte weniger als zwei Körper.")),
            }
        }))
    })
}

fn section(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    plane_operation(kind, &inputs, |solid, plane_origin, plane_normal| BrepOperation::Section { solid, plane_origin, plane_normal }, false)
}

fn split(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    plane_operation(kind, &inputs, |solid, plane_origin, plane_normal| BrepOperation::Split { solid, plane_origin, plane_normal }, true)
}
//#endregion 🔖️Plane

//#region 🔖️Wire
fn wire_of(curve: &Curve3, range: (f64, f64)) -> Result<ShapeValue, WidgetFault> {
    if !range.0.is_finite() || !range.1.is_finite() {
        return Err(WidgetFault::new(UNBOUNDED, "The intersection curve is unbounded.", "Die Schnittkurve ist unbegrenzt."));
    }
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (start, end) = (curve.eval(range.0), curve.eval(range.1));
    let closed = (start - end).norm() <= Tol::DEFAULT.0;
    let first = make_vertex(&mut body, start, Tol::DEFAULT, &mut recorder);
    let last = if closed { first } else { make_vertex(&mut body, end, Tol::DEFAULT, &mut recorder) };
    let curve = body.curves3.insert(curve.clone());
    let edge = make_edge(&mut body, curve, range, first, last, Tol::DEFAULT, &mut recorder);
    let label = body.new_label();
    let vertices = if closed { vec![first] } else { vec![first, last] };
    let value = ShapeValue { root: ShapeRoot::Wire(ShapeWire { members: vec![(edge, true)], vertices, closed, label }), body };
    value.check().map_err(|error| kernel_fault(&error))?;
    Ok(value)
}

fn intersect_fault(error: impl std::fmt::Display) -> WidgetFault {
    WidgetFault::new(INTERSECT_FAILED, format!("The intersection could not be computed: {error}."), format!("Der Schnitt konnte nicht berechnet werden: {error}."))
}

type Hit = (f64, [f64; 3]);

fn distinct(mut hits: Vec<Hit>, tolerance: f64) -> Vec<[f64; 3]> {
    hits.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut kept: Vec<[f64; 3]> = Vec::with_capacity(hits.len());
    for (_, point) in hits {
        let near = |other: &[f64; 3]| (0..3).map(|axis| (other[axis] - point[axis]).powi(2)).sum::<f64>().sqrt() <= tolerance;
        if !kept.iter().any(near) {
            kept.push(point);
        }
    }
    kept
}

fn within_range(range: (f64, f64), parameter: f64) -> bool {
    let slack = 1e-9 * (1.0 + parameter.abs());
    parameter >= range.0 - slack && parameter <= range.1 + slack
}

fn points_output(work: &mut Work, hits: Vec<Hit>, tolerance: f64) -> Result<Outputs, WidgetFault> {
    let points = distinct(hits, tolerance);
    let mut result = outputs([("points", GeometryValue::List(points.iter().copied().map(GeometryValue::Point).collect()))]);
    if points.len() >= 2 {
        let wire = work.session.brep().polyline_wire_sync(&points).map_err(|error| kernel_fault(&error))?;
        result.insert("wire".to_string(), work.export(&wire)?);
    }
    Ok(result)
}

fn curve_curve_hits(a: &CurveSource, b: &CurveSource, tolerance: f64) -> Result<Vec<Hit>, WidgetFault> {
    let mut hits = Vec::new();
    for left in a.pieces() {
        for right in b.pieces() {
            for hit in intersect_curve_curve(&left.curve, &right.curve, tolerance).map_err(intersect_fault)? {
                if within_range(left.curve_range(), hit.t_a) && within_range(right.curve_range(), hit.t_b) {
                    hits.push((left.source_parameter(hit.t_a), [hit.point.x, hit.point.y, hit.point.z]));
                }
            }
        }
    }
    Ok(hits)
}

fn curve_surface_hits(curve: &CurveSource, surface: &SurfaceSource, tolerance: f64) -> Result<Vec<Hit>, WidgetFault> {
    let mut hits = Vec::new();
    for piece in curve.pieces() {
        for hit in intersect_curve_surface(&piece.curve, &surface.surface, tolerance).map_err(intersect_fault)? {
            if within_range(piece.curve_range(), hit.t) {
                hits.push((piece.source_parameter(hit.t), [hit.point.x, hit.point.y, hit.point.z]));
            }
        }
    }
    Ok(hits)
}

fn curve_curve(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let (a, b) = (curve_input(&inputs, "a")?, curve_input(&inputs, "b")?);
        let tolerance = inputs.number("tolerance")?;
        Ok(Pipeline::new(kind).finish(move |work| {
            let hits = curve_curve_hits(&a, &b, tolerance)?;
            points_output(work, hits, tolerance)
        }))
    })
}

fn curve_surface(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let (curve, surface) = (curve_input(&inputs, "curve")?, surface_input(&inputs, "surface")?);
        let tolerance = inputs.number("tolerance")?;
        Ok(Pipeline::new(kind).finish(move |work| {
            let hits = curve_surface_hits(&curve, &surface, tolerance)?;
            points_output(work, hits, tolerance)
        }))
    })
}

fn surface_surface(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let (a, b) = (surface_input(&inputs, "a")?, surface_input(&inputs, "b")?);
        let tolerance = inputs.number("tolerance")?;
        Ok(Pipeline::new(kind).finish(move |_| {
            let branches = intersect_surface_surface(&a.surface, &b.surface, tolerance).map_err(intersect_fault)?;
            let wires = branches.iter().map(|branch| wire_of(&branch.curve3, (branch.domain.min, branch.domain.max)).map(|wire| GeometryValue::Shape(Arc::new(wire)))).collect::<Result<Vec<_>, _>>()?;
            Ok(outputs([("wires", GeometryValue::List(wires))]))
        }))
    })
}
//#endregion 🔖️Wire

/// 🗃️ Every `brep.intersect` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.intersect.section", start: section },
    ComputeEntry { id: "brep.intersect.split", start: split },
    ComputeEntry { id: "brep.intersect.curveCurve", start: curve_curve },
    ComputeEntry { id: "brep.intersect.curveSurface", start: curve_surface },
    ComputeEntry { id: "brep.intersect.surfaceSurface", start: surface_surface },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
