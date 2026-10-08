//! 🧰️ The fixture harness of the `mesh.*` unit tests: decodes language-agnostic fixture cases (meshes as `positions` and `faces`, selections as id arrays) into typed inputs, drives every compute at fuel one and at unbounded fuel,
//! requires both runs and a repeated run to be bit-identical, and compares mesh outputs by independently computed measures and by a canonical, vertex-numbering-free geometry.
//!
//! The fixture format is the one the TypeScript oracle reads, see `🧪️tests/🔬️oracle/🟦️.ts` beside this folder.

#![allow(dead_code)]

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
pub mod oracle;

use crate::standards::v1::subsets::any::schema::catalogue::{Kind, PortType, SelectionComponent};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Measure
/// 📏️ Independent measures of a polygon mesh, computed from its positions and polygon loops only.
#[derive(Clone, Debug, PartialEq)]
pub struct Measure {
    pub vertices: usize,
    pub faces: usize,
    pub edges: usize,
    pub boundary: usize,
    pub non_manifold: usize,
    pub oriented: bool,
    pub euler: i64,
    pub area: f64,
    pub volume: f64,
    pub bbox: [[f64; 3]; 2],
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 📏️ Measures a mesh.
pub fn measure(mesh: &HalfedgeMesh) -> Measure {
    let positions: Vec<[f64; 3]> = mesh.positions().iter().map(|p| p.map(f64::from)).collect();
    let polygons = mesh.polygons();
    let mut uses: BTreeMap<(u32, u32), usize> = BTreeMap::new();
    let mut directed: BTreeMap<(u32, u32), usize> = BTreeMap::new();
    let mut used = BTreeSet::new();
    let (mut area, mut volume) = (0.0, 0.0);
    let mut bbox = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
    for polygon in &polygons {
        for (corner, vertex) in polygon.iter().enumerate() {
            let next = polygon[(corner + 1) % polygon.len()];
            *uses.entry((*vertex.min(&next), *vertex.max(&next))).or_default() += 1;
            *directed.entry((*vertex, next)).or_default() += 1;
            used.insert(*vertex);
            for axis in 0..3 {
                bbox[0][axis] = bbox[0][axis].min(positions[*vertex as usize][axis]);
                bbox[1][axis] = bbox[1][axis].max(positions[*vertex as usize][axis]);
            }
        }
        let origin = positions[polygon[0] as usize];
        let mut sum = [0.0; 3];
        for corner in 1..polygon.len() - 1 {
            let (b, c) = (positions[polygon[corner] as usize], positions[polygon[corner + 1] as usize]);
            let normal = cross([b[0] - origin[0], b[1] - origin[1], b[2] - origin[2]], [c[0] - origin[0], c[1] - origin[1], c[2] - origin[2]]);
            for axis in 0..3 {
                sum[axis] += normal[axis];
            }
            let tetra = cross(b, c);
            volume += (origin[0] * tetra[0] + origin[1] * tetra[1] + origin[2] * tetra[2]) / 6.0;
        }
        area += (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt() / 2.0;
    }
    Measure {
        vertices: used.len(),
        faces: polygons.len(),
        edges: uses.len(),
        boundary: uses.values().filter(|count| **count == 1).count(),
        non_manifold: uses.values().filter(|count| **count > 2).count(),
        oriented: directed.values().all(|count| *count == 1),
        euler: used.len() as i64 - uses.len() as i64 + polygons.len() as i64,
        area,
        volume,
        bbox,
    }
}

fn point_key(point: [f32; 3]) -> String {
    point.iter().map(|axis| format!("{:.4}", if axis.abs() < 5e-5 { 0.0 } else { *axis })).collect::<Vec<_>>().join(",")
}

fn canonical_of(positions: &[[f32; 3]], polygons: &[Vec<u32>]) -> Vec<String> {
    let mut faces: Vec<String> = polygons
        .iter()
        .map(|polygon| {
            let keys: Vec<String> = polygon.iter().map(|vertex| point_key(positions[*vertex as usize])).collect();
            let start = (0..keys.len()).min_by_key(|start| (0..keys.len()).map(|k| keys[(start + k) % keys.len()].clone()).collect::<Vec<_>>()).unwrap_or(0);
            (0..keys.len()).map(|k| keys[(start + k) % keys.len()].clone()).collect::<Vec<_>>().join(" ")
        })
        .collect();
    faces.sort();
    faces
}

/// 🧬️ The mesh as a sorted list of faces, each a cyclic coordinate loop; independent of vertex and face numbering.
pub fn canonical(mesh: &HalfedgeMesh) -> Vec<String> {
    canonical_of(&mesh.positions(), &mesh.polygons())
}

fn bits(mesh: &HalfedgeMesh) -> (Vec<[u32; 3]>, Vec<Vec<u32>>) {
    (mesh.positions().iter().map(|p| p.map(f32::to_bits)).collect(), mesh.polygons())
}
//#endregion 🔖️Measure

//#region 🔖️Decode
fn json_of_default(port: &crate::standards::v1::subsets::any::schema::catalogue::Port) -> Option<Value> {
    let default = port.default.as_ref()?;
    serde_json::from_str(&semio_framework_pack_json::from_dsl_value(default).to_string()).ok()
}

/// 🕸️ The mesh a fixture lists as `positions` and `faces`, with the half-edge handles of `seams` pre-marked.
pub fn mesh_of(recipe: &Value) -> HalfedgeMesh {
    let mut mesh = oracle::build_mesh(recipe);
    if let Some(seams) = recipe.get("seams").and_then(Value::as_array) {
        let edges: Vec<EdgeId> = seams.iter().map(|id| EdgeId(id.as_u64().expect("seam handle") as u32)).collect();
        mesh.mark_uv_seam(&edges, true);
    }
    mesh
}

/// 🔌️ The typed inputs a fixture case states for its kind; a port the case leaves out takes the catalogue default.
pub fn inputs_of(kind: &Kind, case: &Value) -> WidgetInputs {
    let stated = case["inputs"].as_object().expect("case inputs");
    let mut values: BTreeMap<String, GeometryValue> = BTreeMap::new();
    for port in kind.inputs.iter().filter(|port| port.port_type != PortType::Selection) {
        let value = stated.get(&port.name).cloned().or_else(|| json_of_default(port));
        if let Some(value) = value {
            let decoded = if port.port_type == PortType::Mesh { GeometryValue::mesh(mesh_of(&value)) } else { oracle::decode(port, &value) };
            values.insert(port.name.clone(), decoded);
        }
    }
    for port in kind.inputs.iter().filter(|port| port.port_type == PortType::Selection) {
        let Some(value) = stated.get(&port.name).cloned().or_else(|| json_of_default(port)) else { continue };
        let selection = port.selection.as_ref().expect("selection source");
        let component = match selection.component {
            SelectionComponent::Face => SelectionKind::Face,
            SelectionComponent::Edge => SelectionKind::Edge,
            SelectionComponent::Vertex => SelectionKind::Vertex,
            SelectionComponent::Mode => match values.get(selection.mode_from.as_deref().expect("mode port")) {
                Some(GeometryValue::Text(name)) => SelectionKind::parse(name).expect("mode names an element"),
                other => panic!("mode input missing: {other:?}"),
            },
        };
        let ids = value.as_array().expect("selection ids").iter().map(|id| id.as_u64().expect("selection id")).collect();
        values.insert(port.name.clone(), GeometryValue::Selection(SelectionValue { component, ids }));
    }
    WidgetInputs::new(case["name"].as_str().unwrap_or("case"), kind, values)
}
//#endregion 🔖️Decode

//#region 🔖️Compare
fn within(actual: f64, expected: &Value, tolerance: f64) -> bool {
    match (expected.as_f64(), expected.get("min").and_then(Value::as_f64), expected.get("max").and_then(Value::as_f64)) {
        (Some(wanted), _, _) => oracle::close(actual, wanted, tolerance),
        (None, Some(min), Some(max)) => actual >= min && actual <= max,
        _ => false,
    }
}

fn expect_count(name: &str, actual: usize, expected: &Value) -> Result<(), String> {
    if within(actual as f64, expected, 0.0) {
        Ok(())
    } else {
        Err(format!("{name}: expected {expected}, found {actual}"))
    }
}

fn expect_real(name: &str, actual: f64, expected: &Value, tolerance: f64) -> Result<(), String> {
    if within(actual, expected, tolerance) {
        Ok(())
    } else {
        Err(format!("{name}: expected {expected}, found {actual}"))
    }
}

fn mesh_matches(mesh: &HalfedgeMesh, expected: &Value, tolerance: f64) -> Result<(), String> {
    let m = measure(mesh);
    for (key, value) in expected.as_object().ok_or("mesh expectation must be an object")? {
        match key.as_str() {
            "vertices" => expect_count(key, m.vertices, value)?,
            "faces" => expect_count(key, m.faces, value)?,
            "edges" => expect_count(key, m.edges, value)?,
            "boundaryEdges" => expect_count(key, m.boundary, value)?,
            "nonManifoldEdges" => expect_count(key, m.non_manifold, value)?,
            "euler" => {
                if value.as_i64() != Some(m.euler) {
                    return Err(format!("euler: expected {value}, found {}", m.euler));
                }
            }
            "closed" => {
                if value.as_bool() != Some(m.boundary == 0 && m.non_manifold == 0) {
                    return Err(format!("closed: expected {value}, found boundary {} non-manifold {}", m.boundary, m.non_manifold));
                }
            }
            "oriented" => {
                if value.as_bool() != Some(m.oriented) {
                    return Err(format!("oriented: expected {value}, found {}", m.oriented));
                }
            }
            "area" => expect_real(key, m.area, value, tolerance)?,
            "volume" => expect_real(key, m.volume, value, tolerance)?,
            "bbox" => {
                for (side, row) in value.as_array().ok_or("bbox must be two triples")?.iter().enumerate() {
                    for axis in 0..3 {
                        expect_real(&format!("bbox[{side}][{axis}]"), m.bbox[side][axis], &row[axis], tolerance)?;
                    }
                }
            }
            "positions" => {}
            "polygons" => {
                let positions: Vec<[f32; 3]> = expected["positions"].as_array().ok_or("polygons need positions")?.iter().map(|p| [0, 1, 2].map(|axis| p[axis].as_f64().expect("coordinate") as f32)).collect();
                let polygons: Vec<Vec<u32>> = value.as_array().ok_or("polygons")?.iter().map(|loop_ids| loop_ids.as_array().expect("loop").iter().map(|id| id.as_u64().expect("index") as u32).collect()).collect();
                let (found, wanted) = (canonical(mesh), canonical_of(&positions, &polygons));
                if found != wanted {
                    return Err(format!("geometry differs:\nexpected {wanted:#?}\nfound    {found:#?}"));
                }
            }
            "vertexNormals" => {
                for (id, wanted) in value.as_object().ok_or("vertexNormals")? {
                    let normal = mesh.vertex_normal(semio_framework_3d::mesh::VertexId(id.parse().map_err(|_| "vertex id")?)).map_err(|error| error.to_string())?;
                    for axis in 0..3 {
                        expect_real(&format!("normal {id}[{axis}]"), f64::from(normal.0[axis]), &wanted[axis], tolerance)?;
                    }
                }
            }
            "seams" => {
                let wanted: BTreeSet<u64> = value.as_array().ok_or("seams")?.iter().filter_map(Value::as_u64).collect();
                let found: BTreeSet<u64> = (0..mesh.halfedge_count() as u64).filter(|id| mesh.is_uv_seam(EdgeId(*id as u32))).collect();
                if found != wanted {
                    return Err(format!("seams: expected {wanted:?}, found {found:?}"));
                }
            }
            "uv" => {
                let mut islands = BTreeSet::new();
                for face in 0..mesh.face_count() {
                    for halfedge in mesh.face_halfedge_ids(FaceId(face as u32)).map_err(|error| error.to_string())? {
                        let uv = mesh.corner_uv(EdgeId(halfedge)).map_err(|error| error.to_string())?;
                        if !uv[0].is_finite() || !uv[1].is_finite() || !(0.0..=1.0).contains(&uv[0]) || !(0.0..=1.0).contains(&uv[1]) {
                            return Err(format!("uv outside the unit square: {uv:?}"));
                        }
                        islands.insert((uv[0].to_bits(), uv[1].to_bits()));
                    }
                }
                if let Some(minimum) = value["distinctCorners"].as_u64() {
                    if (islands.len() as u64) < minimum {
                        return Err(format!("uv: expected at least {minimum} distinct corner coordinates, found {}", islands.len()));
                    }
                }
            }
            other => return Err(format!("unknown mesh expectation {other}")),
        }
    }
    Ok(())
}

fn text_matches(text: &str, expected: &Value) -> Result<(), String> {
    for (key, value) in expected.as_object().ok_or("text expectation must be an object")? {
        match key.as_str() {
            "lines" => expect_count(key, text.lines().count(), value)?,
            "startsWith" => {
                if !text.starts_with(value.as_str().ok_or("startsWith")?) {
                    return Err(format!("text does not start with {value}"));
                }
            }
            "contains" => {
                for needle in value.as_array().ok_or("contains")? {
                    if !text.contains(needle.as_str().ok_or("needle")?) {
                        return Err(format!("text does not contain {needle}"));
                    }
                }
            }
            "json" => {
                let parsed: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
                if !json_close(&parsed, value) {
                    return Err(format!("json differs: expected {value}, found {parsed}"));
                }
            }
            "binary" => binary_matches(text, value)?,
            "base64Prefix" => {
                if !text.starts_with(value.as_str().ok_or("base64Prefix")?) {
                    return Err(format!("base64 does not start with {value}"));
                }
            }
            other => return Err(format!("unknown text expectation {other}")),
        }
    }
    Ok(())
}

fn json_close(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Number(a), Value::Number(b)) => oracle::close(a.as_f64().unwrap_or(f64::NAN), b.as_f64().unwrap_or(f64::NAN), 1e-6),
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| json_close(a, b)),
        (Value::Object(a), Value::Object(b)) => b.iter().all(|(key, wanted)| a.get(key).is_some_and(|found| json_close(found, wanted))),
        (a, b) => a == b,
    }
}

