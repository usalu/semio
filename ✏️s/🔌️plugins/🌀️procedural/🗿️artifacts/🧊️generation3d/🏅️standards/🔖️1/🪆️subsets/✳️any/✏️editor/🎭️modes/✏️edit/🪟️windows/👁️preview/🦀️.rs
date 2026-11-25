//! 👁️ Generation3d play app — the 3D preview window (edit mode): the tessellated evaluated geometry.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::{graph_port_code, graph_port_id, parse_graph_port_id, GraphTargetKind, GENERATION_3D_PLAY_APP_ID};
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
use semio_s_artifact_procedural_3d_preview_eval::PreviewInlineGeometry;
use semio_s_artifact_procedural_3d_preview_eval::PreviewChannelItem;
use semio_s_artifact_procedural_3d_preview_eval::{PreviewBounds, PreviewEvaluation, preview_mesh_table, preview_tolerance, preview_viewport_items_for_widget, preview_widget_ids};

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

/// 🎚️ Provides shading, detail, and sun controls for both editor preview windows.
pub fn preview_window_measures(config: &Generation3dConfig, is_de: bool, procedural_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    let mut measures = vec![show_mode_measure(&config.show_mode, is_de, procedural_action)];
    measures.extend(crate::windows::flow::window_measures(&config.lod_mode, is_de, procedural_action));
    measures.push(world3d_sun_measures("generation3d", &sun, is_de, procedural_action));
    measures
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, preview_eval_text: Option<&str>, session: &FlowEvalSession, run: Option<semio_plugin::ToolRunPollView<'_>>, active_utility: &str, marks: &PreviewInteractionMarks, labels: &Generation3dLabels, windows: &semio_plugin::TreeWindows<'_>, locale: semio_plugin::Locale, hover_groups: &mut Option<semio_s_artifact_procedural_3d_preview_eval::PreviewHoverGroups>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let evaluation = PreviewEvaluation::from_json(preview_eval_text.unwrap_or_default());
    let mut payload = preview_payload(&evaluation, &document.host_snapshot, config, Some(session), marks);
    *hover_groups = Some(std::mem::take(&mut payload.hover_groups));
    let selection_json = preview_selection_json(config, active_utility, &payload);
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    let preview_status = preview_status_json(&evaluation, &document.host_snapshot);
    let fit_json = semio_s_artifact_procedural_3d_preview_eval::preview_fit_json(&document.host_snapshot, payload.bounds);
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
    let rows = semio_s_artifact_procedural_3d_preview_eval::preview_value_rows(&evaluation);
    let component_output = marks.component_hover().map(|target| graph_port_id(target.widget, &target.channel(), false));
    let hover = semio_s_artifact_procedural_3d_preview_eval::PreviewChannelHover::new(&document.host_snapshot, &marks.hovered, component_output);
    let hovered_ports = hover.matching(rows.iter().map(|row| row.id.as_str()));
    semio_s_procedural_core_semantic_ui::scene_with_channel_values(surface, windows, GENERATION_3D_PLAY_APP_ID, "generation3d-preview-values", &semio_s_procedural_core_semantic_ui::ChannelValueLabels::new(locale, labels.graph_input_port.as_str(), labels.graph_output_port.as_str()), &rows, &hovered_ports)
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
/// Each direction has its own world instance. An id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}` — byte-identical to the
/// port id the node graph's own picks already use) or its widget (`{widgetId}`). That three-level
/// match is exactly what makes hover bidirectional: hovering a node in the graph lights up every
/// one of its channels' preview geometry, and hovering one preview instance in the world lights up
/// its node — and its port — back in the graph.
///
/// 🎯️ Object mode reports the port; component mode uses the full instance in a flat domain.
/// In object mode an instance carries
/// `interactionId` for its namespaced port (see `preview_payload`) because `validate_state` prunes
/// any hover/selection id absent from `interaction_topology`, and the per-index instance count is
/// evaluation-derived so it cannot be declared there.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreviewInteractionMarks {
    pub source_revision: Option<String>,
    pub graph_hover_source: Option<Option<String>>,
    pub component_hover_source: Option<Option<String>>,
    pub components: selection::ComponentSelection,
    pub hovered: Vec<String>,
    pub selected: std::collections::BTreeSet<String>,
    pub selection_granularity: String,
}

