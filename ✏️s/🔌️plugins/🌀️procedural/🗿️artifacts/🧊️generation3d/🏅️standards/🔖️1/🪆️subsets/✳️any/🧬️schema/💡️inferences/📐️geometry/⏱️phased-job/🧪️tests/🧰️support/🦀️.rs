//! 🧰️ Shared test support of the O3 B-Rep categories: decodes the language-agnostic fixture cases into widget inputs, drives started jobs at several fuel grants, and checks shape outputs against analytic expectations, an independent `parry3d` reading of their tessellation and the pinned tessellation of the fixture.
//!
//! A fixture case names a catalogue `kind`, its `inputs` (shape recipes, selections by plane normal, edge ends or label, plain numbers) and either `outputs` or a `fault`. Shape outputs state `kind`, `volume`, `area`, `bbox`, component counts and the pinned `mesh` the TypeScript oracle recomputes with `three`.

#![allow(dead_code)]

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
pub mod base;

use crate::standards::v1::subsets::any::schema::catalogue::{Kind, Port, PortType};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{GeometryHandle, GeometryKind, MeshTransfer, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::queries::analysis::{edge_table, face_table, ShapeScope};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub use base::{drive, kind_of};

//#region 🔖️Recipes
fn f(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("number expected, found {value}"))
}

fn triple(value: &Value) -> [f64; 3] {
    let items = value.as_array().unwrap_or_else(|| panic!("triple expected, found {value}"));
    [f(&items[0]), f(&items[1]), f(&items[2])]
}

fn triples(value: &Value) -> Vec<[f64; 3]> {
    value.as_array().expect("points").iter().map(triple).collect()
}

/// 🧊️ Builds the shape a fixture recipe names in a fresh session: the recipes of the shared oracle support plus curves, wires, planes and planar faces.
pub fn build(recipe: &Value) -> ShapeValue {
    let mut session = KernelSession::new();
    let handle: GeometryHandle = match recipe["recipe"].as_str().expect("recipe name") {
        "line" => session.brep().line_curve_sync(triple(&recipe["start"]), triple(&recipe["end"])).expect("line"),
        "circle" => session.brep().circle_curve_sync(triple(&recipe["center"]), triple(&recipe["normal"]), f(&recipe["radius"])).expect("circle"),
        "arc" => session.brep().arc_curve_sync(triple(&recipe["center"]), triple(&recipe["normal"]), f(&recipe["radius"]), f(&recipe["start"]), f(&recipe["end"])).expect("arc"),
        "ellipse" => session.brep().ellipse_curve_sync(triple(&recipe["center"]), triple(&recipe["normal"]), f(&recipe["major"]), f(&recipe["minor"])).expect("ellipse"),
        "polyline" => session.brep().polyline_wire_sync(&triples(&recipe["points"])).expect("polyline"),
        "rectangleWire" => session.brep().rectangle_wire_sync(f(&recipe["width"]), f(&recipe["height"])).expect("rectangle"),
        "plane" => session.brep().plane_surface_sync(triple(&recipe["origin"]), triple(&recipe["normal"])).expect("plane"),
        "polygonFace" => session.brep().planar_face_from_points_sync(&triples(&recipe["points"])).expect("face"),
        "wireOfPolygon" => {
            let mut points = triples(&recipe["points"]);
            points.push(points[0]);
            session.brep().polyline_wire_sync(&points).expect("closed polyline")
        }
        "shelled" => {
            let mut inner = KernelSession::new();
            let whole = inner.import(&base::build_shape(&recipe["of"])).expect("import").handle;
            let shelled = inner.brep().shell_sync(&whole, f(&recipe["thickness"]), &[]).expect("shell");
            return inner.export(&shelled).expect("shell export");
        }
        _ => return base::build_shape(recipe),
    };
    session.export(&handle).expect("recipe export")
}
//#endregion 🔖️Recipes

