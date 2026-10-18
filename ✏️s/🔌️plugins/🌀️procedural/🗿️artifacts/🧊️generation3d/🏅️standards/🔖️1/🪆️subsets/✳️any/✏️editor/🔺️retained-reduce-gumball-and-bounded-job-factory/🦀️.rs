//! 🔺️ Retained reduce, gumball and bounded job factory: the retained reduction, the gumball gesture, the session command work and the bounded command job factory. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
//#region 🧵️RetainedCommands
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
    pub(super) verb: &'static str,
    pub(super) ids: &'a [String],
    pub(super) motion: semio_s_artifact_procedural_3d_transforms::GumballMotion,
    pub(super) phase: Option<&'a str>,
    pub(super) reason: Option<&'a str>,
    pub(super) window: Option<&'a str>,
}

/// 🛠️ The gumball dispatch a command is, or `None` for every other verb.
pub(super) fn generation3d_gumball_gesture(command: &Generation3dCommand) -> Option<Generation3dGumballGesture<'_>> {
    Some(match command {
        Generation3dCommand::TranslateSelection(payload) => Generation3dGumballGesture { verb: "translateSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        Generation3dCommand::RotateSelection(payload) => Generation3dGumballGesture { verb: "rotateSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        Generation3dCommand::ScaleSelection(payload) => Generation3dGumballGesture { verb: "scaleSelection", ids: &payload.node_ids, motion: payload.motion(), phase: payload.phase.as_deref(), reason: payload.reason.as_deref(), window: payload.window_id.as_deref() },
        _ => return None,
    })
}

/// 🛠️ Whether a command is a gumball verb.
pub(super) fn generation3d_gumball_command(command: &Generation3dCommand) -> bool {
    generation3d_gumball_gesture(command).is_some()
}

pub(super) fn generation3d_component_targets<'a>(command: &'a Generation3dCommand, view: Option<&semio_framework_plugin::ViewModel>, granularity: Option<&str>, geometry: &'a [String], graph: &'a [String]) -> Option<&'a [String]> {
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
pub(super) struct Generation3dSessionCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
    source_read:Option<semio_framework_os_flow::FlowSourceRead>,
    source_read_ready:bool,
    pub(super) closing:bool,
}

impl Generation3dSessionCommandWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
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
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dBoundedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
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
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Generation3dPlayApp
