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
//! 🧵️ Evaluation input is the viewer's own ephemeral local-only `preview_eval_text`, computed
//! chunked and cancellable in `Generation3dViewCommandWork`. `render` NEVER evaluates: a pure render
//! that fell back to a synchronous `FlowHost::evaluate` raced the real command chain and paid for a
//! whole flow evaluation on every repaint that arrived before the first tick landed. Until the chain
//! publishes, the window paints its empty world and the tick chain drives it. Tessellation
//! deflection, shading mode, camera and sun all come from the viewer's own `🎚️config`.
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
/// 🧊️ The geometry half of a preview payload is NOT a read-only twin any more: the handle grammar,
/// the channel walk, the marker meshes, the show-mode filter, the LOD ladder and the mesh lookup
/// all come from `🧵️preview-eval`, the surface-neutral chain both surfaces run. Importing it is
/// legal for a viewer — it is mounted at the ARTIFACT level (`crate::preview_eval`), never reached
/// through the sibling `✏️editor` module, which `policyViewerPurityBreaches` forbids outright
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
use crate::preview_eval::{self, PreviewChannelItem, PreviewInlineGeometry};

/// 🧮️ Evaluates the whole fixture once, IN PROCESS. TEST-ONLY and never on the live path: the
/// served chain evaluates through `ExtensionInvocation` because a guest links no operators at all,
/// while a `--lib` test binary does link them — so a law that needs an evaluation without standing
/// up the whole chain builds one here. `#[cfg(test)]` is the compile-time half of "render never
/// evaluates" (`render_without_a_published_evaluation_paints_the_empty_world`).
#[cfg(test)]
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

/// 🧊️ One built mesh table — everything in a preview payload that does NOT depend on hover or
/// selection. Held across renders keyed by [`PreviewMeshTable::signature`] so orbiting the camera or
/// moving the pointer rebuilds only the instance table.
struct PreviewMeshTable {
    signature: u64,
    meshes_json: String,
    mesh_ids: std::collections::BTreeSet<String>,
    mesh_id_by_handle: std::collections::BTreeMap<String, String>,
}

