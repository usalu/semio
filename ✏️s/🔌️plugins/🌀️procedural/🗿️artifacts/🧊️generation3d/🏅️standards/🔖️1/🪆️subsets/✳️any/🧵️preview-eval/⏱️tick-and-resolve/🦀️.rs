//! ⏱️ Tick and resolve of the preview evaluation: the evaluation tick, the invocation resolution and the release hops. See [`super`].

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError};
use semio_os_flow::{FlowHost, FlowHostCloseSlot};

//#region ⏱️Tick
/// ⏱️ What one evaluation tick owes its caller: the extension work it declared and whether the
/// addressed window's retained publication actually changed. It owes no continuation: the run job
/// schedules the next hop off the window's latch.
pub struct FlowEvalTickOutcome {
    pub extension_invocations: Vec<ExtensionInvocation>,
    pub publication: FlowEvalPublication,
    /// 🕸️ Whether this tick moved the per-node evaluation census the GRAPH body paints. The preview
    /// is republished on every hop because that is what delivers geometry; the graph is the most
    /// expensive body in the app and only its computing chrome depends on the chain, so it is
    /// republished exactly when that chrome moved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub census_moved: bool,
}

/// 🎛️ Owns the original Host between funded preview publication turns.
#[derive(Default)]
pub struct MountedPreviewTick {
    host: Option<Box<FlowHost>>,
    close: FlowHostCloseSlot,
    census_before: u64,
}

impl MountedPreviewTick {
    fn birth_demands() -> RetirementDemand {
        RetirementDemand {
            copy_bytes: std::mem::size_of::<Option<Box<FlowHost>>>() + std::mem::size_of::<u64>(),
            capacity_bytes: std::mem::size_of::<FlowHost>(),
            depth: 1,
            ..Default::default()
        }
    }

    pub fn demands(&self, session: &FlowEvalSession, copy: usize) -> Result<RetirementDemand, ValueError> {
        match self.host.as_ref() {
            Some(host) if !host.baseline_publication_terminal_is_empty() => host.next_baseline_publication_demands(copy),
            Some(host) => session.preview_tick_finish_demands(host, copy),
            None => Ok(Self::birth_demands()),
        }
    }

    /// 🪙️ Separates the original publication from re-reading the window's snapshot and seeds.
    pub fn advance_publication(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Option<Result<RetainedCloneProgress, ValueError>> {
        let host = self.host.as_mut()?;
        if host.baseline_publication_terminal_is_empty() {
            return None;
        }
        Some(host.baseline_publication_step(effect).map(|step| step.progress()))
    }

    /// 🪜️ Offers one phase on the mounted command's original effect.
    pub fn step(
        &mut self,
        window_id: &str,
        window_kind_id: &str,
        host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
        tolerance: f64,
        session: &mut FlowEvalSession,
        retained_eval: Option<&str>,
        source: PreviewEvaluationSource,
        turn_started_us: Option<u64>,
        effect: &mut RetainedAllocationEffect<'_>,
        project: impl FnOnce(&mut std::collections::HashMap<String, semio_framework_artifact_flow_flow::neural::Dictionary>),
    ) -> Result<(Option<FlowEvalTickOutcome>, RetainedCloneProgress), ValueError> {
        let grant = effect.grant();
        let demand = self.demands(session, grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
            return Ok((None, Default::default()));
        }
        if let Some(progress) = self.advance_publication(effect) {
            return Ok((None, progress?));
        }
        if let Some(host) = self.host.as_mut() {
            let (more, progress) = session.finish_preview_tick(host, matches!(source, PreviewEvaluationSource::WithFallback), effect)?;
            let Some(more) = more else { return Ok((None, progress)); };
            session.begin_window_tick(window_id, grant).map_err(|error| error.with_retained_progress(progress))?;
            let outcome = finish_tick(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, source, effect, self.census_before, more, host.take_pending_extension_evals())
                .map_err(|error| error.with_retained_progress(progress))?;
            return Ok((Some(outcome), progress));
        }
        if !effect.performed(1)? {
            return Ok((None, Default::default()));
        }
        let mut host = match host_with_session(host_snapshot, session, grant) {
            Ok(host) => host,
            Err((error, host)) => {
                semio_s_artifact_procedural_3d_model::close_host(host)?;
                return Err(error);
            }
        };
        let prepared = session.prepare_preview_tick(&mut host, turn_started_us, project);
        self.census_before = node_census_digest(session.status_json());
        self.host = Some(Box::new(host));
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default()
        };
        prepared.map_err(|error| error.with_retained_progress(progress))?;
        Ok((None, progress))
    }

