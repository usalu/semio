//! 🧿️ Commands and instance operation owner: the aggregated command enum, the command result bridge and the instance operation owner with its scratch session. See [`super`].

use super::*;

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
    pub(super) eval_session: Option<FlowEvalSession>,
    /// ⏯️ The `previewEval` run's surface-owned half: attached preview roster and the live job's port.
    run_link: semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunLink,
    /// 🛠️ Every window's open gumball gesture — ephemeral local tool state, never history; the previews fold it in.
    pub(super) gumball: semio_s_artifact_procedural_3d_transforms::GumballGestures,
    /// 📐️ The instance's geometry engine: the typed inference cache, the retained base and the stepped run of the `s.procedural.generation3d.inference` geometry service.
    pub(super) geometry: semio_s_artifact_procedural_3d_geometry_service::GeometryHost,
    pub(super) closing: bool,
}

impl Generation3dInstanceOperationOwner {
    pub fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), run_link: semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunLink::default(), gumball: semio_s_artifact_procedural_3d_transforms::GumballGestures::default(), geometry: Default::default(), closing: false }
    }

    /// 📐️ The geometry host a run of the contextual `s.procedural.generation3d.inference` geometry service executes against.
    pub fn geometry(&self) -> &semio_s_artifact_procedural_3d_geometry_service::GeometryHost {
        &self.geometry
    }

    pub(super) fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-eval-session-owner-missing"))
    }

    /// 🛠️ The retained session together with the open gumball gestures — what a gesture command folds into.
    pub(super) fn with_session_and_gumball<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession, &mut semio_s_artifact_procedural_3d_transforms::GumballGestures) -> R) -> Result<R, Fault> {
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
    pub(super) fn owe_attached_previews_for_mutations(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<bool, Fault> {
        let Self { eval_session, run_link, closing, .. } = self;
        let session = eval_session.as_mut().filter(|_| !*closing).ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
        semio_s_artifact_procedural_3d_preview_eval::owe_attached_previews_for_mutations(session, run_link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }

    /// 🚦️ Owes the attached previews an evaluation and puts the run start that debt needs on the
    /// gesture's own emit — what a route that owes unconditionally (a generation command, an example
    /// switch, a contributions install, a closing import chunk) hands back.
    pub(super) fn owe_attached_previews_carrying(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
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
pub(super) fn with_scratch_session<R>(body: impl FnOnce(&mut FlowEvalSession) -> R) -> R {
    let mut session = FlowEvalSession::new();
    let result = body(&mut session);
    session.retire_cold();
    result
}
//#endregion 🔖️InstanceOperationOwner
