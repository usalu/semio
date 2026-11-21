//! 👁️ Generation3d viewer — the Preview window: a read-only `World3dScene` over the evaluated,
//! tessellated flow geometry, bound to the framework's `graph` interaction domain so hover and
//! selection paint here exactly as they do on the sibling surface's own preview.
//!
//! 🕹️ The window declares `SurfaceKind::World3d` with `domain_id = "graph"` and
//! `domain_granularity_id = "handle"`. Each instance carries the directional channel address
//! declared by `interaction_topology`, and overlapping geometry can publish its channel addresses
//! together. Hover comes from the ephemeral pointer channel and selection from the persisted
//! interaction store (see `👁️viewer/🫧️transient/🦀️.rs`).
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

use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_framework_pack_json::{Object, Value};
use semio_plugin::app::InteractionView;
use semio_plugin::world3d_scene;
use semio_plugin::world3d_selection_json;
use semio_plugin::world3d_sun_measures;
use semio_plugin::ActionDescriptor;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MeasureSelectItem;
use semio_plugin::SurfaceKind;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowMeasure;
use semio_plugin::WindowOptions;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

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
        initial_utility_id: None,
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
pub fn show_mode_measure(show_mode: &str, is_de: bool, viewer_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor) -> WindowMeasure {
    let current = if show_mode.is_empty() { "shaded" } else { show_mode };
    WindowMeasure::Select {
        id: "generation3d-view-measure-show".into(),
        label: Some(if is_de { "Anzeige" } else { "Show" }.into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "generation3d-view-measure-show-shaded".into(), value: "shaded".into(), label: if is_de { "Schattiert" } else { "Shaded" }.into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-edges".into(), value: "shaded+edges".into(), label: if is_de { "Schattiert + Kanten" } else { "Shaded + edges" }.into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-wireframe".into(), value: "wireframe".into(), label: if is_de { "Drahtgitter" } else { "Wireframe" }.into() },
            MeasureSelectItem { id: "generation3d-view-measure-show-points".into(), value: "points".into(), label: if is_de { "Punkte" } else { "Points" }.into() },
        ],
        on_change: viewer_action("setShowMode", None),
    }
}

/// 🔬️ Level-of-detail selector — coarser LOD is the viewer's only mesh-payload size lever.
pub fn lod_mode_measure(lod_mode: &str, is_de: bool, viewer_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor) -> WindowMeasure {
    let current = if lod_mode.is_empty() { "medium" } else { lod_mode };
    WindowMeasure::Select {
        id: "generation3d-view-measure-lod".into(),
        label: Some(if is_de { "Detailgrad" } else { "Detail" }.into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "generation3d-view-measure-lod-coarse".into(), value: "coarse".into(), label: if is_de { "Grob" } else { "Coarse" }.into() },
            MeasureSelectItem { id: "generation3d-view-measure-lod-medium".into(), value: "medium".into(), label: if is_de { "Mittel" } else { "Medium" }.into() },
            MeasureSelectItem { id: "generation3d-view-measure-lod-fine".into(), value: "fine".into(), label: if is_de { "Fein" } else { "Fine" }.into() },
        ],
        on_change: viewer_action("setLodMode", None),
    }
}

/// 🎚️ The Preview window's whole chrome row: show mode, LOD and the sun group.
pub fn preview_window_measures(config: &Generation3dViewConfig, is_de: bool, viewer_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, is_de, viewer_action), lod_mode_measure(&config.lod_mode, is_de, viewer_action), world3d_sun_measures("generation3d-view", &sun, is_de, viewer_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Marks
/// 🕹️ One render's resolved `graph`-domain marks — read-only twin of the sibling surface's own.
///
/// Each direction has its own world instance. An id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}`, byte-identical to the
/// declared `handle` target) or its widget (`{widgetId}`). That three-level match is what makes
/// hover bidirectional between a graph surface and this world surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Generation3dViewMarks {
    pub hover_source_window: Option<Option<String>>,
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl Generation3dViewMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The viewer stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            hover_source_window: interaction.hover_source_window(GENERATION3D_VIEW_INTERACTION_DOMAIN, GENERATION3D_VIEW_INTERACTION_CHANNEL).map(|window| window.map(str::to_owned)),
            hovered: interaction.hover(GENERATION3D_VIEW_INTERACTION_DOMAIN, GENERATION3D_VIEW_INTERACTION_CHANNEL).ids.iter().cloned().collect(),
            selected: interaction.selection(GENERATION3D_VIEW_INTERACTION_DOMAIN).ids.iter().cloned().collect(),
        }
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
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;

