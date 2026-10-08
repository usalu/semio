//! 🧰️ Shared support of the fixture-driven unit tests of the construction computes (`brep.curve`, `brep.surface`, `brep.solid`, `brep.boolean`): on top of the oracle support it builds the shape inputs of a case from other widgets of the registry and measures what a case states beyond kind, volume and area.
//!
//! A shape input is `{ "recipe": "widget", "kind": <kind id>, "inputs": { .. }, "port": <output, default "shape"> }`: the output of that widget's compute, so a fixture names a small graph exactly as a document does. Every other recipe is an oracle-support recipe (`box`, `translate`, ..). A shape output may state, besides the oracle support's `kind`, `volume` and `area`: `length`, `start`, `end`, `closed`, `bbox` (`[min, max]`), `faces`, `through` (points the curve passes through), `corners` and `center` (points of a surface at its domain corners and middle) and `plane` (a surface that lies in a plane).
//!
//! The fixture format is the one the TypeScript oracle tests read, see `🧪️tests/🧰️construction-support/🟦️.ts`.

#![allow(dead_code)]

use crate::standards::v1::subsets::any::schema::catalogue::{Kind, PortType};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use crate::standards::v1::subsets::any::schema::inferences::geometry::registry;
use semio_framework_3d::brep::engine::{GeometryKind, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::representation::curve::Curve3;
use semio_framework_3d::brep::representation::surface::Surface;
use serde_json::Value;
use std::collections::BTreeMap;
use std::f64::consts::TAU;
use std::sync::Arc;

#[path = "../🧰️oracle-support/🦀️.rs"]
pub mod oracle;

pub use oracle::{assert_category, cases, close, drive, kind_of};

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("number expected, found {value}"))
}

fn triple(value: &Value) -> [f64; 3] {
    let items = value.as_array().unwrap_or_else(|| panic!("triple expected, found {value}"));
    [number(&items[0]), number(&items[1]), number(&items[2])]
}

//#region 🔖️Inputs
/// 🧊️ The shape a fixture recipe names; a widget recipe runs the registered compute to its end.
pub fn shape_of(recipe: &Value) -> Arc<ShapeValue> {
    if recipe["recipe"].as_str() != Some("widget") {
        return Arc::new(oracle::build_shape(recipe));
    }
    let kind = kind_of(recipe["kind"].as_str().expect("widget kind"));
    let start = registry::lookup(&kind.id).unwrap_or_else(|| panic!("{} has a compute", kind.id));
    let evaluation = drive(start(kind, inputs_of(kind, recipe)), usize::MAX);
    if let Some(fault) = &evaluation.fault {
        panic!("the widget recipe {} faulted: {fault}", kind.id);
    }
    let port = recipe["port"].as_str().unwrap_or("shape");
    match evaluation.outputs.get(port) {
        Some(GeometryValue::Shape(shape)) => shape.clone(),
        other => panic!("{} has no shape output {port}: {other:?}", kind.id),
    }
}

/// 🔌️ The typed inputs a case (or a widget recipe) states for its kind.
pub fn inputs_of(kind: &Kind, case: &Value) -> WidgetInputs {
    let stated = case["inputs"].as_object().expect("case inputs");
    let mut values: BTreeMap<String, GeometryValue> = BTreeMap::new();
    for port in &kind.inputs {
        let Some(value) = stated.get(&port.name) else { continue };
        let decoded = match port.port_type {
            PortType::Shape | PortType::Shapes if port.list || port.port_type == PortType::Shapes => GeometryValue::List(value.as_array().expect("a list of shapes").iter().map(|item| GeometryValue::Shape(shape_of(item))).collect()),
            PortType::Shape => GeometryValue::Shape(shape_of(value)),
            _ => oracle::decode(port, value),
        };
        values.insert(port.name.clone(), decoded);
    }
    WidgetInputs::new(case["name"].as_str().unwrap_or("case"), kind, values)
}
//#endregion 🔖️Inputs

