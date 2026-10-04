//! 🧱️ Imports decoded PLY surfaces as editable polygon inputs with surface channels.
use crate::standards::v1::subsets::any::io::mesh_bridge::{import_polygon_meshes, polygon_import_payload, polygon_import_attribute, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_ply::PlySnapshot;
use semio_s_artifact_stdio_ply::schema::snapshot::{PlyElement, PlyProperty, PlyRow, PlyValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind PLY geometry re-enters the flow graph through, after normalization.
pub const IMPORT_NEURON_KIND: &str = "brep.mesh.construct";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    mesh_from_snapshot(&semio_s_artifact_stdio_ply::engine::decode_ply(bytes).map_err(|error| io_error(format!("generation3d←ply: {error}")))?)
}

pub fn mesh_from_snapshot(from: &PlySnapshot) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    ::semio_framework_async::poll::resolve_ready(SemioMeshFromPly::deserialize(from)).map_err(|error| io_error(format!("generation3d←ply: {error}")))
}

pub fn deserialize(from: &PlySnapshot) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    import_polygon_meshes(vec![polygon_payload(from)?])
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    let source = semio_s_artifact_stdio_ply::engine::decode_ply(bytes).map_err(|error| io_error(format!("generation3d←ply: {error}")))?;
    deserialize(&source)
}

fn scalar(value: &PlyValue) -> Result<f64, semio_framework_diagnostic::TextError> {
    let value = match value { PlyValue::Char(value) => *value as f64, PlyValue::UChar(value) => *value as f64, PlyValue::Short(value) => *value as f64, PlyValue::UShort(value) => *value as f64, PlyValue::Int(value) => *value as f64, PlyValue::UInt(value) => *value as f64, PlyValue::Float(value) => *value as f64, PlyValue::Double(value) => *value, PlyValue::List(_) => return Err(io_error("generation3d←ply: a scalar property contains a list")) };
    if !(value as f32).is_finite() { return Err(io_error("generation3d←ply: file contains a non-finite scalar")); }
    Ok(value)
}

fn cell(value: &PlyValue) -> Result<semio_framework_pack_json::Value, semio_framework_diagnostic::TextError> {
    match value {
        PlyValue::List(values) => Ok(semio_framework_pack_json::array(values.iter().map(|value| scalar(value).map(semio_framework_pack_json::Value::from)).collect::<Result<Vec<_>, _>>()?)),
        value => scalar(value).map(semio_framework_pack_json::Value::from),
    }
}

fn vertex_scalar(element: &PlyElement, row: &PlyRow, name: &str) -> Result<f64, semio_framework_diagnostic::TextError> {
    let index = element.properties.iter().position(|property| property.name() == name).ok_or_else(|| io_error(format!("generation3d←ply: vertex property '{name}' is missing")))?;
    scalar(&row.values[index])
}

fn color(value: &PlyValue) -> Result<f64, semio_framework_diagnostic::TextError> {
    let scale = match value { PlyValue::UChar(_) => 255.0, PlyValue::UShort(_) => 65535.0, PlyValue::UInt(_) => u32::MAX as f64, _ => 1.0 };
    let color = scalar(value)? / scale;
    if !(0.0..=1.0).contains(&color) { return Err(io_error("generation3d←ply: vertex color is outside its declared range")); }
    Ok(color)
}

