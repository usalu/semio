//! 🧰️ Shared support of the fixture-driven unit tests of the O1 compute lanes: decodes the language-agnostic fixture cases into typed widget inputs,
//! builds the shape and mesh recipes they name, drives a started job to completion at several fuel grants and compares typed outputs with the fixture.
//!
//! The fixture format is the one the TypeScript oracle tests read, see `🧪️tests/🧰️oracle-support/🟦️.ts`.

#![allow(dead_code)]

use crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue;
use crate::standards::v1::subsets::any::schema::catalogue::{Kind, Port, PortType};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{GeometryHandle, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::queries::analysis::{edge_table, face_table, ShapeScope};
use semio_framework_3d::mesh::HalfedgeMesh;
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Fixture
/// 📚️ The kind with this id.
pub fn kind_of(id: &str) -> &'static Kind {
    catalogue().kind(id).unwrap_or_else(|| panic!("catalogue kind {id}"))
}

/// 🧾️ The cases of a fixture file.
pub fn cases(text: &str) -> (Value, Vec<Value>) {
    let fixture: Value = serde_json::from_str(text).expect("fixture parses");
    let cases = fixture["cases"].as_array().expect("fixture cases").clone();
    (fixture, cases)
}

fn f(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("number expected, found {value}"))
}

fn triple(value: &Value) -> [f64; 3] {
    let items = value.as_array().unwrap_or_else(|| panic!("triple expected, found {value}"));
    [f(&items[0]), f(&items[1]), f(&items[2])]
}
//#endregion 🔖️Fixture