    pub fn close_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if self.host.is_some() {
            return Ok(FlowHostCloseSlot::intake_demand());
        }
        self.close.next_demand(copy)
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        let demand = self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        if self.host.is_some() {
            return semio_framework_job::close_original_retained_step(cx, demand, |grant| RetainedWorkOutcome {
                result: self.close.retain(&mut self.host, grant).map(RetainedCloneStep::Progress), work: 1,
            });
        }
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() });
        }
        let step = semio_framework_job::close_original_retained_effect_step(cx, demand, |effect| self.close.close_step(effect))?;
        Ok(if self.terminal_is_empty() {
            InteractiveJobCloseStep::Complete { progress: step.progress() }
        } else {
            step
        })
    }

    pub fn terminal_is_empty(&self) -> bool { self.host.is_none() && self.close.terminal_is_empty() }
}

/// 🐢️ What ONE evaluation hop of the chain actually invalidates: the preview window it is addressed
/// to, the panels that read the chain's own status back, and — only when this hop moved the per-node
/// census — the graph body that paints the computing chrome.
///
/// 🐛️ Every hop used to declare the default [`UiDirtyScope::Full`], so one slider step paid THREE
/// whole-shell refreshes of 452, 574 and 558 ms, each re-rendering `procedural.play.main` at a
/// 183–266 ms patch install, for a walk whose own guest work measured 2.3 ms. Worse, the geometry a
/// hop published could not reach the screen until that refresh finished: the first mesh landed 474 ms
/// after the tick that produced it had already settled
/// (`📓️slider-latency-incremental-eval-2026-09-15.md` §1).
///
/// 🚦️ Never [`UiDirtyScope::None`]: the host polls `pending_effects` once per `refreshUi`, so a hop
/// that asked for no refresh at all would be the last hop a settled chain could ever take — the same
/// reason [`owe_attached_previews_carrying`] carries a run start rather than trusting a poll.
pub fn chain_ui_scope(preview_body: &str, graph_body: Option<&str>, panel_bodies: &[&str], census_moved: bool) -> semio_framework::kernel::UiDirtyScope {
    let mut window_bodies = vec![preview_body.to_string()];
    if census_moved {
        window_bodies.extend(graph_body.map(str::to_string));
    }
    semio_framework::kernel::UiDirtyScope::Partial { window_bodies, panel_bodies: panel_bodies.iter().map(|body| (*body).to_string()).collect(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}

/// 🏁️ Whether the tick that just ran leaves its window UNFINISHED — the one rule the run job's
/// settle decision turns on, named so a law can state it without a contributed registry.
///
/// A window owes more work when the evaluation itself said so (`more`) **or** when the tick parked
/// extension answers it is still waiting for. Recording only `more` was the whole defect: `!more` is
/// exactly the branch that emits the `tessellate` invocations, so a window still owing mesh round
/// trips recorded itself FINISHED and [`FlowEvalSession::window_tick_owed`] went false for good — the
/// first answer that came back `Ready` while another handle was untessellated stranded the surface
/// at `inFlight: 1` forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn tick_is_unfinished(more: bool, parked_extension_invocations: usize) -> bool {
    more || parked_extension_invocations > 0
}

/// 🧮️ ONE evaluation tick of the shared chain, for ANY surface.
///
/// ⏱️ The tick's own wall cost is recorded into `semio_os_flow`'s evaluation-step ledger
/// ONLY while `semio_framework_job::runtime_diagnostics_enabled()` is armed — this is the ONE app
/// work step the 8 ms interactive ceiling governs, and a normal boot must not pay two clock reads
/// per tick to measure it.
///
/// 📤️ `retained_eval` is the evaluation the addressed window ALREADY holds — the tick republishes
/// only when it differs, so redispatch ticks that move no node allocate and retire nothing.
///
/// 🔁️ `turn_started_us` is when the guest turn this walk belongs to began. A tick the host
/// dispatched opens its own turn and passes `None`; a tick an answer's fold runs INLINE passes the
/// fold's own turn start, so every walk of one turn shares one wall deadline
/// ([`semio_os_flow::flow_eval_tick_budget`]).
pub fn evaluate_tick(
    window_id: &str,
    window_kind_id: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
    effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
) -> Result<FlowEvalTickOutcome, semio_framework_value::ValueError> {
    evaluate_tick_with_seeds(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, PreviewEvaluationSource::WithFallback, turn_started_us, effect, |_| {})
}

/// 🌱️ Evaluates a generated instance with source values projected into this tick's seed map.
pub fn evaluate_tick_with_seeds(
    window_id: &str,
    window_kind_id: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    source: PreviewEvaluationSource,
    turn_started_us: Option<u64>,
    effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
    project: impl FnOnce(&mut std::collections::HashMap<String, semio_framework_artifact_flow_flow::neural::Dictionary>),
) -> Result<FlowEvalTickOutcome, semio_framework_value::ValueError> {
    let started_us = semio_framework_job::runtime_diagnostics_enabled().then(semio_framework_job::default_now_us).flatten();
    let retained_grant = effect.grant();
    session.begin_window_tick(window_id, retained_grant)?;
    let census_before = node_census_digest(session.status_json());
    let mut host = match host_with_session(host_snapshot, session, retained_grant) {
        Ok(host) => host,
        Err((error, host)) => {
            semio_s_artifact_procedural_3d_model::close_host(host)?;
            return Err(error);
        }
    };
    let more = match source {
        PreviewEvaluationSource::WithFallback => session.tick_cold_with_seeds(&mut host, turn_started_us, project),
        PreviewEvaluationSource::Current => session.tick_cold_current_with_seeds(&mut host, turn_started_us, project),
    };
    let pending_extension_evals = host.take_pending_extension_evals();
    semio_s_artifact_procedural_3d_model::close_host(host)?;
    let outcome = finish_tick(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, source, effect, census_before, more, pending_extension_evals)?;
    if let (Some(started_us), Some(finished_us)) = (started_us, started_us.and_then(|_| semio_framework_job::default_now_us())) {
        semio_os_flow::record_flow_eval_step(finished_us.saturating_sub(started_us));
    }
    Ok(outcome)
}

fn finish_tick(
    window_id: &str,
    window_kind_id: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    source: PreviewEvaluationSource,
    effect: &mut RetainedAllocationEffect<'_>,
    census_before: u64,
    more: bool,
    pending_extension_evals: Vec<semio_framework_artifact_flow_flow::neural::PendingExtensionEval>,
) -> Result<FlowEvalTickOutcome, ValueError> {
    let retained_grant = effect.grant();
    let mut extension_invocations = Vec::new();
    for pending in pending_extension_evals {
        let resume = session.has_evaluation_progress(pending.node_hash);
        let dependency_json = if resume { String::new() } else { geometry_dependency_json(host_snapshot, &pending.neuron_id) };
        extension_invocations.push(evaluate_invocation(pending, resume, dependency_json, window_id, window_kind_id));
    }
    if extension_invocations.is_empty() && !more {
        extension_invocations.extend(preview_tessellate_invocations(window_id, window_kind_id, session, host_snapshot, tolerance, source, effect)?);
    }
    session.note_window_tick_outcome(window_id, tick_is_unfinished(more, extension_invocations.len()), retained_grant)?;
    if !extension_invocations.is_empty() {
        session.note_window_extensions_in_flight(window_id, extension_invocations.len(), retained_grant)?;
    } else if more && !may_rearm(host_snapshot) {
        session.abandon_window_tick(window_id, retained_grant)?;
    }
    let publication = preview_eval_publication_for(session, host_snapshot, retained_eval, source);
    Ok(FlowEvalTickOutcome { extension_invocations, publication, census_moved: node_census_digest(session.status_json()) != census_before })
}

/// 📨️ The original evaluation request carries business identity and address; its caller lends execution authority through the actual context.
pub fn evaluate_invocation(pending: semio_framework_artifact_flow_flow::neural::PendingExtensionEval, resume: bool, dependency_json: String, window_id: &str, window_kind_id: &str) -> ExtensionInvocation {
    let origin=semio_os_flow::FlowInvocationOrigin{
        window_id:window_id.into(),window_kind_id:window_kind_id.into(),neuron_id:pending.neuron_id,
        extension_id:pending.extension_id,operator_id:pending.operator_id,node_hash:pending.node_hash,
        input_json:pending.input_json,dependency_json,
        operator_version:format!("registry:{};geometry:1;policy:1",semio_os_flow::flow_extension_registry_generation()),
        outer_node_hash:None,preview_source:None,
    };
    let request_json=origin.request_json_cold(resume);
    let semio_os_flow::FlowInvocationOrigin{extension_id,..}=origin;
    ExtensionInvocation::new(extension_id,"evaluate",request_json,"flowEvalResolve")
}

/// 🕸️ One number over the part of the per-node census that is worth re-rendering the GRAPH body for:
/// whether the computing chrome is on screen at all, and which nodes carry a fault the user has to
/// see.
///
/// ⚖️ Deliberately NOT the whole census. Every hop of a live evaluation reshuffles `computing` and
/// `queued` among nodes that are all already busy, and paying a 188–265 ms `patch.install` of
/// `procedural.play.main` for that reshuffle costs four times per evaluation — measured at 974 ms of
/// a 2 529 ms re-evaluation, in front of the very geometry whose progress it is reporting. A progress
/// indicator may not outrank the thing whose progress it reports: the chrome is painted when it
/// APPEARS and when it CLEARS, a fault moves it whenever it lands, and the per-hop advance the user
/// actually watches is the preview's own status pill, which is republished every hop
/// (`📓️slider-latency-incremental-eval-2026-09-15.md` §3.1).
fn node_census_digest(status_json: &str) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut digest = OFFSET;
    for byte in census_chrome_marks(status_json).as_bytes() {
        digest = (digest ^ u64::from(*byte)).wrapping_mul(PRIME);
    }
    digest
}

