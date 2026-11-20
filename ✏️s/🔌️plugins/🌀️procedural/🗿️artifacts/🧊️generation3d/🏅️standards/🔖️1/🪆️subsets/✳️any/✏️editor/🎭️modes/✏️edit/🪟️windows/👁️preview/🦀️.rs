//! 👁️ Generation3d play app — the 3D preview window (edit mode): the tessellated evaluated geometry.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_editor_selection as selection;
use semio_s_artifact_procedural_3d_transforms as transform_commands;



use semio_s_artifact_procedural_3d_preview_eval::preview_status_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_window_status_json;

use semio_s_artifact_procedural_3d_preview_eval::PreviewStatusDebug;


use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_os_flow::FlowEvalSession;
use semio_plugin::world3d_scene;
use semio_plugin::world3d_sun_measures;
use semio_plugin::ActionDescriptor;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MeasureSelectItem;
use semio_plugin::SurfaceKind;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowMeasure;
use semio_plugin::WindowOptions;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_os_flow::{FlowHost};
use semio_plugin::app::InteractionView;
use std::collections::HashMap;
use semio_s_artifact_procedural_3d_preview_eval::PreviewInlineGeometry;
use semio_s_artifact_procedural_3d_preview_eval::PreviewChannelItem;
use semio_s_artifact_procedural_3d_preview_eval::{apply_show_mode_mesh, inline_preview_mesh, PreviewMeshLookup, mesh_has_preview_geometry, preview_channel_items_for_widget, preview_mesh_role};

#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
pub const GENERATION_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const GENERATION_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: GENERATION_3D_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION_3D_PLAY_BODY_PREVIEW.into(),
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

/// 👁️ One show-mode row's picker id and human label. Keyed off the tag so
/// [`crate::editor::generation3d::config::GENERATION_3D_SHOW_MODES`] stays the single ladder both
/// this picker and `cycleShowMode` walk.
fn show_mode_row(mode: &str, is_de: bool) -> (&'static str, &'static str) {
    match mode {
        "shaded+edges" => ("generation3d-measure-show-edges", if is_de { "Schattiert + Kanten" } else { "Shaded + edges" }),
        "wireframe" => ("generation3d-measure-show-wireframe", if is_de { "Drahtgitter" } else { "Wireframe" }),
        "points" => ("generation3d-measure-show-points", if is_de { "Punkte" } else { "Points" }),
        _ => ("generation3d-measure-show-shaded", if is_de { "Schattiert" } else { "Shaded" }),
    }
}

/// 👁️ Preview shading mode for the world-3d window.
pub fn show_mode_measure(show_mode: &str, is_de: bool, procedural_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor) -> WindowMeasure {
    let current = if show_mode.is_empty() { "shaded" } else { show_mode };
    WindowMeasure::Select {
        id: "generation3d-measure-show".into(),
        label: Some(if is_de { "Anzeige" } else { "Show" }.into()),
        value: current.into(),
        items: semio_s_artifact_procedural_3d_editor_config::GENERATION_3D_SHOW_MODES
            .iter()
            .map(|mode| {
                let (id, label) = show_mode_row(mode, is_de);
                MeasureSelectItem { id: id.into(), value: (*mode).into(), label: label.into() }
            })
            .collect(),
        on_change: procedural_action("setShowMode", None),
    }
}