//#region 🔖️Inputs
fn selection_of(port: &Port, value: &Value, shape: &ShapeValue) -> GeometryValue {
    let declared = match port.selection.as_ref().expect("selection port").component {
        crate::standards::v1::subsets::any::schema::catalogue::SelectionComponent::Edge => SelectionKind::Edge,
        crate::standards::v1::subsets::any::schema::catalogue::SelectionComponent::Vertex => SelectionKind::Vertex,
        _ => SelectionKind::Face,
    };
    let component = value.get("component").and_then(Value::as_str).and_then(SelectionKind::parse).unwrap_or(declared);
    if let Some(labels) = value.get("labels") {
        return GeometryValue::Selection(SelectionValue { component, ids: labels.as_array().expect("labels").iter().map(|label| label.as_u64().expect("label")).collect() });
    }
    if value.get("stale").is_some() {
        return GeometryValue::Selection(SelectionValue { component, ids: vec![u64::MAX - 7] });
    }
    let ShapeRoot::Solid(id) = shape.root else { panic!("a geometric selection needs a solid") };
    let near = |a: [f64; 3], b: [f64; 3]| (0..3).all(|axis| (a[axis] - b[axis]).abs() < 1e-9);
    let ids = if let Some(normals) = value.get("normals") {
        let rows = face_table(&shape.body, &ShapeScope::solid(id), 1e-6).expect("face table");
        normals.as_array().expect("normals").iter().map(|wanted| rows.iter().find(|row| row.normal.is_some_and(|normal| near(normal, triple(wanted)))).expect("a face with that normal").label).collect()
    } else if let Some(edges) = value.get("edges") {
        let rows = edge_table(&shape.body, &ShapeScope::solid(id)).expect("edge table");
        edges.as_array().expect("edges").iter().map(|wanted| {
            let (from, to) = (triple(&wanted["from"]), triple(&wanted["to"]));
            rows.iter().find(|row| (near(row.start, from) && near(row.end, to)) || (near(row.start, to) && near(row.end, from))).expect("an edge with those ends").label
        }).collect()
    } else if value.get("curved").is_some() {
        face_table(&shape.body, &ShapeScope::solid(id), 1e-6).expect("face table").iter().filter(|row| row.normal.is_none()).map(|row| row.label).collect()
    } else if value.get("all").is_some() {
        match component {
            SelectionKind::Edge => edge_table(&shape.body, &ShapeScope::solid(id)).expect("edge table").iter().map(|row| row.label).collect(),
            _ => face_table(&shape.body, &ShapeScope::solid(id), 1e-6).expect("face table").iter().map(|row| row.label).collect(),
        }
    } else {
        Vec::new()
    };
    GeometryValue::Selection(SelectionValue { component, ids })
}

fn decode(port: &Port, value: &Value) -> GeometryValue {
    match port.port_type {
        PortType::Shape if !port.list => GeometryValue::shape(build(value)),
        PortType::Shapes => GeometryValue::List(value.as_array().expect("shapes").iter().map(|item| GeometryValue::shape(build(item))).collect()),
        _ => base::decode(port, value),
    }
}

/// 🔌️ The typed inputs a fixture case states for its kind.
pub fn inputs_of(kind: &Kind, case: &Value) -> WidgetInputs {
    let stated = case["inputs"].as_object().expect("case inputs");
    let mut values: BTreeMap<String, GeometryValue> = BTreeMap::new();
    for port in kind.inputs.iter().filter(|port| port.port_type != PortType::Selection) {
        if let Some(value) = stated.get(&port.name) {
            values.insert(port.name.clone(), decode(port, value));
        }
    }
    for port in kind.inputs.iter().filter(|port| port.port_type == PortType::Selection) {
        let Some(value) = stated.get(&port.name) else { continue };
        let source = port.selection.as_ref().expect("selection source").source.clone();
        let Some(GeometryValue::Shape(shape)) = values.get(&source) else { panic!("selection source {source} is a shape") };
        let resolved = selection_of(port, value, shape);
        values.insert(port.name.clone(), resolved);
    }
    WidgetInputs::new(case["name"].as_str().unwrap_or("case"), kind, values)
}
//#endregion 🔖️Inputs