//#region 🔖️Measures
/// 🧩️ The stretches of a curve, edge or wire value as `(curve, range, forward)`; a line curve spans its segment `[0, 1]`.
fn pieces(shape: &ShapeValue) -> Option<Vec<(Curve3, (f64, f64))>> {
    let body = &shape.body;
    let natural = |curve: &Curve3| match curve {
        Curve3::Line { .. } => (0.0, 1.0),
        Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, TAU),
        Curve3::Nurbs { knots, .. } => knots.domain(),
    };
    match &shape.root {
        ShapeRoot::Curve { curve, .. } => Some(vec![(curve.clone(), natural(curve))]),
        ShapeRoot::Edge(id) => {
            let edge = body.edges.get(*id)?;
            Some(vec![(body.curves3.get(edge.curve)?.clone(), edge.range)])
        }
        ShapeRoot::Wire(wire) => wire
            .members
            .iter()
            .map(|(id, forward)| {
                let edge = body.edges.get(*id)?;
                let range = if *forward { edge.range } else { (edge.range.1, edge.range.0) };
                Some((body.curves3.get(edge.curve)?.clone(), range))
            })
            .collect(),
        _ => None,
    }
}

const SAMPLES: usize = 4096;

fn polyline(shape: &ShapeValue) -> Option<Vec<[f64; 3]>> {
    let mut points: Vec<[f64; 3]> = Vec::new();
    for (curve, (from, to)) in pieces(shape)? {
        let count = if matches!(curve, Curve3::Line { .. }) { 1 } else { SAMPLES };
        for step in 0..=count {
            let at = curve.eval(from + (to - from) * step as f64 / count as f64);
            points.push([at.x, at.y, at.z]);
        }
    }
    Some(points)
}