impl PreviewInteractionMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The app stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let source = |domain| interaction.hover_source_window(domain, GENERATION_3D_INTERACTION_CHANNEL).map(|window| window.map(str::to_owned));
        Self {
            source_revision: None,
            graph_hover_source: source(GENERATION_3D_INTERACTION_DOMAIN),
            component_hover_source: source(selection::DOMAIN),
            components: selection::ComponentSelection::from_interaction(interaction),
            hovered: interaction.hover(GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_CHANNEL).ids.clone(),
            selected: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).ids.iter().cloned().collect(),
            selection_granularity: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).granularity.clone(),
        }
    }

    /// 🪢️ These marks as an open gumball gesture leaves them once committed: its `graph` nodes selected and, for a
    /// component gesture, its components re-addressed onto the component operator; hover is untouched.
    pub fn following(self, gesture: &transform_commands::GumballSelection) -> Self {
        let components = match &gesture.components {
            Some((_, ids)) => selection::ComponentSelection { selected: ids.clone(), ..self.components },
            None => self.components,
        };
        Self { components, selected: gesture.nodes.iter().map(|id| GraphTargetKind::Node.target(id)).collect(), selection_granularity: "node".into(), ..self }
    }

    fn target_widget(kind: GraphTargetKind, id: &str) -> Option<&str> {
        match kind {
            GraphTargetKind::Node => Some(id),
            GraphTargetKind::Handle => parse_graph_port_id(id).map(|port| port.widget),
            GraphTargetKind::Edge => None,
        }
    }

    fn component_hover(&self)->Option<selection::ComponentTarget<'_>>{
        if !self.hovered.is_empty()||!self.components.active(){return None;}
        self.components.hovered.as_deref().and_then(selection::ComponentTarget::parse)
            .filter(|target|target.granularity==self.components.granularity)
    }

    /// 🕸️ Projects the first framework hover target with its original graph granularity.
    pub fn hovered_graph_target(&self) -> Option<semio_ui_declaration::ui::NodeGraphHover> {
        use semio_s_artifact_procedural_3d_model::GraphTargetKind::{Edge, Handle, Node};
        use semio_ui_declaration::ui::NodeGraphHover;
        if let Some((kind, id)) = self.hovered.iter().find_map(|id| GraphTargetKind::parse(id)) {
            let source = self.graph_hover_source.clone();
            return Some(match kind {
                Node => NodeGraphHover { node_id: Some(id.into()), port_id: None, port_ids: Vec::new(), edge_id: None, hover_source_window_id: source },
                Edge => NodeGraphHover { node_id: None, port_id: None, port_ids: Vec::new(), edge_id: Some(id.into()), hover_source_window_id: source },
                Handle => {
                    let port = parse_graph_port_id(id)?;
                    let mut port_ids: Vec<_> = self.hovered.iter().filter_map(|target| Handle.raw(target)).map(str::to_owned).collect();
                    if port_ids.len() < 2 { port_ids.clear(); }
                    NodeGraphHover { node_id: Some(port.widget.into()), port_id: Some(graph_port_code(&port.channel, port.input).into_owned()), port_ids, edge_id: None, hover_source_window_id: source }
                }
            });
        }
        self.component_hover().map(|target| NodeGraphHover {
            node_id: Some(target.widget.into()),
            port_id: Some(graph_port_code(&target.channel(), false).into_owned()),
            port_ids: Vec::new(),
            edge_id: None,
            hover_source_window_id: self.component_hover_source.clone(),
        })
    }

    /// 💡️ Graph labels use the authored IDs of hovered widgets and the owning widgets of hovered ports.
    pub fn graph_highlight_ids(&self) -> Vec<String> {
        let mut nodes: std::collections::BTreeSet<&str> = self.hovered.iter().filter_map(|id| {
            let (kind, id) = GraphTargetKind::parse(id)?;
            Self::target_widget(kind, id)
        }).collect();
        let component = self.component_hover();
        if let Some(target) = &component { nodes.insert(target.widget); }
        nodes.into_iter().map(str::to_owned).collect()
    }

    /// 🎯️ Keeps component selection while the graph owns a simultaneous pointer hover.
    pub fn preview_components(&self)->selection::ComponentSelection{
        let mut components=self.components.clone();
        if !self.hovered.is_empty(){components.hovered=None;}
        components
    }

    /// 🕸️ Projects graph selection onto the authored widgets and wires used by the context menu.
    pub fn graph_selection_domains(&self) -> (Vec<String>, Vec<String>) {
        let edges = self.selected.iter().filter_map(|id| GraphTargetKind::Edge.raw(id).map(str::to_owned)).collect();
        (self.graph_selection_ids(), edges)
    }

    /// 📌️ The canvas restores selections against its raw nodes, wires and canonical port IDs.
    pub fn scene_selection_ids(&self) -> Vec<String> {
        let Some(kind) = GraphTargetKind::for_granularity(&self.selection_granularity) else { return Vec::new() };
        self.selected.iter().filter_map(|id| kind.raw(id).map(str::to_owned)).collect()
    }

    /// 🕸️ The selected authored widgets, including the owners of selected port handles.
    pub fn graph_selection_ids(&self) -> Vec<String> {
        self.selected.iter().filter_map(|id| {
            let (kind, id) = GraphTargetKind::parse(id)?;
            Self::target_widget(kind, id)
        }).collect::<std::collections::BTreeSet<_>>().into_iter().map(str::to_owned).collect()
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
        object.insert("hoveredIds", semio_framework_pack_json::Value::Array(payload.hovered_ids.iter().cloned().map(semio_framework_pack_json::Value::String).collect()));
        semio_s_artifact_procedural_3d_preview_eval::stamp_preview_hover_source(object, payload.hover_source_window.as_ref());
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
    pub bounds: Option<PreviewBounds>,
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
    pub hovered_ids: Vec<String>,
    pub hover_source_window: Option<Option<String>>,
    pub hover_groups: semio_s_artifact_procedural_3d_preview_eval::PreviewHoverGroups,
}