/// 🧮️ Evaluates the whole fixture once, IN PROCESS. TEST-ONLY and never on the live path: the
/// served chain evaluates through `ExtensionInvocation` because a guest links no operators at all,
/// while a `--lib` test binary does link them — so a law that needs an evaluation without standing
/// up the whole chain builds one here. `#[cfg(test)]` is the compile-time half of "render never
/// evaluates" (`render_without_a_published_evaluation_paints_the_empty_world`).
#[cfg(test)]
pub fn evaluate_host_snapshot(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> String {
    semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| host.evaluate().unwrap_or_default()).expect("test host closes cold")
}

/// 👁️ Each geometry-bearing input or output value gets its own preview instance, carrying its
/// declared `interactionId` plus the live hovered/selected flags the world host paints with.
pub struct ViewPreviewPayload {
    pub bounds: Option<preview_eval::PreviewBounds>,
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
    pub hovered_ids: Vec<String>,
    pub hover_source_window: Option<Option<String>>,
    pub hover_groups: preview_eval::PreviewHoverGroups,
}

impl Default for ViewPreviewPayload {
    fn default() -> Self {
        Self { bounds: None, meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None, hovered_ids: Vec::new(), hover_source_window: None, hover_groups: Default::default() }
    }
}

pub use preview_eval::{preview_tessellation_count, reset_preview_mesh_table};

pub fn preview_payload(evaluation: &preview_eval::PreviewEvaluation<'_>, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, config: &Generation3dViewConfig, session: Option<&semio_os_flow::FlowEvalSession>, marks: &Generation3dViewMarks) -> ViewPreviewPayload {
    let hover_source_window = marks.hover_source_window.clone();
    let Some(eval) = evaluation.geometry() else { return ViewPreviewPayload { hover_source_window, ..Default::default() }; };
    let selected_graph = preview_eval::PreviewGraphTargets::from_ids(&marks.selected);
    let hovered_graph = preview_eval::PreviewGraphTargets::from_ids(&marks.hovered);
    let wire_hover = preview_eval::preview_wire_hover(host_snapshot, &hovered_graph);
    let tolerance = config.tolerance();
    let show_mode = config.effective_show_mode();
    let preview_ids = preview_eval::preview_widget_ids(host_snapshot);
    let Some(table) = preview_eval::preview_mesh_table(evaluation, &preview_ids, session, show_mode, tolerance) else {
        return ViewPreviewPayload { hover_source_window, ..Default::default() };
    };
    let mut instances: Vec<Value> = Vec::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_ids = Vec::new();
    for id in &preview_ids {
        for item in preview_eval::preview_viewport_items_for_widget(eval, id) {
            let address = item.address(id);
            let Some(placement) = table.placements.get(&address.instance) else { continue };
            let selected = address.marked(&selected_graph);
            let hovered = address.marked(&hovered_graph)
                || wire_hover.matches(id, &address.port, item.is_input());
            let label = address.label();
            let interaction_id = address.interaction_id();
            let instance_id = address.instance;
            if selected {
                selected_ids.push(instance_id.clone());
            }
            if hovered { hovered_ids.push(instance_id.clone()); }
            let mut instance_object = Object::new();
            instance_object.insert("id", Value::String(instance_id));
            instance_object.insert("meshId", Value::String(placement.mesh_id.clone()));
            instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
            instance_object.insert("rotation", Value::Array(vec![Value::from(0.0), Value::from(0.0), Value::from(0.0), Value::from(1.0)]));
            instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
            instance_object.insert("label", Value::String(label));
            instance_object.insert("interactionId", Value::String(interaction_id));
            instance_object.insert("selected", Value::Bool(selected));
            instances.push(Value::Object(instance_object));
        }
    }
    let hover_groups = preview_eval::attach_preview_hover_targets(&mut instances, GENERATION3D_VIEW_INTERACTION_GRANULARITY, &table.hover_aliases, table.hover_revision);
    ViewPreviewPayload { bounds: table.bounds, meshes_json: table.meshes_json.clone(), instances_json: semio_framework_pack_json::to_string(&Value::Array(instances)), selected_ids, hovered_id: hovered_ids.first().cloned(), hovered_ids, hover_source_window, hover_groups }
}

