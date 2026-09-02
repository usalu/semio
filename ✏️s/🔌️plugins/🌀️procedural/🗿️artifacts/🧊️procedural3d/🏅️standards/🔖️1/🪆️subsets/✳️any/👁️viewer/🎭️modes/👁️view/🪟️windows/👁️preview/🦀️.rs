//! 👁️ Procedural3d viewer — the Preview window: a read-only world-3d mesh scene built with the
//! shared `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6),
//! evaluated fresh from the same artifact-level `Procedural3dSnapshot` the editor's own Preview
//! window renders — this file itself imports nothing from the sibling editor surface
//! (`policyViewerPurityBreaches` forbids it outright). No selection, no gumball/utility chrome: a
//! viewer emits no mutations by construction (`ViewEmit`).
//!
//! 🧵️ Unlike the sibling surface's own preview pipeline (which threads a live, session-cached
//! `flow::FlowEvalSession` through `handle`/`pending_effects` for incremental re-tessellation), this
//! window carries no persisted per-session state (`Config = NoConfig`) and re-evaluates the whole
//! flow fixture fresh on every render call via `flow::FlowHost`/`flow::tessellate_geometry` directly
//! — an intentional simplification (no cache, no incremental tick chain), documented here rather
//! than silently duplicating the other surface's session machinery.