//#region 🔖️Measures
/// 📏️ What a tessellation measures: signed volume by the divergence theorem, summed triangle area and the bounding box.
#[derive(Clone, Copy, Debug)]
pub struct Measures {
    pub volume: f64,
    pub area: f64,
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub triangles: usize,
}

fn corner(transfer: &MeshTransfer, index: u32) -> [f64; 3] {
    let at = index as usize * 3;
    [transfer.position[at] as f64, transfer.position[at + 1] as f64, transfer.position[at + 2] as f64]
}

/// 📏️ Measures a tessellation.
pub fn measures(transfer: &MeshTransfer) -> Measures {
    let mut result = Measures { volume: 0.0, area: 0.0, min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3], triangles: transfer.index.len() / 3 };
    for triangle in transfer.index.chunks_exact(3) {
        let [a, b, c] = [corner(transfer, triangle[0]), corner(transfer, triangle[1]), corner(transfer, triangle[2])];
        let cross = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])];
        result.area += 0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        result.volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
        for point in [a, b, c] {
            for axis in 0..3 {
                result.min[axis] = result.min[axis].min(point[axis]);
                result.max[axis] = result.max[axis].max(point[axis]);
            }
        }
    }
    result
}

/// 🔮️ The third-party reading: `parry3d` recomputes the enclosed volume and the bounds of the triangle soup.
pub fn oracle(transfer: &MeshTransfer) -> (f64, [f64; 3], [f64; 3]) {
    let points: Vec<parry3d::math::Point<parry3d::math::Real>> = transfer.position.chunks_exact(3).map(|p| parry3d::math::Point::new(p[0], p[1], p[2])).collect();
    let indices: Vec<[u32; 3]> = transfer.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    let properties = parry3d::mass_properties::MassProperties::from_trimesh(1.0, &points, &indices);
    let aabb = parry3d::bounding_volume::Aabb::from_points(points.iter());
    (properties.mass() as f64, [aabb.mins.x as f64, aabb.mins.y as f64, aabb.mins.z as f64], [aabb.maxs.x as f64, aabb.maxs.y as f64, aabb.maxs.z as f64])
}

/// 🔺️ Tessellates a shape value.
pub fn tessellate(shape: &ShapeValue, deflection: f64) -> MeshTransfer {
    shape.tessellate(deflection).unwrap_or_else(|error| panic!("tessellation: {error}"))
}

/// 🧊️ The kernel's own volume and area of a shape; a compound sums its members.
pub fn kernel_measures(shape: &ShapeValue) -> (Option<f64>, Option<f64>) {
    let mut session = KernelSession::new();
    let imported = session.import(shape).expect("import");
    let members = if shape.kind() == GeometryKind::Compound { session.brep().explode_sync(&imported.handle).expect("explode") } else { vec![imported.handle] };
    let (mut volume, mut area) = (Some(0.0), Some(0.0));
    for member in &members {
        volume = volume.zip(session.brep().volume_sync(member).ok()).map(|(sum, part)| sum + part);
        area = area.zip(session.brep().area_sync(member).ok()).map(|(sum, part)| sum + part);
    }
    (volume.filter(|_| matches!(shape.kind(), GeometryKind::Solid | GeometryKind::Compound)), area)
}

/// 📌️ The pinned tessellation of a shape value: rounded to micro units so the file is stable.
pub fn pinned(shape: &ShapeValue, deflection: f64) -> Value {
    let transfer = tessellate(shape, deflection);
    let round = |value: f32| (value as f64 * 1e6).round() / 1e6;
    json!({ "deflection": deflection, "positions": transfer.position.iter().map(|value| round(*value)).collect::<Vec<_>>(), "indices": transfer.index })
}
//#endregion 🔖️Measures

//#region 🔖️Checks
fn components(shape: &ShapeValue, kind: GeometryKind) -> usize {
    shape.components(kind).len()
}

fn relative(actual: f64, expected: f64, tolerance: f64) -> bool {
    actual.is_finite() && (actual - expected).abs() <= tolerance * expected.abs().max(1.0)
}

