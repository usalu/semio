//! 👁️ Generation3d viewer — the Preview window: a read-only `World3dScene` over the evaluated,
//! tessellated flow geometry, bound to the framework's `graph` interaction domain so hover and
//! selection paint here exactly as they do on the sibling surface's own preview.
//!
//! 🕹️ The window declares `SurfaceKind::World3d` with `domain_id = "graph"` and
//! `domain_granularity_id = "handle"`, and every emitted instance carries `interactionId =
//! {widgetId}@{channel}` — the same channel-qualified id the framework's `interaction_topology`
//! declares as a `handle`, so a world pick resolves to a real declared target instead of being
//! pruned by `validate_state`. Hover is read off the ephemeral pointer channel and selection off the
//! persisted interaction store; the viewer stores NEITHER (see `👁️viewer/🫧️transient/🦀️.rs`).
//!
//! 🧵️ Evaluation input is the viewer's own ephemeral local-only `preview_eval_text` when a command
//! has already computed it (chunked, cancellable, in `Generation3dViewCommandWork`); otherwise this
//! window evaluates the fixture inline once so a freshly opened artifact still paints without
//! waiting for a first command. Tessellation deflection, shading mode, camera and sun all come from
//! the viewer's own `🎚️config`.
//!
//! Every helper below is a read-only TWIN of the sibling surface's own — duplicated, never imported
//! (`policyViewerPurityBreaches` forbids a viewer file importing through `✏️editor`).

use crate::viewer::generation3d::config::Generation3dViewConfig;
use dsl::json::{Object, Value};
use semio_framework_plugin::{app::InteractionView, world3d_scene, world3d_selection_json, world3d_sun_measures, ActionDescriptor, BuiltNode, LocalizedLabel, MeasureSelectItem, SurfaceKind, WindowKindDefinition, WindowMeasure, WindowOptions};

use crate::Generation3dSnapshot;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "procedural-view-preview";
pub const BODY_KEY: &str = "procedural.view.preview";
const SURFACE_ID: &str = "procedural.view.preview";

/// 🕹️ The framework interaction domain this window is bound to — the same `graph` domain the
/// artifact's widget DAG is declared under, so a world pick and a graph pick are one selection.
pub const GENERATION3D_VIEW_INTERACTION_DOMAIN: &str = "graph";
/// 🐁️ The single live hover channel per domain.
pub const GENERATION3D_VIEW_INTERACTION_CHANNEL: &str = "pointer";
/// 🎯️ A world-3d instance pick reports the channel-qualified port, i.e. a `handle`.
pub const GENERATION3D_VIEW_INTERACTION_GRANULARITY: &str = "handle";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::generation3d::create_generation3d_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 👁️ Preview shading mode selector — the read-only twin of the sibling surface's own chrome.
pub fn show_mode_measure(show_mode: &str, viewer_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor) -> WindowMeasure {
    let current = if show_mode.is_empty() { "shaded" } else { show_mode };
    WindowMeasure::Select {
        id: "generation3d-view-measure-show".into(),
        label: Some("Show".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "generation3d-view-measure-show-shaded".into(), value: "shaded".into(), label: "Shaded".into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-edges".into(), value: "shaded+edges".into(), label: "Shaded + edges".into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-points".into(), value: "points".into(), label: "Points".into() },
        ],
        on_change: viewer_action("setShowMode", None),
    }
}

/// 🔬️ Level-of-detail selector — coarser LOD is the viewer's only mesh-payload size lever.
pub fn lod_mode_measure(lod_mode: &str, viewer_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor) -> WindowMeasure {
    let current = if lod_mode.is_empty() { "medium" } else { lod_mode };
    WindowMeasure::Select {
        id: "generation3d-view-measure-lod".into(),
        label: Some("Detail".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "generation3d-view-measure-lod-coarse".into(), value: "coarse".into(), label: "Coarse".into() },
            MeasureSelectItem { id: "generation3d-view-measure-lod-medium".into(), value: "medium".into(), label: "Medium".into() },
            MeasureSelectItem { id: "generation3d-view-measure-lod-fine".into(), value: "fine".into(), label: "Fine".into() },
        ],
        on_change: viewer_action("setLodMode", None),
    }
}

