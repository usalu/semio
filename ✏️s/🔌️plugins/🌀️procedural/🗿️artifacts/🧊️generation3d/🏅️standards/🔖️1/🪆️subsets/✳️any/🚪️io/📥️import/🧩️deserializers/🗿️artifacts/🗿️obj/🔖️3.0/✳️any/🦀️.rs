//! 🗿️ Imports decoded OBJ surfaces as editable polygon inputs with surface channels.
use crate::standards::v1::subsets::any::io::mesh_bridge::{import_polygon_meshes, polygon_import_payload, polygon_import_attribute, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_obj::ObjSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind that re-enters OBJ text into the flow graph.
pub const IMPORT_NEURON_KIND: &str = "brep.mesh.construct";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    mesh_from_snapshot(&decode(bytes)?)
}

pub fn mesh_from_snapshot(from: &ObjSnapshot) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    ::semio_framework_async::poll::resolve_ready(SemioMeshFromObj::deserialize(from)).map_err(|error| io_error(format!("generation3d←obj: {error}")))
}

fn decode(bytes: &[u8]) -> Result<ObjSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| io_error(format!("generation3d←obj: obj is not valid utf-8: {error}")))?;
    semio_s_artifact_stdio_obj::engine::decode_obj(text).map_err(|error| io_error(format!("generation3d←obj: {error}")))
}

pub fn deserialize(from: &ObjSnapshot) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    import_polygon_meshes(vec![polygon_payload(from)?])
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&decode(bytes)?)
}