/// 🔎️ Checks one shape output against its fixture description. `tolerance` bounds the kernel's own numbers, `meshTolerance` the numbers read from the tessellation.
pub fn check_shape(name: &str, shape: &ShapeValue, expected: &Value, tolerance: f64, mesh_tolerance: f64) -> Result<(), String> {
    shape.check().map_err(|error| format!("{name}: the value is inconsistent: {error}"))?;
    let kind_name = format!("{:?}", shape.kind()).to_lowercase();
    if let Some(wanted) = expected["kind"].as_str() {
        if wanted != kind_name {
            return Err(format!("{name}: expected a {wanted}, found a {kind_name}"));
        }
    }
    for (key, kind) in [("vertices", GeometryKind::Vertex), ("edges", GeometryKind::Edge), ("faces", GeometryKind::Face), ("shells", GeometryKind::Shell), ("solids", GeometryKind::Solid)] {
        if let Some(wanted) = expected[key].as_u64() {
            let found = components(shape, kind);
            if found as u64 != wanted {
                return Err(format!("{name}: expected {wanted} {key}, found {found}"));
            }
        }
    }
    let (volume, area) = kernel_measures(shape);
    for (key, found) in [("volumeWithin", volume), ("areaWithin", area)] {
        if let Some(range) = expected.get(key) {
            let (low, high) = (f(&range[0]), f(&range[1]));
            let found = found.ok_or_else(|| format!("{name}: the kernel measures no {key}"))?;
            if found < low - 1e-9 || found > high + 1e-9 {
                return Err(format!("{name}: {key} {found} outside [{low}, {high}]"));
            }
        }
    }
    if let Some(wanted) = expected["volume"].as_f64() {
        let found = volume.ok_or_else(|| format!("{name}: the kernel measures no volume"))?;
        if !relative(found, wanted, tolerance) {
            return Err(format!("{name}: expected kernel volume {wanted}, found {found}"));
        }
    }
    if let Some(wanted) = expected["area"].as_f64() {
        let found = area.ok_or_else(|| format!("{name}: the kernel measures no area"))?;
        if !relative(found, wanted, tolerance) {
            return Err(format!("{name}: expected kernel area {wanted}, found {found}"));
        }
    }
    let tessellable = matches!(shape.kind(), GeometryKind::Solid | GeometryKind::Compound | GeometryKind::Face | GeometryKind::Shell | GeometryKind::Wire);
    let measured = expected.get("bbox").is_some() || expected.get("mesh").is_some() || expected["volume"].is_number() || expected["area"].is_number();
    let deflection = expected["deflection"].as_f64().or(expected["mesh"]["deflection"].as_f64()).or(Some(0.1).filter(|_| tessellable && measured));
    let Some(deflection) = deflection else { return Ok(()) };
    let transfer = shape.tessellate(deflection).map_err(|error| format!("{name}: tessellation failed: {error}"))?;
    let read = measures(&transfer);
    if let Some(wanted) = expected["volume"].as_f64() {
        if !relative(read.volume, wanted, mesh_tolerance) {
            return Err(format!("{name}: expected tessellated volume {wanted}, found {}", read.volume));
        }
        let (third_party, ..) = oracle(&transfer);
        if !relative(third_party, read.volume, 1e-4) {
            return Err(format!("{name}: parry3d volume {third_party} disagrees with the tessellation volume {}", read.volume));
        }
    }
    if let Some(wanted) = expected["area"].as_f64() {
        if !relative(read.area, wanted, mesh_tolerance) {
            return Err(format!("{name}: expected tessellated area {wanted}, found {}", read.area));
        }
    }
    if let Some(bounds) = expected.get("bbox") {
        let (low, high) = (triple(&bounds[0]), triple(&bounds[1]));
        let (third_min, third_max) = { let (_, min, max) = oracle(&transfer); (min, max) };
        for axis in 0..3 {
            if !relative(read.min[axis], low[axis], mesh_tolerance) || !relative(read.max[axis], high[axis], mesh_tolerance) {
                return Err(format!("{name}: expected bounds {low:?}..{high:?}, found {:?}..{:?}", read.min, read.max));
            }
            if !relative(third_min[axis], read.min[axis], 1e-4) || !relative(third_max[axis], read.max[axis], 1e-4) {
                return Err(format!("{name}: parry3d bounds {third_min:?}..{third_max:?} disagree with the tessellation"));
            }
        }
    }
    if let Some(mesh) = expected.get("mesh") {
        let positions: Vec<f64> = mesh["positions"].as_array().ok_or("pinned positions")?.iter().map(f).collect();
        let indices: Vec<u64> = mesh["indices"].as_array().ok_or("pinned indices")?.iter().map(|index| index.as_u64().unwrap_or(u64::MAX)).collect();
        if positions.len() != transfer.position.len() || indices.len() != transfer.index.len() {
            return Err(format!("{name}: the tessellation has {} positions and {} indices, the fixture pins {} and {}", transfer.position.len(), transfer.index.len(), positions.len(), indices.len()));
        }
        if transfer.index.iter().zip(&indices).any(|(found, pinned)| *found as u64 != *pinned) || transfer.position.iter().zip(&positions).any(|(found, pinned)| (*found as f64 - pinned).abs() > 2e-6) {
            return Err(format!("{name}: the tessellation differs from the pinned mesh"));
        }
    }
    Ok(())
}