//#region 🔖️Recipes
fn session_handle(session: &mut KernelSession, recipe: &Value) -> GeometryHandle {
    match recipe["recipe"].as_str().expect("recipe name") {
        "box" => {
            let [w, d, h] = triple(&recipe["size"]);
            session.brep().box_prim_sync(w, d, h).expect("box")
        }
        "sphere" => session.brep().sphere_prim_sync(f(&recipe["radius"])).expect("sphere"),
        "cylinder" => session.brep().cylinder_prim_sync(f(&recipe["radius"]), f(&recipe["height"])).expect("cylinder"),
        "translate" => {
            let inner = session_handle(session, &recipe["of"]);
            session.brep().translate_sync(&inner, triple(&recipe["by"])).expect("translate")
        }
        "rotate" => {
            let inner = session_handle(session, &recipe["of"]);
            session.brep().rotate_sync(&inner, triple(&recipe["axis"]), f(&recipe["angle"])).expect("rotate")
        }
        "rectangleWire" => session.brep().rectangle_wire_sync(f(&recipe["width"]), f(&recipe["height"])).expect("rectangle wire"),
        "line" => session.brep().line_curve_sync(triple(&recipe["start"]), triple(&recipe["end"])).expect("line curve"),
        "circle" => session.brep().circle_curve_sync(triple(&recipe["center"]), triple(&recipe["normal"]), f(&recipe["radius"])).expect("circle curve"),
        "cut" => {
            let (target, tool) = (session_handle(session, &recipe["target"]), session_handle(session, &recipe["tool"]));
            session.brep().cut_sync(&target, &tool).expect("cut")
        }
        "fuse" => {
            let (first, second) = (session_handle(session, &recipe["first"]), session_handle(session, &recipe["second"]));
            session.brep().fuse_sync(&first, &second).expect("fuse")
        }
        "compound" => {
            let members: Vec<GeometryHandle> = recipe["of"].as_array().expect("compound members").iter().map(|member| session_handle(session, member)).collect();
            session.brep().compound_sync(&members).expect("compound")
        }
        "vertex" => session.brep().vertex_sync(triple(&recipe["point"])).expect("vertex"),
        "face" => {
            let whole = session_handle(session, &recipe["of"]);
            let wanted = triple(&recipe["normal"]);
            let faces = session.brep().deconstruct_sync(&whole).expect("deconstruct").faces;
            faces
                .into_iter()
                .find(|handle| {
                    let value = session.export(handle).expect("face export");
                    let ShapeRoot::Face(id) = value.root else { return false };
                    let rows = face_table(&value.body, &ShapeScope::face(id), 1e-6).expect("face table");
                    rows.first().and_then(|row| row.normal).is_some_and(|normal| (0..3).all(|axis| (normal[axis] - wanted[axis]).abs() < 1e-9))
                })
                .expect("a face with that normal")
        }
        "curvedFace" => {
            let whole = session_handle(session, &recipe["of"]);
            let faces = session.brep().deconstruct_sync(&whole).expect("deconstruct").faces;
            faces
                .into_iter()
                .find(|handle| {
                    let value = session.export(handle).expect("face export");
                    let ShapeRoot::Face(id) = value.root else { return false };
                    face_table(&value.body, &ShapeScope::face(id), 1e-6).expect("face table").first().is_some_and(|row| row.normal.is_none())
                })
                .expect("a curved face")
        }
        "edge" => {
            let whole = session_handle(session, &recipe["of"]);
            let (from, to) = (triple(&recipe["from"]), triple(&recipe["to"]));
            let edges = session.brep().deconstruct_sync(&whole).expect("deconstruct").edges;
            let near = |a: [f64; 3], b: [f64; 3]| (0..3).all(|axis| (a[axis] - b[axis]).abs() < 1e-9);
            edges
                .into_iter()
                .find(|handle| {
                    let value = session.export(handle).expect("edge export");
                    let ShapeRoot::Edge(id) = value.root else { return false };
                    let rows = edge_table(&value.body, &ShapeScope::edge(id)).expect("edge table");
                    rows.first().is_some_and(|row| (near(row.start, from) && near(row.end, to)) || (near(row.start, to) && near(row.end, from)))
                })
                .expect("an edge with those ends")
        }
        "curvedEdge" => {
            let whole = session_handle(session, &recipe["of"]);
            let edges = session.brep().deconstruct_sync(&whole).expect("deconstruct").edges;
            edges
                .into_iter()
                .find(|handle| {
                    let value = session.export(handle).expect("edge export");
                    let ShapeRoot::Edge(id) = value.root else { return false };
                    edge_table(&value.body, &ShapeScope::edge(id)).expect("edge table").first().is_some_and(|row| row.curve_kind != semio_framework_3d::brep::queries::analysis::CurveKind::Line)
                })
                .expect("a curved edge")
        }
        other => panic!("unknown shape recipe {other}"),
    }
}

/// 🧊️ Builds the shape a fixture recipe names in a fresh kernel session.
pub fn build_shape(recipe: &Value) -> ShapeValue {
    let mut session = KernelSession::new();
    let handle = session_handle(&mut session, recipe);
    session.export(&handle).expect("recipe export")
}

/// 🕸️ Builds the polygon mesh a fixture lists as positions and faces.
pub fn build_mesh(recipe: &Value) -> HalfedgeMesh {
    let positions: Vec<[f32; 3]> = recipe["positions"].as_array().expect("positions").iter().map(|p| triple(p).map(|axis| axis as f32)).collect();
    let faces: Vec<Vec<u32>> = recipe["faces"].as_array().expect("faces").iter().map(|face| face.as_array().expect("face").iter().map(|index| index.as_u64().expect("index") as u32).collect()).collect();
    HalfedgeMesh::from_faces(&positions, &faces).expect("mesh builds")
}
//#endregion 🔖️Recipes

//#region 🔖️Decode
fn typed(value: &Value) -> GeometryValue {
    let payload = &value["value"];
    match value["type"].as_str().expect("typed value") {
        "number" => GeometryValue::Number(f(payload)),
        "integer" => GeometryValue::Integer(payload.as_i64().expect("integer")),
        "boolean" => GeometryValue::Boolean(payload.as_bool().expect("boolean")),
        "text" => GeometryValue::Text(payload.as_str().expect("text").to_string()),
        "vector" => GeometryValue::Vector(triple(payload)),
        "point" => GeometryValue::Point(triple(payload)),
        other => panic!("unknown typed value {other}"),
    }
}