/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields above.
fn vec3_json(v: [f64; 3]) -> Value {
    Value::Array(v.into_iter().map(Value::from).collect())
}

/// 🧭️ World-3d selection payload — the live marks plus the shading flags the show mode implies.
/// A viewer never mounts a gumball, so `transformMode` stays empty and `gumballActive` false: a
/// transform handle is a mutation affordance and a viewer emits no mutations.
pub fn preview_selection_json(config: &Generation3dViewConfig, payload: &ViewPreviewPayload) -> String {
    let mut value = semio_framework_pack_json::parse(&world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref()), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| Value::Object(Object::new()));
    let show_edges = matches!(config.effective_show_mode(), "wireframe" | "shaded+edges");
    if let Some(object) = value.as_object_mut() {
        object.insert("hoveredIds", Value::Array(payload.hovered_ids.iter().cloned().map(Value::String).collect()));
        preview_eval::stamp_preview_hover_source(object, payload.hover_source_window.as_ref());
        object.insert("transformMode", Value::String(String::new()));
        object.insert("gumballActive", Value::Bool(false));
        object.insert("showEdges", Value::Bool(show_edges));
        object.insert("selectionMode", Value::String("mesh".into()));
        object.insert("granularity", Value::String("mesh".into()));
    }
    semio_framework_pack_json::to_string(&value)
}
//#endregion 🔖️Geometry

//#region 🔖️Render
/// 👁️ Pure `(Generation3dSnapshot, Generation3dViewConfig, marks) -> BuiltNode` read: the camera,
/// shading mode, LOD and sun all come from the viewer's own config, hover/selection from the
/// framework-owned `graph` domain, geometry from the ephemeral evaluation when one exists.
pub fn render(document: &Generation3dSnapshot, config: &Generation3dViewConfig, eval_json: Option<&str>, session: Option<&semio_os_flow::FlowEvalSession>, run: Option<&semio_plugin::ToolRunView>, marks: &Generation3dViewMarks, labels: &semio_s_artifact_procedural_3d_terminology::Generation3dLabels, windows: &semio_plugin::TreeWindows<'_>, is_de: bool, hover_groups: &mut Option<semio_s_artifact_procedural_3d_preview_eval::PreviewHoverGroups>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let evaluation = preview_eval::PreviewEvaluation::from_json(eval_json.unwrap_or_default());
    let mut payload = preview_payload(&evaluation, &document.host_snapshot, config, session, marks);
    *hover_groups = Some(std::mem::take(&mut payload.hover_groups));
    let selection_json = preview_selection_json(config, &payload);
    let status_json = preview_eval::preview_window_status_json(
        session,
        run,
        preview_eval::preview_status_json(&evaluation, &document.host_snapshot),
        &preview_eval::PreviewStatusDebug { meshes_json: &payload.meshes_json, instances_json: &payload.instances_json },
        None,
    );
    let fit_json = preview_eval::preview_fit_json(&document.host_snapshot, payload.bounds);
    let sun = config.sun();
    let surface = semio_s_procedural_core_semantic_ui::accessible_scene_surface(
        SURFACE_ID,
        semio_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_ui_declaration::ui::World3dScene {
            status_json,
            fit_json: Some(fit_json),
            domain_id: Some(GENERATION3D_VIEW_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(GENERATION3D_VIEW_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(
                semio_ui_declaration::ui::world3d_camera_json(config.preview_camera.position, config.preview_camera.target, config.preview_camera.fov),
                payload.meshes_json,
                payload.instances_json,
                selection_json,
                &sun,
            )
        },
        labels.preview_canvas.as_str(),
        labels.preview_canvas_hint.as_str(),
        semio_ui_contract::Liveness::Polite,
    )?;
    let rows = preview_eval::preview_value_rows(&evaluation);
    let graph_ids: Vec<_> = marks.hovered.iter().cloned().collect();
    let hover = preview_eval::PreviewChannelHover::new(&document.host_snapshot, &graph_ids, None);
    let hovered_ports = hover.matching(rows.iter().map(|row| row.id.as_str()));
    semio_s_procedural_core_semantic_ui::scene_with_channel_values(surface, windows, "procedural3d-view", "generation3d-view-values", &semio_s_procedural_core_semantic_ui::ChannelValueLabels::new(is_de, labels.graph_input_port.as_str(), labels.graph_output_port.as_str()), &rows, &hovered_ports)
}
//#endregion 🔖️Render