use crate::artifacts::procedural3d::Procedural3dSnapshot;
use dsl::json::{Object, Value};
use semio_framework_plugin::{world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
/// 👁️ Read-only default camera — matches the other surface's own `default_preview_cam_*` literals
/// (duplicated, not imported: `Config = NoConfig`, a viewer has no persisted per-session camera).
const PROCEDURAL3D_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [4.0, -4.0, 3.0];
const PROCEDURAL3D_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const PROCEDURAL3D_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
/// 🧊️ Matches the other surface's own default LOD tessellation deflection (unset `lod_mode` ⇒ 0.05).
const PROCEDURAL3D_VIEW_TOLERANCE: f64 = 0.05;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedural3d::create_procedural3d_viewer`.
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Geometry
/// 👁️ Read-only twin of the other surface's own `is_brep_geometry_handle` — duplicated (not
/// imported) per `policyViewerPurityBreaches`.
fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 👁️ Read-only twin of the other surface's own `PreviewInlineGeometry` — duplicated (not
/// imported) per `policyViewerPurityBreaches`.
#[derive(Clone, Copy, Debug, PartialEq)]
enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
}

/// 👁️ Read-only twin of the other surface's own `PreviewChannelItem`.
struct PreviewChannelItem {
    channel: String,
    index: usize,
    handle: String,
    inline: Option<PreviewInlineGeometry>,
}

/// 👁️ Read-only twin of the other surface's own `preview_channel_list_entries`.
fn preview_channel_list_entries(map: &Object) -> Vec<&Value> {
    let mut entries: Vec<(usize, &Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 👁️ Read-only twin of the other surface's own `collect_preview_channel_items`.
fn collect_preview_channel_items(channel: &str, value: &Value, index: &mut usize, items: &mut Vec<PreviewChannelItem>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(Value::as_str) {
                if is_brep_geometry_handle(handle) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: handle.into(), inline: None });
                    *index += 1;
                    return;
                }
            }
            if map.get("$schema").and_then(Value::as_str) == Some("list") {
                for entry in preview_channel_list_entries(map) {
                    collect_preview_channel_items(channel, entry, index, items);
                }
                return;
            }
            let coords = ["x", "y", "z"].into_iter().map(|key| map.get(key).and_then(Value::as_f64)).collect::<Option<Vec<_>>>();
            if let Some(coords) = coords {
                let (x, y, z) = (coords[0], coords[1], coords[2]);
                let inline = if map.get("$schema").and_then(Value::as_str) == Some("vector") { PreviewInlineGeometry::Vector { x, y, z } } else { PreviewInlineGeometry::Point { x, y, z } };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(inline) });
                *index += 1;
            }
        }
        Value::Array(list) => {
            for entry in list {
                collect_preview_channel_items(channel, entry, index, items);
            }
        }
        _ => {}
    }
}

/// 👁️ Read-only twin of the other surface's own `preview_channel_items_for_widget`.
fn preview_channel_items_for_widget(eval: &Value, widget_id: &str) -> Vec<PreviewChannelItem> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let Some(channels) = widget_eval.get("out").or_else(|| widget_eval.get("in")) else {
        return Vec::new();
    };
    let Some(map) = channels.as_object() else {
        return Vec::new();
    };
    let mut keys: Vec<&str> = map.iter().map(|(key, _)| key).collect();
    keys.sort();
    let mut items = Vec::new();
    for key in keys {
        let mut index = 0usize;
        if let Some(value) = map.get(key) {
            collect_preview_channel_items(key, value, &mut index, &mut items);
        }
    }
    items
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 👁️ Half-extent (world units) of the axis cross drawn for a `PreviewInlineGeometry::Point` —
/// read-only twin of the other surface's own constant.
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 👁️ Read-only twin of the other surface's own `point_marker_mesh`.
fn point_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    semio_framework_plugin::MeshData {
        positions: vec![x, y, z],
        edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e],
        ..Default::default()
    }
}

/// 👁️ Read-only twin of the other surface's own `vector_marker_mesh`.
fn vector_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    semio_framework_plugin::MeshData { positions: vec![0.0, 0.0, 0.0, x, y, z], edge_positions: vec![0.0, 0.0, 0.0, x, y, z], ..Default::default() }
}

/// 👁️ Evaluates the whole fixture fresh (no session cache — see module doc comment) and tessellates
/// every preview widget's geometry handles into meshes/instances at the world origin.
fn evaluated_meshes_and_instances(fixture: &flow::FlowFixture) -> (String, String) {
    let mut host = flow::FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = dsl::json::parse(&eval_json).unwrap_or(Value::Object(Object::new()));
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    // 🔁️ Dedup key is the brep HANDLE, not the widget/channel that emitted it — read-only twin of
    // the editor surface's own dedup rule.
    let mut mesh_id_by_handle: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for widget in &fixture.widgets {
        let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        for item in preview_channel_items_for_widget(&eval, &id) {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(&handle).cloned().unwrap_or(own_mesh_id) };
            if !meshes.iter().any(|entry: &Value| entry.get("id").and_then(Value::as_str) == Some(mesh_id.as_str())) {
                let data = match inline {
                    Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(x, y, z)),
                    Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(vector_marker_mesh(x, y, z)),
                    None => flow::tessellate_geometry(&handle, PROCEDURAL3D_VIEW_TOLERANCE).ok(),
                };
                if let Some(data) = data {
                    if mesh_has_preview_geometry(&data) {
                        let mut mesh_object = Object::new();
                        mesh_object.insert("id", Value::String(mesh_id.clone()));
                        mesh_object.insert("data", Value::from(data));
                        meshes.push(Value::Object(mesh_object));
                        if !handle.is_empty() {
                            mesh_id_by_handle.insert(handle.clone(), mesh_id.clone());
                        }
                    }
                }
            }
            if meshes.iter().any(|entry: &Value| entry.get("id").and_then(Value::as_str) == Some(mesh_id.as_str())) {
                let mut instance_object = Object::new();
                instance_object.insert("id", Value::String(format!("{id}@{channel}#{index}")));
                instance_object.insert("meshId", Value::String(mesh_id));
                instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
                instance_object.insert("rotation", Value::Array(vec![Value::from(0.0), Value::from(0.0), Value::from(0.0), Value::from(1.0)]));
                instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
                instance_object.insert("label", Value::String(format!("{id}@{channel}")));
                instances.push(Value::Object(instance_object));
            }
        }
    }
    (dsl::json::to_string(&Value::Array(meshes)), dsl::json::to_string(&Value::Array(instances)))
}

/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields above.
fn vec3_json(v: [f64; 3]) -> Value {
    Value::Array(v.into_iter().map(Value::from).collect())
}
//#endregion 🔖️Geometry

//#region 🔖️Render
/// 👁️ Pure `Procedural3dSnapshot -> BuiltNode` read: default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gumball/engagement overlay, real evaluated
/// preview geometry — not a fallback placeholder: procedural3d's whole purpose is generated geometry,
/// and the pure evaluate+tessellate path needs no session/config to run once.
pub fn render(document: &Procedural3dSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = evaluated_meshes_and_instances(&document.fixture);
    let view = MeshView {
        camera_json: world3d_camera_json(PROCEDURAL3D_VIEW_DEFAULT_CAMERA_POSITION, PROCEDURAL3D_VIEW_DEFAULT_CAMERA_TARGET, PROCEDURAL3D_VIEW_DEFAULT_CAMERA_FOV),
        meshes_json,
        instances_json,
        selection_json: world3d_selection_json("rectangle", &[], None),
    };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_shared_mesh_window_kit() {
        let def = definition();
        assert_eq!(def.id, MeshWindowKit::KIND_ID);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::procedural3d::schema::default_snapshot();
        let _node = render(&document);
    }

    #[test]
    fn render_emits_real_tessellated_geometry_for_the_default_fixture() {
        let document = crate::artifacts::procedural3d::schema::default_snapshot();
        let (meshes_json, instances_json) = evaluated_meshes_and_instances(&document.fixture);
        assert_ne!(meshes_json, "[]", "default fixture should evaluate and tessellate at least one preview mesh");
        assert_ne!(instances_json, "[]");
    }
}
//#endregion 🧪️Tests