/// 🎚️ Shared preview-window chrome measures (show-mode toggle + sun group) — reused by both preview
/// windows (edit mode's 3D preview and generate mode's generation preview).
pub fn preview_window_measures(config: &Generation3dConfig, is_de: bool, procedural_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, is_de, procedural_action), world3d_sun_measures("generation3d", &sun, is_de, procedural_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, preview_eval_text: Option<&str>, session: &FlowEvalSession, run: Option<&semio_plugin::ToolRunView>, active_utility: &str, marks: &PreviewInteractionMarks, labels: &Generation3dLabels, windows: &semio_plugin::TreeWindows<'_>, is_de: bool) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = preview_eval_text.unwrap_or_default().to_string();
    let payload = preview_payload(&eval_json, &document.host_snapshot, config, Some(session), marks);
    let selection_json = preview_selection_json(config, active_utility, &payload);
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    let preview_status = preview_status_json(&eval_json, &document.host_snapshot);
    let fit_json = semio_s_artifact_procedural_3d_preview_eval::preview_fit_json(&document.host_snapshot, &meshes_json);
    let sun = config.sun();
    let status_json = preview_window_status_json(Some(session), run, preview_status, &PreviewStatusDebug { meshes_json: &meshes_json, instances_json: &instances_json }, None);
    let _ = GENERATION_3D_PLAY_APP_ID;
    let surface = semio_s_procedural_core_semantic_ui::accessible_scene_surface(
        GENERATION_3D_PLAY_SURFACE_PREVIEW,
        semio_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_ui_declaration::ui::World3dScene {
            status_json,
            fit_json: Some(fit_json),
            domain_id: Some(if marks.components.active() { semio_s_artifact_procedural_3d_editor_selection::DOMAIN } else { GENERATION_3D_INTERACTION_DOMAIN }.into()),
            domain_granularity_id: Some(if marks.components.active() { marks.components.granularity.as_str() } else { GENERATION_3D_INTERACTION_GRANULARITY }.into()),
            ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun)
        },
        labels.preview_canvas.as_str(),
        labels.preview_canvas_hint.as_str(),
        semio_ui_contract::Liveness::Polite,
    )?;
    let rows = semio_s_artifact_procedural_3d_preview_eval::preview_value_rows(&eval_json);
    semio_s_procedural_core_semantic_ui::scene_with_channel_values(surface, windows, GENERATION_3D_PLAY_APP_ID, "generation3d-preview-values", if is_de { "Werte" } else { "Values" }, &rows)
}
//#endregion 🔖️Render

//#region 🔖️PreviewPipeline
/// 🎥️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// every function here references [`Generation3dConfig`] (directly, or is reachable only from a
/// function that does), which made this app behavior, not artifact-schema-pure document compute; the
/// snapshot-pure fixture/gumball helpers stayed in `crate::standards::v1::subsets::any::schema` instead.

/// 🧵️ The surface-neutral half of this pipeline lives in [`crate::preview_eval`], shared verbatim
/// with `👁️viewer` — the geometry-handle grammar, the channel walk, the marker meshes, the show-mode
/// filter, the LOD ladder, the mesh-pack decode, the pending-handle scan and the tessellate producer
/// are ONE implementation, not an editor original plus a read-only twin. Re-exported here so this
/// surface's own call sites (and `📌️panels`, `🎭️modes`, the io bridge) keep naming them unqualified
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).


pub fn preview_camera_json(cfg: &Generation3dConfig) -> String {
    semio_ui_declaration::ui::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}


//#region 🔖️PreviewInteraction
/// 🕹️ Shared graph and object selection; editor component picks use [`selection::DOMAIN`].
pub const GENERATION_3D_INTERACTION_DOMAIN: &str = "graph";

/// 🐁️ The channel a pointer hovers on. `InteractionState.hover` holds exactly one live channel per
/// domain, so reading any other channel reads empty rather than stale.
pub const GENERATION_3D_INTERACTION_CHANNEL: &str = "pointer";

/// 🎯️ The granularity a plain world-3d instance pick/hover reports. A preview instance id is
/// channel-qualified (`{widgetId}@{channel}#{index}`), which is the node graph's own `handle`
/// (port) target shape — so a world hit and a graph port hit land on the same granularity.
pub const GENERATION_3D_INTERACTION_GRANULARITY: &str = "handle";