fn element(port_type: PortType, value: &Value) -> GeometryValue {
    match port_type {
        PortType::Number | PortType::Length | PortType::Angle => GeometryValue::Number(f(value)),
        PortType::Integer => GeometryValue::Integer(value.as_i64().unwrap_or_else(|| panic!("integer expected, found {value}"))),
        PortType::Boolean => GeometryValue::Boolean(value.as_bool().expect("boolean")),
        PortType::Text | PortType::Enum => GeometryValue::Text(value.as_str().expect("text").to_string()),
        PortType::Vector => GeometryValue::Vector(triple(value)),
        PortType::Point => GeometryValue::Point(triple(value)),
        PortType::Plane => GeometryValue::Plane(PlaneValue { origin: triple(&value["origin"]), normal: triple(&value["normal"]) }),
        PortType::Shape | PortType::Shapes => GeometryValue::shape(build_shape(value)),
        PortType::Mesh => GeometryValue::mesh(build_mesh(value)),
        PortType::Any => typed(value),
        PortType::Selection => panic!("selections are decoded against their shape"),
    }
}

/// 🔢️ The value a fixture states for a port, by the port's catalogue type.
pub fn decode(port: &Port, value: &Value) -> GeometryValue {
    if port.list || port.port_type == PortType::Shapes {
        return GeometryValue::List(value.as_array().expect("list").iter().map(|item| element(port.port_type, item)).collect());
    }
    element(port.port_type, value)
}

/// 🧲️ A selection resolved on a shape: faces by their planar outward normals, or literal labels.
fn selection(value: &Value, shape: &ShapeValue) -> GeometryValue {
    if let Some(labels) = value.get("labels") {
        return GeometryValue::Selection(SelectionValue { component: SelectionKind::Face, ids: labels.as_array().expect("labels").iter().map(|label| label.as_u64().expect("label")).collect() });
    }
    let ShapeRoot::Solid(id) = shape.root else { panic!("a normal selection needs a solid") };
    let rows = face_table(&shape.body, &ShapeScope::solid(id), 1e-6).expect("face table");
    let ids = value["normals"]
        .as_array()
        .expect("normals")
        .iter()
        .map(|wanted| {
            let wanted = triple(wanted);
            rows.iter().find(|row| row.normal.is_some_and(|normal| (0..3).all(|axis| (normal[axis] - wanted[axis]).abs() < 1e-9))).expect("a face with that normal").label
        })
        .collect();
    GeometryValue::Selection(SelectionValue { component: SelectionKind::Face, ids })
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
        let resolved = selection(value, shape);
        values.insert(port.name.clone(), resolved);
    }
    WidgetInputs::new(case["name"].as_str().unwrap_or("case"), kind, values)
}
//#endregion 🔖️Decode

//#region 🔖️Drive
/// 🏃️ Steps a started job to its evaluation with `fuel` per call, asserting that progress stays inside 0..1 and never decreases.
pub fn drive(mut job: Box<dyn WidgetJob>, fuel: usize) -> WidgetEvaluation {
    let mut last = 0.0f32;
    for _ in 0..1_000_000 {
        match job.step(fuel) {
            WidgetStep::Working { progress } => {
                assert!((0.0..=1.0).contains(&progress), "progress {progress} outside 0..=1");
                assert!(progress + 1e-6 >= last, "progress went back from {last} to {progress}");
                last = progress;
            }
            WidgetStep::Done(evaluation) => return evaluation,
        }
    }
    panic!("the job never finished")
}

