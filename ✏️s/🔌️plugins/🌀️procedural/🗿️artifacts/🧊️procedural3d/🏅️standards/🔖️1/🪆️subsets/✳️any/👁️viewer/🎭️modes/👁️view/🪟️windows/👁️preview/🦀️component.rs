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
use semio_framework_plugin::{world3d_camera_json, world3d_selection_json, MeshView, MeshWindowKit, UiNode, WindowKit, WindowKindDefinition};
use serde_json::{json, Value};

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
pub async fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Geometry
/// 👁️ Read-only twin of the other surface's own `is_brep_geometry_handle` — duplicated (not
/// imported) per `policyViewerPurityBreaches`.
async fn is_brep_geometry_handle(handle: &str) -> bool {
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

async fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

async fn geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"));
    let Some(channels) = channels else {
        return Vec::new();
    };
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles
}

async fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 👁️ Evaluates the whole fixture fresh (no session cache — see module doc comment) and tessellates
/// every preview widget's geometry handles into meshes/instances at the world origin.
async fn evaluated_meshes_and_instances(fixture: &flow::FlowFixture) -> (String, String) {
    let mut host = flow::FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for widget in &fixture.widgets {
        let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        for (index, handle) in geometry_handles_for_widget(&eval, &id).iter().enumerate() {
            let Ok(data) = flow::tessellate_geometry(handle, PROCEDURAL3D_VIEW_TOLERANCE) else { continue };
            if !mesh_has_preview_geometry(&data) {
                continue;
            }
            let mesh_id = format!("eval-{id}#{index}");
            meshes.push(json!({ "id": mesh_id, "data": data }));
            instances.push(json!({
                "id": format!("{id}#{index}"),
                "meshId": mesh_id,
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
            }));
        }
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖️Geometry

//#region 🔖️Render
/// 👁️ Pure `Procedural3dSnapshot -> UiNode` read: default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gumball/engagement overlay, real evaluated
/// preview geometry — not a fallback placeholder: procedural3d's whole purpose is generated geometry,
/// and the pure evaluate+tessellate path needs no session/config to run once.
pub async fn render(document: &Procedural3dSnapshot) -> UiNode {
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

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_shared_mesh_window_kit() {
        let def = definition();
        assert_eq!(def.id, MeshWindowKit::KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::procedural3d::schema::default_snapshot();
        let _node = render(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_emits_real_tessellated_geometry_for_the_default_fixture() {
        let document = crate::artifacts::procedural3d::schema::default_snapshot();
        let (meshes_json, instances_json) = evaluated_meshes_and_instances(&document.fixture);
        assert_ne!(meshes_json, "[]", "default fixture should evaluate and tessellate at least one preview mesh");
        assert_ne!(instances_json, "[]");
    }
}
//#endregion 🧪️Tests