/// 🕹️ One render's graph marks and independent geometry-component selection.
///
/// A preview instance id is `{widgetId}@{channel}#{index}`, so an id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}` — byte-identical to the
/// port id the node graph's own picks already use) or its widget (`{widgetId}`). That three-level
/// match is exactly what makes hover bidirectional: hovering a node in the graph lights up every
/// one of its channels' preview geometry, and hovering one preview instance in the world lights up
/// its node — and its port — back in the graph.
///
/// 🎯️ Object mode reports the port; component mode uses the full instance in a flat domain.
/// In object mode an instance carries
/// `interactionId = {widgetId}@{channel}` (see `preview_payload`) because `validate_state` prunes
/// any hover/selection id absent from `interaction_topology`, and the per-index instance count is
/// evaluation-derived so it cannot be declared there.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreviewInteractionMarks {
    pub source_revision: Option<String>,
    pub components: selection::ComponentSelection,
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl PreviewInteractionMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The app stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self { source_revision: None, components: selection::ComponentSelection::from_interaction(interaction), hovered: interaction.hover(GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_CHANNEL).ids.iter().cloned().collect(), selected: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).ids.iter().cloned().collect() }
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

    /// 🪢️ These marks as an open gumball gesture leaves them once committed: its `graph` nodes selected and, for a
    /// component gesture, its components re-addressed onto the component operator; hover is untouched.
    pub fn following(self, gesture: &transform_commands::GumballSelection) -> Self {
        let components = match &gesture.components {
            Some((_, ids)) => selection::ComponentSelection { selected: ids.clone(), ..self.components },
            None => self.components,
        };
        Self { source_revision: self.source_revision, components, hovered: self.hovered, selected: gesture.nodes.iter().cloned().collect() }
    }

    /// 🕸️ The widget id behind any interaction id — `{w}`, `{w}@{c}` and `{w}@{c}#{i}` all resolve
    /// to `{w}`.
    pub fn widget_of(id: &str) -> &str {
        let base = id.split('#').next().unwrap_or(id);
        base.split('@').next().unwrap_or(base)
    }

    /// 🕸️ The channel (port) id behind an interaction id, `None` for a bare widget id.
    pub fn port_of(id: &str) -> Option<&str> {
        let base = id.split('#').next().unwrap_or(id);
        base.split_once('@').map(|(_, port)| port)
    }

    fn component_hover(&self)->Option<selection::ComponentTarget<'_>>{
        if !self.hovered.is_empty()||!self.components.active(){return None;}
        self.components.hovered.as_deref().and_then(selection::ComponentTarget::parse)
            .filter(|target|target.granularity==self.components.granularity)
    }

    /// 🕸️ Projects the active pointer source to its graph node and optional port.
    pub fn hovered_graph_target(&self)->Option<(String,Option<String>)>{
        if let Some(id)=self.hovered.iter().next(){
            return Some((Self::widget_of(id).to_owned(),Self::port_of(id).map(str::to_owned)));
        }
        self.component_hover().map(|target|(target.widget.to_owned(),Some(target.channel.to_owned())))
    }

    /// 💡️ Returned graph highlights share the same pointer ownership as its hover target.
    pub fn graph_highlight_ids(&self)->Vec<String>{
        let mut ids=self.hovered.clone();
        if let Some(target)=self.component_hover(){
            ids.insert(target.widget.to_owned());
            ids.insert(format!("{}@{}",target.widget,target.channel));
        }
        ids.extend(self.hovered.iter().map(|id|Self::widget_of(id).to_owned()));
        ids.into_iter().collect()
    }

    /// 🎯️ Keeps component selection while the graph owns a simultaneous pointer hover.
    pub fn preview_components(&self)->selection::ComponentSelection{
        let mut components=self.components.clone();
        if !self.hovered.is_empty(){components.hovered=None;}
        components
    }

    /// 🕸️ The `graph` selection split into the node and edge domains a context menu addresses. A
    /// synapse id is an `edge`; everything else projects onto its owning widget id (a preview
    /// instance `{w}@{c}#{i}` selected in the world therefore targets its node exactly like a click
    /// in the graph does).
    pub fn graph_selection_domains(&self, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> (Vec<String>, Vec<String>) {
        let synapses: std::collections::BTreeSet<&str> = host_snapshot.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
        let mut nodes = std::collections::BTreeSet::new();
        let mut edges = std::collections::BTreeSet::new();
        for id in &self.selected {
            if synapses.contains(id.as_str()) {
                edges.insert(id.clone());
            } else {
                nodes.insert(Self::widget_of(id).to_string());
            }
        }
        (nodes.into_iter().collect(), edges.into_iter().collect())
    }

    /// 🕸️ The `graph` selection projected onto widget ids — what `NodeGraphScene::selection` paints.
    pub fn graph_selection_ids(&self) -> Vec<String> {
        self.selected.iter().map(|id| Self::widget_of(id).to_string()).collect::<std::collections::BTreeSet<String>>().into_iter().collect()
    }
}
//#endregion 🔖️PreviewInteraction

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow the window-projected `ViewModel.active_utility_id`, and with the live
/// `graph` marks `render_with_request_context` resolved — the gumball now shows for a real
/// selection instead of always reporting empty. `"rectangle"` (the pre-migration default
/// `selection_method`) is hardcoded: the framework tracks no persistent "last marquee method"
/// outside a live gesture. Every gumball is live (`gumballLiveDispatch`): the host streams its
/// ticks into the guest's open tool transaction and paints the guest's answer — the transform
/// operator re-evaluated with everything downstream of it — instead of moving the instance locally.
pub fn preview_selection_json(cfg: &Generation3dConfig, active_utility: &str, payload: &PreviewPayload) -> String {
    let mut value = semio_framework_pack_json::parse(&semio_plugin::world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref()), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
    let show_mode = if cfg.show_mode.is_empty() { "shaded" } else { cfg.show_mode.as_str() };
    let (show_edges, selection_mode) = match show_mode {
        "wireframe" => (true, "mesh"),
        "points" => (false, "mesh"),
        "shaded+edges" => (true, "mesh"),
        _ => (false, "mesh"),
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode", semio_framework_pack_json::Value::String(active_utility.to_string()));
        object.insert("gumballActive", semio_framework_pack_json::Value::Bool(!payload.selected_ids.is_empty() && !active_utility.is_empty()));
        object.insert("gumballLiveDispatch", semio_framework_pack_json::Value::Bool(true));
        object.insert("showEdges", semio_framework_pack_json::Value::Bool(show_edges));
        object.insert("selectionMode", semio_framework_pack_json::Value::String(selection_mode.to_string()));
        object.insert("granularity", semio_framework_pack_json::Value::String(selection_mode.to_string()));
        payload.components.project(&semio_framework_pack_json::parse(&payload.instances_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::Value::Null), &semio_framework_pack_json::parse(&payload.meshes_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::Value::Null), object);
        if payload.components.active() {
            if let Some(pivot) = payload.component_pivot {
                object.insert("gumballTarget", vec3_json(pivot));
                object.insert("gumballActive", semio_framework_pack_json::Value::Bool(!active_utility.is_empty()));
            }
        }
    }
    semio_framework_pack_json::to_string(&value)
}

