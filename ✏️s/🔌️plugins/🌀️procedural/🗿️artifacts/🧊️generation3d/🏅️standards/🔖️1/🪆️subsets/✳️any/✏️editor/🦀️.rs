//! 🧱️ Generation3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_procedural_3d_navigate_graph::activate_selection;
use semio_s_artifact_procedural_3d_navigate_graph::select_downstream_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_next_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_previous_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_upstream_node;
use semio_s_artifact_procedural_3d_editor_commands::add_generation;
use semio_s_artifact_procedural_3d_add_widget as add_widget;
use semio_s_artifact_procedural_3d_editor_commands::cycle_lod_mode;
use semio_s_artifact_procedural_3d_editor_commands::cycle_show_mode;
use semio_s_artifact_procedural_3d_delete_selection as delete_selection;
use semio_s_artifact_procedural_3d_export_document as export_document;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_import_document as import_document;
use semio_s_artifact_procedural_3d_editor_commands::import_document_request;
use semio_s_artifact_procedural_3d_node_graph_edit as node_graph_edit;
use semio_s_artifact_procedural_3d_editor_commands::node_graph_viewport;
use semio_s_artifact_procedural_3d_editor_commands::patch_flow_widgets;
use semio_s_artifact_procedural_3d_editor_commands::remove_generation;
use semio_s_artifact_procedural_3d_editor_commands::remove_widget;
use semio_s_artifact_procedural_3d_editor_commands::rename_generation;
use semio_s_artifact_procedural_3d_editor_commands::reorganize;
use semio_s_artifact_procedural_3d_rotate_selection as rotate_selection;
use semio_s_artifact_procedural_3d_scale_selection as scale_selection;
use semio_s_artifact_procedural_3d_editor_commands::select_generation;
use semio_s_artifact_procedural_3d_editor_commands::set_active_example;
use semio_s_artifact_procedural_3d_editor_commands::set_camera;
use semio_s_artifact_procedural_3d_set_contributions as set_contributions;
use semio_s_artifact_procedural_3d_editor_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_editor_commands::set_show_mode;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_editor_commands::toggle_sun;
use semio_s_artifact_procedural_3d_editor_commands::translate_selection;
use semio_s_artifact_procedural_3d_editor_commands::update_generation_values;
use semio_s_artifact_procedural_3d_set_widget_input as set_widget_input;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mode_edit::windows::flow as flow_window;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview as edit_preview;
use semio_s_artifact_procedural_3d_mode_generate::windows::form;
use semio_s_artifact_procedural_3d_mode_generate::windows::generations;
use semio_s_artifact_procedural_3d_mode_generate::windows::preview as generate_preview;
use semio_s_artifact_procedural_3d_mode_edit as edit;
use semio_s_artifact_procedural_3d_mode_generate as generate;
use semio_s_artifact_procedural_3d_editor_panels::catalogue as catalogue_panel;
use semio_s_artifact_procedural_3d_editor_panels::artifact as artifact_panel;
use semio_s_artifact_procedural_3d_editor_panels::inspection as inspection_panel;
use semio_s_artifact_procedural_3d_terminology::generation3d_labels;
use semio_s_artifact_procedural_3d_editor_transient::Generation3dTransient;
use semio_s_artifact_procedural_3d_editor_transient::Generation3dTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::artifact_kind;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
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
use semio_s_artifact_procedural_3d_model::{GENERATION3D_EDITOR_APP_ID, generation3d_action};
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
use semio_s_artifact_procedural_3d_export_document::export_mesh_from_session;
use semio_s_artifact_procedural_3d_export_document::export_meshes_from_session;
use semio_s_artifact_procedural_3d_import_document::GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES;


//#region 🔖️Constants


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
        "editMeshSelection" as "edit-mesh-selection" => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::EditMeshSelection,
        "knifeMeshSelection" as "knife-mesh-selection" => semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::KnifeMeshSelection,
        "setWidgetInput" as "set-widget-input" => set_widget_input::SetWidgetInput}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

pub fn generation_command_result_for(command: &Generation3dCommand, projection: &Generation3dSnapshot, config: &Generation3dConfig) -> Option<Generation3dGenerationCommandResult> {
    match command {
        Generation3dCommand::AddGeneration(_) => generation_command_result("addGeneration", None, projection, config),
        Generation3dCommand::RemoveGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]);
            generation_command_result("removeGeneration", Some(&args), projection, config)
        }
        Generation3dCommand::RenameGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".to_string(), semio_framework_value::DslValue::String(payload.name.clone()))]);
            generation_command_result("renameGeneration", Some(&args), projection, config)
        }
        Generation3dCommand::UpdateGenerationValues(payload) => {
            let generation_id = payload.generation_id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String);
            let args = semio_framework_value::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), semio_framework_value::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
            generation_command_result("updateGenerationValues", Some(&args), projection, config)
        }
        Generation3dCommand::SelectGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]);
            generation_command_result("selectGeneration", Some(&args), projection, config)
        }
        _ => None,
    }
}

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
    run_link: semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunLink,
    /// 🛠️ Every window's open gumball gesture — ephemeral local tool state, never history; the previews fold it in.
    gumball: semio_s_artifact_procedural_3d_transforms::GumballGestures,
    /// 📐️ The instance's geometry engine: the typed inference cache, the retained base and the stepped run of the `s.procedural.generation3d.inference` geometry service.
    geometry: semio_s_artifact_procedural_3d_geometry_service::GeometryHost,
    closing: bool,
}