/// 🎚️ The Preview window's whole chrome row: show mode, LOD and the sun group.
pub fn preview_window_measures(config: &Generation3dViewConfig, viewer_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, viewer_action), lod_mode_measure(&config.lod_mode, viewer_action), world3d_sun_measures("generation3d-view", &sun, viewer_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Marks
/// 🕹️ One render's resolved `graph`-domain marks — read-only twin of the sibling surface's own.
///
/// A preview instance id is `{widgetId}@{channel}#{index}`, so an id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}`, byte-identical to the
/// declared `handle` target) or its widget (`{widgetId}`). That three-level match is what makes
/// hover bidirectional between a graph surface and this world surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Generation3dViewMarks {
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl Generation3dViewMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The viewer stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            hovered: interaction.hover(GENERATION3D_VIEW_INTERACTION_DOMAIN, GENERATION3D_VIEW_INTERACTION_CHANNEL).ids.iter().cloned().collect(),
            selected: interaction.selection(GENERATION3D_VIEW_INTERACTION_DOMAIN).ids.iter().cloned().collect(),
        }
    }

    fn marked(set: &std::collections::BTreeSet<String>, widget_id: &str, channel: &str, index: usize) -> bool {
        set.contains(widget_id) || set.contains(&format!("{widget_id}@{channel}")) || set.contains(&format!("{widget_id}@{channel}#{index}"))
    }

    pub fn hovers(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.hovered, widget_id, channel, index)
    }

    pub fn selects(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.selected, widget_id, channel, index)
    }
}
//#endregion 🔖️Marks

//#region 🔖️Geometry
/// 👁️ Read-only twin of the sibling surface's own `is_brep_geometry_handle`.
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

/// 👁️ Read-only twin of the sibling surface's own `PreviewInlineGeometry`.
#[derive(Clone, Copy, Debug, PartialEq)]
enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
}

/// 👁️ Read-only twin of the sibling surface's own `PreviewChannelItem`.
struct PreviewChannelItem {
    channel: String,
    index: usize,
    handle: String,
    inline: Option<PreviewInlineGeometry>,
}

/// 👁️ Read-only twin of the sibling surface's own `preview_channel_list_entries`.
fn preview_channel_list_entries(map: &Object) -> Vec<&Value> {
    let mut entries: Vec<(usize, &Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 👁️ Read-only twin of the sibling surface's own `collect_preview_channel_items`.
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

/// 👁️ Read-only twin of the sibling surface's own `preview_channel_items_for_widget`.
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

/// 👁️ Half-extent (world units) of the axis cross drawn for a `PreviewInlineGeometry::Point`.
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 👁️ Read-only twin of the sibling surface's own `point_marker_mesh`.
fn point_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    semio_framework_plugin::MeshData { positions: vec![x, y, z], edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e], ..Default::default() }
}

/// 👁️ Read-only twin of the sibling surface's own `vector_marker_mesh`.
fn vector_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    semio_framework_plugin::MeshData { positions: vec![0.0, 0.0, 0.0, x, y, z], edge_positions: vec![0.0, 0.0, 0.0, x, y, z], ..Default::default() }
}

/// 👁️ Read-only twin of the sibling surface's own `apply_show_mode_mesh` — the shading mode decides
/// which mesh channels survive into the payload at all, so wireframe/points really are cheaper.
fn apply_show_mode_mesh(mut data: semio_framework_plugin::MeshData, show_mode: &str) -> semio_framework_plugin::MeshData {
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data,
    }
}