/// 📈️ The status projection ALL THREE World3d preview windows publish (edit, generate, view) lives
/// in the surface-neutral [`crate::preview_eval`] — one schema, one place. Re-exported here so this
/// surface's own windows keep naming it unqualified (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).


/// 👁️ One preview render's world-3d payload: the handle-deduplicated mesh table, the per-channel
/// instance table, and the instance ids the live `graph` marks resolved to — so the scene's
/// `selection_json` paints exactly the same hover/selection the instances themselves carry.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewPayload {
    pub components: selection::ComponentSelection,
    pub component_pivot: Option<[f64; 3]>,
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
}

/// 👁️ An empty payload is the empty JSON ARRAY, not the empty string — every consumer feeds these
/// straight into a `World3dScene` and compares them against `"[]"`.
impl Default for PreviewPayload {
    fn default() -> Self {
        Self { components: selection::ComponentSelection::default(), component_pivot: None, meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None }
    }
}

/// 🧊️ The interaction-free, session-free entry point: the mesh-export bridge and the schema tests
/// evaluate geometry without any live window, so they carry no marks and no tessellation cache.
pub fn preview_payload_from_eval(eval_json: &str, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig) -> (String, String) {
    let payload = preview_payload(eval_json, host_snapshot, cfg, None, &PreviewInteractionMarks::default());
    (payload.meshes_json, payload.instances_json)
}

/// 👁️ One preview instance per geometry-bearing value per OUTPUT CHANNEL — the whole point of the
/// channel-qualified ids: a widget with several outputs previews every one of them, not just the
/// first handle its evaluation happened to expose.
/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields below.
fn vec3_json(v: [f64; 3]) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::Value::Array(v.into_iter().map(semio_framework_pack_json::Value::from).collect())
}