/// 🚀️ Starts the compute registered for the kind of `inputs` and drives it with `fuel` per call.
pub fn evaluate(kind: &Kind, inputs: WidgetInputs, entries: &[ComputeEntry], fuel: usize) -> WidgetEvaluation {
    let entry = entries.iter().find(|entry| entry.id == kind.id).unwrap_or_else(|| panic!("{} is registered in this category", kind.id));
    drive((entry.start)(kind, inputs), fuel)
}
//#endregion 🔖️Drive

//#region 🔖️Compare
/// 🔢️ Absolute below magnitude one, relative above.
pub fn close(actual: f64, expected: f64, tolerance: f64) -> bool {
    actual.is_finite() && (actual - expected).abs() <= tolerance * expected.abs().max(1.0)
}

/// ⚖️ Whether an output value equals the fixture's expectation within `tolerance`; shape outputs are checked by their stated `kind`, `volume` and `area`.
pub fn matches(actual: &GeometryValue, port: &Port, expected: &Value, tolerance: f64) -> Result<(), String> {
    let list_of = |items: &[GeometryValue], expected: &[Value]| -> Result<(), String> {
        if items.len() != expected.len() {
            return Err(format!("expected {} items, found {}", expected.len(), items.len()));
        }
        items.iter().zip(expected).try_for_each(|(item, wanted)| matches(item, &Port { list: false, ..port.clone() }, wanted, tolerance))
    };
    match (actual, port.port_type) {
        (GeometryValue::List(items), _) if port.list || port.port_type == PortType::Shapes => list_of(items, expected.as_array().ok_or("list expected")?),
        (GeometryValue::Number(found), _) if close(*found, f(expected), tolerance) => Ok(()),
        (GeometryValue::Integer(found), _) if Some(*found) == expected.as_i64() => Ok(()),
        (GeometryValue::Boolean(found), _) if Some(*found) == expected.as_bool() => Ok(()),
        (GeometryValue::Text(found), _) if Some(found.as_str()) == expected.as_str() => Ok(()),
        (GeometryValue::Vector(found) | GeometryValue::Point(found), _) if (0..3).all(|axis| close(found[axis], triple(expected)[axis], tolerance)) => Ok(()),
        (GeometryValue::Plane(found), _) if (0..3).all(|axis| close(found.origin[axis], triple(&expected["origin"])[axis], tolerance) && close(found.normal[axis], triple(&expected["normal"])[axis], tolerance)) => Ok(()),
        (GeometryValue::Shape(found), _) => {
            let mut session = KernelSession::new();
            let imported = session.import(found).map_err(|fault| fault.message.en)?;
            let kind_name = format!("{:?}", found.kind()).to_lowercase();
            if let Some(wanted) = expected["kind"].as_str() {
                if wanted != kind_name {
                    return Err(format!("expected a {wanted}, found a {kind_name}"));
                }
            }
            if let Some(wanted) = expected["volume"].as_f64() {
                let volume = session.brep().volume_sync(&imported.handle).map_err(|error| error.to_string())?;
                if !close(volume, wanted, tolerance) {
                    return Err(format!("expected volume {wanted}, found {volume}"));
                }
            }
            if let Some(wanted) = expected["area"].as_f64() {
                let area = session.brep().area_sync(&imported.handle).map_err(|error| error.to_string())?;
                if !close(area, wanted, tolerance) {
                    return Err(format!("expected area {wanted}, found {area}"));
                }
            }
            Ok(())
        }
        _ => Err(format!("expected {expected}, found {actual:?}")),
    }
}

