//! 🌐 Shared world-3d scene payload builders for plugin apps.

use semio_framework_core::{
    mesh_from_kind, mesh_to_glb, mesh_to_obj, MeshData, World3dScene, world3d_camera_json,
    world3d_default_selection_json,
};
use serde_json::{json, Value};

pub fn mesh_kind_from_json(mesh_json: &str) -> String {
    serde_json::from_str::<Value>(mesh_json)
        .ok()
        .and_then(|value| value.get("kind").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| "box".into())
}

pub fn world3d_meshes_json_from_kinds(kinds: &[String]) -> String {
    let meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
    json!({
        "method": method,
        "mode": "replace",
        "ids": ids,
        "hoveredId": hovered_id,
    })
    .to_string()
}

pub fn world3d_scene(
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
) -> World3dScene {
    World3dScene {
        camera_json,
        meshes_json,
        instances_json,
        selection_json,
    }
}

pub fn world3d_default_camera() -> String {
    world3d_camera_json([4.0, -4.0, 3.0], [0.0, 0.0, 0.0], 45.0)
}

pub fn export_mesh_obj(mesh: &MeshData, name: &str) -> (String, String) {
    (mesh_to_obj(mesh, name), "text/plain".into())
}

pub fn export_mesh_glb_bytes(mesh: &MeshData) -> (Vec<u8>, String) {
    (mesh_to_glb(mesh), "model/gltf-binary".into())
}

pub fn merge_world_selection_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<String> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "toggle" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

pub fn default_world3d_selection() -> String {
    world3d_default_selection_json()
}