thread_local! {
    /// 🧊️ The single live mesh table. One entry, not an LRU: a surface paints one document at one
    /// LOD at a time, and holding stale tables would retain their whole vertex payloads.
    static PREVIEW_MESH_TABLE: std::cell::RefCell<Option<PreviewMeshTable>> = const { std::cell::RefCell::new(None) };
    /// 🧮️ How many real tessellations the preview path has run in this process — the observable a
    /// hover-only re-render must leave untouched.
    static PREVIEW_TESSELLATIONS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// 🧮️ Preview tessellations run so far. A hover/camera re-render of an unchanged document must not
/// move this (`🎭️modes/👁️view/🧪️tests/👁️preview/🔬️unit/🦀️.rs`).
pub fn preview_tessellation_count() -> u64 {
    PREVIEW_TESSELLATIONS.with(std::cell::Cell::get)
}

/// 🧹️ Drops the retained mesh table — for a test that wants a cold measurement.
pub fn reset_preview_mesh_table() {
    PREVIEW_MESH_TABLE.with(|table| table.borrow_mut().take());
}

/// 🔒 What a built mesh table is valid for: the evaluation it was tessellated from, the deflection
/// its LOD asked for, the shading mode applied to it, the preview widgets it covered — and the
/// chain's own answers so far.
///
/// ⏱️ That last term is what makes the retained table correct under the ADDRESSED chain: a
/// `flowTessellateResolve` folds one more mesh pack into the session without moving the evaluation
/// text one byte, so a signature that ignored the session would pin the first (empty) table and no
/// arriving mesh would ever reach the screen. The pack BODIES are never hashed — only each
/// handle's resolved length, which is O(handles) per render and changes exactly when a round trip
/// lands (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn preview_mesh_signature(eval_json: &str, tolerance_bits: u64, show_mode: &str, preview_ids: &[String], eval: &Value, session: Option<&semio_framework_os_flow::FlowEvalSession>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    eval_json.hash(&mut hasher);
    tolerance_bits.hash(&mut hasher);
    show_mode.hash(&mut hasher);
    preview_ids.hash(&mut hasher);
    if let Some(session) = session {
        for id in preview_ids {
            for item in preview_eval::preview_channel_items_for_widget(eval, id) {
                if item.handle.is_empty() {
                    continue;
                }
                session.preview_mesh_pack(&item.handle).map_or(0usize, str::len).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

/// 🧊️ Tessellates every preview handle exactly once, in the declaration order the instance table
/// replays. Only reached when [`preview_mesh_signature`] says the retained table is stale.
fn build_preview_mesh_table(signature: u64, eval: &Value, preview_ids: &[String], tolerance: f64, show_mode: &str, session: Option<&semio_framework_os_flow::FlowEvalSession>) -> PreviewMeshTable {
    let mut meshes: Vec<Value> = Vec::new();
    let mut mesh_ids = std::collections::BTreeSet::new();
    let mut mesh_id_by_handle = std::collections::BTreeMap::new();
    for id in preview_ids {
        for item in preview_eval::preview_channel_items_for_widget(eval, id) {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(&handle).cloned().unwrap_or(own_mesh_id) };
            if mesh_ids.contains(&mesh_id) {
                continue;
            }
            let data = match inline {
                Some(PreviewInlineGeometry::Point { x, y, z }) => Some(preview_eval::point_marker_mesh(x, y, z)),
                Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(preview_eval::vector_marker_mesh(x, y, z)),
                // ⏱️ The CHAIN's answer first: a served guest links no geometry kernel at all, so
                // `flowTessellateResolve`'s mesh pack is the only mesh a viewer can ever paint.
                // Reaching the in-process kernel is counted, and only a session-free caller (a law
                // measuring a cold render) ever gets there.
                None => match session.and_then(|session| preview_eval::session_preview_mesh(&handle, session)) {
                    Some(data) => Some(data),
                    None => {
                        PREVIEW_TESSELLATIONS.with(|count| count.set(count.get() + 1));
                        preview_eval::mesh_data_for_preview_handle(&handle, tolerance, session)
                    }
                },
            };
            let Some(data) = data.map(|data| preview_eval::apply_show_mode_mesh(data, show_mode)).filter(preview_eval::mesh_has_preview_geometry) else {
                continue;
            };
            let mut mesh_object = Object::new();
            mesh_object.insert("id", Value::String(mesh_id.clone()));
            mesh_object.insert("data", Value::from(data));
            meshes.push(Value::Object(mesh_object));
            if !handle.is_empty() {
                mesh_id_by_handle.insert(handle.clone(), mesh_id.clone());
            }
            mesh_ids.insert(mesh_id);
        }
    }
    PreviewMeshTable { signature, meshes_json: dsl::json::to_string(&Value::Array(meshes)), mesh_ids, mesh_id_by_handle }
}

pub fn preview_payload(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, config: &Generation3dViewConfig, session: Option<&semio_framework_os_flow::FlowEvalSession>, marks: &Generation3dViewMarks) -> ViewPreviewPayload {
    if eval_json.is_empty() {
        return ViewPreviewPayload::default();
    }
    let eval = match dsl::json::parse(eval_json) {
        Ok(value) if value.get("error").and_then(Value::as_str).is_none() => value,
        _ => return ViewPreviewPayload::default(),
    };
    let tolerance = config.tolerance();
    let show_mode = config.effective_show_mode();
    let preview_ids = preview_eval::preview_widget_ids(fixture);
    let signature = preview_mesh_signature(eval_json, tolerance.to_bits(), show_mode, &preview_ids, &eval, session);
    PREVIEW_MESH_TABLE.with(|retained| {
        let mut retained = retained.borrow_mut();
        if retained.as_ref().is_none_or(|table| table.signature != signature) {
            *retained = Some(build_preview_mesh_table(signature, &eval, &preview_ids, tolerance, show_mode, session));
        }
        let table = retained.as_ref().expect("preview mesh table was just built");
        let mut instances: Vec<Value> = Vec::new();
        let mut selected_ids: Vec<String> = Vec::new();
        let mut hovered_id: Option<String> = None;
        for id in &preview_ids {
            for item in preview_eval::preview_channel_items_for_widget(&eval, id) {
                let PreviewChannelItem { channel, index, handle, .. } = item;
                let own_mesh_id = format!("eval-{id}@{channel}#{index}");
                let mesh_id = if handle.is_empty() { own_mesh_id } else { table.mesh_id_by_handle.get(&handle).cloned().unwrap_or(own_mesh_id) };
                if !table.mesh_ids.contains(&mesh_id) {
                    continue;
                }
                let instance_id = format!("{id}@{channel}#{index}");
                let selected = marks.selects(id, &channel, index);
                let hovered = marks.hovers(id, &channel, index);
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
        ViewPreviewPayload { meshes_json: table.meshes_json.clone(), instances_json: dsl::json::to_string(&Value::Array(instances)), selected_ids, hovered_id }
    })
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
pub fn render(document: &Generation3dSnapshot, config: &Generation3dViewConfig, eval_json: Option<&str>, session: Option<&semio_framework_os_flow::FlowEvalSession>, marks: &Generation3dViewMarks) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = eval_json.unwrap_or_default();
    let payload = preview_payload(eval_json, &document.fixture, config, session, marks);
    let selection_json = preview_selection_json(config, &payload);
    // 📈️ The SAME projection both editor preview windows publish — the surface-neutral
    // `🧵️preview-eval` one, reached at the artifact level and never through `::editor::`. A viewer
    // that published no status left the shell with no phase to show and no cancel affordance to
    // offer, while its meshes rendered fine (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let status_json = preview_eval::preview_window_status_json(
        session,
        preview_eval::preview_status_json(eval_json, &document.fixture),
        &preview_eval::PreviewStatusDebug { meshes_json: &payload.meshes_json, instances_json: &payload.instances_json },
        None,
    );
    let sun = config.sun();
    crate::scene_surface(
        SURFACE_ID,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_framework_ui::wgpu::World3dScene {
            status_json,
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
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
