//! 🧱️ Generation3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use crate::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::generation3d::modes::{edit, generate};
use crate::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use crate::editor::generation3d::terminology::generation3d_labels;
use crate::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
// 🚧️ SDK note (ticket 26/08/16 contract §2.1/§2.4): `ArtifactEditor`/`Editor`/`Dialect` are curated at
// `semio_framework_plugin`'s crate root as of W0-F/W2-FIX — imported bare here, no `app::` prefix
// needed (unlike the earlier cad pilot, written before that gap closed). `app::InteractionView` is a
// separate, still-uncurated gap (unrelated to this ticket) — kept qualified.
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::ExampleSource;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d::compute::EngineHandles;

#[path = "🎯️selection/🦀️.rs"]
pub mod selection;
#[path = "🎮️commands/🧭️transforms/🦀️.rs"]
pub mod transform_commands;
#[path = "🎮️commands/🥽️edit-mesh-selection/🦀️.rs"]
pub mod edit_mesh_selection;
#[path = "🎮️commands/🔪️knife-mesh-selection/🦀️.rs"]
pub mod knife_mesh_selection;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_APP_ID: &str = "procedural3d-play";

/// 🪪️ The editor surface id every tool transaction's `tool` is scoped by (`<appId>#<verb>`, design §2).
pub const GENERATION3D_EDITOR_APP_ID: &str = "s.procedural.generation3d@1/*#editor";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🍱️panes/*`, `☑️options/*`) builds its `on_change`/item actions with.
pub fn generation3d_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GENERATION_3D_PLAY_APP_ID.into(), action: action.into(), args }
}

fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, kind).with_category(category)
}

/// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Generation3dPlayApp::Command` — the SOLE dispatch surface for generation3d's own behavior,
    /// covering EVERY declared action. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Generation3dCommand for Generation3dSnapshot, Generation3dMutation, Generation3dConfig, Generation3dConfigMutation, ctx = FlowEvalSession {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "nodeGraphEdit" as "graph-edit" => node_graph_edit::NodeGraphEdit,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "patchFlowWidgets" as "patch-flow-widgets" => patch_flow_widgets::PatchFlowWidgets,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "viewport" => node_graph_viewport::NodeGraphViewport,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setShowMode" as "show-mode" => set_show_mode::SetShowMode,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setCamera" as "camera" => set_camera::SetCamera,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => flow_tessellate_resolve::FlowTessellateResolve,
        "flowEvalRelease" as "flow-eval-release" => flow_eval_release::FlowEvalRelease,
        "flowTessellateCancelResolve" as "flow-tessellate-cancel-resolve" => flow_tessellate_cancel_resolve::FlowTessellateCancelResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
        "importDocumentRequest" as "import-document-request" => import_document_request::ImportDocumentRequest,
        "importDocument" as "import-document" => import_document::ImportDocument,
        "exportDocument" as "export-document" => export_document::ExportDocument,
        "cycleShowMode" as "cycle-show-mode" => cycle_show_mode::CycleShowMode,
        "cycleLodMode" as "cycle-lod-mode" => cycle_lod_mode::CycleLodMode,
        "selectNextNode" as "select-next-node" => select_next_node::SelectNextNode,
        "selectPreviousNode" as "select-previous-node" => select_previous_node::SelectPreviousNode,
        "selectUpstreamNode" as "select-upstream-node" => select_upstream_node::SelectUpstreamNode,
        "selectDownstreamNode" as "select-downstream-node" => select_downstream_node::SelectDownstreamNode,
        "activateSelection" as "activate-selection" => activate_selection::ActivateSelection,
        "editMeshSelection" as "edit-mesh-selection" => edit_mesh_selection::EditMeshSelection,
        "knifeMeshSelection" as "knife-mesh-selection" => knife_mesh_selection::KnifeMeshSelection,
        "setWidgetInput" as "set-widget-input" => set_widget_input::SetWidgetInput}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️InstanceOperationOwner
/// 🧠️ The ONE `FlowEvalSession` this app instance retains across turns — its neural cache, its
/// `eval_json`, its live preview meshes and, critically, its `pending_tessellate_by_hash` table.
/// Every entry point used to build a throwaway `FlowEvalSession::new()`, so an in-flight tessellate
/// handle noted while emitting the request was gone before its answer could arrive and
/// `resolve_preview_tessellate` always returned `false` — the 3d preview could never paint. (The
/// throwaway also violated `FlowEvalSession`'s own `Drop` contract, which rejects a live drop.)
/// Mirrors `FlowInstanceOperationOwner` in the flow artifact — the framework's reference owner.
pub struct Generation3dInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    /// ⏯️ The `previewEval` run's surface-owned half: attached preview roster and the live job's port.
    run_link: crate::preview_eval::PreviewEvalRunLink,
    /// 🛠️ Every window's open gumball gesture — ephemeral local tool state, never history; the previews fold it in.
    gumball: transform_commands::GumballGestures,
    /// 📐️ The instance's geometry engine: the typed inference cache, the retained base and the stepped run of `s.procedural.generation3d.geometry`.
    geometry: crate::host::geometry_service::GeometryHost,
    closing: bool,
}

impl Generation3dInstanceOperationOwner {
    pub fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), run_link: crate::preview_eval::PreviewEvalRunLink::default(), gumball: transform_commands::GumballGestures::default(), geometry: Default::default(), closing: false }
    }

    /// 📐️ The geometry host a run of the contextual `s.procedural.generation3d.geometry` service executes against.
    pub fn geometry(&self) -> &crate::host::geometry_service::GeometryHost {
        &self.geometry
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-eval-session-owner-missing"))
    }

    /// 🛠️ The retained session together with the open gumball gestures — what a gesture command folds into.
    fn with_session_and_gumball<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession, &mut transform_commands::GumballGestures) -> R) -> Result<R, Fault> {
        let Self { eval_session, gumball, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        Ok(body(session, gumball))
    }

    /// ⏰️ A fold that changed the session wakes the live `previewEval` run job afterwards.
    pub fn with_session_waking<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        let result = self.with_session(body);
        self.run_link.wake();
        result
    }

    /// 🩹️ Owes the attached previews exactly what this gesture's own emit says it owes them — a
    /// landed artifact mutation moved the document the evaluation reads, nothing else did — and puts
    /// the run start the debt needs on that same emit when no run is live to be woken instead.
    fn owe_attached_previews_for_mutations(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>) -> Result<bool, Fault> {
        let Self { eval_session, run_link, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        Ok(crate::preview_eval::owe_attached_previews_for_mutations(session, run_link, windows, servable, emit))
    }

    /// 🚦️ Owes the attached previews an evaluation and puts the run start that debt needs on the
    /// gesture's own emit — what a route that owes unconditionally (a generation command, an example
    /// switch, a contributions install, a closing import chunk) hands back.
    fn owe_attached_previews_carrying(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>) -> Result<(), Fault> {
        let Self { eval_session, run_link, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        crate::preview_eval::owe_attached_previews_carrying(session, run_link, windows, servable, emit);
        Ok(())
    }
}

impl crate::preview_eval::PreviewEvalRunOwner for Generation3dInstanceOperationOwner {
    fn preview_eval_parts(&mut self) -> Option<(&mut FlowEvalSession, &mut crate::preview_eval::PreviewEvalRunLink)> {
        let Self { eval_session, run_link, closing, .. } = self;
        eval_session.as_mut().filter(|_| !*closing).map(|session| (session, run_link))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation3dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spent the runtime's
    /// zero-progress credit every idle turn (26/09/09/PROCEDURAL-3D-END-TO-END, close-ladder lane).
    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        if !self.closing {
            return Ok(semio_framework_plugin::PluginCloseStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(session) = self.eval_session.as_mut() else { return Ok(semio_framework_plugin::PluginCloseStep::Complete) };
        let step = session.close_step(maximum_items, maximum_bytes);
        if session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Generation3d evaluation session awaits its exact close grant" },
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_plugin::PluginCloseStep::Complete,
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if let Some(session) = self.eval_session.as_mut() {
            session.begin_close();
        }
        self.maintenance_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none()
    }
}

/// 🧹️ The ownerless fallback: a scratch session retired through the explicit `begin_close` +
/// granted `close_step` loop `FlowEvalSession`'s `Drop` contract demands. Reachable only from the
/// marks-free `render` and the non-retained `handle`, neither of which is handed the app-instance
/// operation owner by the framework — every live path uses the RETAINED session instead, so nothing
/// that must survive a turn is ever computed here.
fn with_scratch_session<R>(body: impl FnOnce(&mut FlowEvalSession) -> R) -> R {
    let mut session = FlowEvalSession::new();
    let result = body(&mut session);
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
    result
}
//#endregion 🔖️InstanceOperationOwner

//#region 🔖️Generation3dPlayApp
/// 🧪️ Unit struct: the retained evaluation session lives in [`Generation3dInstanceOperationOwner`]
/// and every other former runtime field lives in [`Generation3dConfig`], written through
/// [`Generation3dConfigMutation`]s.
#[derive(Default)]
pub struct Generation3dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s JSON args' nested `{viewport: {x, y, zoom}}`
/// object (`nodeGraphViewportActionArgs`, `🧱️elements/🕸️NodeGraph/🟦️.tsx`). An ABSENT `viewport` decodes to
/// the identity camera, because `ActionDefinition::new("nodeGraphViewport", …)` declares no args and every
/// declared action must bridge from its own id under the shell's staged args alone — the same contract
/// `parse_preview_camera_json` keeps for `setCamera`. A PRESENT but malformed one still faults: a camera the
/// graph cannot express must never be silently replaced by one it can.
fn parse_flow_viewport(args: &semio_framework_value::DslValue) -> Result<semio_framework_os_kernel::Viewport2d, Fault> {
    let Some(value) = args.get("viewport").cloned() else { return Ok(semio_framework_os_kernel::Viewport2d::default()) };
    semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))
}

/// 🎥️ Parses the 3D preview camera out of `command_from_action`'s JSON args; falls back to the default
/// camera on any malformed/missing `camera` object.
fn parse_preview_camera_json(args: &semio_framework_value::DslValue) -> crate::editor::generation3d::config::Generation3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = <crate::editor::generation3d::config::Generation3dPreviewCamera as semio_framework_value::FromValue>::from_value(camera.clone()) {
            return parsed;
        }
    }
    crate::editor::generation3d::config::Generation3dPreviewCamera::default()
}

/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`), read from the SAME
/// `dag_host_snapshot_to_workflow` projection the node-graph window paints — so an interaction target and a
/// graph pick can never drift apart.
pub fn generation3d_port_ids_by_node(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = crate::standards::v1::subsets::any::schema::with_host(host_snapshot, |host| crate::standards::v1::subsets::any::schema::dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}

fn generation3d_addressed_preview_eval(window: &semio_framework_plugin::WindowTransientSnapshot, eval_text: Option<String>) -> Result<semio_framework_plugin::WindowTransientMutation, Fault> {
    if window.window_kind_id() == generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW {
        generate_preview::transient::addressed(window, eval_text)
    } else {
        edit_preview::transient::addressed(window, eval_text)
    }
}

/// 🧱️ Every window body of the generation3d editor, rendered against one already-resolved set of
/// `graph` marks. Shared by `render` (marks-free) and `render_with_request_context` (live marks) so
/// there is exactly one body-key match in the app.
pub fn generation3d_render_body(
    body_key: &str,
    document: &Generation3dSnapshot,
    config: &Generation3dConfig,
    preview_eval_text: Option<&str>,
    view_state: &semio_framework_plugin::ViewModel,
    marks: &PreviewInteractionMarks,
    session: &FlowEvalSession,
    run: Option<&semio_framework_plugin::ToolRunView>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = generation3d_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().unwrap_or("move");
    let selected_generation_id = config.selected_generation_id.as_deref();
    if semio_framework_job::runtime_diagnostics_enabled() {
        eprintln!("[TRACE] gen3d render body={body_key} generations={} selected={selected_generation_id:?}", document.generation.as_state().generations.len());
    }
    let node = match body_key {
        flow_window::GENERATION_3D_PLAY_BODY_MAIN => flow_window::render(document, config, session, marks, labels),
        edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, preview_eval_text, session, run, active_utility, marks, labels),
        generations::GENERATION_3D_PLAY_BODY_GENERATIONS => generations::render(
            &document.generation,
            selected_generation_id,
            view_state.locale,
            view_state.terminology,
            &semio_framework_plugin::TreeWindows::for_body(view_state, generations::GENERATION_3D_PLAY_BODY_GENERATIONS),
        ),
        form::GENERATION_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.host_snapshot, &document.generation, selected_generation_id, labels),
        generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.host_snapshot, &document.generation, selected_generation_id, preview_eval_text, config, labels, active_utility, marks, session, run),
        artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT => artifact_panel::render(document, config, session, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT)),
        catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE)),
        inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION => {
            let evaluation = preview_eval_text.unwrap_or_else(|| session.eval_json());
            let selected = marks.graph_selection_ids();
            let export_widget = selected.first().filter(|id| document.host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::OutputExport { id: target, .. } if target == *id)));
            let exported = match export_widget { Some(widget) => export_meshes_from_session(document, config, session, Some(widget.as_str())), None => Ok(Vec::new()) };
            let export_ready = exported.is_ok();
            let meshes = exported.unwrap_or_default();
            inspection_panel::render(&document.host_snapshot, &marks.graph_selection_ids(), labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION), view_state.locale, view_state.terminology, &meshes, evaluation, session.status_json(), export_ready)
        },
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🧵️RetainedCommands
/// 🧾️ Every gen3d GESTURE tool id, in `Generation3dCommand` declaration order. Together with
/// [`GENERATION3D_FLOW_EVAL_TOOL_IDS`] and [`GENERATION3D_CONTRIBUTIONS_TOOL_IDS`] this is a
/// bijection with `Generation3dCommand`'s 34 rows (asserted by
/// `retained_route_dispositions_are_exact_and_exhaustive` below) and with
/// `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "nodeGraphEdit",
    "deleteSelection",
    "removeWidget",
    "addWidget",
    "patchFlowWidgets",
    "setWidgetInput",
    "reorganize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "addGeneration",
    "removeGeneration",
    "renameGeneration",
    "updateGenerationValues",
    "nodeGraphViewport",
    "setLodMode",
    "setShowMode",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "selectGeneration",
    "cycleShowMode",
    "cycleLodMode",
    // 🧭️ The node graph's keyboard traversal (`🎮️commands/🧭️navigate-graph`). Retained like every
    // other gesture, but publishing ONLY the framework's selection lane.
    "selectNextNode",
    "selectPreviousNode",
    "selectUpstreamNode",
    "selectDownstreamNode",
    "activateSelection",
    "editMeshSelection",
    "knifeMeshSelection",
];
/// ⏱️ The preview run's hop tool ids — a HOST route, never a user gesture: nobody clicks a tick,
/// and the mesh body an extension answer carries is nothing like a gesture's wire payload. Split
/// out of [`GENERATION3D_RETAINED_TOOL_IDS`] for exactly the reason
/// [`GENERATION3D_CONTRIBUTIONS_TOOL_IDS`] was, and in exactly the shape the sibling viewer surface
/// already declares (`GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS`): one factory registers ONE execution
/// contract for all its keys, so a chain that must carry 48 KiB of `pack` body cannot live under
/// the 8 KiB gesture quota without widening 24 unrelated routes with it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
const GENERATION3D_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"];
/// 🪟️ The chain routes that are ADDRESSED AT A PREVIEW WINDOW and therefore share one publication
/// layer ([`Generation3dFlowEvalWindowWork`]): the dispatched hop and the two extension folds that
/// continue the chain inline. `flowEvalRelease` and `flowTessellateCancelResolve` stay off it on
/// purpose — a release must reach the geometry extension even once the window it names is gone, and
/// `retained_window_transient_target` refuses a window that has left the roster.
const GENERATION3D_FLOW_EVAL_WINDOW_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve"];
/// 📄️ The user's OWN import/export route: its own tool ids, its own factory and its own execution
/// contract, for the same reason the preview chain has one — what it carries is nothing like a
/// gesture. `importDocument` hands over one WHOLE picked file — reassembled by the framework
/// (`semio_framework::kernel::ImportStaging`) and bounded by one Artifact-lane edit. Widening the
/// 8 KiB gesture quota to admit it would widen 23 unrelated interactive routes with it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, io-surface lane).
const GENERATION3D_DOCUMENT_IO_TOOL_IDS: &[&str] = &["importDocumentRequest", "importDocument", "exportDocument"];
const GENERATION3D_DOCUMENT_IO_PAYLOAD_SCHEMA: &str = "generation.3d.document-io-command.v1";
/// 🎒️ Real bound for one document-IO hop's wire payload: one whole import
/// ([`import_document::GENERATION3D_IMPORT_TOTAL_BYTES`]) plus the addressed envelope and the command
/// id — the whole public-invocation body the host may send, pinned to
/// `PUBLIC_INVOCATION_BODY_BYTES` by `document_io_route_declares_a_reachable_wire_ceiling` rather
/// than guessed.
const GENERATION3D_DOCUMENT_IO_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;
const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";
const GENERATION3D_FLOW_EVAL_PAYLOAD_SCHEMA: &str = "generation.3d.flow-eval-command.v1";
const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;
/// 🎒️ Real bound for one preview-chain hop's wire payload: a `flowTessellateResolve` carries one
/// whole `tessellate` envelope, i.e. one [`MESH_PACK_CHUNK_BASE64_CHARS`] mesh-body chunk plus its
/// accounting header — pinned to `brep_geometry::tessellate_envelope_maximum_bytes()` by
/// `tessellate_envelope_fits_the_declared_wire_bound`, so the transfer unit and this bound are one
/// fact. 64 KiB is the guest-contiguous page the answer already crosses in, never a rubber stamp.
const GENERATION3D_FLOW_EVAL_RAW_BYTES: usize = 65_536;
const GENERATION3D_PREVIEW_TOOL_IDS: &[&str] = &["setActiveExample", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];
/// 🧮️ The ONE declared quantity of every generation3d retained route: at most 32 point-invertible
/// durable items per gesture. Everything the runtime compares is read off it and nothing is ever
/// spelled again — the preflight ceiling ([`GENERATION3D_RETAINED_WORK_ITEMS`], handed to
/// `ArtifactRetainedCommandPayload::try_new`), the extent every `ArtifactCommandWork` answers, the
/// one-item store footprint both durable preflights declare, and the bounded proof's work units.
/// They used to be three literals in two units — a two-row footprint, a one-ITEM extent and a
/// 32-ITEM ceiling — so nothing could compare them and only the store ever measured rows
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_RETAINED_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);
const GENERATION3D_RETAINED_WORK_ITEMS: usize = GENERATION3D_RETAINED_CAPACITY.work_items();
/// 🎒️ Real bound for one Artifact-lane edit: the 8 built-in example DSLs top out around 1.6 KB of text
/// (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`), and `setActiveExample`'s full-fixture replacement is the
/// single largest Artifact mutation any of the 27 tools ever emits — 64 KiB stays a real ceiling, not a
/// rubber stamp, for every example plus ordinary interactive graph edits.
pub(crate) const GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: a full snapshot containing the flow/preview cameras,
/// selected generation, and sun JSON remains bounded independently of computed preview output.
const GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 262_144;

/// 🧾️ The ONE execution contract all 29 routes publish — the factory's `execution_contract()` and
/// every row of `bounded_first_step_tool_proofs!` alike, so the proof catalogue can never drift from
/// the factory it proves. Its work-unit budget is the route capacity itself; `max_decoded_items`
/// stays its own quantity (decoded JSON items on the wire, not staged edit rows).
pub fn generation3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_RETAINED_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧾️ Decoded JSON items one retained command's args may carry — a WIRE bound, deliberately not the
/// work capacity.
const GENERATION3D_RETAINED_DECODED_ITEMS: usize = 32;

/// 🪟️ Every attached preview window, in roster order — the windows a `flowEvalTick` chain can be
/// addressed to. Each holds its OWN retained evaluation publication
/// (`Generation3dPreviewWindowTransientOwner`), so each gets its own armed chain; an empty answer
/// means no surface is mounted yet and nothing may be armed at all.
fn generation3d_preview_kind(kind: &str) -> Option<&'static str> {
    if kind == edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW {
        Some(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW)
    } else if kind == generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW {
        Some(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW)
    } else {
        None
    }
}

fn generation3d_preview_windows(view: Option<&semio_framework_plugin::ViewModel>) -> Vec<(&str, &'static str)> {
    view.map(|view| {
        view.window_instances
            .iter()
            .filter_map(|window| generation3d_preview_kind(&window.window_kind_id).map(|kind| (window.id.as_str(), kind)))
            .collect()
    })
    .unwrap_or_default()
}

fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GENERATION3D_RETAINED_CAPACITY.rows_for_items(1)
}

struct Generation3dPreviewCommandWork {
    tool_id: &'static str,
    /// 🔒️ The app instance's retained evaluation session, reached ONLY to arm the preview chain
    /// through its per-window latch — a gesture that changed what the evaluation would produce owes
    /// every attached preview a tick, and owes it at most once.
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    emit: Option<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>>,
    host: Option<FlowHost>,
    /// 🧹️ The host's explicit retirement ladder, armed by `close_step` when the evaluation host is
    /// handed over. A bare `Option<FlowHost>::take()`-and-drop panics on the fixture's
    /// `OrderedMap<WidgetLayout>` root, so the host is drained under the caller's grant instead.
    host_retirement: Option<semio_framework_os_flow::FlowHostRetirement>,
    session: Option<FlowEvalSession>,
    started: bool,
    complete: bool,
    closing: bool,
}

impl Generation3dPreviewCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, emit: None, host: None, host_retirement: None, session: None, started: false, complete: false, closing: false }
    }

    /// 🔒️ Owes every attached preview window an evaluation on the RETAINED session's latch, so two
    /// gestures in a row still leave exactly one debt per window for the run to pay.
    fn owe_attached_previews(&self, view: Option<&semio_framework_plugin::ViewModel>, servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>) -> Result<(), Fault> {
        let windows = generation3d_preview_windows(view);
        self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.owe_attached_previews_carrying(&windows, servable, emit))
    }

    fn complete(&mut self) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        self.complete = true;
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-preview-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dPreviewCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        if matches!(command, Generation3dCommand::SetActiveExample(_)) {
            return GENERATION3D_RETAINED_CAPACITY.rows_for_items(1);
        }
        if !matches!(command, Generation3dCommand::AddGeneration(_) | Generation3dCommand::RemoveGeneration(_) | Generation3dCommand::RenameGeneration(_) | Generation3dCommand::UpdateGenerationValues(_) | Generation3dCommand::SelectGeneration(_)) {
            return None;
        }
        GENERATION3D_RETAINED_CAPACITY.rows_for_items(snapshot.host_snapshot.widgets.len().checked_add(2)?)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.closing || self.complete {
            return Err(Fault::from("generation3d-preview-work-is-terminal"));
        }
        if !self.started {
            self.started = true;
            let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
            if let Generation3dCommand::SetActiveExample(payload) = input.command {
                let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
                let cfg = ConfigView { snapshot: input.config, window: None };
                let mut emit = set_active_example::emit(payload, &doc, &cfg)?;
                // 🔁️ Only a switch that actually MOVED the graph owes the previews an evaluation. An id
                // the dialect never published, and a re-pick of the example already open, both emit
                // zero artifact mutations (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
                if !emit.artifact_mutations.is_empty() {
                    self.owe_attached_previews(input.context.and_then(|context| context.view_state.as_ref()), servable, &mut emit)?;
                }
                self.emit = Some(emit);
                return self.complete();
            }
            let result = crate::editor::generation3d::commands::generation::generation_command_result_for(input.command, input.snapshot, input.config).ok_or_else(|| Fault::from("generation3d-preview-command-unowned"))?;
            let mut emit = result.emit;
            self.owe_attached_previews(input.context.and_then(|context| context.view_state.as_ref()), servable, &mut emit)?;
            if let Some(fixture) = result.preview_fixture {
                fixture.retire_cold();
            }
            self.emit = Some(emit);
            return self.complete();
        }
        Err(Fault::from("generation3d-preview-work-is-terminal"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        if let Some(session) = self.session.as_mut() {
            match session.close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete => {
                    if !session.terminal_is_empty() {
                        return InteractiveJobCloseStep::Blocked;
                    }
                    self.session.take();
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if let Some(host) = self.host.take() {
            self.host_retirement = Some(semio_framework_os_flow::FlowHostRetirement::new(host));
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(retirement) = self.host_retirement.as_mut() {
            return match retirement.close_page(maximum_items, maximum_bytes) {
                Ok(false) => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                Ok(true) => {
                    self.host_retirement = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => InteractiveJobCloseStep::Blocked,
            };
        }
        if self.emit.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.emit.is_none() && self.host.is_none() && self.host_retirement.is_none() && self.session.is_none()
    }
}

/// ⏱️ The ONE publication layer of the preview chain: every route that may move the addressed
/// window's retained evaluation runs here and nowhere else — the dispatched `flowEvalTick` hop and
/// both window-addressed extension folds, `flowEvalResolve` and `flowTessellateResolve`.
///
/// 🔁️ The folds joined it because they now CONTINUE the chain: the last answer of a wave runs the
/// next wave inline (`flow_eval_tick::continue_inline`) rather than paying a
/// `flowEvalResolve` → `flowEvalTick` pair per dependency level. A continuation that advanced the
/// chain on a route with no window transient would publish no intermediate geometry and no advanced
/// status pill — and intermediate publication is exactly what a user cancels out of — so the fold
/// moved to the publisher rather than the publication moving to the fold
/// (`📓️flow-tick-coalescing-2026-09-14.md` §7 item 4).
struct Generation3dFlowEvalWindowWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    complete: bool,
    closing: bool,
}

impl Generation3dFlowEvalWindowWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self { Self { tool_id, instance_owner, complete: false, closing: false } }
}

/// 🪟️ The preview window one chain route addresses, or `None` for a route that addresses none.
fn generation3d_flow_eval_window_address(command: &Generation3dCommand) -> Option<(&str, &str)> {
    match command {
        Generation3dCommand::FlowEvalTick(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        Generation3dCommand::FlowEvalResolve(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        Generation3dCommand::FlowTessellateResolve(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        _ => None,
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dFlowEvalWindowWork {
    fn tool_id(&self) -> &'static str { self.tool_id }

    /// 🪟️ The route's window is the one its PAYLOAD names, never the one the shell happened to be
    /// focused on when it redispatched the self-armed effect — and an extension answer names it for
    /// the same reason a tick does, because `reactor::extension_response_args` echoes the request's
    /// own window fields back onto the response action. `retained_window_transient_target` captured
    /// that exact window's transient authority (validating it against the trusted ViewModel roster
    /// first), so the two only have to agree here — the same shape `▶️run-query/🧵️job` checks for
    /// its `results_window_id` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn extent(
        &self,
        command: &Generation3dCommand,
        _snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        let (window_id, window_kind_id) = generation3d_flow_eval_window_address(command)?;
        let context = context?;
        let window = context.window_transient.as_ref()?;
        (!window_id.is_empty()
            && window.window_id() == window_id
            && window_kind_id == window.window_kind_id()
            && generation3d_preview_kind(window.window_kind_id()).is_some()
            && (window.get::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>().is_some()
                || window.get::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>().is_some()))
        .then(|| GENERATION3D_RETAINED_CAPACITY.rows_for_items(1))
        .flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.complete || self.closing { return Err(Fault::from("generation3d-flow-eval-window-work-terminal")); }
        let turn_started_us = semio_framework_job::default_now_us();
        let (payload_window_id, payload_window_kind_id) = generation3d_flow_eval_window_address(input.command).ok_or_else(|| Fault::from("generation3d-flow-eval-window-command-mismatch"))?;
        let context = input.context.ok_or_else(|| Fault::from("generation3d-flow-eval-window-context-required"))?;
        let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("generation3d-flow-eval-window-transient-required"))?;
        if window.window_id() != payload_window_id || payload_window_kind_id != window.window_kind_id() || generation3d_preview_kind(window.window_kind_id()).is_none() {
            return Err(Fault::from("generation3d-flow-eval-window-owner-mismatch"));
        }
        let retained_eval = window
            .get::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()
            .map(|state| state.preview_eval_text.as_deref())
            .or_else(|| window.get::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>().map(|state| state.preview_eval_text.as_deref()))
            .ok_or_else(|| Fault::from("generation3d-flow-eval-window-owner-required"))?;
        let cfg = ConfigView { snapshot: input.config, window: context.window_config.as_ref() };
        let (emit, publication) = self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            let overlay = generation3d_provisional_snapshot(input.snapshot, context.provisional(), &owner.gumball);
            let evaluated = {
                let doc = ArtifactView::with_operation(overlay.as_ref().unwrap_or(input.snapshot), input.history, input.operation.clone());
                owner.with_session_waking(|session| match input.command {
                    Generation3dCommand::FlowEvalTick(_) => flow_eval_tick::evaluate(window.window_id(), window.window_kind_id(), &doc, &cfg, session, retained_eval, None),
                    Generation3dCommand::FlowEvalResolve(payload) => flow_eval_resolve::resolve(payload, &doc, &cfg, session, retained_eval, turn_started_us),
                    Generation3dCommand::FlowTessellateResolve(payload) => flow_tessellate_resolve::resolve(payload, &doc, &cfg, session, retained_eval, turn_started_us),
                    _ => Err(Fault::from("generation3d-flow-eval-window-command-mismatch")),
                })
            };
            if let Some(overlay) = overlay {
                overlay.retire_cold();
            }
            evaluated?
        })?;
        self.complete = true;
        let window_transient = match publication {
            semio_framework_os_flow::FlowEvalPublication::Retained => Vec::new(),
            semio_framework_os_flow::FlowEvalPublication::Changed(eval_text) => vec![generation3d_addressed_preview_eval(window, eval_text)?],
        };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient },
        })
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool { self.closing }
}

/// 🪞️ The document a derived view evaluates while a tool transaction is open (design §13, F-7): the committed snapshot
/// with every open press's and typing run's provisional leaves (the framework's, as of admission) and every open gumball
/// gesture (this instance's) folded in — `None` while nothing is open, so the committed snapshot is read as is. The
/// overlay is retired cold by the caller, never dropped.
fn generation3d_provisional_snapshot(committed: &Generation3dSnapshot, provisional: &[semio_framework_value::DslValue], gumball: &transform_commands::GumballGestures) -> Option<Generation3dSnapshot> {
    if provisional.is_empty() && gumball.is_empty() {
        return None;
    }
    let mut overlay = committed.clone();
    for value in provisional {
        if let Ok(leaf) = <Generation3dMutation as semio_framework_value::FromValue>::from_value(value.clone()) {
            generation3d_fold_provisional(&mut overlay, leaf);
        }
    }
    let gestures = gumball.provisional(&overlay.host_snapshot);
    for row in gestures.rows {
        generation3d_fold_provisional(&mut overlay, row);
    }
    Some(overlay)
}

/// 🫥️ Folds one provisional leaf into a preview overlay and retires it. A leaf the overlay refuses paints nothing — a
/// preview shows only what applies — and its refusal is never lost: the commit that lands the same leaf on the committed
/// document reports it as the history row's outcome.
fn generation3d_fold_provisional(overlay: &mut Generation3dSnapshot, leaf: Generation3dMutation) {
    let _refused_paints_nothing = crate::central_apply::apply_generation3d_mutation(overlay, &leaf);
    leaf.retire_cold();
}

/// 🪩️ The document and marks a preview body paints while a gumball gesture is open (design §13, F-7): the committed
/// snapshot with every open gesture folded in — the overlay its flow evaluation reads — and the selection carried onto
/// the operators the gestures splice, so the first grab of a shape keeps painting and selecting the instance the live
/// evaluation moves. `None` while no gesture is open; the caller retires the overlay cold.
pub fn generation3d_gumball_preview(committed: &Generation3dSnapshot, marks: &PreviewInteractionMarks, gumball: &transform_commands::GumballGestures) -> Option<(Generation3dSnapshot, PreviewInteractionMarks)> {
    if gumball.is_empty() {
        return None;
    }
    let gestures = gumball.provisional(&committed.host_snapshot);
    let mut overlay = committed.clone();
    for row in gestures.rows {
        generation3d_fold_provisional(&mut overlay, row);
    }
    let marks = gestures.selections.iter().fold(marks.clone(), PreviewInteractionMarks::following);
    Some((overlay, marks))
}

/// 🕹️ `nodeGraphEdit`/`deleteSelection`/`{translate,rotate,scale}Selection` read real `graph` selection
/// directly off `protocol::InteractionState` (the raw, crate-public half of what `app::InteractionView`
/// wraps) — plugin code cannot construct an `InteractionView` itself (`state`/`hover`/`peers` are
/// `pub(crate)` to `semio_framework_plugin`), so this is the only way a retained-command-job reducer can
/// preserve the same real-selection behavior `Generation3dPlayApp::handle` gives those five commands above.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn generation3d_retained_reduce(
    command: &Generation3dCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    operation: &AppOperationContext,
    session: &mut FlowEvalSession,
    gumball: &mut transform_commands::GumballGestures,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()) && !GENERATION3D_FLOW_EVAL_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation3d-command-retained-route-rejected"));
    }
    if let Some(gesture) = generation3d_gumball_gesture(command) {
        let view = context.and_then(|context| context.view_state.as_ref());
        let window = gesture.window.or_else(|| view.and_then(|view| view.window_id.as_deref())).unwrap_or_default();
        let phase = semio_framework_tool_machine::GesturePhase::parse(gesture.phase, gesture.reason).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), "the gumball gesture phase or abort reason is unknown"))?;
        if let semio_framework_tool_machine::GesturePhase::Abort(reason) = phase {
            gumball.abort(window, reason);
            return Ok(Emit::default());
        }
        let graph: &[String] = interaction.selection.get("graph").map_or(&[], |selection| selection.ids.as_slice());
        let components = selection::edits_components(view, interaction.active_granularity.get(selection::DOMAIN).map(String::as_str)).then(|| interaction.selection.get(selection::DOMAIN).map_or(&[][..], |selection| selection.ids.as_slice()));
        let ids = transform_commands::gumball_ids(&snapshot.host_snapshot, gesture.ids, components, graph)?;
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        if ids.iter().any(|id| selection::ComponentTarget::parse(id).is_some()) {
            selection::validate_cached_components(&snapshot.host_snapshot, session, &ids, Some(&operation.canonical_base_revision)).map_err(Fault::from)?;
        }
        return gumball.dispatch(
            transform_commands::GumballDispatch { verb: gesture.verb, window, ids, motion: gesture.motion, phase, authoring_seed: &operation.authoring_seed, base_revision: operation.canonical_base_revision },
            &snapshot.host_snapshot,
        );
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let selected = || interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
    let component_ids: &[String] = interaction.selection.get(selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice());
    let graph_ids: &[String] = interaction.selection.get("graph").map_or(&[], |selection| selection.ids.as_slice());
    if let Some(ids) = generation3d_component_targets(command, context.and_then(|context| context.view_state.as_ref()), interaction.active_granularity.get(selection::DOMAIN).map(String::as_str), component_ids, graph_ids).filter(|ids| !ids.is_empty()) {
        selection::validate_cached_components(&snapshot.host_snapshot, session, ids, Some(&operation.canonical_base_revision)).map_err(Fault::from)?;
    }
    match command {
        Generation3dCommand::EditMeshSelection(payload) => edit_mesh_selection::apply_selected("editMeshSelection", payload, &doc, interaction.selection.get(selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
        Generation3dCommand::KnifeMeshSelection(payload) => knife_mesh_selection::apply_selected(payload, &doc, interaction.selection.get(selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
        Generation3dCommand::DeleteSelection(_) if selection::edits_components(context.and_then(|context| context.view_state.as_ref()), interaction.active_granularity.get(selection::DOMAIN).map(String::as_str)) => edit_mesh_selection::delete_selected(&doc, interaction.selection.get(selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
        Generation3dCommand::DeleteSelection(_payload) => Ok(delete_selection::apply_selected(&doc, &selected())),
        // 🧭️ Keyboard traversal reads the SAME `graph` selection the pointer writes and hands the next
        // one back through `Emit.interaction_writes`, so an arrow key and a click are the same gesture
        // to everything downstream (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        Generation3dCommand::SelectNextNode(_payload) => Ok(select_next_node::apply_selected(&doc, &selected())),
        Generation3dCommand::SelectPreviousNode(_payload) => Ok(select_previous_node::apply_selected(&doc, &selected())),
        Generation3dCommand::SelectUpstreamNode(_payload) => Ok(select_upstream_node::apply_selected(&doc, &selected())),
        Generation3dCommand::SelectDownstreamNode(_payload) => Ok(select_downstream_node::apply_selected(&doc, &selected())),
        Generation3dCommand::ActivateSelection(_payload) => Ok(activate_selection::apply_ports(&doc, &generation3d_port_ids_by_node(&doc.snapshot.host_snapshot), &selected())),
        _ => command.dispatch(&doc, &cfg, session),
    }
}

/// 🛠️ One gumball verb's dispatch fields, borrowed off its command: the verb, the explicit ids, the tick's motion, its
/// phase and abort reason, and the window it names (a host event's window).
struct Generation3dGumballGesture<'a> {
    verb: &'static str,
    ids: &'a [String],
    motion: transform_commands::GumballMotion,
    phase: Option<&'a str>,
    reason: Option<&'a str>,
    window: Option<&'a str>,
}

/// 🛠️ The gumball dispatch a command is, or `None` for every other verb.
fn generation3d_gumball_gesture(command: &Generation3dCommand) -> Option<Generation3dGumballGesture<'_>> {
    Some(match command {
        Generation3dCommand::TranslateSelection(payload) => Generation3dGumballGesture { verb: "translateSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        Generation3dCommand::RotateSelection(payload) => Generation3dGumballGesture { verb: "rotateSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        Generation3dCommand::ScaleSelection(payload) => Generation3dGumballGesture { verb: "scaleSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        _ => return None,
    })
}

/// 🛠️ Whether a command is a gumball verb.
fn generation3d_gumball_command(command: &Generation3dCommand) -> bool {
    generation3d_gumball_gesture(command).is_some()
}

fn generation3d_component_targets<'a>(command: &'a Generation3dCommand, view: Option<&semio_framework_plugin::ViewModel>, granularity: Option<&str>, geometry: &'a [String], graph: &'a [String]) -> Option<&'a [String]> {
    let components = selection::edits_components(view, granularity);
    if matches!(command, Generation3dCommand::EditMeshSelection(_) | Generation3dCommand::KnifeMeshSelection(_)) || components && matches!(command, Generation3dCommand::DeleteSelection(_)) { return Some(geometry); }
    let explicit = match command {
        Generation3dCommand::TranslateSelection(payload) => &payload.node_ids,
        Generation3dCommand::RotateSelection(payload) => &payload.node_ids,
        Generation3dCommand::ScaleSelection(payload) => &payload.node_ids,
        _ => return None,
    };
    if components { return Some(geometry); }
    let ids = if explicit.is_empty() { graph } else { explicit.as_slice() };
    ids.iter().any(|id| selection::ComponentTarget::parse(id).is_some()).then_some(ids)
}

/// 🧵️ The retained-session twin of `BoundedArtifactCommandWork`: identical one-shot reduce, except
/// the reducer runs against the app instance's RETAINED [`Generation3dInstanceOperationOwner`]
/// session instead of a session born and destroyed inside the same dispatch. That is what makes
/// `flowEvalResolve`'s `seed_node_cache` and `flowTessellateResolve`'s `resolve_preview_tessellate`
/// land on the same cache the next render reads.
struct Generation3dSessionCommandWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl Generation3dSessionCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dSessionCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation3d-session-command-work-repeated"));
        }
        self.consumed = true;
        // 🩹️ Every gesture on this route that MOVED THE DOCUMENT owes the attached previews a fresh
        // evaluation — `patchFlowWidgets`, `addWidget`, `removeWidget`, `nodeGraphEdit`,
        // `deleteSelection`, `reorganize` and every transform. The rule is read off the emit, never off
        // a roster of tool ids: the roster carried only `setActiveExample` and the generation commands,
        // so an inspector slider moved `height` 6 → 7 and the preview kept painting the old geometry
        // (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
        let windows = generation3d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
        let emit = self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            let gestures = owner.gumball.revision();
            let mut emit = owner.with_session_and_gumball(|session, gumball| generation3d_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation, session, gumball))??;
            if owner.gumball.revision() != gestures {
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit)?;
            } else {
                owner.owe_attached_previews_for_mutations(&windows, servable, &mut emit)?;
            }
            Ok(emit)
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

struct Generation3dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setWidgetInput", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "cycleShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "cycleLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
        // 🧭️ Keyboard traversal publishes the framework's selection lane and NOTHING else: a step is
        // not an edit, so it authors no document op and takes no config row — which is also why it
        // never enters undo (`HistoryLane::Interaction`).
        ArtifactToolPublicationContract { tool_id: "selectNextNode", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "selectPreviousNode", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "selectUpstreamNode", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "selectDownstreamNode", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "activateSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "editMeshSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "knifeMeshSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
    ];
}

//#region ⏱️FlowEvalRoute
/// ⏱️ The preview chain's OWN route: its own tool ids, its own factory and its own execution
/// contract, because what it carries is nothing like a gesture. `flowTessellateResolve` folds one
/// `tessellate` envelope — up to one whole [`MESH_PACK_CHUNK_BASE64_CHARS`] mesh-body chunk — and
/// under the 8 KiB gesture quota that envelope had to be cut into 4 KiB slices, one per
/// `flowEvalTick` round trip. Five of the eight bundled examples therefore never painted: at
/// seconds per round trip a ten-chunk body is minutes of streaming with `phase: idle` on the
/// surface the whole time (`📓️preview-mesh-delivery-2026-09-12.md`).
pub fn generation3d_flow_eval_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_FLOW_EVAL_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

struct Generation3dFlowEvalJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dFlowEvalJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_FLOW_EVAL_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dFlowEvalJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_FLOW_EVAL_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_flow_eval_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_FLOW_EVAL_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d flow-eval command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dFlowEvalJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_FLOW_EVAL_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        // ⏱️ The tick's ONLY store lane is the addressed preview window's OWN transient
        // (`Generation3dFlowEvalWindowWork::step`'s `CompleteWithEphemeral`); its extension
        // invocations are not store lanes at all and need no declaration.
        // `Config` was doubly wrong — it grants a lane the tick never writes and withholds the one it
        // does, so every dispatched tick was refused with `typed-operation emitted a store lane absent
        // from its exact factory publication contract`. Pairing `HostOnly` with it was equally wrong:
        // `HostOnly` MEANS "no store lane" and `ArtifactToolFactoryRegistry::register` rejects it
        // alongside any other lane with `interactive-job.publication-contract`
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        // 🔁️ Both window-addressed FOLDS publish the same lane the tick does, because they now run
        // the wave the settle unblocked inline and owe the addressed window the geometry and the
        // status it advanced to. A fold that declines the continuation publishes nothing — a lane is
        // what a route MAY write, never what it must (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalRelease", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateCancelResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

struct Generation3dFlowEvalJobFactoryProofs;

impl Generation3dFlowEvalJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dFlowEvalJobFactory",
        factory_type: Generation3dFlowEvalJobFactory,
        contract: generation3d_flow_eval_contract(),
        tools: ["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"]
    }
}
//#endregion ⏱️FlowEvalRoute

//#region 📄️DocumentIoRoute
const GENERATION3D_DOCUMENT_SOURCE_ALLOCATION_BYTES: usize = 1_048_576;
const GENERATION3D_DOCUMENT_OWNERSHIP_SOURCE_MULTIPLES: usize = 32;
const GENERATION3D_DOCUMENT_SCAFFOLD_BYTES: usize = 16_384;
const GENERATION3D_DOCUMENT_DOWNLOAD_BYTES: usize = 33_554_432;

/// 📄️ The user's import/export route. Its three tools are the whole UI surface of this artifact's
/// nine `🚪️io` leaves, which ticket 26/09/09/PROCEDURAL-3D-END-TO-END found round-trip tested and
/// reachable by nothing at all (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
pub fn generation3d_document_io_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_DOCUMENT_IO_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, GENERATION3D_DOCUMENT_DOWNLOAD_BYTES, 7_500)
}

/// 📄️ Runs one import/export hop against the app instance's RETAINED owner — the export reads its
/// retained preview, and a completed import re-arms every attached preview through the same
/// per-window latch an example switch uses.
enum Generation3dDocumentRetirement {
    Projection(semio_framework_dsl_record::native_encoding::RetainedFieldProjection<Generation3dSnapshot>),
    Projected(semio_framework_dsl_record::FieldValue),
    Writer(semio_framework_dsl_record::RetainedRecordWriter),
    Text(store::semio_format::RetainedTextEnvelope),
    Envelope(crate::standards::v1::subsets::any::io::document_io::Generation3dDocumentEnvelope),
    Export(crate::standards::v1::subsets::any::io::Generation3dDocumentExport),
}

impl semio_framework_value::retirement::RetireOwned for Generation3dDocumentRetirement {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Projection(value)=>value.retirement(),Self::Projected(value)=>value.retirement(),Self::Writer(value)=>value.retirement(),Self::Text(value)=>value.retirement(),Self::Envelope(value)=>value.retirement(),Self::Export(value)=>value.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Projection(value)=>value.retirement_birth_bytes(),Self::Projected(value)=>value.retirement_birth_bytes(),Self::Writer(value)=>value.retirement_birth_bytes(),Self::Text(value)=>value.retirement_birth_bytes(),Self::Envelope(value)=>value.retirement_birth_bytes(),Self::Export(value)=>value.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

struct Generation3dDocumentIoWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
    closing: bool,
    source_identity: Option<usize>,
    source_demand: Option<usize>,
    expand_admission: bool,
    projection: Option<semio_framework_dsl_record::native_encoding::RetainedFieldProjection<Generation3dSnapshot>>,
    projected: Option<semio_framework_dsl_record::FieldValue>,
    writer: Option<semio_framework_dsl_record::RetainedRecordWriter>,
    text: Option<store::semio_format::RetainedTextEnvelope>,
    envelope: Option<crate::standards::v1::subsets::any::io::document_io::Generation3dDocumentEnvelope>,
    candidate: Option<crate::standards::v1::subsets::any::io::Generation3dDocumentExport>,
    retirement: Option<semio_framework_value::retirement::ControlledRetirement<Generation3dDocumentRetirement>>,
    encoding: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    phase: u8,
}

impl Generation3dDocumentIoWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false, closing:false, source_identity:None, source_demand:None, expand_admission:false, projection:None, projected:None, writer:None, text:None, envelope:None, candidate:None, retirement:None, encoding:None, phase:0 }
    }

    fn park_retirement(&mut self,value:Generation3dDocumentRetirement)->Result<(),semio_framework_value::ValueError>{
        match semio_framework_value::retirement::ControlledRetirement::new(value){
            Ok(owner)=>{self.retirement=Some(owner);Ok(())},
            Err((error,value))=>{match value{Generation3dDocumentRetirement::Projection(value)=>self.projection=Some(value),Generation3dDocumentRetirement::Projected(value)=>self.projected=Some(value),Generation3dDocumentRetirement::Writer(value)=>self.writer=Some(value),Generation3dDocumentRetirement::Text(value)=>self.text=Some(value),Generation3dDocumentRetirement::Envelope(value)=>self.envelope=Some(value),Generation3dDocumentRetirement::Export(value)=>self.candidate=Some(value)}Err(error)},
        }
    }
    fn owns_retirement_source(&self)->bool{self.projection.is_some()||self.projected.is_some()||self.writer.is_some()||self.text.is_some()||self.envelope.is_some()||self.candidate.is_some()}
    fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::ValueError>{
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        if let Some(owner)=self.retirement.as_ref(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand()?,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:owner.next_release_byte_demand()?,maximum_depth:owner.next_depth_demand()?})}
        if !self.owns_retirement_source(){return Ok(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0})}
        let frontier=semio_framework_value::list::PagedList::<Box<dyn semio_framework_value::retirement::RetirementCursor>,{usize::MAX}>::default();
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:frontier.next_capacity_allocation_bytes(1).map_err(semio_framework_value::ValueError::from)?.unwrap_or(0),maximum_release_bytes:0,maximum_depth:1})
    }

    fn text_step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<Generation3dPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>,Fault>{
        use semio_framework_value::NativeEncodeControl;
        let identity=input.snapshot as*const Generation3dSnapshot as usize;
        if self.source_identity.is_some_and(|source|source!=identity){return Err(Fault::from("generation3d-document-source-changed"))}
        if cx.is_cancelled(){return Err(Fault::from("generation3d-document-canceled"))}
        let stage=match self.phase{0|1 if self.source_demand.is_none()=>"document-measure",0|1=>"document-project",2=>"document-physical",3=>"document-envelope",4=>"document-base64",_=>"document-retirement"};
        cx.set_stage(stage);
        while !cx.should_yield(){
            let mut accepted=|_|!cx.is_cancelled();
            let mut control=match self.encoding.take(){Some(receipt)=>NativeEncodeControl::resume(receipt,&mut accepted),None=>{self.expand_admission=self.phase==0;Ok(NativeEncodeControl::new(GENERATION3D_DOCUMENT_SCAFFOLD_BYTES,&mut accepted))}}.map_err(|error|Fault::from(error.to_string()))?;
            if self.retirement.is_some(){
                let result=(||->Result<(),semio_framework_value::ValueError>{
                    control.checkpoint()?;
                    let mut grant=self.retirement_demands(8)?;grant.maximum_copy_bytes=8;
                    control.charge(grant.maximum_capacity_bytes)?;
                    let step=self.retirement.as_mut().unwrap().step(grant)?;
                    if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){self.retirement.take();}
                    Ok(())
                })();
                self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);
                result.map_err(|error|Fault::from(error.to_string()))?;cx.consume_fuel(1);continue;
            }
            if self.phase==1&&self.source_demand.is_none(){
                let measured=self.projection.as_mut().unwrap().measure_step(input.snapshot,1,GENERATION3D_DOCUMENT_SOURCE_ALLOCATION_BYTES,&mut control);
                match measured{
                    Ok(Some(bytes))=>{
                        self.source_demand=Some(bytes);
                        if self.expand_admission{control=match control.admit_capacity(bytes,GENERATION3D_DOCUMENT_OWNERSHIP_SOURCE_MULTIPLES,GENERATION3D_DOCUMENT_SCAFFOLD_BYTES){Ok(control)=>control,Err((control,error))=>{self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);return Err(Fault::from(error.to_string()))}};self.expand_admission=false;}
                    },
                    Ok(None)=>{},
                    Err(error)=>{self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);return Err(Fault::from(error.to_string()))},
                }
                self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);cx.consume_fuel(1);continue;
            }
            let result=(||->Result<(),semio_framework_value::ValueError>{
                match self.phase{
                    0=>{self.source_identity=Some(identity);self.projection=Some(semio_framework_dsl_record::native_encoding::RetainedFieldProjection::new(input.snapshot));self.phase=1;},
                    1=>if let Some(value)=self.projection.as_mut().unwrap().step(input.snapshot,1,&mut control)?{
                        self.projected=Some(value);
                        let spec=crate::standards::v1::subsets::any::io::text::snapshot::generation3d_document_spec_controlled(&mut control)?;
                        if !matches!(self.projected,Some(semio_framework_dsl_record::FieldValue::Record(_))){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"generation3d projection root is not a record"))}
                        let Some(semio_framework_dsl_record::FieldValue::Record(record))=self.projected.take()else{unreachable!()};
                        self.writer=Some(semio_framework_dsl_record::RetainedRecordWriter::new(record,spec,semio_framework_dsl_record::JoinMode::Document,GENERATION3D_DOCUMENT_DOWNLOAD_BYTES));
                        let retired=self.projection.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Projection(retired))?;self.phase=2;
                    },
                    2=>if let Some(body)=self.writer.as_mut().unwrap().step(1,&mut control)?{
                        self.text=Some(store::semio_format::RetainedTextEnvelope::new("procedural.generation3d".into(),store::semio_format::Component::Dsl,1,body));
                        let retired=self.writer.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Writer(retired))?;self.phase=3;
                    },
                    3=>if let Some(text)=self.text.as_mut().unwrap().step(1,8,&mut control)?{
                        let row=crate::standards::v1::subsets::any::io::document_io::EXPORT_FORMATS.iter().find(|row|row.id=="txt").unwrap();
                        self.envelope=Some(crate::standards::v1::subsets::any::io::document_io::Generation3dDocumentEnvelope::new(row,text.into_bytes()));
                        let retired=self.text.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Text(retired))?;self.phase=4;
                    },
                    4=>if let Some(export)=self.envelope.as_mut().unwrap().step(1,&mut control).map_err(|error|semio_framework_value::ValueError::new(error.kind,error.message))?{
                        self.candidate=Some(export);let retired=self.envelope.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Envelope(retired))?;self.phase=5;
                    },
                    _=>{},
                }
                Ok(())
            })();
            self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);
            result.map_err(|error|Fault::from(error.to_string()))?;
            cx.consume_fuel(1);
            if self.phase==5&&self.retirement.is_none(){
                let export=self.candidate.take().ok_or_else(||Fault::from("generation3d-document-output-missing"))?;
                self.consumed=true;
                return Ok(ArtifactCommandWorkStep::Complete(Emit::effect(Effect::DownloadMediaExport{filename:export.filename,mime_type:export.mime_type,data:export.data,encoding:export.encoding})));
            }
        }
        Ok(ArtifactCommandWorkStep::Progress{stage,preview:b"{}"})
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dDocumentIoWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🧮️ An import replaces the whole graph: it removes every widget the document holds and plants
    /// the three-widget import fixture, so its extent is the document's own width plus that fixture —
    /// exactly the accounting `Generation3dPreviewCommandWork` does for an example switch. A request
    /// and an export author no durable row at all and declare the one-item extent.
    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        if matches!(command, Generation3dCommand::ImportDocument(_)) {
            return GENERATION3D_RETAINED_CAPACITY.rows_for_items(snapshot.host_snapshot.widgets.len().checked_add(3)?);
        }
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.consumed || self.closing { return Err(Fault::from("generation3d-document-io-work-repeated")); }
        if matches!(input.command,Generation3dCommand::ExportDocument(payload) if payload.format=="txt"){return self.text_step(input,cx)}
        self.consumed = true;
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let emit = match input.command {
            Generation3dCommand::ImportDocumentRequest(payload) => import_document_request::emit(payload, &doc.snapshot.host_snapshot)?,
            Generation3dCommand::ExportDocument(payload) => {
                let meshes = if payload.format == "txt" { None } else {
                    self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.with_session(|session| export_document::retained_meshes(&doc, &cfg, session, payload.widget_id.as_deref())))??
                };
                export_document::emit(payload, &doc, meshes.as_deref())?
            }
            Generation3dCommand::ImportDocument(payload) => {
                let windows = generation3d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
                let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
                self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                    let mut emit = import_document::emit(payload, &doc, &cfg)?;
                    // 🔁️ An import that changed nothing owes the previews nothing.
                    if !emit.artifact_mutations.is_empty() {
                        owner.owe_attached_previews_carrying(&windows, servable, &mut emit)?;
                    }
                    Ok(emit)
                })?
            }
            _ => return Err(Fault::from("generation3d-document-io-route-rejected")),
        };
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn begin_close(&mut self){self.closing=true;}

    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if !self.closing{return InteractiveJobCloseStep::Blocked}
        if grant.maximum_items==0{return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()}}
        if self.retirement.is_none(){
            let source=if let Some(value)=self.projection.take(){Some(Generation3dDocumentRetirement::Projection(value))}
            else if let Some(value)=self.projected.take(){Some(Generation3dDocumentRetirement::Projected(value))}
            else if let Some(value)=self.writer.take(){Some(Generation3dDocumentRetirement::Writer(value))}
            else if let Some(value)=self.text.take(){Some(Generation3dDocumentRetirement::Text(value))}
            else if let Some(value)=self.envelope.take(){Some(Generation3dDocumentRetirement::Envelope(value))}
            else{self.candidate.take().map(Generation3dDocumentRetirement::Export)};
            if let Some(value)=source{if let Err(error)=self.park_retirement(value){return InteractiveJobCloseStep::Refused(error.kind)}}
            else{self.encoding.take();return InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()}}
        }
        match self.retirement.as_mut().unwrap().step(grant){
            Ok(RetainedCloneStep::Complete(progress))=>{self.retirement.take();InteractiveJobCloseStep::Pending{progress}},
            Ok(RetainedCloneStep::Progress(progress))=>InteractiveJobCloseStep::Pending{progress},
            Err(error)=>InteractiveJobCloseStep::Refused(error.kind),
        }
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(copy).map(|grant|grant.maximum_capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_depth)}

    fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.projected.is_none()&&self.writer.is_none()&&self.text.is_none()&&self.envelope.is_none()&&self.candidate.is_none()&&self.retirement.is_none()&&self.encoding.is_none()}
}

struct Generation3dDocumentIoJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dDocumentIoJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_DOCUMENT_IO_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dDocumentIoJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_DOCUMENT_IO_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_document_io_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_DOCUMENT_IO_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d document-io command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dDocumentIoJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_DOCUMENT_IO_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 📄️ `importDocumentRequest` and `exportDocument` author no store row at all — one asks the
    /// shell for a file, the other hands it a download — so both are `HostOnly`. `importDocument`
    /// replaces the graph on the Artifact lane and carries the fresh fixture's camera on the Config
    /// lane, exactly as `setActiveExample` does.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "importDocumentRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importDocument", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

struct Generation3dDocumentIoJobFactoryProofs;

impl Generation3dDocumentIoJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dDocumentIoJobFactory",
        factory_type: Generation3dDocumentIoJobFactory,
        contract: generation3d_document_io_contract(),
        tools: ["importDocumentRequest", "importDocument", "exportDocument"]
    }
}
//#endregion 📄️DocumentIoRoute

struct Generation3dBoundedCommandJobFactoryProofs;

impl Generation3dBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dBoundedCommandJobFactory",
        factory_type: Generation3dBoundedCommandJobFactory,
        contract: generation3d_bounded_contract(),
        tools: [
            "setActiveExample",
            "nodeGraphEdit",
            "setWidgetInput",
            "deleteSelection",
            "removeWidget",
            "addWidget",
            "patchFlowWidgets",
            "reorganize",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "addGeneration",
            "removeGeneration",
            "renameGeneration",
            "updateGenerationValues",
            "nodeGraphViewport",
            "setLodMode",
            "setShowMode",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setCamera",
            "selectGeneration",
            "cycleShowMode",
            "cycleLodMode",
            "selectNextNode",
            "selectPreviousNode",
            "selectUpstreamNode",
            "selectDownstreamNode",
            "activateSelection",
            "editMeshSelection",
            "knifeMeshSelection",
        ]
    }
}

//#endregion 🧵️RetainedCommands

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route: its OWN tool, its OWN factory and its OWN execution
/// contract, because the payload it carries is nothing like a gesture. `setContributions` delivers
/// the shell's whole `flow.extension` closure — 293 642 characters for this app's nine staged
/// extension plugins — into the plugin's process-wide flow extension registry, which is what makes
/// `brep`/`math` resolvable at all; without it the served app has NO operators and every preview
/// faults `flow.extension-not-contributed` (`📓️extension-addressing-2026-09-10.md` §6).
const GENERATION3D_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const GENERATION3D_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.3d.contributions-command.v1";
/// 📐️ The REAL wire ceiling of one contributions push: what the paged command ingress can deliver
/// into this guest at all, and nothing narrower.
///
/// 🧊️ The pack crosses WHOLE and pack-encoded — `PluginRuntime.performInvocation` encodes the
/// invocation and the shard streams it one 4 KiB page per turn — so the JSON entry point's
/// `PUBLIC_INVOCATION_STRING_BYTES` page run never applies to it and
/// `PUBLIC_INVOCATION_BODY_BYTES` is not this route's bound. Declaring that JSON body cap here
/// refused a 273 136-byte pack-encoded contributions command at the tool factory
/// (`tool factory '…/setContributions' rejected 273136 raw bytes before decoding; maximum is
/// 262144`) even after the transport itself had delivered all 67 pages — ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14. The one bound that actually binds is
/// `COMMAND_MAXIMUM_BYTES`, itself the guest linear-memory budget's assembled-answer ceiling, and
/// the contract reserves per DECLARED extent, never per maximum, so naming it costs nothing.
const GENERATION3D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

/// 🧾️ The contributions route's ONE execution contract — deliberately NOT
/// [`generation3d_bounded_contract`]: raising the 8 KiB gesture quota every interactive command
/// lives under, just to let a boot-time host push through, would widen 29 unrelated routes.
pub fn generation3d_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_CONTRIBUTIONS_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page. Runs against the app instance's RETAINED session for
/// symmetry with every other retained route, but touches nothing in it: the registry the page
/// assembles into is process-wide, shared by every app of this component, which is exactly why one
/// app owning this route is enough to give the whole plugin its operators.
struct Generation3dContributionsWork {
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation3d-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation3dCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation3d-contributions-route-rejected"));
        };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        // 🪟️ An install that moved the registry owes the windows the SHELL says are attached an
        // evaluation — the same trusted `ViewModel` roster `pending_effects` starts the run off, never a
        // window this route invents (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let windows = generation3d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let _ = (&doc, &cfg);
        let mut emit = Emit::default();
        self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            if owner.with_session(|session| set_contributions::install(payload, session))?? {
                let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit)?;
            }
            Ok(())
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

struct Generation3dContributionsJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dContributionsJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_contributions_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dContributionsJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🧩️ `HostOnly`: the flow extension registry is process-wide runtime state, never a document,
    /// config, transient or window-transient lane — the route emits an empty `Emit`.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

struct Generation3dContributionsJobFactoryProofs;

impl Generation3dContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dContributionsJobFactory",
        factory_type: Generation3dContributionsJobFactory,
        contract: generation3d_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 📬️ArtifactStorePreparation

fn generation3d_artifact_mutation_retained_bytes(mutation: &Generation3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-artifact-mutation-encode-failed".to_string())
}

fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation3dDiff` owns the
/// projections it displaces (a `FlowHostSnapshot` whose `layout` is an `OrderedMap` root that rejects a bare
/// drop), so the intermediate delta is retired rather than dropped: leaving it to drop glue aborted the
/// whole store-publication turn the moment a layout entry existed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Mirrors the `🌀️generation2d` twin exactly.
fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>, Generation3dMutation), String> {
    admit_generation3d_artifact_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let applied = protocol::apply_diff(&diff, base);
    diff.retire_cold();
    let post = applied.map_err(|_| "generation3d-artifact-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Generation3dSnapshot>>,
    mutation: Option<Generation3dMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Generation3dMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("generation3d-artifact-lane".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>> {
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-artifact-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-artifact-authority-missing".to_string())?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "generation3d-artifact-base-retirement-rejected"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation

//#region 📬️ConfigStorePreparation
fn generation3d_config_mutation_retained_bytes(mutation: &Generation3dConfigMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-config-mutation-encode-failed".to_string())
}

fn admit_generation3d_config_mutation(mutation: &Generation3dConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_config_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("generation3d-config-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

fn prepare_generation3d_config(base: &Generation3dConfig, mutation: Generation3dConfigMutation) -> Result<(Generation3dConfig, Vec<Generation3dConfigMutation>, Generation3dConfigMutation), String> {
    admit_generation3d_config_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::apply_diff(&diff, base).map_err(|_| "generation3d-config-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Generation3dConfigPreparationFactory;

struct Generation3dConfigPreparation {
    base: Option<store::SnapshotRead<Generation3dConfig>>,
    mutation: Option<Generation3dConfigMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparationFactory {
    fn preflight(&self, mutation: &Generation3dConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("generation3d-config-lane".into());
        }
        admit_generation3d_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>> {
        let retained_bytes = generation3d_config_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-config-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-config-authority-missing".to_string())?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "generation3d-config-base-retirement-rejected"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

impl ArtifactEditor for Generation3dPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        examples()
    }
    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation3dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `editor did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(crate::GENERATION3D_CHILD_PROJECTION), error.to_string()))
    }

    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        semio_framework_pack_json::to_json_string(&catalogue_panel::catalogue())
    }

    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dConfig;
    type ConfigMutation = Generation3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::generation3d::presence::Generation3dPresence;
    type PresenceMutation = crate::editor::generation3d::presence::Generation3dPresenceMutation;
    type Transient = Generation3dTransient;
    type TransientMutation = Generation3dTransientMutation;

    type Command = Generation3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::generation3d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation3dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dInstanceOperationOwner::new())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::generation3d_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::host::generation3d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::host::generation3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation3d atomic publication authority is absent or stale"))?;
        crate::host::generation3d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Generation3d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()?;
        registry.register::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>()
    }

    /// 🧭️ The leaf that names each tool's history row (design §19.1): a first grab's transaction splices the transform
    /// operator in before its relative leaf, and the row is still labelled by that leaf; a mesh edit's row is labelled by the
    /// `create-widget` of the operator it inserts ("Insert …", audit P4).
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        match tool.strip_prefix(GENERATION3D_EDITOR_APP_ID).and_then(|verb| verb.strip_prefix('#')) {
            Some("translateSelection") => &["drag-transforms"],
            Some("rotateSelection") => &["rotate-transforms"],
            Some("scaleSelection") => &["scale-transforms"],
            Some("editMeshSelection" | "knifeMeshSelection" | "deleteSelection") => &["create-widget"],
            _ => &[],
        }
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12): the gumball's `generation3d.gumball.*`, the
    /// preview evaluation's and the document-level ones.
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<Vec<(&'static str, semio_framework_ui_locale::LocalizedLabel)>> = std::sync::LazyLock::new(|| {
            transform_commands::gumball_fault_notices().iter().chain(crate::preview_eval::preview_eval_fault_notices()).chain(crate::generation3d_document_fault_notices()).cloned().collect()
        });
        NOTICES.as_slice()
    }

    /// 🎯️ `flowEvalTick` publishes the evaluation into ONE preview window's retained transient, and
    /// the whole chain is self-dispatched: an `Effect::DispatchAction` reaches the shell with no
    /// window of its own and is redispatched under whichever window is current — the flow window in
    /// the served app. Naming the target off the PAYLOAD makes the runtime validate it against the
    /// trusted ViewModel roster and capture that exact window's mutation authority, instead of the
    /// current window's (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🪟️ Every route of the preview chain that may publish — the dispatched hop and both
    /// window-addressed folds, which now continue the chain inline and therefore publish the
    /// intermediate geometry and the advanced status pill the hop used to.
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        let (window_id, window_kind_id) = generation3d_flow_eval_window_address(command)?;
        if window_id.is_empty() {
            return None;
        }
        generation3d_preview_kind(window_kind_id).map(|kind| (window_id, kind))
    }

    const DIALECT: Dialect = crate::GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation3dConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation3dFlowEvalJobFactory::new(&controller))?;
        registry.register(Generation3dContributionsJobFactory::new(&controller))?;
        registry.register(Generation3dDocumentIoJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_FLOW_EVAL_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "the generation 3d command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let contributions = GENERATION3D_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let work: Box<dyn ArtifactCommandWork<EditorApp<Generation3dPlayApp>>> =
            if contributions {
                Box::new(Generation3dContributionsWork { instance_owner: request.instance_operation_owner, consumed: false })
            } else if GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dDocumentIoWork::new(tool_id, request.instance_operation_owner))
            } else if GENERATION3D_FLOW_EVAL_WINDOW_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dFlowEvalWindowWork::new(tool_id, request.instance_operation_owner))
            } else if GENERATION3D_PREVIEW_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dPreviewCommandWork::new(tool_id, request.instance_operation_owner))
            } else {
                Box::new(Generation3dSessionCommandWork::new(tool_id, request.instance_operation_owner))
            };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Generation3dCommand::command_id,
            if contributions {
                GENERATION3D_CONTRIBUTIONS_RAW_BYTES
            } else if GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
                GENERATION3D_DOCUMENT_IO_RAW_BYTES
            } else if GENERATION3D_FLOW_EVAL_TOOL_IDS.contains(&tool_id) {
                GENERATION3D_FLOW_EVAL_RAW_BYTES
            } else {
                GENERATION3D_RETAINED_RAW_BYTES
            },
            GENERATION3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🧾️ BOTH factories' proofs, in registration order — `validate_tool_job_rows` matches this
    /// catalogue against the registered factories exactly, so a second factory that is registered
    /// and not proved (or proved and not registered) fails the app closed at boot.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation3dBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation3dFlowEvalJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dDocumentIoJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::generation3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(::semio_framework_async::poll::resolve_ready(generation3d_io()))
    }

    /// 📤️ Exports the declared document without evaluating its flow graph.
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
        Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }

    /// 🪪️ Reads geometry from the explicitly supplied instance's retained evaluation session.
    fn export_media_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        _transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>,
    ) -> Result<semio_framework_plugin::Media, MediaError> {
        if port != "geometry:out" {
            return Self::export_media(port, doc);
        }
        let mesh = owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| export_mesh_from_session(doc.snapshot, &Generation3dConfig::default(), session))
        }).map_err(|error| MediaError::Payload(port.into(), error.message))?.map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
        crate::standards::v1::subsets::any::io::mesh_bridge::semio_mesh_from_mesh_data(&mesh)
            .map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
        Ok(semio_framework_plugin::Media {
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: semio_framework_pack_json::to_json_string(&mesh) },
        })
    }

    /// 🎞️ `"params:in"` — sets matching `InputSlider` widgets from a `{widgetId: number}` JSON object as ABSOLUTE
    /// `change-slider-value` leaves; unmatched keys, non-slider widgets and unchanged values are ignored.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let parsed = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let object = parsed.as_object().cloned().ok_or_else(|| MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()))?;
                let host_snapshot = &doc.snapshot.host_snapshot;
                let operations = object
                    .iter()
                    .filter_map(|(target_id, value)| {
                        let number = value.as_f64().filter(|number| number.is_finite())?;
                        host_snapshot
                            .widgets
                            .iter()
                            .any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::InputSlider { id, value: current, .. } if id == target_id && *current != number))
                            .then(|| crate::standards::v1::subsets::any::schema::mutations::change_slider_value::change_slider_value(target_id.to_string(), number))
                    })
                    .collect();
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Generation3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        match action {
            "setActiveExample" => Ok(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => {
                semio_framework_tool_machine::node_graph_edit_rows(&args).map_err(Fault::from)?;
                Ok(Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: args.get("operations").map(semio_framework_pack_json::to_json_string).unwrap_or_else(|| "[]".into()) }))
            }
            "deleteSelection" => Ok(Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "addWidget" => Ok(Generation3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind"]), format: str_arg(&["format"]), action: str_arg(&["action"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "setWidgetInput" => {
                let mesh_source = str_arg(&["facet"]).as_deref() == Some("meshSource");
                let index = |key: &str| -> Result<Option<u32>, Fault> {
                    let maximum = if mesh_source && key == "destination" { 599999.0 } else { 1023.0 };
                    args.get(key).map(|value| value.as_f64().filter(|value| value.is_finite() && *value >= 0.0 && *value <= maximum && value.fract() == 0.0).map(|value| value as u32).ok_or_else(|| Fault::from("Choose an integer within the input bounds"))).transpose()
                };
                let operation = args.get("operation").map(|value| value.as_str().filter(|value| matches!(*value,"set"|"add"|"remove"|"move")).map(str::to_string).ok_or_else(|| Fault::from("Unknown list operation"))).transpose()?;
                Ok(Generation3dCommand::SetWidgetInput(set_widget_input::SetWidgetInput {
                widget_id: str_arg(&["widgetId"]).ok_or_else(|| Fault::from("Choose a widget"))?,
                channel: str_arg(&["channel"]).ok_or_else(|| Fault::from("Choose an input"))?,
                value: args.get("value").map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| semio_framework_pack_json::to_json_string(value))).ok_or_else(|| Fault::from("Input value is missing"))?,
                component: str_arg(&["component"]),
                operation,
                index: index("index")?,
                destination: index("destination")?,
                facet: args.get("facet").map(|value| value.as_str().filter(|value| matches!(*value, "port" | "variableName" | "variableSchema" | "exportFormat" | "meshSource")).map(str::to_string).ok_or_else(|| Fault::from("Unknown widget facet"))).transpose()?,
                path: args.get("path").map(|value| value.as_array().filter(|parts| !parts.is_empty() && parts.len() <= 16).ok_or_else(|| Fault::from("Choose a structured mesh field"))?.iter().map(|part| part.as_str().filter(|part| !part.is_empty() && part.chars().count() <= 128).map(str::to_owned).ok_or_else(|| Fault::from("Choose a bounded field name"))).collect::<Result<Vec<_>, _>>()).transpose()?,
            }))
            }
            "patchFlowWidgets" => Ok(Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: {
                    let mut ids = string_list("widgetIds");
                    if ids.is_empty() {
                        ids = string_list("widget_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: f64_arg(&["value"]),
            })),
            "editMeshSelection" => {
                let number = |key: &str, default: f64| -> Result<f64, Fault> {
                    match args.get(key) {
                        None => Ok(default),
                        Some(value) => value.as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from(format!("{key} must be a finite number"))),
                    }
                };
                let text = |key: &str, default: &str| -> Result<String, Fault> {
                    match args.get(key) {
                        None => Ok(default.into()),
                        Some(value) => value.as_str().map(str::to_string).ok_or_else(|| Fault::from(format!("{key} must be text"))),
                    }
                };
                let integer = |key: &str, maximum: f64| -> Result<u32, Fault> {
                    let value = number(key, 1.0)?;
                    if !value.is_finite() || value.fract() != 0.0 || !(1.0..=maximum).contains(&value) { return Err(Fault::from(format!("{key} must be an integer from 1 to {maximum}"))); }
                    Ok(value as u32)
                };
                let center = match args.get("center") {
                    None => [0.0; 3],
                    Some(value) => {
                        let values = value.as_array().filter(|values| values.len() == 3).ok_or_else(|| Fault::from("Center requires three coordinates"))?;
                        let mut center = [0.0; 3];
                        for axis in 0..3 { center[axis] = values[axis].as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from("Center coordinates must be finite"))?; }
                        center
                    }
                };
                Ok(Generation3dCommand::EditMeshSelection(edit_mesh_selection::EditMeshSelection {
                    operation: text("operation", "extrude")?, amount: number("amount", 0.1)?, cuts: integer("cuts", 256.0)?,
                    dx: number("dx", 0.0)?, dy: number("dy", 0.0)?, dz: number("dz", 0.0)?,
                    width: number("width", 0.1)?, segments: integer("segments", 64.0)?, merge_mode: text("mergeMode", "center")?,
                    tolerance: number("tolerance", 0.0001)?, radius: number("radius", 1.0)?, grid: number("grid", 1.0)?, center,
                }))
            }
            "knifeMeshSelection" => {
                let point = |key: &str, default: [f64; 3]| -> Result<[f64; 3], Fault> {
                    let Some(value) = args.get(key) else { return Ok(default); };
                    let values = value.as_array().filter(|values| values.len() == 3).ok_or_else(|| Fault::from("Knife points require three coordinates"))?;
                    let mut point = [0.0; 3];
                    for axis in 0..3 {
                        point[axis] = values[axis].as_f64().filter(|value| value.is_finite() && value.abs() <= f32::MAX as f64).ok_or_else(|| Fault::from("Knife points must be finite mesh coordinates"))?;
                    }
                    Ok(point)
                };
                Ok(Generation3dCommand::KnifeMeshSelection(knife_mesh_selection::KnifeMeshSelection { start: point("start", [0.0, -1.0, 0.0])?, end: point("end", [0.0, 1.0, 0.0])? }))
            }
            "reorganize" => Ok(Generation3dCommand::Reorganize(reorganize::Reorganize {})),
            "translateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids, dx: f64_arg(&["dx"]).unwrap_or(0.0), dy: f64_arg(&["dy"]).unwrap_or(0.0), dz: f64_arg(&["dz"]).unwrap_or(0.0), phase: str_arg(&["phase"]), reason: str_arg(&["reason"]), window_id: str_arg(&["windowId"]) }))
            }
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::RotateSelection(rotate_selection::RotateSelection {
                    node_ids,
                    ax: f64_arg(&["ax"]).unwrap_or(0.0),
                    ay: f64_arg(&["ay"]).unwrap_or(0.0),
                    az: f64_arg(&["az"]).unwrap_or(0.0),
                    angle: f64_arg(&["angle"]).unwrap_or(0.0),
                    phase: str_arg(&["phase"]),
                    reason: str_arg(&["reason"]),
                    window_id: str_arg(&["windowId"]),
                }))
            }
            "scaleSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids, sx: f64_arg(&["sx"]).unwrap_or(1.0), sy: f64_arg(&["sy"]).unwrap_or(1.0), sz: f64_arg(&["sz"]).unwrap_or(1.0), phase: str_arg(&["phase"]), reason: str_arg(&["reason"]), window_id: str_arg(&["windowId"]) }))
            }
            "addGeneration" => Ok(Generation3dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            // 🖊️ `value` is where the inline rename editor's typed text lands: a scalar `Trigger::Commit`
            // payload is NAMED by its trigger and merged over the authored args (`uiIntentPayload`,
            // `🛠️ShellHelpers/🟦️.tsx`), so the row editor dispatches `{id, value}` and never `{id, name}`.
            "renameGeneration" => Ok(Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name", "value"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").cloned().unwrap_or(semio_framework_value::DslValue::Null);
                Ok(Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: parse_flow_viewport(&args)? })),
            "setLodMode" => Ok(Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setShowMode" => Ok(Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "cycleShowMode" => Ok(Generation3dCommand::CycleShowMode(cycle_show_mode::CycleShowMode {})),
            "cycleLodMode" => Ok(Generation3dCommand::CycleLodMode(cycle_lod_mode::CycleLodMode {})),
            "selectNextNode" => Ok(Generation3dCommand::SelectNextNode(select_next_node::SelectNextNode {})),
            "selectPreviousNode" => Ok(Generation3dCommand::SelectPreviousNode(select_previous_node::SelectPreviousNode {})),
            "selectUpstreamNode" => Ok(Generation3dCommand::SelectUpstreamNode(select_upstream_node::SelectUpstreamNode {})),
            "selectDownstreamNode" => Ok(Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {})),
            "activateSelection" => Ok(Generation3dCommand::ActivateSelection(activate_selection::ActivateSelection {})),
            "toggleSun" => Ok(Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setCamera" => Ok(Generation3dCommand::SetCamera(set_camera::SetCamera { camera: parse_preview_camera_json(&args) })),
            "selectGeneration" => Ok(Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "flowEvalTick" => Ok(Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowEvalResolve" => Ok(Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
                extension_id: str_arg(&["extensionId", "extension_id"]).unwrap_or_default(),
                ok: args.get("ok").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false),
                fault_code: str_arg(&["faultCode", "fault_code"]).unwrap_or_default(),
                fault_message: str_arg(&["faultMessage", "fault_message"]).unwrap_or_default(),
            })),
            "flowTessellateResolve" => Ok(Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "flowEvalRelease" => Ok(Generation3dCommand::FlowEvalRelease(flow_eval_release::FlowEvalRelease {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowTessellateCancelResolve" => Ok(Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
                ok: args.get("ok").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false),
            })),
            "setContributions" => Ok(Generation3dCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            "importDocumentRequest" => Ok(Generation3dCommand::ImportDocumentRequest(import_document_request::ImportDocumentRequest { widget_id: str_arg(&["widgetId"]), channel: str_arg(&["channel"]), texture_id: str_arg(&["textureId"]) })),
            // 📥️ The framework hands the whole picked file (`semio_framework::kernel::ImportStaging`).
            "importDocument" => Ok(Generation3dCommand::ImportDocument(import_document::ImportDocument {
                name: str_arg(&["name"]).unwrap_or_default(),
                payload: str_arg(&["payload", "contents"]).unwrap_or_default(),
                widget_id: str_arg(&["widgetId"]), channel: str_arg(&["channel"]), texture_id: str_arg(&["textureId"]),
            })),
            "exportDocument" => Ok(Generation3dCommand::ExportDocument(export_document::ExportDocument { format: str_arg(&["format", "value"]).unwrap_or_else(|| "stl".into()), widget_id: str_arg(&["widgetId"]) })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 📨️ Every host event ends the window's open gumball gesture with zero trace under the reason the tool records: a blur
    /// `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an opened history edit
    /// `frozen` and a remote edit `baseMoved` — the typed `translateSelection{phase: "abort"}` of that window.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<Self::Command> {
        use semio_framework_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => ToolAbortReason::BaseMoved,
        };
        Some(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection {
            node_ids: Vec::new(),
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            phase: Some("abort".into()),
            reason: Some(reason.as_str().into()),
            window_id: Some(event.window_id().to_string()),
        }))
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`{translate,rotate,scale}Selection` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose
    /// per-row `$module::handle(payload, doc, cfg, ctx)` signature is framework-fixed and has no
    /// `interaction` slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Generation3dCommand,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, Fault> {
        with_scratch_session(|session| {
            let component_ids = &interaction.selection(selection::DOMAIN).ids;
            if let Some(ids) = generation3d_component_targets(command, view_state, interaction.active_granularity(selection::DOMAIN), component_ids, &interaction.selection("graph").ids).filter(|ids| !ids.is_empty()) {
                selection::validate_cached_components(&doc.snapshot.host_snapshot, session, ids, doc.operation().ok().map(|operation| &operation.canonical_base_revision)).map_err(Fault::from)?;
            }
            match command {
            Generation3dCommand::EditMeshSelection(payload) => edit_mesh_selection::apply_selected("editMeshSelection", payload, doc, &interaction.selection(selection::DOMAIN).ids),
            Generation3dCommand::KnifeMeshSelection(payload) => knife_mesh_selection::apply_selected(payload, doc, &interaction.selection(selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(_) if selection::edits_components(view_state, interaction.active_granularity(selection::DOMAIN)) => edit_mesh_selection::delete_selected(doc, &interaction.selection(selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, session),
            _ if generation3d_gumball_command(command) => {
                let gesture = generation3d_gumball_gesture(command).ok_or_else(|| Fault::from("generation3d-gumball-command"))?;
                let components = selection::edits_components(view_state, interaction.active_granularity(selection::DOMAIN)).then(|| interaction.selection(selection::DOMAIN).ids.as_slice());
                let ids = transform_commands::gumball_ids(&doc.snapshot.host_snapshot, gesture.ids, components, &interaction.selection("graph").ids)?;
                transform_commands::gumball_once(gesture.verb, ids, gesture.motion, doc)
            }
            Generation3dCommand::SelectNextNode(_payload) => Ok(select_next_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectPreviousNode(_payload) => Ok(select_previous_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectUpstreamNode(_payload) => Ok(select_upstream_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectDownstreamNode(_payload) => Ok(select_downstream_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::ActivateSelection(_payload) => Ok(activate_selection::apply_ports(doc, &generation3d_port_ids_by_node(&doc.snapshot.host_snapshot), &interaction.selection("graph").ids)),
            _ => command.dispatch(doc, cfg, session),
            }
        })
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every widget's visible ports become `handle`
    /// targets (`{nodeId}@{portId}`, byte-identical to the node-graph's own pick ids) parented to
    /// their widget, which is what makes `HoverSpec { transitive: true }` light up every channel's
    /// preview geometry from a single node hover, and resolve a preview-instance hover back to its
    /// node. Every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let host_snapshot = &doc.snapshot.host_snapshot;
        let mut ordered = Vec::new();
        let ports_by_node = generation3d_port_ids_by_node(host_snapshot);
        for widget in &host_snapshot.widgets {
            let id = crate::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            for port in ports_by_node.get(&id).into_iter().flatten() {
                ordered.push(TopologyNode { id: port.clone(), granularity: "handle".into(), parent: Some(id.clone()) });
            }
            if let semio_framework_artifact_flow_flow::Widget::Cluster { tree, .. } = widget {
                for child in &tree.neurons {
                    walk_neuron(child, id.clone(), &mut ordered);
                }
            }
        }
        for synapse in &host_snapshot.synapses {
            ordered.push(TopologyNode { id: synapse.id.clone(), granularity: "edge".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    /// ⏯️ Starts, finalizes or wakes the `previewEval` run for the attached preview windows
    /// ([`crate::preview_eval::preview_eval_run_effects`]).
    ///
    /// 🪟️ Every hop the run dispatches names its window: an unaddressed `Effect::DispatchAction` is
    /// redispatched by the shell under the CURRENT window — the flow window `procedural-main` in the
    /// served app — and the route's own work then refuses it (`📓️work-capacity-2026-09-10.md` §7).
    /// Before the first `Event::SurfaceVisible` the host has no roster at all (`view` is `None`) and
    /// there is nothing to publish into, so nothing starts.
    ///
    /// 🚧️ A graph the live registry cannot serve is not slow work: a hop would recompute the identical
    /// `flow.extension-not-contributed` miss. Only `setContributions` can move it, and it owes every
    /// attached preview an evaluation itself.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        let windows = generation3d_preview_windows(view);
        let servable = flow_eval_tick::may_rearm(&doc.snapshot.host_snapshot);
        let applied_edits = crate::preview_eval::applied_document_edits_digest(doc.history) ^ doc.provisional_generation().wrapping_mul(0x9e37_79b9_7f4a_7c15);
        owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                use crate::preview_eval::PreviewEvalRunOwner as _;
                let (session, link) = owner.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
                Ok(crate::preview_eval::preview_eval_run_effects(session, link, &windows, doc.tool_run(), servable, applied_edits))
            })
            .unwrap_or_default()
    }

    /// ⏯️ The `previewEval` run job over the run's base document: the preview widgets it observes are
    /// the document's own, and its hops read the retained session this instance's commands fold into.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != crate::preview_eval::PREVIEW_EVAL_TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let preview_widget_ids = crate::preview_eval::preview_widget_ids(&request.snapshot.host_snapshot);
        Ok(Some(Box::new(crate::preview_eval::PreviewEvalRunJob::<Generation3dInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity, preview_widget_ids)?)))
    }

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        with_scratch_session(|session| generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, None, view_state, &PreviewInteractionMarks::default(), session, doc.tool_run()))
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into every
    /// window body — the node graph paints the hovered node/port, the world previews paint the
    /// hovered/selected instances, and the inspection panel finally sees a real selection.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient
            .window::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()
            .or_else(|| transient.window::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>())
            .and_then(|state| state.preview_eval_text.as_deref());
        let mut marks = PreviewInteractionMarks::from_interaction(interaction);
        marks.source_revision = doc.render_operation().map(|operation| operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect());
        let previews = matches!(body_key, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW | generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW);
        owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                let gesture = previews.then(|| generation3d_gumball_preview(doc.snapshot, &marks, &owner.gumball)).flatten();
                let (document, marks) = gesture.as_ref().map_or((doc.snapshot, &marks), |(document, marks)| (document, marks));
                let rendered = owner.with_session(|session| generation3d_render_body(body_key, document, cfg.snapshot, preview_eval_text, view_state, marks, session, doc.tool_run()));
                if let Some((document, _)) = gesture {
                    document.retire_cold();
                }
                rendered
            })
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation3d.eval-session-owner", error.message))?
    }

    fn window_engagements_with_request_context(_doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel, _transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>, interaction: &InteractionView<'_>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let engagement = selection::engagement(&selection::ComponentSelection::from_interaction(interaction), view_state.locale == semio_framework_ui_locale::Locale::De);
        HashMap::from([(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
        let measures = edit_preview::preview_window_measures(config, is_de, generation3d_action);
        HashMap::from([
            (flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.to_string(), flow_window::window_measures(&config.lod_mode, is_de, generation3d_action)),
            (edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ The marks-free entry point the framework still offers — every live right-click goes
    /// through `context_menu_with_request_context` instead, so this delegate builds the same menu
    /// against an empty selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::default(), registry)
    }

    /// 🕹️ `context_menu`'s interaction-aware twin — the one the runtime actually calls
    /// (`VcsArtifactApp::context_menu`). `ContextMenuRequest.surface.selection` only ever carries what
    /// the clicked surface itself painted, so a world-preview instance selected from the node graph
    /// (or vice versa) never reaches a menu row through it; `interaction` is the authoritative
    /// framework-owned `graph` selection, the same one `render_with_request_context` paints from.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::from_interaction(interaction), registry)
    }
}

impl Generation3dPlayApp {
    /// 🗂️ The ONE context-menu implementation — both `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    ///
    /// Grouped disclosure: `reorganize` stays top-level, the transform trio appears only for a real
    /// selection, creation/removal/generation methods fold into taxonomy groups, and
    /// `delete-selection` stays a direct destructive item last.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        _cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        marks: &PreviewInteractionMarks,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let labels = generation3d_labels(view_state);
        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.host_snapshot);
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected_nodes, &selected_edges);
        let has_selection = !nodes.is_empty() || !edges.is_empty();
        let mut menu = Menu::of(registry, view_state).action("reorganize");
        if has_selection {
            menu = menu.action("translateSelection").action("rotateSelection").action("scaleSelection");
        }
        menu = menu.group("create", |m| m.action("addWidget").action("addGeneration"));
        if has_selection {
            menu = menu.group("targets", |m| m.action("removeWidget").action("removeGeneration"));
        }
        menu = menu.group("methods", |m| m.action("renameGeneration").action("updateGenerationValues").action("patchFlowWidgets"));
        menu = menu.group(CONTEXT_MENU_TRANSFER_CATEGORY, |m| m.action("importDocumentRequest").action("exportDocument"));
        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()
    }
}

//#endregion 🔖️Generation3dPlayApp

//#region 🔖️Manifest
pub fn create_generation3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::GENERATION3D_DIALECT).document(["semio", "procedural", "3d"])
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung aufnehmen"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowTessellateResolve", LocalizedLabel::native("Resolve Preview Tessellation", "Vorschau-Tessellierung aufnehmen"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalRelease", LocalizedLabel::native("Release Preview Kernel Work", "Vorschau-Kernelarbeit freigeben"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowTessellateCancelResolve", LocalizedLabel::native("Resolve Preview Cancellation", "Vorschauabbruch aufnehmen"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition {
                in_palette: false,
                ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                    ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                    ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                    ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
                ])
            }))
            .artifact_kind(artifact_kind())
            .icon_id("workflow")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Generator"])
            .mode_def(edit::definition())
            .mode_def(generate::definition())
            .default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)
            .mode_layout(generate::GENERATION_3D_PLAY_MODE_GENERATE, generate::GENERATION_3D_PLAY_LAYOUT_GENERATE)
            .window_kind_def(flow_window::definition())
            .window_kind_def(edit_preview::definition())
            .window_kind_def(generations::definition())
            .window_kind_def(form::definition())
            .window_kind_def(generate_preview::definition())
            // ⏯️ The read-only preview evaluation run: declared so the framework owns its lifecycle and
            // injects its reserved actions. Both modes reference it (`🎭️modes/✏️edit`, `🎭️modes/🧬️generate`)
            // because both mount a preview window that starts it — a declared tool no mode references is
            // refused by `build_definition` (`app-definition.invalid: … not referenced by any mode`).
            .tool(crate::preview_eval::preview_eval_tool_definition())
            .default_layout(edit::layout())
            .named_layout(generate::layout())
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_with(categorized_action("editMeshSelection", LocalizedLabel::native("Edit Mesh Selection", "Netzauswahl bearbeiten"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("knifeMeshSelection", LocalizedLabel::native("Knife Cut Selected Face", "Ausgewählte Fläche schneiden"), ActionKind::Mutation, "methods"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_destructive("deleteSelection")
            .action_with(categorized_action("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Mutation, "targets"))
            .action_destructive("removeWidget")
            .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("setWidgetInput", LocalizedLabel::native("Edit Widget Input", "Elementeingabe bearbeiten"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Mutation, "targets"))
            .action_destructive("removeGeneration")
            .action_with(categorized_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Mutation, "methods"))
            // 📄️ Palette-visible host round-trips — the artifact's whole IO surface. `Shell` because
            // neither authors a document row by itself: one asks the shell for a file, the other
            // hands it a download (`🎮️commands/📂️import-document-request`, `📤️export-document`).
            .shell_action("importDocumentRequest", LocalizedLabel::native("Import Artifact…", "Artefakt importieren…"))
            .shell_action("exportDocument", LocalizedLabel::native("Export Artifact", "Artefakt exportieren"))
            // 📥️ Not palette-worthy: the shell dispatches it with the picked file, args no human
            // types (`dispatchOpenedFiles`, `🛠️ShellHelpers/🟦️.tsx`).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importDocument", LocalizedLabel::native("Import Artifact File", "Artefaktdatei importieren"), ActionKind::Mutation) })
            // 👁️ Ephemeral view actions — world picking, graph camera, sun/LOD/show-mode display toggles, preview camera.
            // Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
            // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
            // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
            .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
            // 🔁️ The keyboard halves of the two display pickers. `AppDefinition.keybinding` carries a
            // chord and an action id and NOTHING else, so `setShowMode`/`setLodMode` — both of which
            // require a `value` — can never be bound to one: a chord that opened a staged arg form
            // would be slower than the picker it replaces. These two read their own next value off the
            // config ladder instead (`config::next_show_mode`/`next_lod_mode`), which is the same
            // ladder the pickers build their rows from.
            .action_with(ActionDefinition::new("cycleShowMode", LocalizedLabel::native("Cycle Show Mode", "Anzeigemodus wechseln"), ActionKind::View, "eye"))
            .action_with(ActionDefinition::new("cycleLodMode", LocalizedLabel::native("Cycle Lod Mode", "LOD-Modus wechseln"), ActionKind::View, "layers"))
            // 🧭️ Node-by-node keyboard traversal of the graph canvas — the five verbs
            // `📓️react-i18n-a11y-customization-2026-09-13.md` §4.1 named as missing. All five are
            // ARG-FREE by construction (a chord carries a key and an action id and nothing else) and
            // read their next target off the document's own wires and layout
            // (`🎮️commands/🧭️navigate-graph`). `ActionKind::View`, never `Mutation`: a traversal moves
            // the framework's selection lane and authors no document op.
            .action_with(ActionDefinition::new("selectNextNode", LocalizedLabel::native("Select Next Node", "Nächsten Knoten auswählen"), ActionKind::View, "arrow-down"))
            .action_with(ActionDefinition::new("selectPreviousNode", LocalizedLabel::native("Select Previous Node", "Vorherigen Knoten auswählen"), ActionKind::View, "arrow-up"))
            .action_with(ActionDefinition::new("selectUpstreamNode", LocalizedLabel::native("Select Upstream Node", "Vorgelagerten Knoten auswählen"), ActionKind::View, "arrow-left"))
            .action_with(ActionDefinition::new("selectDownstreamNode", LocalizedLabel::native("Select Downstream Node", "Nachgelagerten Knoten auswählen"), ActionKind::View, "arrow-right"))
            .action_with(ActionDefinition::new("activateSelection", LocalizedLabel::native("Open Node Ports", "Knotenanschlüsse öffnen"), ActionKind::View, "circle"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_destructive("setActiveExample")
            .action_interactive_job("editMeshSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("knifeMeshSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFlowWidgets", InteractiveJobClassification::Migrated)
            .action_interactive_job("setWidgetInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleShowMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectNextNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectPreviousNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectUpstreamNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectDownstreamNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("activateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowEvalResolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowTessellateResolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowTessellateCancelResolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .action_interactive_job("importDocumentRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportDocument", InteractiveJobClassification::Migrated)
            .action_destructive("exportDocument")
            .action_args("editMeshSelection", vec![
                ActionArgDef::select("operation", LocalizedLabel::native("Operation", "Operation"), vec![
                    ActionArgOption::new("extrude", LocalizedLabel::native("Extrude Faces", "Flächen extrudieren")),
                    ActionArgOption::new("inset", LocalizedLabel::native("Inset Faces", "Flächen einziehen")),
                    ActionArgOption::new("subdivide", LocalizedLabel::native("Subdivide Faces", "Flächen unterteilen")),
                    ActionArgOption::new("flip", LocalizedLabel::native("Flip Faces", "Flächen umkehren")),
                    ActionArgOption::new("deleteFaces", LocalizedLabel::native("Delete Faces", "Flächen löschen")),
                    ActionArgOption::new("moveVertices", LocalizedLabel::native("Move Vertices", "Eckpunkte verschieben")),
                    ActionArgOption::new("loopCut", LocalizedLabel::native("Cut Edge Loops", "Kantenschleifen schneiden")),
                    ActionArgOption::new("bevel", LocalizedLabel::native("Bevel Edges", "Kanten abschrägen")),
                    ActionArgOption::new("dissolveEdges", LocalizedLabel::native("Dissolve Edges", "Kanten auflösen")),
                    ActionArgOption::new("dissolveVertices", LocalizedLabel::native("Dissolve Vertices", "Eckpunkte auflösen")),
                    ActionArgOption::new("mergeVertices", LocalizedLabel::native("Merge Vertices", "Eckpunkte zusammenführen")),
                    ActionArgOption::new("moveProportional", LocalizedLabel::native("Move Proportionally", "Proportional verschieben")),
                    ActionArgOption::new("snapVertices", LocalizedLabel::native("Snap Vertices to Grid", "Eckpunkte am Raster ausrichten")),
                ]).required().default_value(&"extrude"),
                ActionArgDef::number("amount", LocalizedLabel::native("Distance / Inset", "Abstand / Einzug")).default_value(&0.1),
                ActionArgDef { schema: semio_framework::ArgSchema::number(Some(1.0), Some(256.0), Some(1.0), true), ..ActionArgDef::number("cuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(&1) },
                ActionArgDef::number("dx", LocalizedLabel::native("Move X", "Verschieben X")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Move Y", "Verschieben Y")).default_value(&0.0),
                ActionArgDef::number("dz", LocalizedLabel::native("Move Z", "Verschieben Z")).default_value(&0.0),
                ActionArgDef::number("width", LocalizedLabel::native("Bevel Width", "Fasenbreite")).default_value(&0.1),
                ActionArgDef { schema: semio_framework::ArgSchema::number(Some(1.0), Some(64.0), Some(1.0), true), ..ActionArgDef::number("segments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(&1) },
                ActionArgDef::select("mergeMode", LocalizedLabel::native("Merge Mode", "Zusammenführungsmodus"), vec![
                    ActionArgOption::new("first", LocalizedLabel::native("First Vertex", "Erster Eckpunkt")),
                    ActionArgOption::new("center", LocalizedLabel::native("Center", "Mittelpunkt")),
                    ActionArgOption::new("distance", LocalizedLabel::native("By Distance", "Nach Abstand")),
                ]).default_value(&"center"),
                ActionArgDef::number("tolerance", LocalizedLabel::native("Merge Tolerance", "Zusammenführungstoleranz")).default_value(&0.0001),
                ActionArgDef::number("radius", LocalizedLabel::native("Proportional Radius", "Proportionaler Radius")).default_value(&1.0),
                ActionArgDef::number("grid", LocalizedLabel::native("Grid Spacing", "Rasterabstand")).default_value(&1.0),
                ActionArgDef::vector("center", LocalizedLabel::native("Proportional Center", "Proportionaler Mittelpunkt"), 3).default_value(&[0.0, 0.0, 0.0]),
            ])
            .action_args("knifeMeshSelection", vec![
                ActionArgDef::vector("start", LocalizedLabel::native("Cut Start", "Schnittanfang"), 3).required().default_value(&[0.0, -1.0, 0.0]),
                ActionArgDef::vector("end", LocalizedLabel::native("Cut End", "Schnittende"), 3).required().default_value(&[0.0, 1.0, 0.0]),
            ])
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("neuron", LocalizedLabel::native("Neuron", "Neuron")),
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value(&"inputSlider"),
                ActionArgDef::text("neuronKind", LocalizedLabel::native("Operator", "Operator")),
                ActionArgDef::text("format", LocalizedLabel::native("Export Format", "Exportformat")),
                ActionArgDef::text("action", LocalizedLabel::native("Action", "Aktion")),
            ])
            .action_args("setWidgetInput", vec![
                ActionArgDef::select("facet", LocalizedLabel::native("Widget field", "Elementfeld"), vec![ActionArgOption::new("port", LocalizedLabel::native("Operator input", "Operatoreingang")), ActionArgOption::new("variableName", LocalizedLabel::native("Variable name", "Variablenname")), ActionArgOption::new("variableSchema", LocalizedLabel::native("Variable type", "Variablentyp")), ActionArgOption::new("exportFormat", LocalizedLabel::native("Export format", "Exportformat")), ActionArgOption::new("meshSource", LocalizedLabel::native("Mesh source", "Netzquelle"))]),
                ActionArgDef::text("widgetId", LocalizedLabel::native("Widget", "Element")).required(),
                ActionArgDef::text("channel", LocalizedLabel::native("Input", "Eingabe")).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
                ActionArgDef::text("component", LocalizedLabel::native("Coordinate", "Koordinate")),
                ActionArgDef::select("operation", LocalizedLabel::native("List Operation", "Listenoperation"), vec![ActionArgOption::new("set",LocalizedLabel::native("Edit Item","Eintrag bearbeiten")),ActionArgOption::new("add",LocalizedLabel::native("Add Item","Eintrag hinzufügen")),ActionArgOption::new("remove",LocalizedLabel::native("Remove Item","Eintrag entfernen")),ActionArgOption::new("move",LocalizedLabel::native("Move Item","Eintrag verschieben"))]),
                ActionArgDef::number("index", LocalizedLabel::native("Item Index", "Eintragsindex")),
                ActionArgDef::number("destination", LocalizedLabel::native("Destination Index", "Zielindex")),
                ActionArgDef::text_list("path", LocalizedLabel::native("Mesh field", "Netzfeld")),
            ])
            // 📤️ One option per `document_io::EXPORT_FORMATS` row, in that table's order — asserted
            // equal to it by `export_document_action_offers_every_declared_format`, so a format this
            // artifact stops claiming cannot linger in the picker.
            .action_args("exportDocument", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), crate::standards::v1::subsets::any::io::document_io::export_format_options()).required().default_value(&"stl"),
                ActionArgDef::text("widgetId", LocalizedLabel::native("Export output", "Exportausgabe")),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH, LocalizedLabel::native("Mesh Workbench", "Netzwerkstatt")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🕹️ The generate preview is the SAME World3d surface as the edit preview, over the same
            // `graph` interaction domain and the same `preview_selection_json` (whose `transformMode`/
            // `gumballActive` pair is what `World3dHost` gates the gumball on) — it was simply never
            // handed the transform utility rail, so `ViewModel.active_utility_id` stayed empty there and
            // the gumball could never appear. A user could select a generated instance in generate mode
            // and then had no way to move it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #5).
            .window_kind_utilities(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            // 📇️ Window-scoped action ownership (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Each list is
            // exactly what THAT window's own surface dispatches — the node-graph host's verbs and the LOD
            // measure on the flow window, the world host's camera/gumball verbs and the show/sun measures on
            // the two 3D previews, the generation tree's four row verbs on the Generations window, the form's
            // one change verb on the Form window. Asserted by
            // `every_emitted_action_is_declared_on_its_window_kind`.
            //
            // Everything NOT listed here stays deliberately unowned and is therefore copied onto every window
            // by `build_definition` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`) —
            // that is the correct home for the app-scoped verbs no single window emits: the navbar's
            // `setActiveExample`, the catalogue palette's `addWidget`, the inspector's `patchFlowWidgets`, the
            // context menu's `reorganize`/`removeWidget`/`deleteSelection`, and the
            // framework's own history/clipboard/tutorial ids.
            // 🧭️ The five keyboard-traversal verbs are OWNED by the flow window, and that ownership is
            // their whole scope: `ShellHost`'s keybinding loop resolves a chord against the FOCUSED
            // window kind's own actions, so the bare arrow chords below are live exactly while the node
            // graph has focus and are inert in every other window — which is what lets them be bare
            // arrows at all. `activateSelection` is owned here for the same reason and because the flow
            // canvas's own `Trigger::Activate` binding dispatches it (`declaredAction` gate).
            .window_kind_action_refs(
                flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
                vec![
                    "nodeGraphEdit".into(),
                    "nodeGraphViewport".into(),
                    "setLodMode".into(),
                    "selectNextNode".into(),
                    "selectPreviousNode".into(),
                    "selectUpstreamNode".into(),
                    "selectDownstreamNode".into(),
                    "activateSelection".into(),
                ],
            )
            .window_kind_action_refs(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![
                "editMeshSelection".into(),
                "knifeMeshSelection".into(),
                "setCamera".into(),
                "setShowMode".into(),
                "toggleSun".into(),
                "setSunAzimuth".into(),
                "setSunElevation".into(),
                "setSunIntensity".into(),
                "translateSelection".into(),
                "rotateSelection".into(),
                "scaleSelection".into(),
            ])
            .window_kind_action_refs(generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS, vec!["addGeneration".into(), "selectGeneration".into(), "renameGeneration".into(), "removeGeneration".into()])
            .window_kind_action_refs(form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM, vec!["updateGenerationValues".into()])
            .window_kind_action_refs(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![
                "setCamera".into(),
                "setShowMode".into(),
                "toggleSun".into(),
                "setSunAzimuth".into(),
                "setSunElevation".into(),
                "setSunIntensity".into(),
                "translateSelection".into(),
                "rotateSelection".into(),
                "scaleSelection".into(),
            ])
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the flow-graph widget DAG, node/edge/handle granularities,
            // `HierarchyProvider::Topology` (see `Generation3dPlayApp::interaction_topology` below) —
            // transitive hover is the headline feature: hovering a Cluster group node highlights every
            // widget nested in its tree.
            .interaction(InteractionDefinition {
                id: "graph".into(),
                label: LocalizedLabel::native("Graph", "Graph"),
                granularities: vec![
                    GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                    GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                    GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "move".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: false,
                    broadcast: true,
                },
            })
            .interaction(InteractionDefinition {
                id: selection::DOMAIN.into(),
                label: LocalizedLabel::native("Geometry", "Geometrie"),
                granularities: vec![
                    GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
                    GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle".into() },
                    GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                    GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
                ],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle], merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive], transitive: false, broadcast: true },
            })
            .window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
            .window_kind_interactions(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph"), InteractionRef::new(selection::DOMAIN)])
            .window_kind_interactions(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
            .keybinding("mod+shift+m", "editMeshSelection")
            .keybinding("mod+shift+k", "knifeMeshSelection")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🗺️ `reorganize` had exactly ONE reachable trigger, the flow canvas's right-click menu —
            // mouse-only, and therefore unreachable by keyboard (`📓️audit-user-journey-gaps-2026-09-13.md`
            // §3 "every other one of the 30 editor commands"). `mod+alt+l` ("layout") is arg-free, so
            // `ShellHost`'s keybinding loop fires it straight through `onAction` with no staged form.
            .keybinding("mod+alt+l", "reorganize")
            // 📄️ The two IO verbs get the chords every application in the world uses for them.
            // `importDocumentRequest` is arg-free, so `ShellHost`'s keybinding loop fires it straight
            // through `onAction`; `exportDocument` carries a required `format`, so the same loop opens
            // its staged arg form rather than exporting a format nobody picked.
            .keybinding("mod+o", "importDocumentRequest")
            .keybinding("mod+shift+e", "exportDocument")
            // ⌨️ The graph / preview / generate verbs a user repeats, each on the chord its own verb
            // class already uses elsewhere in this repo: `delete,backspace` is the deleteSelection
            // chord three sibling artifacts declare, `mod+shift+g` follows the `mod+shift+<initial>` create
            // chords. Stopping a preview evaluation is the framework's `toolRunAbort` chord, injected with
            // the `previewEval` run, never a plugin binding. All four are ARG-FREE actions, which is what `ShellHost`'s keybinding loop requires to fire
            // an action straight through `onAction` instead of opening a staged form
            // (`📓️audit-user-journey-gaps-2026-09-13.md` §3, "every other one of the 30 editor
            // commands: MISSING keyboard path"). Framing the flow graph is the app action `zoomToFlow`,
            // not shell overlay chrome — declare it here once the command lands.
            .keybinding("delete,backspace", "deleteSelection")
            .keybinding("mod+shift+g", "addGeneration")
            .keybinding("mod+alt+d", "cycleShowMode")
            .keybinding("mod+alt+k", "cycleLodMode")
            // 🧭️ Node-by-node traversal of the graph canvas. BARE arrows, with no modifier, because
            // `role="application"` — which `accessibility_role` already implies for every
            // `Component::Surface` and which the React shell now paints on the canvas — is the ARIA
            // contract for "this widget handles its own arrow keys"; a modifier would break the promise
            // the canvas makes to a screen reader. They are safe as bare chords for two measured
            // reasons: all five verbs are owned by the flow window above, and `ShellHost`'s keybinding
            // loop resolves a chord against the FOCUSED window kind's actions only, so they are inert
            // anywhere else; and that loop returns early on `event.defaultPrevented` and on any
            // editable target, so a tree row or a text field that handles its own arrows keeps them.
            // ⌨️ The key token is the DOM `event.key` lowercased (`arrowdown`), never a shorthand:
            // `keyboardEventMatchesChord` compares the last `+` segment against `event.key` verbatim,
            // which is the same rule that made `mod+period` a silently dead chord.
            .keybinding("arrowdown", "selectNextNode")
            .keybinding("arrowup", "selectPreviousNode")
            .keybinding("arrowleft", "selectUpstreamNode")
            .keybinding("arrowright", "selectDownstreamNode")
            .config(Generation3dPlayApp::config_spec())
            .io(::semio_framework_async::poll::resolve_ready(generation3d_io()))
            .action_describe("addGeneration", LocalizedLabel::native("Adds a new generation, a named set of input values for the 3D generator, to the generation list and selects it.", "Fügt der Generationsliste eine neue Generation hinzu, einen benannten Satz von Eingabewerten für den 3D-Generator, und wählt sie aus."))
            .action_describe("selectGeneration", LocalizedLabel::native("Selects the generation with the given id, whose input values the generator then evaluates and shows.", "Wählt die Generation mit der angegebenen Id aus, deren Eingabewerte der Generator dann auswertet und zeigt."))
            .action_describe("renameGeneration", LocalizedLabel::native("Renames one generation of the generation list.", "Benennt eine Generation der Generationsliste um."))
            .action_describe("removeGeneration", LocalizedLabel::native("Removes one generation by id from the generation list, with its input values.", "Entfernt eine Generation anhand ihrer Id samt ihrer Eingabewerte aus der Generationsliste."))
            .action_describe("updateGenerationValues", LocalizedLabel::native("Sets the value one input question takes in a generation (the selected one when no id is given) and re-evaluates the result.", "Setzt den Wert, den eine Eingabefrage in einer Generation annimmt (ohne Id in der ausgewählten), und wertet das Ergebnis neu aus."))
            .action_describe("addWidget", LocalizedLabel::native("Adds a new widget of the given kind (an input or an operator of the 3D generator graph) to the canvas.", "Fügt der Fläche ein neues Widget der angegebenen Art hinzu (eine Eingabe oder einen Operator des 3D-Generatorgraphen)."))
            .action_describe("removeWidget", LocalizedLabel::native("Removes one widget by id from the generator graph together with its connections.", "Entfernt ein Widget anhand seiner Id samt seiner Verbindungen aus dem Generatorgraphen."))
            .action_describe("reorganize", LocalizedLabel::native("Lays out every widget of the generator graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Generatorgraphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."))
            .action_describe("setShowMode", LocalizedLabel::native("Sets what the editor shows, such as the generator graph or the generated result; only the view changes.", "Legt fest, was der Editor zeigt, etwa den Generatorgraphen oder das erzeugte Ergebnis; nur die Ansicht ändert sich."))
            .action_describe("editMeshSelection", LocalizedLabel::native("Edits the faces, edges, or vertices selected in the preview by inserting an adjustable mesh widget; downstream geometry and analysis follow the edit.", "Bearbeitet die in der Vorschau ausgewählten Flächen, Kanten oder Eckpunkte mit einem einstellbaren Netz-Widget; nachgelagerte Geometrie und Analyse folgen der Änderung."))
            .action_describe("knifeMeshSelection", LocalizedLabel::native("Cuts one selected mesh face along the line through two points. The cut remains editable and downstream measurements update.", "Schneidet eine ausgewählte Netzfläche entlang der Geraden durch zwei Punkte. Der Schnitt bleibt einstellbar und nachgelagerte Messungen werden aktualisiert."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 3D generator with one of the plugin's bundled examples, by example id.", "Ersetzt den gesamten 3D-Generator durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the 3D preview draws the generated result with; only the view changes.", "Legt die Detailstufe fest, mit der die 3D-Vorschau das erzeugte Ergebnis zeichnet; nur die Ansicht ändert sich."))
            .action_describe("selectNextNode", LocalizedLabel::native("Moves the graph selection to the next widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum nächsten Widget."))
            .action_describe("selectPreviousNode", LocalizedLabel::native("Moves the graph selection to the previous widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum vorherigen Widget."))
            .action_describe("selectUpstreamNode", LocalizedLabel::native("Moves the graph selection to a widget feeding the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das das ausgewählte speist."))
            .action_describe("selectDownstreamNode", LocalizedLabel::native("Moves the graph selection to a widget fed by the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das vom ausgewählten gespeist wird."))
            .action_describe("activateSelection", LocalizedLabel::native("Opens the ports of the selected widget in the graph window, as pressing Enter on it does; only the view changes.", "Öffnet die Ports des ausgewählten Widgets im Graphfenster, wie ein Druck auf die Eingabetaste; nur die Ansicht ändert sich."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the sun light of the 3D preview on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Vorschau ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D preview's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Vorschau scheint; nur die Ansicht ändert sich."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D preview's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Vorschau über dem Horizont steht; nur die Ansicht ändert sich."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D preview's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Vorschau fest; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the given or selected generated objects by dx, dy and dz through a translate transform in the generator graph; one gumball drag is one undo step.", "Verschiebt die angegebenen oder ausgewählten erzeugten Objekte über eine Verschiebe-Transformation im Generatorgraphen um dx, dy und dz; ein Gumball-Zug ist ein Rückgängig-Schritt."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the given or selected generated objects around an axis by an angle through a rotate transform in the generator graph.", "Dreht die angegebenen oder ausgewählten erzeugten Objekte über eine Dreh-Transformation im Generatorgraphen um eine Achse und einen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the given or selected generated objects by per-axis factors through a scale transform in the generator graph.", "Skaliert die angegebenen oder ausgewählten erzeugten Objekte über eine Skalier-Transformation im Generatorgraphen um Faktoren je Achse."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected widget from the generator graph together with its connections.", "Löscht alle ausgewählten Widgets samt ihrer Verbindungen aus dem Generatorgraphen."))
            .action_describe("patchFlowWidgets", LocalizedLabel::native("Sets one numeric field (such as a slider value) on several widgets at once; one drag of the control is one undo step.", "Setzt ein Zahlenfeld (etwa einen Schiebereglerwert) auf mehreren Widgets zugleich; ein Zug des Bedienelements ist ein Rückgängig-Schritt."))
            .action_describe("importDocumentRequest", LocalizedLabel::native("Opens the host's file picker for a 3D artifact file; the chosen file is then imported as the generator document.", "Öffnet die Dateiauswahl des Hosts für eine 3D-Artefaktdatei; die gewählte Datei wird dann als Generatordokument importiert."))
            .action_describe("exportDocument", LocalizedLabel::native("Writes the generated 3D result in the chosen format to a downloaded file on the user's machine.", "Schreibt das erzeugte 3D-Ergebnis im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
            .action_describe("importDocument", LocalizedLabel::native("Replaces the generator document with one read from an imported 3D artifact file.", "Ersetzt das Generatordokument durch eines aus einer importierten 3D-Artefaktdatei."))
            .action_describe("cycleShowMode", LocalizedLabel::native("Switches the editor to the next show mode in turn (such as graph, result or wireframe); only the view changes.", "Wechselt den Editor reihum in den nächsten Anzeigemodus (etwa Graph, Ergebnis oder Drahtgitter); nur die Ansicht ändert sich."))
            .action_describe("cycleLodMode", LocalizedLabel::native("Switches the 3D preview to the next level of detail in turn; only the view changes.", "Wechselt die 3D-Vorschau reihum zur nächsten Detailstufe; nur die Ansicht ändert sich."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("reorganize")
            .action_destructive("importDocument")
            .build_definition()
}

/// 📚️ The eight bundled `📚️examples/🎬️<slug>` fixtures, in `schema::is_generation3d_example_id`'s
/// order. `Generation3dPlayApp::examples` returns this catalogue, and `.editor` stamps it onto
/// the manifest the navbar dropdown reads.
///
/// 🚫️ `✏️editor/📚️examples/🎬️demo-session` is deliberately NOT here, and its absence is a statement,
/// not an oversight (`📓️audit-user-journey-gaps-2026-09-13.md` §9 item 12 read it as one). That leaf
/// carries `.cmd.semio` command-REPLAY text, and `setActiveExample`'s only vocabulary is "load a
/// registered example document" — `🖨️raster` states the same rule for the same leaf, and `🧩️puzzle`
/// mounts and tests three of them without registering any. Held by
/// `the_example_picker_offers_the_flow_examples_and_never_the_command_session`.
pub fn examples() -> Vec<ExampleSource> {
    vec![
        crate::examples::art_generation3d_hexagonal_mushroom_column::source(),
        crate::examples::art_generation3d_mesh_workbench::source(),
        crate::examples::art_generation3d_rectangle_extrude_volume::source(),
        crate::examples::art_generation3d_sphere_cut_with_torus::source(),
        crate::examples::art_generation3d_box_fillet_preview::source(),
        crate::examples::art_generation3d_sphere_box_fuse::source(),
        crate::examples::art_generation3d_face_sweep_extrude::source(),
        crate::examples::art_generation3d_rectangle_wire_preview::source(),
        crate::examples::art_generation3d_box_shell_preview::source(),
    ]
}
//#endregion 🔖️Manifest

//#region 🔖️ArtifactIo
/// 🔌️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// this app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_generation3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub async fn generation3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_artifact(
        "generation.3d",
        MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.generation".into(), name: "3D Generation".into(), dimension: "3d".into(), component_kind: "generation3d".into() },
    )
    .await
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "geometry:out".into(),
            label: "Geometry".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            kind_id: Some("3d.mesh".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

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
pub use crate::preview_eval::{
    apply_show_mode_mesh, decode_preview_mesh_pack, geometry_extension_address, is_brep_geometry_handle, mesh_data_for_preview_handle, mesh_data_for_session_preview_channel, mesh_has_preview_geometry, pending_preview_tessellate_handles, point_marker_mesh, preview_channel_items_for_widget,
    preview_mesh_role, preview_tolerance, vector_marker_mesh, PreviewChannelItem, PreviewInlineGeometry, GENERATION_3D_GEOMETRY_EXTENSION_ID, PREVIEW_MESH_ROLES, PREVIEW_TESSELLATE_STEP_BUDGET,
};

/// 📨 Extension invocations that tessellate this surface's pending preview handles — the editor's
/// binding of [`crate::preview_eval::preview_tessellate_invocations`], which reads the deflection
/// off this surface's own config LOD.
pub fn preview_tessellate_invocations(window_id: &str, window_kind_id: &str, session: &mut FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig) -> Vec<semio_framework_plugin::ExtensionInvocation> {
    crate::preview_eval::preview_tessellate_invocations(window_id, window_kind_id, session, host_snapshot, preview_tolerance(&cfg.lod_mode))
}

pub fn preview_camera_json(cfg: &Generation3dConfig) -> String {
    semio_framework_ui::wgpu::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}

/// 🗂️ The context-menu group the two artifact round-trips (`Import Artifact…`, `Export Artifact`)
/// live under.
///
/// A `Menu::group(category)` row travels to the shell with `label: None` BY CONTRACT and the shell
/// resolves it from the closed 20-id ribbon-parent taxonomy (`RIBBON_PARENT_CATEGORIES`,
/// `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`). A category outside that table has no label to resolve
/// to and falls back to the raw id, so this menu used to show a user a row literally reading
/// `menu.group.io` in both locales — measured on the React serve 2026-09-14, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END. `transfer` is the taxonomy's own name for moving a document in
/// and out; asserted against the table by `context_menu_groups_are_taxonomy_categories`.
const CONTEXT_MENU_TRANSFER_CATEGORY: &str = "transfer";

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

    /// 🕸️ `(nodeId, portId)` the node graph paints as hovered — a preview instance hovered in the
    /// world resolves to its node AND its channel, so the graph highlights the exact output port
    /// whose geometry the pointer is over.
    pub fn hovered_graph_target(&self) -> Option<(String, Option<String>)> {
        let id = self.hovered.iter().next()?;
        Some((Self::widget_of(id).to_string(), Self::port_of(id).map(str::to_string)))
    }

    /// 🕸️ Every id the node graph should highlight: each hovered id plus the widget it belongs to,
    /// deduplicated and ordered.
    pub fn graph_highlight_ids(&self) -> Vec<String> {
        let mut ids: std::collections::BTreeSet<String> = self.hovered.clone();
        for id in &self.hovered {
            ids.insert(Self::widget_of(id).to_string());
        }
        ids.into_iter().collect()
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
    let mut value = semio_framework_pack_json::parse(&semio_framework_plugin::world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref()), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
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
pub use crate::preview_eval::{preview_progress_status_json, preview_progress_status_json_for, preview_scene_status_json, preview_status_json, preview_window_status_json, PreviewStatusDebug};

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
    if eval_json.is_empty() {
        return PreviewPayload::default();
    }
    if let Ok(parsed) = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        if parsed.get("error").and_then(semio_framework_pack_json::Value::as_str).is_some() {
            return PreviewPayload::default();
        }
    }
    let eval = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<semio_framework_pack_json::Value> = Vec::new();
    let mut instances: Vec<semio_framework_pack_json::Value> = Vec::new();
    // 🔁️ Dedup key is the brep HANDLE, not the widget/channel that emitted it: two channels (even
    // on different widgets) that resolve to the same handle share one tessellated mesh entry and
    // still each get their own instance — see the mesh-id lookup below.
    let mut mesh_id_by_handle: HashMap<String, String> = HashMap::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    let component_selected: std::collections::BTreeSet<&str> = marks.components.selected.iter().filter_map(|id| selection::ComponentTarget::parse(id)).filter(|target| target.granularity == marks.components.granularity).map(|target| target.instance).collect();
    let component_hovered = marks.components.hovered.as_deref().and_then(selection::ComponentTarget::parse).filter(|target| target.granularity == marks.components.granularity);
    let component_group = selection::component_group(&marks.components.selected).ok().filter(|(target, _)| target.index == 0 && target.granularity == marks.components.granularity);
    let mut component_pivot = None;
    for widget in &host_snapshot.widgets {
        let id = crate::widget_id(widget).to_string();
        let preview = crate::widget_previews(widget);
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
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(handle).cloned().unwrap_or(own_mesh_id) };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let data = match inline {
                    Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(*x, *y, *z)),
                    Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(vector_marker_mesh(*x, *y, *z)),
                    Some(PreviewInlineGeometry::Mesh { preview, .. }) => decode_preview_mesh_pack(preview),
                    None => session
                        .map(|session| mesh_data_for_session_preview_channel(handle, &id, channel, *index, tolerance, session, host_snapshot))
                        .unwrap_or_else(|| mesh_data_for_preview_handle(handle, tolerance, session)),
                };
                if let Some(data) = data {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        let mut mesh_object = semio_framework_pack_json::Object::new();
                        mesh_object.insert("id", semio_framework_pack_json::Value::String(mesh_id.clone()));
                        mesh_object.insert("role", semio_framework_pack_json::Value::String(preview_mesh_role(inline.as_ref(), &data).to_string()));
                        mesh_object.insert("data", semio_framework_pack_json::Value::from(data));
                        meshes.push(semio_framework_pack_json::Value::Object(mesh_object));
                        if !handle.is_empty() {
                            mesh_id_by_handle.insert(handle.clone(), mesh_id.clone());
                        }
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let (Some((target, ids)), Some(PreviewInlineGeometry::Mesh { data, .. })) = (&component_group, inline) {
                    if target.instance == instance_id { component_pivot = selection::component_pivot(data, target.granularity, ids); }
                }
                let selected = if marks.components.active() { component_selected.contains(instance_id.as_str()) } else { marks.selects(&id, channel, *index) };
                let hovered = if marks.components.active() { component_hovered.as_ref().is_some_and(|target| target.instance == instance_id) } else { marks.hovers(&id, channel, *index) };
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
                if let Some(revision) = marks.source_revision.as_ref().filter(|_| inline.is_none() && !handle.is_empty()) {
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
    }
    PreviewPayload { components: marks.components.clone(), component_pivot, meshes_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(meshes)), instances_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(instances)), selected_ids, hovered_id }
}
//#endregion 🔖️PreviewPipeline

//#region 🔖️MeshBridge
/// 👁️ Prepares geometry from the retained evaluation and delivered mesh packs.
pub fn export_mesh_from_session(snapshot: &Generation3dSnapshot, cfg: &Generation3dConfig, session: &FlowEvalSession) -> Result<semio_framework_plugin::MeshData, semio_framework_diagnostic::TextError> {
    crate::standards::v1::subsets::any::io::mesh_bridge::merge_meshes(&export_meshes_from_session(snapshot, cfg, session, None)?)
}

/// 🎨️ Retains each prepared surface and its authored channel set for format diagnostics and export.
pub fn export_meshes_from_session(snapshot: &Generation3dSnapshot, _cfg: &Generation3dConfig, session: &FlowEvalSession, widget_id: Option<&str>) -> Result<Vec<semio_framework_plugin::MeshData>, semio_framework_diagnostic::TextError> {
    export_meshes_from_evaluation(&snapshot.host_snapshot, session, widget_id)
}

/// 🪪️ Reads exact retained channel packs without display filtering or another kernel evaluation.
pub fn export_meshes_from_evaluation(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession, widget_id: Option<&str>) -> Result<Vec<semio_framework_plugin::MeshData>, semio_framework_diagnostic::TextError> {
    use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
    if session.preview_chain_status().working || session.preview_eval_status().in_flight > 0 || session.preview_tessellate_status().in_flight > 0 { return Err(io_error("generation3d export is waiting for the current geometry evaluation")); }
    let sources = crate::standards::v1::subsets::any::io::mesh_bridge::export_source_channels(snapshot, widget_id)?;
    let eval = semio_framework_pack_json::parse(session.eval_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| io_error(error.to_string()))?;
    if let Some(error) = eval.get("error").and_then(semio_framework_pack_json::Value::as_str) { return Err(io_error(error)); }
    let mut meshes = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (source, channel) in sources {
        for item in preview_channel_items_for_widget(&eval, &source).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)) {
            let key = if item.handle.is_empty() { format!("{source}@{}#{}", item.channel, item.index) } else { item.handle.clone() };
            if !seen.insert(key) { continue; }
            let mesh = match item.inline {
                Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(x, y, z)),
                Some(PreviewInlineGeometry::Vector { .. }) => continue,
                Some(PreviewInlineGeometry::Mesh { preview, .. }) => decode_preview_mesh_pack(&preview),
                None => crate::preview_eval::session_preview_mesh(&item.handle, session),
            }.ok_or_else(|| io_error(format!("generation3d export geometry '{source}@{}' is unavailable", item.channel)))?;
            meshes.push(mesh);
        }
    }
    if meshes.is_empty() { return Err(io_error("generation3d export has no connected geometry")); }
    Ok(meshes)
}

//#endregion 🔖️MeshBridge


#[cfg(test)]
#[path = "🧪️tests/🔬️fold-contract/🦀️.rs"]
mod fold_contract;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;

#[cfg(test)]
#[path = "🎮️commands/🔪️knife-mesh-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod knife_selection_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️work-capacity/🦀️.rs"]
mod work_capacity;

/// 🧬️ The whole-document replacement that picking an example or importing a file emits: the artifact's load effect, outside undo history and
/// never a mutation row. `store::empty_document_spr` (never a minted `create_document_envelope`) keeps the guest off the
/// `terminal shell reached Drop before its app-owned bounded retirement authority detached` trap on this path.
pub fn reset_generation3d_document_effect(document: &Generation3dSnapshot) -> semio_framework_plugin::Effect {
    let pack = <Generation3dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("generation3d", crate::GENERATION_3D_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
