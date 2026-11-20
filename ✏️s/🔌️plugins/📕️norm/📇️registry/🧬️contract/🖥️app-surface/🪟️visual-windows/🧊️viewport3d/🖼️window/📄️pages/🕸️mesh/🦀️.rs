//! 🕸️ Splits oversized indexed triangle records without changing their vertices or picking owner.

use super::{geometry_fits, Footprint, Mesh};
use semio_framework_pack_json::{json, to_string, Value};
use semio_plugin::{ui_assembly_error, UiAssemblyResult};
use std::collections::HashMap;

pub(super) fn instance_json(instance: &Value, owner: &str, mesh_id: &str) -> UiAssemblyResult<String> {
    let mut instance = instance.clone();
    let object = instance.as_object_mut().ok_or_else(|| ui_assembly_error("scene.viewport.instance-shape"))?;
    let id = object.get("id").and_then(Value::as_str).ok_or_else(|| ui_assembly_error("scene.viewport.instance-identity"))?;
    let (_, part) = mesh_id.rsplit_once("#mesh-page=").ok_or_else(|| ui_assembly_error("scene.viewport.mesh-identity"))?;
    let id = format!("{id}#mesh-page={part}");
    object.insert("id".to_owned(), json!(id));
    object.insert("meshId".to_owned(), json!(mesh_id));
    object.insert("interactionId".to_owned(), json!(owner));
    Ok(to_string(&instance))
}

struct Triangles<'a> {
    indices: &'a [Value],
    positions: &'a [Value],
    normals: &'a [Value],
    colors: &'a [Value],
}

impl<'a> Triangles<'a> {
    fn read(mesh: &'a Value) -> UiAssemblyResult<Self> {
        let data = mesh.get("data").ok_or_else(|| ui_assembly_error("scene.viewport.mesh-data"))?;
        let lane = |name| data.get(name).and_then(Value::as_array).map(Vec::as_slice)
            .ok_or_else(|| ui_assembly_error("scene.viewport.mesh-lane"));
        let triangles = Self { indices: lane("indices")?, positions: lane("positions")?, normals: lane("normals")?, colors: lane("colors")? };
        if triangles.indices.is_empty() || triangles.indices.len() % 3 != 0 {
            return Err(ui_assembly_error("scene.viewport.mesh-triangles"));
        }
        Ok(triangles)
    }

    fn part(&self, indices: &[Value], id: String, source: usize) -> UiAssemblyResult<Mesh> {
        let mut remapped = HashMap::new();
        let mut vertices = Vec::new();
        let mut index = Vec::with_capacity(indices.len());
        for value in indices {
            let vertex = value.as_u64().and_then(|value| usize::try_from(value).ok()).filter(|&vertex| vertex < self.positions.len() / 3)
                .ok_or_else(|| ui_assembly_error("scene.viewport.mesh-index"))?;
            let local = *remapped.entry(vertex).or_insert_with(|| {
                let local = vertices.len();
                vertices.push(vertex);
                local
            });
            index.push(json!(local));
        }
        let values = |lane: &[Value], stride: usize| {
            vertices.iter().flat_map(|&vertex| lane[vertex * stride..(vertex + 1) * stride].iter().cloned()).collect::<Vec<_>>()
        };
        let positions = values(self.positions, 3);
        let normals = values(self.normals, 3);
        let colors = values(self.colors, 4);
        let json = to_string(&json!({ "id": id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": index } }));
        Ok(Mesh { id, source, vertices: Some(vertices), json })
    }
}

pub(super) fn split(mesh: &Value, instance: &Value, owner: &str, source: &Mesh, fixed: Footprint) -> UiAssemblyResult<Vec<Mesh>> {
    let triangles = Triangles::read(mesh)?;
    let vertices = triangles.positions.len() / 3;
    if triangles.positions.len() != vertices * 3 || triangles.normals.len() != vertices * 3 || triangles.colors.len() != vertices * 4 {
        return Err(ui_assembly_error("scene.viewport.mesh-vertices"));
    }
    let mut pieces = Vec::new();
    let mut remaining = vec![triangles.indices];
    while let Some(indices) = remaining.pop() {
        let id = format!("{}#mesh-page={}", source.id, pieces.len());
        let piece = triangles.part(indices, id, source.source)?;
        let instance = instance_json(instance, owner, &piece.id)?;
        if geometry_fits(fixed, piece.json.len() + 1, instance.len() + 1) {
            pieces.push(piece);
            continue;
        }
        if indices.len() <= 3 {
            return Err(ui_assembly_error("scene.viewport.geometry-page"));
        }
        let (first, second) = indices.split_at((indices.len() / 6) * 3);
        remaining.push(second);
        remaining.push(first);
    }
    Ok(pieces)
}