/// 📏️ The arc length of a curve, edge or wire value.
pub fn length(shape: &ShapeValue) -> Option<f64> {
    let points = polyline(shape)?;
    let mut total = 0.0;
    let mut pieces_start = 0usize;
    for (curve, _) in pieces(shape)? {
        let count = if matches!(curve, Curve3::Line { .. }) { 1 } else { SAMPLES };
        total += points[pieces_start..=pieces_start + count].windows(2).map(|pair| (0..3).map(|axis| (pair[1][axis] - pair[0][axis]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();
        pieces_start += count + 1;
    }
    Some(total)
}

fn surface_points(surface: &Surface) -> Vec<[f64; 3]> {
    let ((u0, u1), (v0, v1)) = surface.domain();
    let (u0, u1) = if u0.is_finite() && u1.is_finite() { (u0, u1) } else { (-1.0, 1.0) };
    let (v0, v1) = if v0.is_finite() && v1.is_finite() { (v0, v1) } else { (-1.0, 1.0) };
    let mut points = Vec::new();
    for i in 0..=16 {
        for j in 0..=16 {
            let at = surface.eval(u0 + (u1 - u0) * i as f64 / 16.0, v0 + (v1 - v0) * j as f64 / 16.0);
            points.push([at.x, at.y, at.z]);
        }
    }
    points
}

/// 🧱️ The bounding box of any shape value: sampled curves and surfaces, tessellated faces, shells, solids and compounds.
pub fn bbox(shape: &ShapeValue) -> Option<[[f64; 3]; 2]> {
    let points: Vec<[f64; 3]> = match &shape.root {
        ShapeRoot::Surface { surface, .. } => surface_points(surface),
        ShapeRoot::Curve { .. } | ShapeRoot::Edge(_) | ShapeRoot::Wire(_) => polyline(shape)?,
        _ => shape.tessellate(0.001).ok()?.position.chunks(3).map(|c| [c[0] as f64, c[1] as f64, c[2] as f64]).collect(),
    };
    let mut bounds = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
    for point in &points {
        for axis in 0..3 {
            bounds[0][axis] = bounds[0][axis].min(point[axis]);
            bounds[1][axis] = bounds[1][axis].max(point[axis]);
        }
    }
    (!points.is_empty()).then_some(bounds)
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|axis| (a[axis] - b[axis]).powi(2)).sum::<f64>().sqrt()
}

fn near(actual: [f64; 3], wanted: [f64; 3], tolerance: f64) -> bool {
    (0..3).all(|axis| close(actual[axis], wanted[axis], tolerance))
}

/// 📍️ The distance of a point to the sampled curve, refined between samples by projecting onto the neighbouring segments.
fn distance_to_curve(shape: &ShapeValue, point: [f64; 3]) -> f64 {
    let points = polyline(shape).expect("a curve to measure against");
    points
        .windows(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let edge = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let length_squared: f64 = edge.iter().map(|axis| axis * axis).sum();
            let t = if length_squared == 0.0 { 0.0 } else { ((0..3).map(|axis| (point[axis] - a[axis]) * edge[axis]).sum::<f64>() / length_squared).clamp(0.0, 1.0) };
            distance(point, [a[0] + t * edge[0], a[1] + t * edge[1], a[2] + t * edge[2]])
        })
        .fold(f64::INFINITY, f64::min)
}

fn corners_of(surface: &Surface) -> ([[f64; 3]; 4], [f64; 3]) {
    let ((u0, u1), (v0, v1)) = surface.domain();
    let at = |u: f64, v: f64| {
        let point = surface.eval(u, v);
        [point.x, point.y, point.z]
    };
    ([at(u0, v0), at(u1, v0), at(u0, v1), at(u1, v1)], at((u0 + u1) / 2.0, (v0 + v1) / 2.0))
}
//#endregion 🔖️Measures

//#region 🔖️Extras
/// 📐️ Checks the statements of one shape output beyond kind, volume and area; panics with the reason.
pub fn assert_extras(name: &str, shape: &ShapeValue, expected: &Value, tolerance: f64) {
    let check = |key: &str, ok: bool, detail: String| assert!(ok, "{name}: {key}: {detail}");
    if let Some(wanted) = expected["length"].as_f64() {
        let found = length(shape).unwrap_or(f64::NAN);
        check("length", close(found, wanted, tolerance), format!("expected {wanted}, found {found}"));
    }
    if let Some(wanted) = expected["closed"].as_bool() {
        let found = matches!(&shape.root, ShapeRoot::Wire(wire) if wire.closed);
        check("closed", found == wanted, format!("expected {wanted}, found {found}"));
    }
    if expected.get("start").is_some() || expected.get("end").is_some() {
        let points = polyline(shape).expect("a curve with end points");
        for (key, found) in [("start", points[0]), ("end", points[points.len() - 1])] {
            if let Some(wanted) = expected.get(key) {
                check(key, near(found, triple(wanted), tolerance), format!("expected {wanted}, found {found:?}"));
            }
        }
    }
    if let Some(wanted) = expected.get("bbox") {
        let found = bbox(shape).unwrap_or_else(|| panic!("{name}: a shape with a bounding box"));
        for (side, label) in [(0usize, "min"), (1, "max")] {
            let wanted = triple(&wanted[side]);
            check("bbox", near(found[side], wanted, tolerance), format!("{label} expected {wanted:?}, found {:?}", found[side]));
        }
    }
    if let Some(wanted) = expected["faces"].as_u64() {
        let found = shape.components(GeometryKind::Face).len() as u64;
        check("faces", found == wanted, format!("expected {wanted}, found {found}"));
    }
    if let Some(through) = expected.get("through").and_then(Value::as_array) {
        for point in through {
            let found = distance_to_curve(shape, triple(point));
            check("through", found <= tolerance.max(1e-4), format!("{point} is {found} away from the curve"));
        }
    }
    let surface = match &shape.root {
        ShapeRoot::Surface { surface, .. } => Some(surface),
        _ => None,
    };
    if let Some(wanted) = expected.get("corners").and_then(Value::as_array) {
        let (corners, _) = corners_of(surface.expect("a surface"));
        for (found, wanted) in corners.iter().zip(wanted) {
            check("corners", near(*found, triple(wanted), tolerance), format!("expected {wanted}, found {found:?}"));
        }
    }
    if let Some(wanted) = expected.get("center") {
        let (_, center) = corners_of(surface.expect("a surface"));
        check("center", near(center, triple(wanted), tolerance), format!("expected {wanted}, found {center:?}"));
    }
    if let Some(plane) = expected.get("plane") {
        let (origin, normal) = (triple(&plane["origin"]), triple(&plane["normal"]));
        let length_of_normal = distance(normal, [0.0; 3]);
        for point in surface_points(surface.expect("a surface")) {
            let off = (0..3).map(|axis| (point[axis] - origin[axis]) * normal[axis]).sum::<f64>() / length_of_normal;
            check("plane", off.abs() <= tolerance, format!("{point:?} is {off} off the plane"));
        }
    }
}
//#endregion 🔖️Extras

//#region 🔖️Run
/// ✅️ Asserts one fixture case against an evaluation: the oracle support's checks, then the extra statements of every shape output.
pub fn assert_case(case: &Value, kind: &Kind, evaluation: &WidgetEvaluation, tolerance: f64) {
    oracle::assert_case(case, kind, evaluation, tolerance);
    let name = case["name"].as_str().unwrap_or("case");
    if let Some(expected) = case.get("outputs").and_then(Value::as_object) {
        for (port, wanted) in expected {
            if let Some(GeometryValue::Shape(shape)) = evaluation.outputs.get(port) {
                assert_extras(name, shape, wanted, tolerance);
            }
        }
    }
}

/// 🧪️ Runs every case of a fixture at fuel 1 and at unbounded fuel, asserts both against the fixture and requires the two evaluations to be equal value for value.
pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {
    let (fixture, cases) = cases(text);
    let tolerance = fixture["tolerance"].as_f64().expect("fixture tolerance");
    for case in &cases {
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let case_tolerance = case["tolerance"].as_f64().unwrap_or(tolerance);
        let stepped = oracle::evaluate(kind, inputs_of(kind, case), entries, 1);
        let whole = oracle::evaluate(kind, inputs_of(kind, case), entries, usize::MAX);
        assert_case(case, kind, &whole, case_tolerance);
        assert_eq!(stepped, whole, "{}: stepping at fuel 1 never changes the evaluation", case["name"]);
    }
    cases.len()
}
//#endregion 🔖️Run

//#region 🔖️Laws
fn shape_bytes(evaluation: &WidgetEvaluation) -> Vec<String> {
    evaluation.outputs.values().filter_map(|value| if let GeometryValue::Shape(shape) = value { Some(semio_framework_pack_json::to_json_string(&**shape)) } else { None }).collect()
}

/// 🎲️ Equal inputs give equal value bytes: every successful case is computed twice from scratch and its shape outputs must encode to the same JSON.
pub fn assert_deterministic(text: &str, entries: &[ComputeEntry]) -> usize {
    let (_, cases) = cases(text);
    let mut checked = 0;
    for case in cases.iter().filter(|case| case.get("fault").is_none()) {
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let first = oracle::evaluate(kind, inputs_of(kind, case), entries, usize::MAX);
        let second = oracle::evaluate(kind, inputs_of(kind, case), entries, 3);
        assert_eq!(shape_bytes(&first), shape_bytes(&second), "{}: equal inputs give equal shape bytes", case["name"]);
        assert!(!shape_bytes(&first).is_empty(), "{}: a shape output to compare", case["name"]);
        checked += 1;
    }
    checked
}

/// 🛑️ A job cancelled between two steps releases its session and answers `cancelled` from then on; a case that finishes within its first step cannot be cancelled and is skipped.
pub fn assert_cancellable(text: &str, entries: &[ComputeEntry]) -> usize {
    let (_, cases) = cases(text);
    let mut cancelled = 0;
    for case in cases.iter().filter(|case| case.get("fault").is_none()) {
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let entry = entries.iter().find(|entry| entry.id == kind.id).expect("registered kind");
        let mut job = (entry.start)(kind, inputs_of(kind, case));
        if let WidgetStep::Done(_) = job.step(1) {
            continue;
        }
        job.cancel();
        job.cancel();
        match job.step(1) {
            WidgetStep::Done(evaluation) => {
                let fault = evaluation.fault.expect("a cancelled job answers with a fault");
                assert_eq!(fault.code, "generation3d.geometry.cancelled", "{}: cancelled code", case["name"]);
                assert!(evaluation.outputs.is_empty() && !fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de, "{}: cancelled evaluation", case["name"]);
            }
            WidgetStep::Working { .. } => panic!("{}: a cancelled job must answer", case["name"]),
        }
        cancelled += 1;
    }
    cancelled
}

/// 🔁️ The shape of the first successful output of a case.
pub fn first_shape(text: &str, entries: &[ComputeEntry], name: &str) -> Arc<ShapeValue> {
    let (_, cases) = cases(text);
    let case = cases.iter().find(|case| case["name"] == name).unwrap_or_else(|| panic!("case {name}"));
    let kind = kind_of(case["kind"].as_str().expect("case kind"));
    match oracle::evaluate(kind, inputs_of(kind, case), entries, usize::MAX).outputs.remove("shape") {
        Some(GeometryValue::Shape(shape)) => shape,
        other => panic!("{name} has a shape output, found {other:?}"),
    }
}
//#endregion 🔖️Laws