/// ✅️ Asserts one fixture case against an evaluation: either exactly the stated outputs (and no others) or exactly the stated fault with distinct EN and DE text.
pub fn assert_case(case: &Value, kind: &Kind, evaluation: &WidgetEvaluation, tolerance: f64) {
    let name = case["name"].as_str().unwrap_or("case");
    if let Some(expected) = case.get("fault") {
        let fault = evaluation.fault.as_ref().unwrap_or_else(|| panic!("{name}: expected fault {expected}, found outputs {:?}", evaluation.outputs));
        assert_eq!(fault.code, expected["code"].as_str().expect("fault code"), "{name}: fault code ({fault})");
        assert_eq!(fault.port.as_deref(), expected["port"].as_str(), "{name}: fault port");
        assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de, "{name}: fault carries distinct EN and DE text");
        assert!(evaluation.outputs.is_empty(), "{name}: a faulted evaluation has no outputs");
        return;
    }
    assert!(evaluation.fault.is_none(), "{name}: unexpected fault {:?}", evaluation.fault);
    let expected = case["outputs"].as_object().unwrap_or_else(|| panic!("{name}: case states neither outputs nor a fault"));
    let unchecked: Vec<&str> = case.get("unchecked").and_then(Value::as_array).map(|ports| ports.iter().filter_map(Value::as_str).collect()).unwrap_or_default();
    let found: Vec<&str> = evaluation.outputs.keys().map(String::as_str).collect();
    let mut wanted: Vec<&str> = expected.keys().map(String::as_str).chain(unchecked.iter().copied()).collect();
    wanted.sort();
    assert_eq!(found, wanted, "{name}: output ports");
    for (port_name, value) in expected {
        let port = kind.output(port_name).unwrap_or_else(|| panic!("{name}: {port_name} is an output of {}", kind.id));
        let actual = evaluation.outputs.get(port_name).expect("output present");
        if let Err(reason) = matches(actual, port, value, tolerance) {
            panic!("{name}: output {port_name}: {reason}");
        }
    }
}

/// 🧪️ Runs every case of a fixture at fuel 1 and at unbounded fuel, asserts both against the fixture and requires the two evaluations to be equal (stepping never changes a result).
pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {
    let (fixture, cases) = cases(text);
    let tolerance = fixture["tolerance"].as_f64().expect("fixture tolerance");
    for case in &cases {
        let kind = kind_of(case["kind"].as_str().expect("case kind"));
        let case_tolerance = case["tolerance"].as_f64().unwrap_or(tolerance);
        let stepped = evaluate(kind, inputs_of(kind, case), entries, 1);
        let whole = evaluate(kind, inputs_of(kind, case), entries, usize::MAX);
        assert_case(case, kind, &whole, case_tolerance);
        assert_case(case, kind, &stepped, case_tolerance);
        assert_eq!(stepped.fault, whole.fault, "{}: fault at fuel 1 equals the whole run", case["name"]);
        assert_eq!(stepped.quality, whole.quality, "{}: quality at fuel 1 equals the whole run", case["name"]);
    }
    cases.len()
}

/// 📚️ The kinds of a fixture, to assert it covers every kind of a category.
pub fn fixture_kinds(text: &str) -> Vec<String> {
    let (_, cases) = cases(text);
    let mut kinds: Vec<String> = cases.iter().map(|case| case["kind"].as_str().expect("case kind").to_string()).collect();
    kinds.sort();
    kinds.dedup();
    kinds
}

/// 📚️ Asserts that the compute table of a category lists exactly the catalogue kinds of that category, and that the fixture states at least one case per kind and one fault case per kind that can fault.
pub fn assert_category(category: &str, entries: &[ComputeEntry], text: &str) {
    let mut registered: Vec<&str> = entries.iter().map(|entry| entry.id).collect();
    registered.sort();
    let mut declared: Vec<&str> = catalogue().category(category).unwrap_or_else(|| panic!("category {category}")).kinds.iter().map(|kind| kind.id.as_str()).collect();
    declared.sort();
    assert_eq!(registered, declared, "{category}: the compute table lists exactly the catalogue kinds");
    assert_eq!(fixture_kinds(text), declared, "{category}: the fixture covers exactly the catalogue kinds");
}
//#endregion 🔖️Compare
