//! 🛠️ Preview and flow eval window command work: the preview command work step, the flow eval window work and the provisional snapshot and gumball preview helpers. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
//#region 🧵️RetainedCommands
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
pub(super) struct Generation3dFlowEvalWindowWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) tessellate: preview_eval::PreviewTessellateReceive,
    pub(super) evaluation:preview_eval::PreviewEvalReceive,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl Generation3dFlowEvalWindowWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self { Self { tool_id, instance_owner, tessellate: preview_eval::PreviewTessellateReceive::new(),evaluation:preview_eval::PreviewEvalReceive::new(), complete: false, closing: false } }
}

/// 🪟️ The preview window one chain route addresses, or `None` for a route that addresses none.
pub(super) fn generation3d_flow_eval_window_address(command: &Generation3dCommand) -> Option<(&str, &str)> {
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

/// 🪄️ with every open press's and typing run's provisional leaves (the framework's, as of admission) and every open gumball
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
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Generation3dPlayApp