/// 🔎️ Checks a text output: `equals`, `prefix`, `contains` (all of), `lines` (count of lines starting with a prefix), `bytes` (length after base64 decoding) and `decodedPrefix`.
pub fn check_text(name: &str, found: &str, expected: &Value) -> Result<(), String> {
    if let Some(wanted) = expected["equals"].as_str() {
        if found != wanted {
            return Err(format!("{name}: text differs from {wanted:?}"));
        }
    }
    if expected["decimal"].as_bool() == Some(true) && (found.is_empty() || !found.bytes().all(|byte| byte.is_ascii_digit())) {
        return Err(format!("{name}: {found:?} is not a decimal number"));
    }
    if let Some(wanted) = expected["prefix"].as_str() {
        if !found.starts_with(wanted) {
            return Err(format!("{name}: text does not start with {wanted:?}"));
        }
    }
    for wanted in expected["contains"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        if !found.contains(wanted) {
            return Err(format!("{name}: text does not contain {wanted:?}"));
        }
    }
    for (needle, count) in expected["occurrences"].as_object().into_iter().flatten() {
        let counted = found.matches(needle.as_str()).count();
        if Some(counted as u64) != count.as_u64() {
            return Err(format!("{name}: expected {count} occurrences of {needle:?}, found {counted}"));
        }
    }
    if expected["base64"].as_bool() == Some(true) && semio_framework_value::base64_standard_decode(found).map_or(true, |bytes| bytes.is_empty()) {
        return Err(format!("{name}: not non-empty base64"));
    }
    for (prefix, count) in expected["lines"].as_object().into_iter().flatten() {
        let counted = found.lines().filter(|line| line.starts_with(prefix.as_str())).count();
        if Some(counted as u64) != count.as_u64() {
            return Err(format!("{name}: expected {count} lines starting with {prefix:?}, found {counted}"));
        }
    }
    if expected.get("bytes").is_some() || expected.get("decodedPrefix").is_some() {
        let bytes = semio_framework_value::base64_standard_decode(found).map_err(|error| format!("{name}: not base64: {error}"))?;
        if let Some(wanted) = expected["bytes"].as_u64() {
            if bytes.len() as u64 != wanted {
                return Err(format!("{name}: expected {wanted} decoded bytes, found {}", bytes.len()));
            }
        }
        if let Some(wanted) = expected["decodedPrefix"].as_str() {
            if !bytes.starts_with(wanted.as_bytes()) {
                return Err(format!("{name}: decoded bytes do not start with {wanted:?}"));
            }
        }
    }
    Ok(())
}