/// 🧮️ Evaluates the whole fixture once. Callers prefer the viewer's ephemeral `preview_eval_text`
/// and only fall back here when no command has computed it yet.
pub fn evaluate_fixture(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> String {
    crate::standards::v1::subsets::any::schema::with_host(fixture, |host| host.evaluate().unwrap_or_default())
}

/// 👁️ One preview instance per geometry-bearing value per OUTPUT CHANNEL, each carrying its
/// declared `interactionId` plus the live hovered/selected flags the world host paints with.
pub struct ViewPreviewPayload {
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
}

impl Default for ViewPreviewPayload {
    fn default() -> Self {
        Self { meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None }
    }
}

pub fn preview_payload(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, config: &Generation3dViewConfig, marks: &Generation3dViewMarks) -> ViewPreviewPayload {
    if eval_json.is_empty() {
        return ViewPreviewPayload::default();
    }
    let eval = match dsl::json::parse(eval_json) {
        Ok(value) if value.get("error").and_then(Value::as_str).is_none() => value,
        _ => return ViewPreviewPayload::default(),
    };
    let tolerance = config.tolerance();
    let show_mode = config.effective_show_mode();
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    // 🔁️ Dedup key is the brep HANDLE, not the widget/channel that emitted it.
    let mut mesh_id_by_handle: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    for widget in &fixture.widgets {
        let preview = matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { preview: true, .. } | semio_framework_artifact_flow_flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let id = crate::widget_id(widget).to_string();
        for item in preview_channel_items_for_widget(&eval, &id) {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let instance_id = format!("{id}@{channel}#{index}");
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(&handle).cloned().unwrap_or(own_mesh_id) };
            if !meshes.iter().any(|entry: &Value| entry.get("id").and_then(Value::as_str) == Some(mesh_id.as_str())) {
                let data = match inline {
                    Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(x, y, z)),
                    Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(vector_marker_mesh(x, y, z)),
                    None => semio_framework_os_flow::tessellate_geometry(&handle, tolerance).ok(),
                };
                if let Some(data) = data {
                    let data = apply_show_mode_mesh(data, show_mode);
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
                let selected = marks.selects(&id, &channel, index);
                let hovered = marks.hovers(&id, &channel, index);
                if selected {
                    selected_ids.push(instance_id.clone());
                }
                if hovered && hovered_id.is_none() {
                    hovered_id = Some(instance_id.clone());
                }
                let mut instance_object = Object::new();
                instance_object.insert("id", Value::String(instance_id));
                instance_object.insert("meshId", Value::String(mesh_id));
                instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
                instance_object.insert("rotation", Value::Array(vec![Value::from(0.0), Value::from(0.0), Value::from(0.0), Value::from(1.0)]));
                instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
                instance_object.insert("label", Value::String(format!("{id}@{channel}")));
                instance_object.insert("interactionId", Value::String(format!("{id}@{channel}")));
                instance_object.insert("selected", Value::Bool(selected));
                instance_object.insert("hovered", Value::Bool(hovered));
                instances.push(Value::Object(instance_object));
            }
        }
    }
    ViewPreviewPayload { meshes_json: dsl::json::to_string(&Value::Array(meshes)), instances_json: dsl::json::to_string(&Value::Array(instances)), selected_ids, hovered_id }
}

/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields above.
fn vec3_json(v: [f64; 3]) -> Value {
    Value::Array(v.into_iter().map(Value::from).collect())
}

/// 🧭️ World-3d selection payload — the live marks plus the shading flags the show mode implies.
/// A viewer never mounts a gumball, so `transformMode` stays empty and `gumballActive` false: a
/// transform handle is a mutation affordance and a viewer emits no mutations.
pub fn preview_selection_json(config: &Generation3dViewConfig, payload: &ViewPreviewPayload) -> String {
    let mut value = dsl::json::parse(&world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref())).unwrap_or_else(|_| Value::Object(Object::new()));
    let show_edges = matches!(config.effective_show_mode(), "wireframe" | "shaded+edges");
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode", Value::String(String::new()));
        object.insert("gumballActive", Value::Bool(false));
        object.insert("showEdges", Value::Bool(show_edges));
        object.insert("selectionMode", Value::String("mesh".into()));
        object.insert("granularity", Value::String("mesh".into()));
    }
    dsl::json::to_string(&value)
}
//#endregion 🔖️Geometry

//#region 🔖️Render
/// 👁️ Pure `(Generation3dSnapshot, Generation3dViewConfig, marks) -> BuiltNode` read: the camera,
/// shading mode, LOD and sun all come from the viewer's own config, hover/selection from the
/// framework-owned `graph` domain, geometry from the ephemeral evaluation when one exists.
pub fn render(document: &Generation3dSnapshot, config: &Generation3dViewConfig, eval_json: Option<&str>, marks: &Generation3dViewMarks) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let owned_eval = match eval_json {
        Some(text) if !text.is_empty() => None,
        _ => Some(evaluate_fixture(&document.fixture)),
    };
    let eval_text = owned_eval.as_deref().or(eval_json).unwrap_or_default();
    let payload = preview_payload(eval_text, &document.fixture, config, marks);
    let selection_json = preview_selection_json(config, &payload);
    let sun = config.sun();
    crate::scene_surface(
        SURFACE_ID,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_framework_ui::wgpu::World3dScene {
            domain_id: Some(GENERATION3D_VIEW_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(GENERATION3D_VIEW_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(
                semio_framework_ui::wgpu::world3d_camera_json(config.preview_camera.position, config.preview_camera.target, config.preview_camera.fov),
                payload.meshes_json,
                payload.instances_json,
                selection_json,
                &sun,
            )
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