impl Generation3dInstanceOperationOwner {
    pub fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), run_link: semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunLink::default(), gumball: semio_s_artifact_procedural_3d_transforms::GumballGestures::default(), geometry: Default::default(), closing: false }
    }

    /// 📐️ The geometry host a run of the contextual `s.procedural.generation3d.inference` geometry service executes against.
    pub fn geometry(&self) -> &semio_s_artifact_procedural_3d_geometry_service::GeometryHost {
        &self.geometry
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-eval-session-owner-missing"))
    }

    /// 🛠️ The retained session together with the open gumball gestures — what a gesture command folds into.
    fn with_session_and_gumball<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession, &mut semio_s_artifact_procedural_3d_transforms::GumballGestures) -> R) -> Result<R, Fault> {
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
    fn owe_attached_previews_for_mutations(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<bool, Fault> {
        let Self { eval_session, run_link, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        semio_s_artifact_procedural_3d_preview_eval::owe_attached_previews_for_mutations(session, run_link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }

    /// 🚦️ Owes the attached previews an evaluation and puts the run start that debt needs on the
    /// gesture's own emit — what a route that owes unconditionally (a generation command, an example
    /// switch, a contributions install, a closing import chunk) hands back.
    fn owe_attached_previews_carrying(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
        let Self { eval_session, run_link, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        semio_s_artifact_procedural_3d_preview_eval::owe_attached_previews_carrying(session, run_link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }
}

impl semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunOwner for Generation3dInstanceOperationOwner {
    fn preview_eval_parts(&mut self) -> Option<(&mut FlowEvalSession, &mut semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunLink)> {
        let Self { eval_session, run_link, closing, .. } = self;
        eval_session.as_mut().filter(|_| !*closing).map(|session| (session, run_link))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation3dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(Default::default());
        }
        let Some(session) = self.eval_session.as_ref() else { return Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.geometry.terminal_is_empty()), ..Default::default() }) };
        if session.terminal_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: session.next_close_copy_byte_demand()?, capacity_bytes: session.next_close_capacity_byte_demand(body)?, release_bytes: session.next_close_release_byte_demand()?, depth: session.next_close_depth_demand()? })
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spent the runtime's
    /// zero-progress credit every idle turn (26/09/09/PROCEDURAL-3D-END-TO-END, close-ladder lane).
    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        use semio_framework_plugin::PluginLifecycleStep;
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !self.closing {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let Some(session) = self.eval_session.as_mut() else {
            if self.geometry.terminal_is_empty() {
                return Ok(PluginLifecycleStep::Complete(Default::default()));
            }
            return Ok(PluginLifecycleStep::Blocked { reason: "Generation3d geometry engine requires its retained close producer" });
        };
        let (progress, finished) = if session.terminal_is_empty() {
            (RetainedCloneProgress { copied_items: 1, ..Default::default() }, true)
        } else {
            match session.close_step(grant)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => (progress, false),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => (progress, true),
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(PluginLifecycleStep::Blocked { reason: "Generation3d evaluation session awaits its exact close grant" }),
            }
        };
        if finished && session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(if self.terminal_is_empty() { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        self.closing = true;
        if let Some(session) = self.eval_session.as_mut() {
            session.begin_close();
        }
        self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none() && self.geometry.terminal_is_empty()
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
    session.retire_cold();
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
fn parse_preview_camera_json(args: &semio_framework_value::DslValue) -> semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = <semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera as semio_framework_value::FromValue>::from_value(camera.clone()) {
            return parsed;
        }
    }
    semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera::default()
}

/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`), read from the SAME
/// `dag_host_snapshot_to_workflow` projection the node-graph window paints — so an interaction target and a
/// graph pick can never drift apart.
pub fn generation3d_port_ids_by_node(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| semio_s_artifact_procedural_3d_preview_eval::dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
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
            let evaluation = session.eval_json();
            let selected = marks.graph_selection_ids();
            let export_widget = selected.first().filter(|id| document.host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::OutputExport { id: target, .. } if target == *id)));
            let exported = match export_widget { Some(widget) => export_meshes_from_session(document, config, session, Some(widget.as_str())), None => Ok(Vec::new()) };
            let export_ready = exported.is_ok();
            let meshes = exported.unwrap_or_default();
            inspection_panel::render(&document.host_snapshot, &marks.graph_selection_ids(), labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION), view_state.locale, view_state.terminology, &meshes, evaluation, session.status_json(), export_ready,marks.source_revision.as_deref())
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
pub const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &[
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
pub const GENERATION3D_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"];
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
pub const GENERATION3D_DOCUMENT_IO_TOOL_IDS: &[&str] = &["importDocumentRequest", "importDocument", "exportDocument"];
const GENERATION3D_DOCUMENT_IO_PAYLOAD_SCHEMA: &str = "generation.3d.document-io-command.v1";
/// 🎒️ Real bound for one document-IO hop's wire payload: one whole import
/// ([`import_document::GENERATION3D_IMPORT_TOTAL_BYTES`]) plus the addressed envelope and the command
/// id — the whole public-invocation body the host may send, pinned to
/// `PUBLIC_INVOCATION_BODY_BYTES` by `document_io_route_declares_a_reachable_wire_ceiling` rather
/// than guessed.
pub const GENERATION3D_DOCUMENT_IO_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;
const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";
const GENERATION3D_FLOW_EVAL_PAYLOAD_SCHEMA: &str = "generation.3d.flow-eval-command.v1";
pub const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;
/// 🎒️ Real bound for one preview-chain hop's wire payload: a `flowTessellateResolve` carries one
/// whole `tessellate` envelope, i.e. one [`MESH_PACK_CHUNK_BASE64_CHARS`] mesh-body chunk plus its
/// accounting header — pinned to `brep_geometry::tessellate_envelope_maximum_bytes()` by
/// `tessellate_envelope_fits_the_declared_wire_bound`, so the transfer unit and this bound are one
/// fact. 64 KiB is the guest-contiguous page the answer already crosses in, never a rubber stamp.
pub const GENERATION3D_FLOW_EVAL_RAW_BYTES: usize = 65_536;
pub const GENERATION3D_PREVIEW_TOOL_IDS: &[&str] = &["setActiveExample", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];
/// 🧮️ The ONE declared quantity of every generation3d retained route: at most 32 point-invertible
/// durable items per gesture. Everything the runtime compares is read off it and nothing is ever
/// spelled again — the preflight ceiling ([`GENERATION3D_RETAINED_WORK_ITEMS`], handed to
/// `ArtifactRetainedCommandPayload::new`), the extent every `ArtifactCommandWork` answers, the
/// one-item store footprint both durable preflights declare, and the bounded proof's work units.
/// They used to be three literals in two units — a two-row footprint, a one-ITEM extent and a
/// 32-ITEM ceiling — so nothing could compare them and only the store ever measured rows
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const GENERATION3D_RETAINED_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);
pub const GENERATION3D_RETAINED_WORK_ITEMS: usize = GENERATION3D_RETAINED_CAPACITY.work_items();

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

pub fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GENERATION3D_RETAINED_CAPACITY.rows_for_items(1)
}

pub struct Generation3dPreviewCommandWork {
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
    emit_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>>>,
    started: bool,
    complete: bool,
    closing: bool,
}

impl Generation3dPreviewCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, emit: None, host: None, host_retirement: None, session: None, emit_retirement: None, started: false, complete: false, closing: false }
    }

    /// 🔒️ Owes every attached preview window an evaluation on the RETAINED session's latch, so two
    /// gestures in a row still leave exactly one debt per window for the run to pay.
    fn owe_attached_previews(&self, view: Option<&semio_framework_plugin::ViewModel>, servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
        let windows = generation3d_preview_windows(view);
        self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.owe_attached_previews_carrying(&windows, servable, emit, grant))
    }

    fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::RetirementDemand;
        let leaf = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(session) = self.session.as_ref() {
            return if session.terminal_is_empty() { Ok(leaf) } else { Ok(RetirementDemand { copy_bytes: session.next_close_copy_byte_demand()?, capacity_bytes: session.next_close_capacity_byte_demand(copy)?, release_bytes: session.next_close_release_byte_demand()?, depth: session.next_close_depth_demand()? }) };
        }
        if self.host.is_some() || self.emit.is_some() {
            return Ok(leaf);
        }
        if let Some(retirement) = self.host_retirement.as_ref() {
            return if retirement.terminal_is_empty() { Ok(leaf) } else { Ok(RetirementDemand { copy_bytes: retirement.next_close_copy_byte_demand()?, capacity_bytes: retirement.next_close_capacity_byte_demand(copy)?, release_bytes: retirement.next_close_release_byte_demand()?, depth: retirement.next_close_depth_demand()? }) };
        }
        if let Some(retirement) = self.emit_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retirement.next_capacity_byte_demand(copy)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()? });
        }
        Ok(Default::default())
    }

    fn complete(&mut self) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        self.complete = true;
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-preview-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

/// 🧮️ The transient bytes one work step may materialize: the work's own frame plus the one emit it can stage.
fn command_work_step_demands<A: semio_framework_plugin::ArtifactApp>(work_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    Ok(semio_framework_value::RetirementDemand { copy_bytes: work_bytes.saturating_add(size_of::<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>()), depth: 1, ..Default::default() })
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
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
                    self.owe_attached_previews(input.context.and_then(|context| context.view_state.as_ref()), servable, &mut emit, input.operation.retained)?;
                }
                self.emit = Some(emit);
                return self.complete();
            }
            let result = generation_command_result_for(input.command, input.snapshot, input.config).ok_or_else(|| Fault::from("generation3d-preview-command-unowned"))?;
            let mut emit = result.emit;
            self.owe_attached_previews(input.context.and_then(|context| context.view_state.as_ref()), servable, &mut emit, input.operation.retained)?;
            if let Some(fixture) = result.preview_fixture {
                fixture.retire_cold();
            }
            self.emit = Some(emit);
            return self.complete();
        }
        Err(Fault::from("generation3d-preview-work-is-terminal"))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{(
            ||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let refused = |error: semio_framework_value::ValueError| Err(error);
        let idle = RetainedCloneProgress::default();
        let one = RetainedCloneProgress { copied_items: 1, ..idle };
        if !self.closing {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(InteractiveJobCloseStep::Pending { progress: idle });
        }
        if let Some(session) = self.session.as_mut() {
            if session.terminal_is_empty() {
                self.session.take();
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            return session.close_step(grant);
        }
        if let Some(host) = self.host.take() {
            self.host_retirement = Some(semio_framework_os_flow::FlowHostRetirement::new(host));
            return Ok(InteractiveJobCloseStep::Pending { progress: one });
        }
        if let Some(retirement) = self.host_retirement.as_mut() {
            if retirement.terminal_is_empty() {
                self.host_retirement = None;
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            return match retirement.close_step(grant) {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => Ok(InteractiveJobCloseStep::Pending { progress }),
                Err(error) => refused(error),
            };
        }
        if let Some(emit) = self.emit.take() {
            return match semio_framework_value::retirement::controlled::ControlledRetirement::new(emit) {
                Ok(owner) => {
                    self.emit_retirement = Some(owner);
                    Ok(InteractiveJobCloseStep::Pending { progress: one })
                }
                Err((error, emit)) => {
                    self.emit = Some(emit);
                    refused(error)
                }
            };
        }
        if let Some(retirement) = self.emit_retirement.as_mut() {
            let step = retirement.step(grant);
            if retirement.terminal_is_empty() {
                self.emit_retirement = None;
            }
            return match step {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => Ok(InteractiveJobCloseStep::Pending { progress }),
                Err(error) => refused(error),
            };
        }
        Ok(InteractiveJobCloseStep::Complete { progress: idle })
    })().map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }


    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.emit.is_none() && self.emit_retirement.is_none() && self.host.is_none() && self.host_retirement.is_none() && self.session.is_none()
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
    tessellate: preview_eval::PreviewTessellateReceive,
    evaluation:preview_eval::PreviewEvalReceive,
    complete: bool,
    closing: bool,
}

impl Generation3dFlowEvalWindowWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self { Self { tool_id, instance_owner, tessellate: preview_eval::PreviewTessellateReceive::new(),evaluation:preview_eval::PreviewEvalReceive::new(), complete: false, closing: false } }
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

    fn receive_original_command(&mut self,command:&mut Generation3dCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Generation3dCommand::FlowTessellateResolve(payload)=command{self.tessellate.configure_scope([Some(flow_eval_tick::preview_body_key(&payload.window_kind_id)),Some(semio_s_artifact_procedural_3d_mode_edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN)],[Some(semio_s_artifact_procedural_3d_editor_panels::artifact::GENERATION_3D_PLAY_BODY_ARTIFACT),Some(semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY)]);return self.tessellate.receive(payload,cx)}if let Generation3dCommand::FlowEvalResolve(payload)=command{self.evaluation.configure_scope([Some(flow_eval_tick::preview_body_key(&payload.window_kind_id)),Some(semio_s_artifact_procedural_3d_mode_edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN)],[Some(semio_s_artifact_procedural_3d_editor_panels::artifact::GENERATION_3D_PLAY_BODY_ARTIFACT),Some(semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY)]);return self.evaluation.receive(payload,cx)}Ok(true)
    }

    fn work_demands(&self,input:&ArtifactCommandInputs<'_,EditorApp<Generation3dPlayApp>>,maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Generation3dCommand::FlowTessellateResolve(payload)=input.command{return self.instance_owner.inspect_original::<Generation3dInstanceOperationOwner,_>(|owner|owner.eval_session.as_ref().map_or_else(||Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview session is closing")),|session|self.tessellate.demands(payload,session,maximum_copy_bytes)))?.unwrap_or(Ok(Default::default()))}
        if let Generation3dCommand::FlowEvalResolve(payload)=input.command{return self.instance_owner.inspect_original::<Generation3dInstanceOperationOwner,_>(|owner|owner.eval_session.as_ref().map_or_else(||Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview session is closing")),|session|self.evaluation.demands(payload,session,maximum_copy_bytes)))?.unwrap_or(Ok(Default::default()))}
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Generation3dCommand::FlowTessellateResolve(payload)=input.command{
            if self.complete||self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview command work is terminal").into())}
            if self.tessellate.is_folded(){if let Some(ui_scope)=self.tessellate.publish_scope(cx)?{self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(Emit{ui_scope,..Default::default()}))}}else{self.instance_owner.with_mut::<Generation3dInstanceOperationOwner,_>(|owner|owner.with_session_waking(|session|self.tessellate.step(payload,session,cx)))??;}
            return Ok(ArtifactCommandWorkStep::Progress{stage:"previewReceive",preview:br#"{"en":"Receiving preview geometry","de":"Vorschaugeometrie wird empfangen"}"#})
        }
        if let Generation3dCommand::FlowEvalResolve(payload)=input.command{
            if self.complete||self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview command work is terminal").into())}
            if self.evaluation.is_folded(){if let Some(ui_scope)=self.evaluation.publish_scope(cx)?{self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(Emit{ui_scope,..Default::default()}))}}else{self.instance_owner.with_mut::<Generation3dInstanceOperationOwner,_>(|owner|owner.with_session_waking(|session|self.evaluation.step(payload,session,cx)))??;}
            return Ok(ArtifactCommandWorkStep::Progress{stage:"previewReceive",preview:br#"{"en":"Receiving evaluation output","de":"Auswertungsergebnis wird empfangen"}"#})
        }
        let original: Result<_, Fault> = (|| {
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
                    Generation3dCommand::FlowEvalResolve(_) => unreachable!(),
                    Generation3dCommand::FlowTessellateResolve(_) => unreachable!(),
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
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self){self.closing=true;self.tessellate.begin_close();self.evaluation.begin_close();}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.begin_close();if !self.tessellate.terminal_is_empty(){self.tessellate.close_step(cx)}else{self.evaluation.close_step(cx)}}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.copy_bytes}else{self.evaluation.close_demands(0)?.copy_bytes})}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(copy)?.capacity_bytes}else{self.evaluation.close_demands(copy)?.capacity_bytes})}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.release_bytes}else{self.evaluation.close_demands(0)?.release_bytes})}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.depth}else{self.evaluation.close_demands(0)?.depth})}
    fn terminal_is_empty(&self)->bool{self.closing&&self.tessellate.terminal_is_empty()&&self.evaluation.terminal_is_empty()}
}

/// with every open press's and typing run's provisional leaves (the framework's, as of admission) and every open gumball
/// gesture (this instance's) folded in — `None` while nothing is open, so the committed snapshot is read as is. The
/// overlay is retired cold by the caller, never dropped.
fn generation3d_provisional_snapshot(committed: &Generation3dSnapshot, provisional: &[semio_framework_value::DslValue], gumball: &semio_s_artifact_procedural_3d_transforms::GumballGestures) -> Option<Generation3dSnapshot> {
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
    let _refused_paints_nothing = semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(overlay, &leaf);
    leaf.retire_cold();
}

/// 🪩️ The document and marks a preview body paints while a gumball gesture is open (design §13, F-7): the committed
/// snapshot with every open gesture folded in — the overlay its flow evaluation reads — and the selection carried onto
/// the operators the gestures splice, so the first grab of a shape keeps painting and selecting the instance the live
/// evaluation moves. `None` while no gesture is open; the caller retires the overlay cold.
pub fn generation3d_gumball_preview(committed: &Generation3dSnapshot, marks: &PreviewInteractionMarks, gumball: &semio_s_artifact_procedural_3d_transforms::GumballGestures) -> Option<(Generation3dSnapshot, PreviewInteractionMarks)> {
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
    gumball: &mut semio_s_artifact_procedural_3d_transforms::GumballGestures,
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
        let components = semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view, interaction.active_granularity.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map(String::as_str)).then(|| interaction.selection.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map_or(&[][..], |selection| selection.ids.as_slice()));
        let ids = semio_s_artifact_procedural_3d_transforms::gumball_ids(&snapshot.host_snapshot, gesture.ids, components, graph)?;
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        if ids.iter().any(|id| semio_s_artifact_procedural_3d_editor_selection::ComponentTarget::parse(id).is_some()) {
            semio_s_artifact_procedural_3d_editor_selection::validate_cached_components(&snapshot.host_snapshot, session, &ids, Some(&operation.canonical_base_revision), operation.retained).map_err(Fault::from)?;
        }
        return gumball.dispatch(
            semio_s_artifact_procedural_3d_transforms::GumballDispatch { verb: gesture.verb, window, ids, motion: gesture.motion, phase, authoring_seed: &operation.authoring_seed, base_revision: operation.canonical_base_revision },
            &snapshot.host_snapshot,
        );
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let selected = || interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
    let component_ids: &[String] = interaction.selection.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice());
    let graph_ids: &[String] = interaction.selection.get("graph").map_or(&[], |selection| selection.ids.as_slice());
    if let Some(ids) = generation3d_component_targets(command, context.and_then(|context| context.view_state.as_ref()), interaction.active_granularity.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map(String::as_str), component_ids, graph_ids).filter(|ids| !ids.is_empty()) {
        semio_s_artifact_procedural_3d_editor_selection::validate_cached_components(&snapshot.host_snapshot, session, ids, Some(&operation.canonical_base_revision), operation.retained).map_err(Fault::from)?;
    }
    match command {
        Generation3dCommand::EditMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::apply_selected("editMeshSelection", payload, &doc, interaction.selection.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
        Generation3dCommand::KnifeMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::apply_selected(payload, &doc, interaction.selection.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
        Generation3dCommand::DeleteSelection(_) if semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(context.and_then(|context| context.view_state.as_ref()), interaction.active_granularity.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map(String::as_str)) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::delete_selected(&doc, interaction.selection.get(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).map_or(&[], |selection| selection.ids.as_slice())),
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
    motion: semio_s_artifact_procedural_3d_transforms::GumballMotion,
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
    let components = semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view, granularity);
    if matches!(command, Generation3dCommand::EditMeshSelection(_) | Generation3dCommand::KnifeMeshSelection(_)) || components && matches!(command, Generation3dCommand::DeleteSelection(_)) { return Some(geometry); }
    let explicit = match command {
        Generation3dCommand::TranslateSelection(payload) => &payload.node_ids,
        Generation3dCommand::RotateSelection(payload) => &payload.node_ids,
        Generation3dCommand::ScaleSelection(payload) => &payload.node_ids,
        _ => return None,
    };
    if components { return Some(geometry); }
    let ids = if explicit.is_empty() { graph } else { explicit.as_slice() };
    ids.iter().any(|id| semio_s_artifact_procedural_3d_editor_selection::ComponentTarget::parse(id).is_some()).then_some(ids)
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
    source_read:Option<semio_framework_os_flow::FlowSourceRead>,
    source_read_ready:bool,
    closing:bool,
}

impl Generation3dSessionCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false,source_read:None,source_read_ready:false,closing:false }
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dSessionCommandWork {
    fn receive_original_command(&mut self,command:&mut Generation3dCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Generation3dCommand::FlowTessellateCancelResolve(payload)=command{let g=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}if let Some(fault)=payload.fault.take(){cx.consume_fuel(1);let original=semio_framework_job::OriginalTurnFailure::from(fault);let receipt_refusal=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).err();return Err(semio_framework_job::OriginalTurnFailure{receipt_refusal,..original})}if !payload.input_valid{cx.consume_fuel(1);return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original cancel reply routing input is malformed").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}}Ok(true)
    }
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

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Generation3dCommand::AddWidget(payload)=input.command{if payload.source_widget.is_some(){return match self.source_read.as_ref(){Some(source)if !self.source_read_ready=>source.next_demands(&input.snapshot.host_snapshot,maximum_copy_bytes),Some(source)=>{let read=source.next_demands(&input.snapshot.host_snapshot,maximum_copy_bytes)?;let command=command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())?;Ok(semio_framework_value::RetirementDemand{copy_bytes:read.copy_bytes.saturating_add(command.copy_bytes),capacity_bytes:read.capacity_bytes.saturating_add(command.capacity_bytes),release_bytes:read.release_bytes.saturating_add(command.release_bytes),depth:read.depth.max(command.depth)})},None=>Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<semio_framework_os_flow::FlowSourceRead>(),depth:1,..Default::default()})}}}
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed||self.closing {
            return Err(Fault::from("generation3d-session-command-work-repeated"));
        }
        if let Generation3dCommand::AddWidget(payload)=input.command{if let Some(request)=payload.source_request()?{
            if self.source_read.is_none(){let grant=_cx.retained_grant();let bytes=size_of::<semio_framework_os_flow::FlowSourceRead>();if _cx.is_cancelled()||_cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(ArtifactCommandWorkStep::Replay{stage:"generation3d-analysis-source",preview:r#"{"en":"Checking current geometry","de":"Aktuelle Geometrie wird geprüft"}"#.as_bytes()})}self.source_read=Some(semio_framework_os_flow::FlowSourceRead::new(payload.source_revision_bytes()?));_cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).map_err(Fault::from)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"generation3d-analysis-source",preview:r#"{"en":"Checking current geometry","de":"Aktuelle Geometrie wird geprüft"}"#.as_bytes()})}
            let read=self.source_read.as_mut().unwrap();
            if !self.source_read_ready{
                let step=self.instance_owner.with_mut::<Generation3dInstanceOperationOwner,_>(|owner|owner.with_session_and_gumball(|session,_|read.step(&input.snapshot.host_snapshot,input.operation.canonical_base_revision,session,&request,_cx).map_err(Fault::from)))???;
                let preview=r#"{"en":"Checking current geometry","de":"Aktuelle Geometrie wird geprüft"}"#.as_bytes();
                return match step{
                    semio_framework_os_flow::FlowSourceReadStep::Working=>Ok(if read.step_progress()==Default::default(){ArtifactCommandWorkStep::Replay{stage:"generation3d-analysis-source",preview}}else{ArtifactCommandWorkStep::Progress{stage:"generation3d-analysis-source",preview}}),
                    semio_framework_os_flow::FlowSourceReadStep::Refused(reason)=>Err(add_widget::source_fault(reason)),
                    semio_framework_os_flow::FlowSourceReadStep::Ready=>{self.source_read_ready=true;Ok(ArtifactCommandWorkStep::Progress{stage:"generation3d-analysis-source",preview})},
                };
            }
            let windows=generation3d_preview_windows(input.context.and_then(|context|context.view_state.as_ref()));
            let servable=flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
            let result=self.instance_owner.with_mut::<Generation3dInstanceOperationOwner,_>(|owner|{
                let step=owner.with_session_and_gumball(|session,_|read.admit_current_output(&input.snapshot.host_snapshot,input.operation.canonical_base_revision,session,&request,_cx).map_err(Fault::from))??;
                match step{
                    semio_framework_os_flow::FlowCurrentOutputRead::Working=>Ok(ArtifactCommandWorkStep::Replay{stage:"generation3d-analysis-source",preview:r#"{"en":"Checking current geometry","de":"Aktuelle Geometrie wird geprüft"}"#.as_bytes()}),
                    semio_framework_os_flow::FlowCurrentOutputRead::Refused(reason)=>Err(add_widget::source_fault(reason)),
                    semio_framework_os_flow::FlowCurrentOutputRead::Ready(source)=>{
                        let doc=ArtifactView::with_operation(input.snapshot,input.history,input.operation.clone());
                        let mut emit=add_widget::create_connected(payload,&doc,&source)?;
                        owner.owe_attached_previews_for_mutations(&windows,servable,&mut emit,input.operation.retained)?;
                        Ok(ArtifactCommandWorkStep::Complete(emit))
                    },
                }
            })??;
            self.consumed=matches!(result,ArtifactCommandWorkStep::Complete(_));
            return Ok(result);
        }}
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
            let mut emit = if matches!(input.command, Generation3dCommand::Reorganize(_)) {
                reorganize::apply(input.snapshot, _cx)?

            } else {
                owner.with_session_and_gumball(|session, gumball| generation3d_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation, session, gumball))??
            };
            if owner.gumball.revision() != gestures {
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
            } else {
                owner.owe_attached_previews_for_mutations(&windows, servable, &mut emit, input.operation.retained)?;
            }
            Ok(emit)
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn begin_close(&mut self){self.closing=true;if let Some(read)=self.source_read.as_mut(){read.begin_close();}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.source_read.as_ref().map(|read|read.next_close_demands(cx.retained_grant().maximum_copy_bytes)).transpose()?.unwrap_or_default();
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{(
            ||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{use semio_framework_job::InteractiveJobCloseStep;if let Some(read)=self.source_read.as_mut(){match read.close_step(grant){Ok(step)if read.terminal_is_empty()=>Ok(InteractiveJobCloseStep::Complete{progress:step.progress()}),Ok(step)=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),Err(error)=>return Err(error)}}else{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}})().map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.source_read.as_ref().map(|read|read.next_close_demands(0)).transpose()?.unwrap_or_default().copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.source_read.as_ref().map(|read|read.next_close_demands(copy)).transpose()?.unwrap_or_default().capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.source_read.as_ref().map(|read|read.next_close_demands(0)).transpose()?.unwrap_or_default().release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.source_read.as_ref().map(|read|read.next_close_demands(0)).transpose()?.unwrap_or_default().depth)}
    fn terminal_is_empty(&self)->bool{self.source_read.as_ref().is_none_or(|read|read.terminal_is_empty())}
}

pub struct Generation3dBoundedCommandJobFactory {
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

pub struct Generation3dFlowEvalJobFactory {
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
pub const GENERATION3D_DOCUMENT_SOURCE_ALLOCATION_BYTES: usize = 1_048_576;
pub const GENERATION3D_DOCUMENT_OWNERSHIP_SOURCE_MULTIPLES: usize = 32;
pub const GENERATION3D_DOCUMENT_SCAFFOLD_BYTES: usize = 16_384;
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
    Projection(semio_framework_dsl_record::native_encoding::RetainedFieldProjection),
    Projected(semio_framework_dsl_record::FieldValue),
    Writer(semio_framework_dsl_record::RetainedRecordWriter),
    Text(store::semio_format::RetainedTextEnvelope),
    Envelope(semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope),
    Export(semio_s_artifact_procedural_3d_io::Generation3dDocumentExport),
}

impl semio_framework_value::retirement::RetireOwned for Generation3dDocumentRetirement {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Projection(value)=>value.retirement(),Self::Projected(value)=>value.retirement(),Self::Writer(value)=>value.retirement(),Self::Text(value)=>value.retirement(),Self::Envelope(value)=>value.retirement(),Self::Export(value)=>value.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Projection(value)=>value.retirement_birth_bytes(),Self::Projected(value)=>value.retirement_birth_bytes(),Self::Writer(value)=>value.retirement_birth_bytes(),Self::Text(value)=>value.retirement_birth_bytes(),Self::Envelope(value)=>value.retirement_birth_bytes(),Self::Export(value)=>value.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

pub struct Generation3dDocumentIoWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
    closing: bool,
    source_identity: Option<usize>,
    source_demand: Option<usize>,
    expand_admission: bool,
    projection: Option<semio_framework_dsl_record::native_encoding::RetainedFieldProjection>,
    projected: Option<semio_framework_dsl_record::FieldValue>,
    writer: Option<semio_framework_dsl_record::RetainedRecordWriter>,
    text: Option<store::semio_format::RetainedTextEnvelope>,
    envelope: Option<semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope>,
    candidate: Option<semio_s_artifact_procedural_3d_io::Generation3dDocumentExport>,
    retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Generation3dDocumentRetirement>>,
    encoding: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    phase: u8,
}

impl Generation3dDocumentIoWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false, closing:false, source_identity:None, source_demand:None, expand_admission:false, projection:None, projected:None, writer:None, text:None, envelope:None, candidate:None, retirement:None, encoding:None, phase:0 }
    }

    fn park_retirement(&mut self,value:Generation3dDocumentRetirement)->Result<(),semio_framework_value::ValueError>{
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){
            Ok(owner)=>{self.retirement=Some(owner);Ok(())},
            Err((error,value))=>{match value{Generation3dDocumentRetirement::Projection(value)=>self.projection=Some(value),Generation3dDocumentRetirement::Projected(value)=>self.projected=Some(value),Generation3dDocumentRetirement::Writer(value)=>self.writer=Some(value),Generation3dDocumentRetirement::Text(value)=>self.text=Some(value),Generation3dDocumentRetirement::Envelope(value)=>self.envelope=Some(value),Generation3dDocumentRetirement::Export(value)=>self.candidate=Some(value)}Err(error)},
        }
    }
    fn owns_retirement_source(&self)->bool{self.projection.is_some()||self.projected.is_some()||self.writer.is_some()||self.text.is_some()||self.envelope.is_some()||self.candidate.is_some()}
    fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::ValueError>{
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        if let Some(owner)=self.retirement.as_ref(){let d=owner.selected_demand(copy)?;return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth})}
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
                        let spec=semio_s_artifact_procedural_3d_io_text::snapshot::generation3d_document_spec_controlled(&mut control)?;
                        if !matches!(self.projected,Some(semio_framework_dsl_record::FieldValue::Record(_))){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"generation3d projection root is not a record"))}
                        let Some(semio_framework_dsl_record::FieldValue::Record(record))=self.projected.take()else{unreachable!()};
                        self.writer=Some(semio_framework_dsl_record::RetainedRecordWriter::new(record,spec,semio_framework_dsl_record::JoinMode::Document,GENERATION3D_DOCUMENT_DOWNLOAD_BYTES));
                        let retired=self.projection.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Projection(retired))?;self.phase=2;
                    },
                    2=>if let Some(body)=self.writer.as_mut().unwrap().step(1,&mut control)?{
                        self.text=Some(store::semio_format::RetainedTextEnvelope::new("procedural.generation3d".into(),store::semio_format::Component::Dsl,1,body,cx.retained_grant()));
                        let retired=self.writer.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Writer(retired))?;self.phase=3;
                    },
                    3=>if let Some(text)=self.text.as_mut().unwrap().step(1,8,&mut control)?{
                        let row=semio_s_artifact_procedural_3d_io::document_io::EXPORT_FORMATS.iter().find(|row|row.id=="txt").unwrap();
                        self.envelope=Some(semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope::new(row,text.into_bytes()));
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
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
                        owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
                    }
                    Ok(emit)
                })?
            }
            _ => return Err(Fault::from("generation3d-document-io-route-rejected")),
        };
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self){self.closing=true;}

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand={let d=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_value::RetirementDemand{copy_bytes:d.maximum_copy_bytes,capacity_bytes:d.maximum_capacity_bytes,release_bytes:d.maximum_release_bytes,depth:d.maximum_depth}};
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{(
            ||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
        if grant.maximum_items==0{return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()})}
        if self.retirement.is_none(){
            let source=if let Some(value)=self.projection.take(){Some(Generation3dDocumentRetirement::Projection(value))}
            else if let Some(value)=self.projected.take(){Some(Generation3dDocumentRetirement::Projected(value))}
            else if let Some(value)=self.writer.take(){Some(Generation3dDocumentRetirement::Writer(value))}
            else if let Some(value)=self.text.take(){Some(Generation3dDocumentRetirement::Text(value))}
            else if let Some(value)=self.envelope.take(){Some(Generation3dDocumentRetirement::Envelope(value))}
            else{self.candidate.take().map(Generation3dDocumentRetirement::Export)};
            if let Some(value)=source{if let Err(error)=self.park_retirement(value){return Err(error)}}
            else{self.encoding.take();return Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()})}
        }
        match self.retirement.as_mut().unwrap().step(grant){
            Ok(RetainedCloneStep::Complete(progress))=>{self.retirement.take();Ok(InteractiveJobCloseStep::Pending{progress})},
            Ok(RetainedCloneStep::Progress(progress))=>Ok(InteractiveJobCloseStep::Pending{progress}),
            Err(error)=>Err(error),
        }
    })().map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(copy).map(|grant|grant.maximum_capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_depth)}

    fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.projected.is_none()&&self.writer.is_none()&&self.text.is_none()&&self.envelope.is_none()&&self.candidate.is_none()&&self.retirement.is_none()&&self.encoding.is_none()}
}

pub struct Generation3dDocumentIoJobFactory {
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
pub const GENERATION3D_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
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
pub const GENERATION3D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
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
            if owner.with_session(|session| set_contributions::install(payload, session, input.operation.retained))?? {
                let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
            }
            Ok(())
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub struct Generation3dContributionsJobFactory {
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

pub fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation3dDiff` owns the
/// projections it displaces (a `FlowHostSnapshot` whose `layout` is an `OrderedMap` root that rejects a bare
/// drop), so the intermediate delta is retired rather than dropped: leaving it to drop glue aborted the
/// whole store-publication turn the moment a layout entry existed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The mutation stays with its caller, so a refusal never
/// loses an original owner.
fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>), semio_framework_value::ValueError> {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    admit_generation3d_artifact_mutation(mutation).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "generation3d-artifact-mutation-envelope"))?;
    let inverse = protocol::Mutation::inverse(mutation, base)?;
    let diff = protocol::Mutation::diff(mutation, base).into_parts().0;
    let applied = protocol::apply_diff(&diff, base);
    diff.retire_cold();
    match applied {
        Ok(post) => Ok((post, inverse)),
        Err(_) => {
            inverse.into_iter().for_each(|row| protocol::Mutation::<Generation3dSnapshot>::retire_cold(row));
            Err(ValueError::literal(ValueRefusalKind::InvalidValue, "generation3d-artifact-diff-apply-failed"))
        }
    }
}