fn read_f32(bytes: &[u8], at: usize) -> f64 {
    f64::from(f32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]))
}

fn read_u32(bytes: &[u8], at: usize) -> usize {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize
}

/// 🧮️ Triangle count, area and signed volume of triangles given as nine coordinates each.
fn soup_measures(triangles: &[[[f64; 3]; 3]]) -> (usize, f64, f64) {
    let (mut area, mut volume) = (0.0, 0.0);
    for [a, b, c] in triangles {
        let normal = cross([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
        area += (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt() / 2.0;
        let tetra = cross(*b, *c);
        volume += (a[0] * tetra[0] + a[1] * tetra[1] + a[2] * tetra[2]) / 6.0;
    }
    (triangles.len(), area, volume)
}

/// 📼️ Decodes base64 written by an exporter with the parsers of this file only and checks the stated file facts.
fn binary_matches(encoded: &str, expected: &Value) -> Result<(), String> {
    let bytes = semio_framework_io_base64::base64_standard_decode(encoded).map_err(|error| error.to_string())?;
    let triangles: Vec<[[f64; 3]; 3]> = match expected["format"].as_str() {
        Some("stl") => {
            let count = read_u32(&bytes, 80);
            if bytes.len() != 84 + 50 * count {
                return Err(format!("stl length {} does not fit {count} triangles", bytes.len()));
            }
            (0..count).map(|t| [0, 1, 2].map(|corner| [0, 1, 2].map(|axis| read_f32(&bytes, 84 + 50 * t + 12 + 12 * corner + 4 * axis)))).collect()
        }
        Some("glb") => {
            if &bytes[0..4] != b"glTF" || read_u32(&bytes, 4) != 2 || read_u32(&bytes, 8) != bytes.len() {
                return Err("glb header is wrong".into());
            }
            let json_length = read_u32(&bytes, 12);
            let json: Value = serde_json::from_slice(&bytes[20..20 + json_length]).map_err(|error| error.to_string())?;
            let binary = &bytes[20 + json_length + 8..];
            let view = |accessor: &Value| -> (usize, usize, usize) {
                let view = &json["bufferViews"][accessor["bufferView"].as_u64().expect("view") as usize];
                (view["byteOffset"].as_u64().unwrap_or(0) as usize + accessor["byteOffset"].as_u64().unwrap_or(0) as usize, accessor["count"].as_u64().expect("count") as usize, accessor["componentType"].as_u64().expect("type") as usize)
            };
            let primitive = &json["meshes"][0]["primitives"][0];
            let positions = &json["accessors"][primitive["attributes"]["POSITION"].as_u64().expect("position") as usize];
            let (at, count, _) = view(positions);
            let points: Vec<[f64; 3]> = (0..count).map(|v| [0, 1, 2].map(|axis| read_f32(binary, at + 12 * v + 4 * axis))).collect();
            let indices: Vec<usize> = match primitive.get("indices") {
                Some(accessor) => {
                    let (at, count, component) = view(&json["accessors"][accessor.as_u64().expect("indices") as usize]);
                    (0..count).map(|i| if component == 5125 { read_u32(binary, at + 4 * i) } else { u16::from_le_bytes([binary[at + 2 * i], binary[at + 2 * i + 1]]) as usize }).collect()
                }
                None => (0..count).collect(),
            };
            indices.chunks(3).map(|t| [points[t[0]], points[t[1]], points[t[2]]]).collect()
        }
        other => return Err(format!("unknown binary format {other:?}")),
    };
    let (count, area, volume) = soup_measures(&triangles);
    if let Some(wanted) = expected["bytes"].as_u64() {
        if wanted as usize != bytes.len() {
            return Err(format!("expected {wanted} bytes, found {}", bytes.len()));
        }
    }
    expect_count("triangles", count, &expected["triangles"])?;
    expect_real("area", area, &expected["area"], 1e-5)?;
    expect_real("volume", volume, &expected["volume"], 1e-5)
}

/// ⚖️ Whether an output value matches the fixture's expectation: meshes by measures and canonical geometry, structured text by its stated properties, everything else as the shared support compares it.
pub fn matches(actual: &GeometryValue, port: &crate::standards::v1::subsets::any::schema::catalogue::Port, expected: &Value, tolerance: f64) -> Result<(), String> {
    match actual {
        GeometryValue::Mesh(mesh) => mesh_matches(mesh, expected, tolerance),
        GeometryValue::Text(text) if expected.is_object() => text_matches(text, expected),
        other => oracle::matches(other, port, expected, tolerance),
    }
}

/// ✅️ Asserts one case against an evaluation.
pub fn assert_case(case: &Value, kind: &Kind, evaluation: &WidgetEvaluation, tolerance: f64) {
    let name = case["name"].as_str().unwrap_or("case");
    if let Some(expected) = case.get("fault") {
        let fault = evaluation.fault.as_ref().unwrap_or_else(|| panic!("{name}: expected fault {expected}, found outputs {:?}", evaluation.outputs.keys().collect::<Vec<_>>()));
        assert_eq!(fault.code, expected["code"].as_str().expect("fault code"), "{name}: fault code ({fault})");
        assert_eq!(fault.port.as_deref(), expected["port"].as_str(), "{name}: fault port");
        assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de, "{name}: fault carries distinct EN and DE text");
        assert!(evaluation.outputs.is_empty(), "{name}: a faulted evaluation has no outputs");
        return;
    }
    assert!(evaluation.fault.is_none(), "{name}: unexpected fault {:?}", evaluation.fault);
    let expected = case["outputs"].as_object().unwrap_or_else(|| panic!("{name}: case states neither outputs nor a fault"));
    let found: Vec<&str> = evaluation.outputs.keys().map(String::as_str).collect();
    let mut wanted: Vec<&str> = expected.keys().map(String::as_str).collect();
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

/// 🔒️ The exact bits of every mesh output, so two runs can be required to be identical.
fn fingerprint(evaluation: &WidgetEvaluation) -> BTreeMap<String, (Vec<[u32; 3]>, Vec<Vec<u32>>)> {
    evaluation.outputs.iter().filter_map(|(port, value)| if let GeometryValue::Mesh(mesh) = value { Some((port.clone(), bits(mesh))) } else { None }).collect()
}

/// 🧪️ Runs every case at fuel 1 and at unbounded fuel, asserts both against the fixture, and requires the two runs and a repeated run to agree bit for bit on every mesh output.
pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {
    let (fixture, cases) = oracle::cases(text);
    let tolerance = fixture["tolerance"].as_f64().expect("fixture tolerance");
    for case in &cases {
        let kind = oracle::kind_of(case["kind"].as_str().expect("case kind"));
        let case_tolerance = case["tolerance"].as_f64().unwrap_or(tolerance);
        let stepped = oracle::evaluate(kind, inputs_of(kind, case), entries, 1);
        let whole = oracle::evaluate(kind, inputs_of(kind, case), entries, usize::MAX);
        let again = oracle::evaluate(kind, inputs_of(kind, case), entries, 7);
        assert_case(case, kind, &whole, case_tolerance);
        assert_case(case, kind, &stepped, case_tolerance);
        let name = case["name"].as_str().unwrap_or("case");
        assert_eq!(stepped.fault, whole.fault, "{name}: fault at fuel 1 equals the whole run");
        assert_eq!(stepped.quality, whole.quality, "{name}: quality at fuel 1 equals the whole run");
        assert_eq!(fingerprint(&stepped), fingerprint(&whole), "{name}: fuel 1 and unbounded fuel build the same mesh bit for bit");
        assert_eq!(fingerprint(&again), fingerprint(&whole), "{name}: a repeated run builds the same mesh bit for bit");
        assert_eq!(again.fault, whole.fault, "{name}: a repeated run faults the same way");
    }
    cases.len()
}
//#endregion 🔖️Compare