fn check_value(name: &str, actual: &GeometryValue, port: &Port, expected: &Value, tolerance: f64, mesh_tolerance: f64) -> Result<(), String> {
    match actual {
        GeometryValue::Shape(shape) => check_shape(name, shape, expected, tolerance, mesh_tolerance),
        GeometryValue::Text(text) if expected.is_object() => check_text(name, text, expected),
        GeometryValue::List(items) if expected.get("count").is_some() => {
            if Some(items.len() as u64) != expected["count"].as_u64() {
                return Err(format!("{name}: expected {} items, found {}", expected["count"], items.len()));
            }
            items.iter().enumerate().try_for_each(|(index, item)| check_value(&format!("{name}[{index}]"), item, port, &expected["each"], tolerance, mesh_tolerance))
        }
        GeometryValue::List(items) => {
            let wanted = expected.as_array().ok_or_else(|| format!("{name}: a list is expected as an array"))?;
            if items.len() != wanted.len() {
                return Err(format!("{name}: expected {} items, found {}", wanted.len(), items.len()));
            }
            items.iter().zip(wanted).enumerate().try_for_each(|(index, (item, wanted))| check_value(&format!("{name}[{index}]"), item, port, wanted, tolerance, mesh_tolerance))
        }
        other => base::matches(other, port, expected, tolerance),
    }
}

/// ✅️ Asserts one fixture case against an evaluation: exactly the stated outputs (and no others) or exactly the stated fault with distinct EN and DE text.
pub fn assert_case(case: &Value, kind: &Kind, evaluation: &WidgetEvaluation, tolerance: f64, mesh_tolerance: f64) {
    let name = case["name"].as_str().unwrap_or("case");
    if let Some(expected) = case.get("fault") {
        let fault = evaluation.fault.as_ref().unwrap_or_else(|| panic!("{name}: expected fault {expected}, found outputs {:?}", evaluation.outputs.keys().collect::<Vec<_>>()));
        assert_eq!(fault.code, expected["code"].as_str().expect("fault code"), "{name}: fault code ({fault})");
        assert_eq!(fault.port.as_deref(), expected["port"].as_str(), "{name}: fault port");
        assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de, "{name}: fault carries distinct EN and DE text");
        assert!(evaluation.outputs.is_empty(), "{name}: a faulted evaluation has no outputs");
        assert_eq!(evaluation.quality, kind.quality, "{name}: quality");
        return;
    }
    assert!(evaluation.fault.is_none(), "{name}: unexpected fault {:?}", evaluation.fault);
    let expected = case["outputs"].as_object().unwrap_or_else(|| panic!("{name}: case states neither outputs nor a fault"));
    let found: Vec<&String> = evaluation.outputs.keys().collect();
    let wanted: Vec<&String> = { let mut names: Vec<&String> = expected.keys().collect(); names.sort(); names };
    assert_eq!(found, wanted, "{name}: output ports");
    for (port_name, value) in expected {
        let port = kind.output(port_name).unwrap_or_else(|| panic!("{name}: {port_name} is an output of {}", kind.id));
        let actual = evaluation.outputs.get(port_name).expect("output present");
        if let Err(reason) = check_value(&format!("{name}.{port_name}"), actual, port, value, tolerance, mesh_tolerance) {
            panic!("{reason}");
        }
    }
}
//#endregion 🔖️Checks

//#region 🔖️Run
/// 🚀️ Starts the compute registered for the kind of `inputs`.
pub fn start(kind: &Kind, inputs: WidgetInputs, entries: &[ComputeEntry]) -> Box<dyn WidgetJob> {
    let entry = entries.iter().find(|entry| entry.id == kind.id).unwrap_or_else(|| panic!("{} is registered in this category", kind.id));
    (entry.start)(kind, inputs)
}

/// 🏃️ Counts the `Working` slices a job needs at `fuel` per call and returns them with its evaluation.
pub fn slices(mut job: Box<dyn WidgetJob>, fuel: usize) -> (usize, WidgetEvaluation) {
    let mut working = 0;
    let mut last = 0.0f32;
    for _ in 0..1_000_000 {
        match job.step(fuel) {
            WidgetStep::Working { progress } => {
                assert!((0.0..=1.0).contains(&progress), "progress {progress} outside 0..=1");
                assert!(progress + 1e-6 >= last, "progress went back from {last} to {progress}");
                last = progress;
                working += 1;
            }
            WidgetStep::Done(evaluation) => return (working, evaluation),
        }
    }
    panic!("the job never finished")
}