/// 👁️ An empty payload is the empty JSON ARRAY, not the empty string — every consumer feeds these
/// straight into a `World3dScene` and compares them against `"[]"`.
impl Default for PreviewPayload {
    fn default() -> Self {
        Self { components: selection::ComponentSelection::default(), component_pivot: None, bounds: None, meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None, hovered_ids: Vec::new(), hover_source_window: None, hover_groups: Default::default() }
    }
}

/// 🧊️ Projects a published evaluation without live marks or a tessellation cache.
pub fn preview_payload_from_eval(eval_json: &str, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig) -> (String, String) {
    let payload = preview_payload(&PreviewEvaluation::from_json(eval_json), host_snapshot, cfg, None, &PreviewInteractionMarks::default());
    (payload.meshes_json, payload.instances_json)
}

/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields below.
fn vec3_json(v: [f64; 3]) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::Value::Array(v.into_iter().map(semio_framework_pack_json::Value::from).collect())
}

/// 👁️ Projects one world instance for each spatial value in each preview-enabled input or output.
pub fn preview_payload(evaluation: &PreviewEvaluation<'_>, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig, session: Option<&FlowEvalSession>, marks: &PreviewInteractionMarks) -> PreviewPayload {
    let component_mode = marks.components.active();
    let component_hovered = marks.component_hover();
    let hover_source_window = if !marks.hovered.is_empty() { marks.graph_hover_source.clone() }
        else { component_hovered.as_ref().and(marks.component_hover_source.as_ref()).cloned() };
    let Some(eval) = evaluation.geometry() else { return PreviewPayload { hover_source_window, ..Default::default() }; };
    let selected_graph = semio_s_artifact_procedural_3d_preview_eval::PreviewGraphTargets::from_ids(&marks.selected);
    let hovered_graph = semio_s_artifact_procedural_3d_preview_eval::PreviewGraphTargets::from_ids(&marks.hovered);
    let wire_hover = semio_s_artifact_procedural_3d_preview_eval::preview_wire_hover(host_snapshot, &hovered_graph);
    let preview_ids = preview_widget_ids(host_snapshot);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let Some(table) = preview_mesh_table(evaluation, &preview_ids, session, show_mode, preview_tolerance(&cfg.lod_mode)) else {
        return PreviewPayload { hover_source_window, ..Default::default() };
    };
    let mut instances: Vec<semio_framework_pack_json::Value> = Vec::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    let mut hovered_ids = Vec::new();
    let component_selected: std::collections::BTreeSet<&str> = marks.components.selected.iter().filter_map(|id| selection::ComponentTarget::parse(id)).filter(|target| target.granularity == marks.components.granularity).map(|target| target.instance).collect();
    let component_group = selection::component_group(&marks.components.selected).ok().filter(|(target, _)| target.index == 0 && target.granularity == marks.components.granularity);
    let mut component_pivot = None;
    for id in &preview_ids {
        let items = preview_viewport_items_for_widget(eval, id);
        if items.is_empty() {
            continue;
        }
        for item in &items {
            let PreviewChannelItem { handle, inline, .. } = item;
            let address = item.address(id);
            let instance_id = &address.instance;
            let Some(placement) = table.placements.get(instance_id) else { continue };
            let source_revision = marks.source_revision.as_ref().filter(|_| !item.is_input() && placement.source_handle.as_deref() == Some(handle.as_str()));
            let component_pickable = source_revision.is_some() || (!item.is_input() && item.is_channel_value() && matches!(inline, Some(PreviewInlineGeometry::Mesh { .. })));
            if let (true, Some((target, ids)), Some(PreviewInlineGeometry::Mesh { data, .. })) = (component_pickable, &component_group, inline) {
                if target.instance == instance_id.as_str() { component_pivot = selection::component_pivot(data, target.granularity, ids); }
            }
            let selected = if component_mode { component_pickable && component_selected.contains(instance_id.as_str()) } else { address.marked(&selected_graph) };
            let graph_hovered = address.marked(&hovered_graph)
                || wire_hover.matches(id, &address.port, item.is_input());
            let hovered = graph_hovered || component_pickable && component_hovered.as_ref().is_some_and(|target| target.instance == instance_id.as_str());
            if selected {
                selected_ids.push(instance_id.clone());
            }
            if graph_hovered { hovered_ids.push(instance_id.clone()); }
            if hovered && hovered_id.is_none() {
                hovered_id = Some(instance_id.clone());
            }
            let mut instance_object = semio_framework_pack_json::Object::new();
            instance_object.insert("id", semio_framework_pack_json::Value::String(instance_id.clone()));
            instance_object.insert("meshId", semio_framework_pack_json::Value::String(placement.mesh_id.clone()));
            instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
            instance_object.insert("rotation", semio_framework_pack_json::Value::Array(vec![semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(0.0), semio_framework_pack_json::Value::from(1.0)]));
            instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
            instance_object.insert("label", semio_framework_pack_json::Value::String(address.label()));
            instance_object.insert("interactionId", semio_framework_pack_json::Value::String(if component_mode && component_pickable { instance_id.clone() } else { address.interaction_id() }));
            if component_mode && !component_pickable {
                instance_object.insert("componentPickable", semio_framework_pack_json::Value::Bool(false));
            }
            if let Some(revision) = source_revision {
                let mut source = semio_framework_pack_json::Object::new();
                source.insert("handle", semio_framework_pack_json::Value::String(handle.clone()));
                source.insert("revision", semio_framework_pack_json::Value::String(revision.clone()));
                instance_object.insert("componentSource", semio_framework_pack_json::Value::Object(source));
            }
            instance_object.insert("selected", semio_framework_pack_json::Value::Bool(selected));
            instances.push(semio_framework_pack_json::Value::Object(instance_object));
        }
    }
    let hover_groups = if component_mode { Default::default() } else {
        semio_s_artifact_procedural_3d_preview_eval::attach_preview_hover_targets(&mut instances, GENERATION_3D_INTERACTION_GRANULARITY, &table.hover_aliases, table.hover_revision)
    };
    PreviewPayload { components: marks.preview_components(), component_pivot, bounds: table.bounds, meshes_json: table.meshes_json.clone(), instances_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(instances)), selected_ids, hovered_id, hovered_ids, hover_source_window, hover_groups }
}
//#endregion 🔖️PreviewPipeline

pub fn edits_components(view: Option<&semio_plugin::ViewModel>, granularity: Option<&str>) -> bool {
    let Some(view) = view else { return false };
    let kind = view.window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id)).map(|window| window.window_kind_id.as_str()).or(view.active_window_kind_id.as_deref());
    kind == Some(GENERATION_3D_PLAY_WINDOW_PREVIEW) && matches!(granularity, Some("vertex" | "edge" | "face"))
}
