//! 🎯️ The `brep.evaluate` widgets: points, tangents, curvature, domains, normals and closest points of curves, edges, wires, surfaces and faces.
//!
//! Every evaluation reads the value directly through its curve or surface source, so it is as cheap as the formula and never opens a kernel session. A parameter outside the domain of the curve or surface is refused instead of being clamped silently.

use super::brep_sources::{curve_input, surface_input};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;

const PARAMETER_RANGE: &str = "generation3d.geometry.parameter-range";
const UNBOUNDED: &str = "generation3d.geometry.domain-unbounded";
const SINGULAR: &str = "generation3d.geometry.evaluate-singular";

fn within(domain: (f64, f64), value: f64, port: &str) -> Result<(), WidgetFault> {
    let slack = 1e-9 * (1.0 + value.abs());
    if value.is_finite() && value >= domain.0 - slack && value <= domain.1 + slack {
        return Ok(());
    }
    Err(WidgetFault::new(PARAMETER_RANGE, format!("The parameter {value} lies outside the domain [{}, {}].", domain.0, domain.1), format!("Der Parameter {value} liegt ausserhalb des Definitionsbereichs [{}, {}].", domain.0, domain.1)).at(port))
}

fn singular(what: (&str, &str)) -> WidgetFault {
    WidgetFault::new(SINGULAR, format!("The {} is undefined here.", what.0), format!("{} ist hier nicht definiert.", what.1))
}

fn curve_point(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let curve = curve_input(&inputs, "curve")?;
        let parameter = inputs.number("parameter")?;
        within(curve.domain(), parameter, "parameter")?;
        Ok(outputs([("point", GeometryValue::Point(curve.point(parameter)))]))
    })())
}

fn curve_tangent(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let curve = curve_input(&inputs, "curve")?;
        let parameter = inputs.number("parameter")?;
        within(curve.domain(), parameter, "parameter")?;
        let tangent = curve.tangent(parameter).ok_or_else(|| singular(("tangent", "Die Tangente")))?;
        Ok(outputs([("tangent", GeometryValue::Vector(tangent))]))
    })())
}

fn curve_domain(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let curve = curve_input(&inputs, "curve")?;
        let (start, end) = curve.domain();
        if !start.is_finite() || !end.is_finite() {
            return Err(WidgetFault::new(UNBOUNDED, "The curve is unbounded and has no parameter range.", "Die Kurve ist unbegrenzt und hat keinen Parameterbereich.").at("curve"));
        }
        Ok(outputs([("start", GeometryValue::Number(start)), ("end", GeometryValue::Number(end)), ("span", GeometryValue::Number(end - start))]))
    })())
}

fn curve_curvature(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let curve = curve_input(&inputs, "curve")?;
        let parameter = inputs.number("parameter")?;
        within(curve.domain(), parameter, "parameter")?;
        Ok(outputs([("curvature", GeometryValue::Number(curve.curvature(parameter)))]))
    })())
}

fn surface_point(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let surface = surface_input(&inputs, "surface")?;
        let (u, v) = (inputs.number("u")?, inputs.number("v")?);
        let (u_domain, v_domain) = surface.surface.domain();
        within(u_domain, u, "u")?;
        within(v_domain, v, "v")?;
        Ok(outputs([("point", GeometryValue::Point(surface.point(u, v)))]))
    })())
}

fn surface_normal(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let surface = surface_input(&inputs, "surface")?;
        let (u, v) = (inputs.number("u")?, inputs.number("v")?);
        let (u_domain, v_domain) = surface.surface.domain();
        within(u_domain, u, "u")?;
        within(v_domain, v, "v")?;
        let normal = surface.normal(u, v).ok_or_else(|| singular(("normal", "Die Normale")))?;
        Ok(outputs([("normal", GeometryValue::Vector(normal))]))
    })())
}

fn curve_closest_parameter(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let curve = curve_input(&inputs, "curve")?;
        let (parameter, point, distance) = curve.closest(inputs.point("point")?);
        Ok(outputs([("parameter", GeometryValue::Number(parameter)), ("point", GeometryValue::Point(point)), ("distance", GeometryValue::Number(distance))]))
    })())
}

fn surface_closest_uv(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let surface = surface_input(&inputs, "surface")?;
        let (u, v, point, distance) = surface.closest(inputs.point("point")?);
        Ok(outputs([("u", GeometryValue::Number(u)), ("v", GeometryValue::Number(v)), ("point", GeometryValue::Point(point)), ("distance", GeometryValue::Number(distance))]))
    })())
}

/// 🗃️ Every `brep.evaluate` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.evaluate.curvePoint", start: curve_point },
    ComputeEntry { id: "brep.evaluate.curveTangent", start: curve_tangent },
    ComputeEntry { id: "brep.evaluate.curveDomain", start: curve_domain },
    ComputeEntry { id: "brep.evaluate.curveCurvature", start: curve_curvature },
    ComputeEntry { id: "brep.evaluate.surfacePoint", start: surface_point },
    ComputeEntry { id: "brep.evaluate.surfaceNormal", start: surface_normal },
    ComputeEntry { id: "brep.evaluate.curveClosestParameter", start: curve_closest_parameter },
    ComputeEntry { id: "brep.evaluate.surfaceClosestUv", start: surface_closest_uv },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