/// 🧪️ Runs every case of a fixture at fuel 1 and at unbounded fuel, asserts both against the fixture and requires the two evaluations to be equal and a repeated run to equal the first.
pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {
    let (fixture, cases) = base::cases(text);
    let tolerance = fixture["tolerance"].as_f64().expect("fixture tolerance");
    let mesh_tolerance = fixture["meshTolerance"].as_f64().unwrap_or(tolerance);
    for case in &cases {
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let (_, stepped) = slices(start(kind, inputs_of(kind, case), entries), 1);
        let (_, whole) = slices(start(kind, inputs_of(kind, case), entries), usize::MAX);
        let (_, again) = slices(start(kind, inputs_of(kind, case), entries), usize::MAX);
        assert_case(case, kind, &whole, case["tolerance"].as_f64().unwrap_or(tolerance), case["meshTolerance"].as_f64().unwrap_or(mesh_tolerance));
        assert_eq!(stepped, whole, "{}: fuel 1 gives the same evaluation as unbounded fuel", case["name"]);
        assert_eq!(again, whole, "{}: a repeated run is deterministic", case["name"]);
    }
    cases.len()
}

/// 📚️ Asserts that the compute table lists exactly the catalogue kinds of the category and the fixture covers each at least once.
pub fn assert_category(category: &str, entries: &[ComputeEntry], text: &str) {
    base::assert_category(category, entries, text);
}

/// ✍️ Rewrites the fixture at `path`: every shape output of every successful case gets its `deflection` and pinned `mesh` from a fresh run. Only used with `GEN3D_WRITE_FIXTURES=1`.
pub fn refresh(path: &str, text: &str, entries: &[ComputeEntry]) {
    let mut fixture: Value = serde_json::from_str(text).expect("fixture parses");
    let default_deflection = fixture["deflection"].as_f64().unwrap_or(0.1);
    let cases = fixture["cases"].as_array_mut().expect("cases");
    for case in cases.iter_mut() {
        if case.get("outputs").is_none() {
            continue;
        }
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let (_, evaluation) = slices(start(kind, inputs_of(kind, case), entries), usize::MAX);
        let deflection = case["deflection"].as_f64().unwrap_or(default_deflection);
        if case.get("pinInputs").and_then(Value::as_bool) == Some(true) {
            let mut meshes = serde_json::Map::new();
            for port in kind.inputs.iter().filter(|port| port.port_type == PortType::Shape && !port.list) {
                if let Some(recipe) = case["inputs"].get(&port.name) {
                    meshes.insert(port.name.clone(), pinned(&build(recipe), deflection));
                }
            }
            case["inputMeshes"] = Value::Object(meshes);
        }
        let outputs = case["outputs"].as_object_mut().expect("outputs");
        for (port, expected) in outputs.iter_mut() {
            let pin = |shape: &ShapeValue, expected: &mut Value| {
                if matches!(shape.kind(), GeometryKind::Solid | GeometryKind::Compound | GeometryKind::Face | GeometryKind::Shell | GeometryKind::Wire) && expected.is_object() && expected.get("pin").and_then(Value::as_bool) != Some(false) {
                    expected["mesh"] = pinned(shape, deflection);
                }
            };
            match evaluation.outputs.get(port) {
                Some(GeometryValue::Shape(shape)) => pin(shape, expected),
                Some(GeometryValue::List(items)) => {
                    for (item, wanted) in items.iter().zip(expected.as_array_mut().into_iter().flatten()) {
                        if let GeometryValue::Shape(shape) = item {
                            pin(shape, wanted);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut written = serde_json::to_string_pretty(&fixture).expect("fixture serialises");
    written.push('\n');
    std::fs::write(path, written).expect("fixture written");
}

/// ✍️ Whether the run refreshes the fixtures instead of comparing against them.
pub fn writing() -> bool {
    std::env::var_os("GEN3D_WRITE_FIXTURES").is_some()
}
//#endregion 🔖️Run