/// 🕸️ The census reduced to what the graph body actually paints differently: one mark per widget that
/// is NOT `ok` — `e` for an error, `b` for a blocked port, `·` for merely busy — in declaration
/// order. Two censuses that differ only in WHICH busy node is the active one reduce to the same
/// marks, and an empty answer means the chrome is off.
pub fn census_chrome_marks(status_json: &str) -> String {
    let Ok(status) = semio_framework_pack_json::parse(status_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
        return status_json.to_string();
    };
    let Some(map) = status.as_object() else {
        return status_json.to_string();
    };
    let mut marks = String::new();
    for (id, entry) in map.iter() {
        match entry.get("status").and_then(semio_framework_pack_json::Value::as_str).unwrap_or("ok") {
            "ok" => continue,
            "error" => {
                marks.push('e');
                marks.push_str(entry.get("message").and_then(semio_framework_pack_json::Value::as_str).unwrap_or_default());
            }
            "blocked" => marks.push('b'),
            _ => marks.push('\u{b7}'),
        }
        marks.push_str(id);
    }
    marks
}

/// ✅️ Folds one `evaluate` answer into the retained session and settles the window's outstanding
/// answer.
///
/// 🔁️ The fold itself still arms and dispatches NOTHING. What changed is who runs the hop the settle
/// leaves owed: the answer's own route is window-addressed, so once the LAST answer of a wave lands
/// it holds everything the next wave needs — the retained session, the document, the config and the
/// addressed window's transient — and runs that wave INLINE inside this turn rather than costing a
/// `flowEvalResolve` → `flowEvalTick` pair (1 391 ms on React,
/// `📓️react-hop-latency-2026-09-14.md` §2.1). The decision is
/// [`FlowEvalSession::inline_continuation_admitted`] and the surface makes it after this fold; a
/// declined continuation leaves the latch exactly as this fold left it, which is the park — the
/// `previewEval` run job reads the same debt on its next step and dispatches the round trip.
///
/// 🚧️ An answer that could NOT be folded is a fault settle, not slow work: the host answers a
/// faulted `invokeExtension` with an empty `outputJson`, the node cache keeps no entry, and the next
/// tick would park the identical request and fault again at the host's own cadence forever. Such a
/// settle gives the window up, for the same reason [`may_rearm`] gives up an uncontributed graph, and so
/// does a `cancelled` envelope: whoever cancelled the kernel job owes nobody a continuation.
///
/// ⏱️ A budgeted `evaluate` answers an ENVELOPE: a step that spent its wall allowance without
/// finishing says `done: false` and parks its job under this same `nodeHash`, so the window stays
/// unfinished and owes one more identical hop (`📓️extension-evaluate-budget-2026-09-12.md`).
pub fn resolve_eval(payload:&FlowEvalResolve,receiver:&mut semio_os_flow::FlowEvaluationReplyReceiver,session:&mut FlowEvalSession,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let address=semio_os_flow::FlowEvaluationReplyAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id};receiver.step(session,&address,cx)
}