pub fn preview_payload(eval_json: &str, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig, session: Option<&FlowEvalSession>, marks: &PreviewInteractionMarks) -> PreviewPayload {
    let eval = match semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(value) if value.get("error").and_then(semio_framework_pack_json::Value::as_str).is_none() => value,
        _ => return PreviewPayload::default(),
    };
    let hover_sources = semio_s_artifact_procedural_3d_preview_eval::preview_hover_sources(host_snapshot, &marks.hovered);
    let lookup = session.map(PreviewMeshLookup::new);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<semio_framework_pack_json::Value> = Vec::new();
    let mut instances: Vec<semio_framework_pack_json::Value> = Vec::new();
    let mut mesh_id_by_handle: HashMap<String, String> = HashMap::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    let component_selected: std::collections::BTreeSet<&str> = marks.components.selected.iter().filter_map(|id| selection::ComponentTarget::parse(id)).filter(|target| target.granularity == marks.components.granularity).map(|target| target.instance).collect();
    let component_hovered = marks.component_hover();
    let component_group = selection::component_group(&marks.components.selected).ok().filter(|(target, _)| target.index == 0 && target.granularity == marks.components.granularity);
    let mut component_pivot = None;
    for widget in &host_snapshot.widgets {
        let id = semio_s_artifact_procedural_3d_model::widget_id(widget).to_string();
        let preview = semio_s_artifact_procedural_3d_model::widget_previews(widget);
        if !preview {
            continue;
        }
        let items = preview_channel_items_for_widget(&eval, &id);
        if items.is_empty() {
            continue;
        }
        for item in &items {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let instance_id = format!("{id}@{channel}#{index}");
            let resolved = lookup.as_ref().and_then(|lookup| lookup.resolve(&id, item));
            let source_handle = resolved.as_ref().map(|(source, _)| source.as_ref());
            let exact_native = source_handle == Some(handle.as_str());
            if marks.components.active() && inline.is_none() && !exact_native { continue; }
            let mesh_id = if let Some(cached) = source_handle.and_then(|source| mesh_id_by_handle.get(source)) {
                cached.clone()
            } else {
                let data = match inline {
                    Some(inline) => inline_preview_mesh(inline),
                    None => resolved.as_ref().map(|(_, mesh)| (*mesh).clone()),
                };
                let Some(data) = data.map(|mesh| apply_show_mode_mesh(mesh, show_mode)).filter(mesh_has_preview_geometry) else { continue };
                let mesh_id = format!("eval-{instance_id}");
                let mut mesh_object = semio_framework_pack_json::Object::new();
                mesh_object.insert("id", semio_framework_pack_json::Value::String(mesh_id.clone()));
                mesh_object.insert("role", semio_framework_pack_json::Value::String(preview_mesh_role(inline.as_ref(), &data).to_string()));
                mesh_object.insert("data", semio_framework_pack_json::Value::from(data));
                meshes.push(semio_framework_pack_json::Value::Object(mesh_object));
                if let Some(source) = source_handle {
                    mesh_id_by_handle.insert(source.to_owned(), mesh_id.clone());
                }
                mesh_id
            };
            if let (Some((target, ids)), Some(PreviewInlineGeometry::Mesh { data, .. })) = (&component_group, inline) {
                if target.instance == instance_id { component_pivot = selection::component_pivot(data, target.granularity, ids); }
            }
            let selected = if marks.components.active() { component_selected.contains(instance_id.as_str()) } else { marks.selects(&id, channel, *index) };
            let hovered = component_hovered.as_ref().is_some_and(|target| target.instance == instance_id)
                || marks.hovers(&id,channel,*index)
                || hover_sources.contains(&id) || hover_sources.contains(&format!("{id}@{channel}"));
            if selected {
                selected_ids.push(instance_id.clone());
            }
            if hovered && hovered_id.is_none() {
                hovered_id = Some(instance_id.clone());
            }
            let mut instance_object = semio_framework_pack_json::Object::new();
            instance_object.insert("id", semio_framework_pack_json::Value::String(instance_id.clone()));
            instance_object.insert("meshId", semio_framework_pack_json::Value::String(mesh_id));
            instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
            instance_object.insert("rotation", semio_framework_pack_json::Value::Array(vec![semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(1.0)]));
            instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
            instance_object.insert("label", semio_framework_pack_json::Value::String(format!("{id}@{channel}")));
            instance_object.insert("interactionId", semio_framework_pack_json::Value::String(if marks.components.active() { instance_id } else { format!("{id}@{channel}") }));
            if let Some(revision) = marks.source_revision.as_ref().filter(|_| exact_native) {
                let mut source = semio_framework_pack_json::Object::new();
                source.insert("handle", semio_framework_pack_json::Value::String(handle.clone()));
                source.insert("revision", semio_framework_pack_json::Value::String(revision.clone()));
                instance_object.insert("componentSource", semio_framework_pack_json::Value::Object(source));
            }
            instance_object.insert("selected", semio_framework_pack_json::Value::Bool(selected));
            instance_object.insert("hovered", semio_framework_pack_json::Value::Bool(hovered));
            instances.push(semio_framework_pack_json::Value::Object(instance_object));
        }
    }
    PreviewPayload { components: marks.preview_components(), component_pivot, meshes_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(meshes)), instances_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(instances)), selected_ids, hovered_id }
}
//#endregion 🔖️PreviewPipeline

pub fn edits_components(view: Option<&semio_plugin::ViewModel>, granularity: Option<&str>) -> bool {
    let Some(view) = view else { return false };
    let kind = view.window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id)).map(|window| window.window_kind_id.as_str()).or(view.active_window_kind_id.as_deref());
    kind == Some(GENERATION_3D_PLAY_WINDOW_PREVIEW) && matches!(granularity, Some("vertex" | "edge" | "face"))
}