fn generation3d_preparation_refusal(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    owners: store::OneItemOwners<Generation3dSnapshot, Generation3dMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<Generation3dMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Generation3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Generation3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("generation3d-artifact-lane".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Generation3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(generation3d_preparation_refusal("generation3d-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Generation3dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation, Generation3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation, Generation3dMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err((generation3d_preparation_refusal("generation3d-artifact-request-refused"), request));
        }
        Ok((Box::new(Generation3dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(generation3d_preparation_refusal("generation3d-artifact-original-refusal-retained"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-mutation-owner-missing"))?;
        let (post, inverse) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-mutation-owner-missing"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: self.retained_bytes, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ArtifactStorePreparation


impl ArtifactEditor for Generation3dPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<ExampleSource> {
        examples()
    }
    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation3dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `editor did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(semio_s_artifact_procedural_3d_model::GENERATION3D_CHILD_PROJECTION), error.to_string()))
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
    type Presence = semio_s_artifact_procedural_3d_editor_presence::Generation3dPresence;
    type PresenceMutation = semio_s_artifact_procedural_3d_editor_presence::Generation3dPresenceMutation;
    type Transient = Generation3dTransient;
    type TransientMutation = Generation3dTransientMutation;

    type Command = Generation3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_procedural_3d_envelope::generation3d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation3dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dInstanceOperationOwner::new())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_procedural_3d_initialization::generation3d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        semio_s_artifact_procedural_3d_host::generation3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation3d atomic publication authority is absent or stale"))?;
        semio_s_artifact_procedural_3d_host::generation3d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
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
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| {
            semio_s_artifact_procedural_3d_transforms::gumball_fault_notices().iter().chain(add_widget::source_fault_notices()).chain(semio_s_artifact_procedural_3d_preview_eval::preview_eval_fault_notices()).chain(semio_s_artifact_procedural_3d_model::generation3d_document_fault_notices()).cloned().collect()
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

    const DIALECT: Dialect = semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Generation3dConfig, Generation3dConfigMutation>())
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
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "the generation 3d command does not match its exact registered tool"));
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
            retained: request.retained,
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
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
        );
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
        Some(semio_s_artifact_procedural_3d_editor_config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        semio_s_artifact_procedural_3d_io_text::snapshot::default_snapshot()
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
        let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
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
        semio_s_artifact_procedural_3d_mesh_bridge::semio_mesh_from_mesh_data(&mesh)
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
                            .then(|| semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::change_slider_value(target_id.to_string(), number))
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
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, Fault> {
        let mut original_fault=original_fault;if !matches!(action,"flowEvalResolve"|"flowTessellateResolve"|"flowTessellateCancelResolve"){if let Some(fault)=original_fault.take(){return Err(fault)}}
        let mut args = args.get_or_insert(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        fn original_reply_valid(args:&semio_framework_value::DslValue,tessellate:bool,cancel:bool)->bool{
            let semio_framework_value::DslValue::Object(rows)=args else{return false};
            let field=|key:&str|{let mut values=rows.iter().filter(|(name,_)|name==key);let value=values.next().map(|(_,value)|value);if values.next().is_some(){None}else{value}};
            if !["windowId","windowKindId"].iter().all(|key|field(key).and_then(|value|value.as_str()).is_some_and(|value|!value.is_empty()))||field("output").is_none(){return false}
            if cancel{return true}
            if field("nodeHash").and_then(semio_framework_value::DslValue::as_u64).is_none(){return false}
            if tessellate{return field("handle").and_then(|value|value.as_str()).is_some_and(|value|!value.is_empty())&&field("tolerance").and_then(|value|value.as_f64()).is_some_and(|value|value.is_finite()&&value>0.0)}
            let extensions=rows.iter().filter(|(key,_)|key=="extensionId").count();extensions==0||(extensions==1&&field("extensionId").and_then(|value|value.as_str()).is_some())
        }
        fn original_reply_text(args:&mut semio_framework_value::DslValue,key:&str,required:bool)->Result<String,Fault>{
            let semio_framework_value::DslValue::Object(rows)=args else{return Err(Fault::from("original reply argument object is absent"))};let Some((_,source))=rows.iter_mut().find(|(name,_)|name==key)else{return if required{Err(Fault::from("original reply text is absent"))}else{Ok(String::new())}};if !matches!(source,semio_framework_value::DslValue::String(_)){return Err(Fault::from("original reply text has another type"))}let semio_framework_value::DslValue::String(value)=std::mem::replace(source,semio_framework_value::DslValue::Null)else{unreachable!()};Ok(value)
        }
        fn original_reply_output(args:&mut semio_framework_value::DslValue)->Result<semio_framework_value::DslValue,Fault>{let semio_framework_value::DslValue::Object(rows)=args else{return Err(Fault::from("original reply argument object is absent"))};rows.iter_mut().find(|(key,_)|key=="output").map(|(_,source)|std::mem::replace(source,semio_framework_value::DslValue::Null)).ok_or_else(||Fault::from("original reply output is absent"))}

        match action {
            "setActiveExample" => Ok(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => {
                semio_framework_tool_machine::node_graph_edit_rows(&args).map_err(Fault::from)?;
                Ok(Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: args.get("operations").map(semio_framework_pack_json::to_json_string).unwrap_or_else(|| "[]".into()) }))
            }
            "deleteSelection" => Ok(Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "addWidget" => Ok(Generation3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind"]), format: str_arg(&["format"]), action: str_arg(&["action"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]),source_widget:str_arg(&["sourceWidget"]),source_port:str_arg(&["sourcePort"]),target_port:str_arg(&["targetPort"]),source_revision:str_arg(&["sourceRevision"]) })),
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
                let point=|key:&str,default:[f64;3]|->Result<[f64;3],Fault>{let Some(value)=args.get(key)else{return Ok(default)};let values=value.as_array().filter(|values|values.len()==3).ok_or_else(||Fault::from("Point inputs require three coordinates"))?;let mut point=[0.0;3];for axis in 0..3{point[axis]=values[axis].as_f64().filter(|value|value.is_finite()).ok_or_else(||Fault::from("Point inputs must be finite"))?;}Ok(point)};
                Ok(Generation3dCommand::EditMeshSelection(semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::EditMeshSelection {
                    operation: text("operation", "extrude")?, amount: number("amount", 0.1)?, cuts: integer("cuts", 256.0)?,
                    dx: number("dx", 0.0)?, dy: number("dy", 0.0)?, dz: number("dz", 0.0)?,
                    width: number("width", 0.1)?, segments: integer("segments", 64.0)?, merge_mode: text("mergeMode", "center")?,
                    tolerance: number("tolerance", 0.0001)?, radius: number("radius", 1.0)?, grid: number("grid", 1.0)?, center,angle:number("angle",0.1)?,pull_direction:point("pullDirection",[0.0,0.0,1.0])?,neutral_point:point("neutralPoint",[0.0;3])?,
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
                Ok(Generation3dCommand::KnifeMeshSelection(semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::KnifeMeshSelection { start: point("start", [0.0, -1.0, 0.0])?, end: point("end", [0.0, 1.0, 0.0])? }))
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
            "flowEvalResolve" => {
                let input_valid=original_reply_valid(&args,false,false);
                if !input_valid{return Ok(Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,extension_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let extension_id=original_reply_text(&mut args,"extensionId",false).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve{window_id,window_kind_id,node_hash,extension_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowTessellateResolve" => {
                let input_valid=original_reply_valid(&args,true,false);
                if !input_valid{return Ok(Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,handle:String::new(),tolerance:0.0,output:semio_framework_value::DslValue::Null,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();let tolerance=args.get("tolerance").and_then(|value|value.as_f64()).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let handle=original_reply_text(&mut args,"handle",true).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve{window_id,window_kind_id,node_hash,handle,tolerance,output,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowEvalRelease" => Ok(Generation3dCommand::FlowEvalRelease(flow_eval_release::FlowEvalRelease {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowTessellateCancelResolve"=>{let input_valid=original_reply_valid(&args,false,true);if !input_valid{return Ok(Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve{window_id:String::new(),window_kind_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let output=original_reply_output(&mut args).unwrap();Ok(Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve{window_id,window_kind_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))},
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
            let component_ids = &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids;
            if let Some(ids) = generation3d_component_targets(command, view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN), component_ids, &interaction.selection("graph").ids).filter(|ids| !ids.is_empty()) {
                semio_s_artifact_procedural_3d_editor_selection::validate_cached_components(&doc.snapshot.host_snapshot, session, ids, doc.operation().ok().map(|operation| &operation.canonical_base_revision), doc.retained_grant()?).map_err(Fault::from)?;
            }
            match command {
            Generation3dCommand::EditMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::apply_selected("editMeshSelection", payload, doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::KnifeMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::apply_selected(payload, doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(_) if semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN)) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::delete_selected(doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, session),
            _ if generation3d_gumball_command(command) => {
                let gesture = generation3d_gumball_gesture(command).ok_or_else(|| Fault::from("generation3d-gumball-command"))?;
                let components = semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN)).then(|| interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids.as_slice());
                let ids = semio_s_artifact_procedural_3d_transforms::gumball_ids(&doc.snapshot.host_snapshot, gesture.ids, components, &interaction.selection("graph").ids)?;
                semio_s_artifact_procedural_3d_transforms::gumball_once(gesture.verb, ids, gesture.motion, doc)
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
            let id = semio_s_artifact_procedural_3d_model::widget_id(widget).to_string();
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
        let applied_edits = semio_s_artifact_procedural_3d_preview_eval::applied_document_edits_digest(doc.history) ^ doc.provisional_generation().wrapping_mul(0x9e37_79b9_7f4a_7c15);
        owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                use semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunOwner as _;
                let (session, link) = owner.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
                Ok(semio_s_artifact_procedural_3d_preview_eval::preview_eval_run_effects(session, link, &windows, doc.tool_run(), servable, applied_edits))
            })
            .unwrap_or_default()
    }

    /// ⏯️ The `previewEval` run job over the run's base document: the preview widgets it observes are
    /// the document's own, and its hops read the retained session this instance's commands fold into.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != semio_s_artifact_procedural_3d_preview_eval::PREVIEW_EVAL_TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let preview_widget_ids = semio_s_artifact_procedural_3d_preview_eval::preview_widget_ids(&request.snapshot.host_snapshot);
        Ok(Some(Box::new(semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunJob::<Generation3dInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity, preview_widget_ids)?)))
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
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient
            .window::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()
            .or_else(|| transient.window::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>())
            .and_then(|state| state.preview_eval_text.as_deref());
        let mut marks = PreviewInteractionMarks::from_interaction(interaction);
        marks.source_revision = doc.render_operation().map(|operation| operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect());
        let previews = matches!(body_key, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW | generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW);
        let rendered=owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                let gesture = previews.then(|| generation3d_gumball_preview(doc.snapshot, &marks, &owner.gumball)).flatten();
                let (document, marks) = gesture.as_ref().map_or((doc.snapshot, &marks), |(document, marks)| (document, marks));
                let rendered = owner.with_session(|session| generation3d_render_body(body_key, document, cfg.snapshot, preview_eval_text, view_state, marks, session, doc.tool_run()));
                if let Some((document, _)) = gesture {
                    document.retire_cold();
                }
                rendered
            })
            .map_err(|original|semio_framework_plugin::OriginalRenderFailure::from(semio_framework_job::OriginalTurnFailure::from(original)))?;
        rendered.map(Some).map_err(Into::into)
    }

    fn window_engagements_with_request_context(_doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel, _transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>, interaction: &InteractionView<'_>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let engagement = semio_s_artifact_procedural_3d_editor_selection::engagement(&semio_s_artifact_procedural_3d_editor_selection::ComponentSelection::from_interaction(interaction), view_state.locale == semio_framework_ui_locale::Locale::De);
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

//#region 🔖️Manifest
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
        semio_s_artifact_procedural_3d_examples::art_generation3d_hexagonal_mushroom_column::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_mesh_workbench::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_rectangle_extrude_volume::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_sphere_cut_with_torus::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_box_fillet_preview::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_sphere_box_fuse::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_face_sweep_extrude::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_rectangle_wire_preview::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_box_shell_preview::source(),
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

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo_session {
        #[path = "📚️examples/🎬️demo-session/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