/// 🧩️ Projects the owning OBJ model into original polygons and indexed corner channels.
pub fn polygon_payload(source: &ObjSnapshot) -> Result<String, semio_framework_diagnostic::TextError> {
    use semio_framework_pack_json::{array, object, Object, Value};
    if source.vertices.len() > 100_000 || source.faces.len() > 100_000 || source.faces.iter().map(|face| face.vertices.len()).sum::<usize>() > 600_000 { return Err(io_error("generation3d←obj: surface exceeds its capacity")); }
    let number = |number: f64| if (number as f32).is_finite() { Ok(number) } else { Err(io_error("generation3d←obj: surface contains a non-finite scalar")) };
    let vertices = source.vertices.iter().map(|point| {
        let w = number(point.w.unwrap_or(1.0))?;
        if w == 0.0 { return Err(io_error("generation3d←obj: homogeneous position has zero weight")); }
        Ok([number(point.x)? / w, number(point.y)? / w, number(point.z)? / w])
    }).collect::<Result<Vec<_>, _>>()?;
    let faces = source.faces.iter().map(|face| face.vertices.iter().map(|corner| corner.vertex).collect::<Vec<_>>()).collect::<Vec<_>>();
    let corners = source.faces.iter().flat_map(|face| &face.vertices).collect::<Vec<_>>();
    let mut attributes = Object::new();
    let normals = source.normals.iter().map(|normal| {
        let point = [number(normal.x)?, number(normal.y)?, number(normal.z)?];
        if point == [0.0; 3] { return Err(io_error("generation3d←obj: authored normal cannot be zero")); }
        Ok(array(point.map(Value::from)))
    }).collect::<Result<Vec<_>, _>>()?;
    let uvs = source.texcoords.iter().map(|uv| Ok(array([Value::from(number(uv.u)?), Value::from(number(uv.v)?)]))).collect::<Result<Vec<_>, semio_framework_diagnostic::TextError>>()?;
    let mut uv_indices = None;
    for (field, values) in [("normal", normals), ("uv", uvs)] {
        let refs = corners.iter().map(|corner| if field == "normal" { corner.normal } else { corner.texcoord }).collect::<Vec<_>>();
        if refs.iter().all(Option::is_none) { continue; }
        let indices = refs.into_iter().map(|index| index.filter(|index| (*index as usize) < values.len()).ok_or_else(|| io_error(format!("generation3d←obj: {field} references are incomplete")))).collect::<Result<Vec<_>, _>>()?;
        if field == "uv" { uv_indices = Some(indices.clone()); }
        attributes.insert(field, polygon_import_attribute("corner", field, "linear", values, Some(indices)));
    }
    let face_count = faces.len();
    let face_channel = |values| polygon_import_attribute("face", "custom", "constant", values, None);
    if !source.groups.is_empty() {
        let mut groups = vec![Vec::new(); face_count];
        for group in &source.groups {
            for face in &group.faces { groups.get_mut(*face as usize).ok_or_else(|| io_error("generation3d←obj: group references an unknown face"))?.push(Value::from(group.name.clone())); }
        }
        attributes.insert("obj.groups", face_channel(groups.into_iter().map(array).collect()));
    }
    if !source.objects.is_empty() {
        let mut objects = vec![None; face_count];
        for object in &source.objects {
            for face in &object.faces {
                let slot = objects.get_mut(*face as usize).ok_or_else(|| io_error("generation3d←obj: object references an unknown face"))?;
                if slot.is_some() { return Err(io_error("generation3d←obj: face belongs to overlapping objects")); }
                *slot = Some(object.name.clone());
            }
        }
        attributes.insert("obj.object", face_channel(objects.into_iter().map(|name| Value::from(name.unwrap_or_default())).collect()));
    }
    if source.usemtl.iter().any(|range| range.face_index_from > face_count as u64) || source.usemtl.windows(2).any(|pair| pair[0].face_index_from > pair[1].face_index_from) || source.smoothing_groups.iter().any(|range| range.face_index_from > face_count as u64) || source.smoothing_groups.windows(2).any(|pair| pair[0].face_index_from > pair[1].face_index_from) { return Err(io_error("generation3d←obj: face ranges are invalid")); }
    if !source.usemtl.is_empty() {
        let mut cursor = 0;
        let values = (0..face_count).map(|face| {
            while cursor < source.usemtl.len() && source.usemtl[cursor].face_index_from <= face as u64 { cursor += 1; }
            Value::from(cursor.checked_sub(1).map(|index| source.usemtl[index].material.clone()).unwrap_or_default())
        }).collect();
        attributes.insert("obj.material", face_channel(values));
    }
    if !source.smoothing_groups.is_empty() {
        let mut cursor = 0;
        let values = (0..face_count).map(|face| {
            while cursor < source.smoothing_groups.len() && source.smoothing_groups[cursor].face_index_from <= face as u64 { cursor += 1; }
            cursor.checked_sub(1).and_then(|index| source.smoothing_groups[index].group).map(Value::from).unwrap_or(Value::Null)
        }).collect();
        attributes.insert("obj.smoothing", face_channel(values));
    }
    if source.vertices.iter().any(|point| point.w.is_some()) {
        let values = source.vertices.iter().map(|point| point.w.map(&number).transpose().map(|value| value.map(Value::from).unwrap_or(Value::Null))).collect::<Result<Vec<_>, _>>()?;
        attributes.insert("obj.vertex.w", polygon_import_attribute("vertex", "custom", "nearest", values, None));
    }
    if source.texcoords.iter().any(|point| point.w.is_some()) {
        if let Some(indices) = uv_indices {
            let values = source.texcoords.iter().map(|point| point.w.map(&number).transpose().map(|value| value.map(Value::from).unwrap_or(Value::Null))).collect::<Result<Vec<_>, _>>()?;
            attributes.insert("obj.texcoord.w", polygon_import_attribute("corner", "custom", "nearest", values, Some(indices)));
        }
    }
    if source.unknown_statements.iter().any(|statement| !statement.raw.trim_start().starts_with('#')) { return Err(io_error("generation3d←obj: file contains unsupported non-surface statements")); }
    if source.mtllib.is_some() || !source.unknown_statements.is_empty() {
        let mut data = Object::new();
        if let Some(library) = &source.mtllib { data.insert("materialLibrary", library.clone().into()); }
        data.insert("comments", array(source.unknown_statements.iter().map(|statement| object([("line".into(), statement.line_index.to_string().into()), ("text".into(), statement.raw.clone().into())]))));
        attributes.insert("obj.source", polygon_import_attribute("face", "custom", "constant", vec![Value::Object(data)], Some(vec![0; face_count])));
    }
    polygon_import_payload(vertices, faces, attributes)
}