/// 🧩️ Preserves generic vertex and face properties alongside original PLY polygons.
pub fn polygon_payload(source: &PlySnapshot) -> Result<String, semio_framework_diagnostic::TextError> {
    use semio_framework_pack_json::{array, Object, Value};
    if source.elements.iter().any(|element| element.count != element.rows.len() as u64 || element.rows.iter().any(|row| row.values.len() != element.properties.len()) || element.properties.iter().map(PlyProperty::name).collect::<std::collections::BTreeSet<_>>().len() != element.properties.len()) { return Err(io_error("generation3d←ply: element occurrences do not match their declaration")); }
    let element = |name: &str| {
        let mut matches = source.elements.iter().filter(|element| element.name == name);
        let found = matches.next().ok_or_else(|| io_error(format!("generation3d←ply: '{name}' element is missing")))?;
        if matches.next().is_some() { return Err(io_error(format!("generation3d←ply: '{name}' element is repeated"))); }
        Ok(found)
    };
    let vertex = element("vertex")?;
    let face = element("face")?;
    if vertex.rows.len() > 100_000 || face.rows.len() > 100_000 { return Err(io_error("generation3d←ply: surface exceeds its capacity")); }
    if source.elements.iter().any(|element| !["vertex", "face"].contains(&element.name.as_str()) && !element.rows.is_empty()) { return Err(io_error("generation3d←ply: file contains unsupported non-surface elements")); }
    let vertices = vertex.rows.iter().map(|row| Ok([vertex_scalar(vertex, row, "x")?, vertex_scalar(vertex, row, "y")?, vertex_scalar(vertex, row, "z")?])).collect::<Result<Vec<_>, semio_framework_diagnostic::TextError>>()?;
    let names = vertex.properties.iter().map(PlyProperty::name).collect::<Vec<_>>();
    let mut used = std::collections::BTreeSet::from(["x", "y", "z"]);
    let mut attributes = Object::new();
    for (semantic, fields) in [("normal", vec!["nx", "ny", "nz"]), ("uv", if names.contains(&"u") || names.contains(&"v") { vec!["u", "v"] } else { vec!["s", "t"] }), ("color", vec!["red", "green", "blue"])] {
        if fields.iter().all(|field| !names.contains(field)) { continue; }
        let columns = fields.iter().map(|field| names.iter().position(|name| name == field).ok_or_else(|| io_error(format!("generation3d←ply: '{field}' surface property is missing")))).collect::<Result<Vec<_>, _>>()?;
        let values = vertex.rows.iter().map(|row| {
            let mut tuple = columns.iter().map(|column| if semantic == "color" { color(&row.values[*column]) } else { scalar(&row.values[*column]) }).collect::<Result<Vec<_>, _>>()?;
            if semantic == "normal" && tuple.iter().all(|value| *value == 0.0) { return Err(io_error("generation3d←ply: authored normal cannot be zero")); }
            if semantic == "color" { tuple.push(names.iter().position(|name| *name == "alpha").map_or(Ok(1.0), |column| color(&row.values[column]))?); }
            Ok(array(tuple.into_iter().map(Value::from)))
        }).collect::<Result<Vec<_>, semio_framework_diagnostic::TextError>>()?;
        used.extend(fields);
        if semantic == "color" && names.contains(&"alpha") { used.insert("alpha"); }
        attributes.insert(semantic, polygon_import_attribute("vertex", semantic, "linear", values, None));
    }
    let face_property = face.properties.iter().position(|property| ["vertex_indices", "vertex_index"].contains(&property.name())).ok_or_else(|| io_error("generation3d←ply: polygon vertex indices are missing"))?;
    let faces = face.rows.iter().map(|row| {
        let PlyValue::List(values) = &row.values[face_property] else { return Err(io_error("generation3d←ply: polygon indices must be a list")); };
        values.iter().map(|value| {
            let index = scalar(value)?;
            if index < 0.0 || index > u32::MAX as f64 || index.fract() != 0.0 { return Err(io_error("generation3d←ply: polygon indices must be unsigned integers")); }
            Ok(index as u32)
        }).collect::<Result<Vec<_>, _>>()
    }).collect::<Result<Vec<_>, _>>()?;
    for (domain, element) in [("vertex", vertex), ("face", face)] {
        for (index, property) in element.properties.iter().enumerate() {
            if (domain == "vertex" && used.contains(property.name())) || (domain == "face" && index == face_property) { continue; }
            let interpolation = if domain == "face" { "constant" } else if matches!(property, PlyProperty::Scalar { .. }) { "linear" } else { "nearest" };
            let values = element.rows.iter().map(|row| cell(&row.values[index])).collect::<Result<Vec<_>, _>>()?;
            attributes.insert(format!("ply.{domain}.{}", property.name()), polygon_import_attribute(domain, "custom", interpolation, values, None));
        }
    }
    if !source.comments.is_empty() { attributes.insert("ply.comments", polygon_import_attribute("face", "custom", "constant", vec![array(source.comments.iter().cloned().map(Value::from))], Some(vec![0; face.rows.len()]))); }
    polygon_import_payload(vertices, faces, attributes)
}