/// 💥 Retains (or forgets) the evaluate fault one answer carried. Publication only — no arming.

/// 🧯️ A closed run reaches the geometry extension's owned Session jobs and parked evaluations through
/// the extension's own `evaluateCancel` and `tessellateCancel` capabilities — two doors, because
/// there are two registries and each names its own (`📓️preview-eval-cancellation-2026-09-12.md`).
/// Both answers land on `flowTessellateCancelResolve`, which arms nothing.
pub fn release_invocations(payload: &FlowEvalRelease) -> Vec<ExtensionInvocation> {
    release_invocations_for(payload, geometry_extension_address())
}

/// 🧯️ The release over an ALREADY RESOLVED geometry address — pure in both inputs, for the same
/// reason [`preview_progress_status_json_for`] is. An unaddressable extension has no actor to tell.
pub fn release_invocations_for(payload: &FlowEvalRelease, address: Result<String, semio_os_flow::FlowExtensionAddressMiss>) -> Vec<ExtensionInvocation> {
    let Ok(address) = address else {
        return Vec::new();
    };
    vec![
        ExtensionInvocation::new(address.clone(), "evaluateCancel", FlowEvalSession::preview_eval_cancel_invocation_request_json(&payload.window_id, &payload.window_kind_id), "flowTessellateCancelResolve"),
        ExtensionInvocation::new(address, "tessellateCancel", FlowEvalSession::preview_cancel_invocation_request_json(&payload.window_id, &payload.window_kind_id), "flowTessellateCancelResolve"),
    ]
}

/// 🧯️ Pending cancellation continues bounded retirement through the same response chain.
pub fn resolve_tessellate_cancel(payload: &FlowTessellateCancelResolve, session: &mut FlowEvalSession) -> Vec<ExtensionInvocation> {
    resolve_tessellate_cancel_for(payload, session, geometry_extension_address())
}

/// 🔓️ A resolved address keeps cancellation independent of any new evaluator or task registry.
pub fn resolve_tessellate_cancel_for(payload: &FlowTessellateCancelResolve, session: &mut FlowEvalSession, address: Result<String, semio_os_flow::FlowExtensionAddressMiss>) -> Vec<ExtensionInvocation> {
    let _ = session;
    if !payload.ok { eprintln!("flowTessellateCancelResolve: geometry extension refused the cancel for window {}", payload.window_id); return Vec::new(); }
    let pending=payload.output.get("ok").and_then(semio_framework_value::DslValue::as_bool)==Some(true)&&payload.output.get("pending").and_then(semio_framework_value::DslValue::as_bool)==Some(true);
    let Ok(address) = address else { return Vec::new() };
    if !pending { return Vec::new(); }
    vec![ExtensionInvocation::new(address, "evaluateCancel", FlowEvalSession::preview_eval_cancel_invocation_request_json(&payload.window_id, &payload.window_kind_id), "flowTessellateCancelResolve")]
}

/// ✅️ Folds one budgeted `tessellate` round trip into the retained session and settles the window's
/// outstanding answer. The tick that parked it left the window unfinished, so once its LAST answer
/// lands the terminal hop is owed — the one that finds whatever handle is still untessellated, or
/// finds nothing and records the window finished.
///
/// 🔁️ That hop is the one a mesh body pays PER CHUNK, so it is the same round trip
/// [`resolve_eval`] now runs inline and for the same reason: this route is window-addressed too, and
/// the surface continues the chain here whenever
/// [`FlowEvalSession::inline_continuation_admitted`] says the turn still has room.
pub fn resolve_tessellate(payload:&FlowTessellateResolve,receiver:&mut semio_os_flow::FlowPreviewCompletedReceiver,session:&mut FlowEvalSession,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let address=semio_os_flow::FlowPreviewCompletedAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id,handle:&payload.handle,tolerance:payload.tolerance};session.resolve_preview_tessellate(receiver,&address,cx)
}
//#endregion ⏱️Tick
