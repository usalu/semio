//! ⏱️ Tick and resolve of the preview evaluation: the evaluation tick, the invocation resolution and the release hops. See [`super`].

use super::*;

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
/// ⏱️ The tick's own wall cost is recorded into `semio_framework_os_flow`'s evaluation-step ledger
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
/// ([`semio_framework_os_flow::flow_eval_tick_budget`]).
pub fn evaluate_tick(
    window_id: &str,
    window_kind_id: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
    retained_grant: semio_framework_value::RetainedCloneGrant,
) -> Result<FlowEvalTickOutcome, semio_framework_value::ValueError> {
    let started_us = semio_framework_job::runtime_diagnostics_enabled().then(semio_framework_job::default_now_us).flatten();
    // ▶️ The armed tick is now RUNNING, so its latch is free for whatever THIS tick decides to arm.
    session.begin_window_tick(window_id, retained_grant)?;
    let census_before = node_census_digest(session.status_json());
    let mut host = host_with_session(host_snapshot, session, retained_grant)?;
    let more = session.tick_cold(&mut host, turn_started_us);
    let pending_extension_evals = host.take_pending_extension_evals();
    host.retire_cold();
    let mut extension_invocations = Vec::new();
    // 🌊️ ONE WAVE, ONE HOP: every request the walk parked had its inputs ready in the same tick, so
    // no member consumes another member's output and all of them may be outstanding at once. The
    // per-window latch already counts a fan-out and hands the re-arm to whichever answer lands LAST
    // (`FlowEvalSession::settle_window_extension`), so the chain now costs one round trip per
    // dependency LEVEL rather than one per contributed node — which is what made a four-operator
    // example spend seven `flowEvalTick` hops at roughly a second apiece
    // (`📓️react-perf-ceilings-audit-2026-09-14.md` §1, §3 item 3).
    for pending in pending_extension_evals {
        let resume = session.has_evaluation_progress(pending.node_hash);
        let dependency_json = if resume { String::new() } else { geometry_dependency_json(host_snapshot, &pending.neuron_id) };
        extension_invocations.push(evaluate_invocation(pending, resume, dependency_json, window_id, window_kind_id));
    }
    if extension_invocations.is_empty() && !more {
        extension_invocations.extend(preview_tessellate_invocations(window_id, window_kind_id, session, host_snapshot, tolerance, retained_grant)?);
    }
    // ⏳️ A tick that parked extension work is emphatically NOT FINISHED, whatever the evaluation's own
    // `more` says: `!more` is exactly the branch above that parks `tessellate`, and a window recorded
    // finished while it still owes mesh round trips strands its surface at `inFlight: 1`
    // (`📓️wgpu-example-chain-2026-09-13.md` §6.1). A graph nothing in this process can serve gives up.
    session.note_window_tick_outcome(window_id, tick_is_unfinished(more, extension_invocations.len()), retained_grant)?;
    if !extension_invocations.is_empty() {
        session.note_window_extensions_in_flight(window_id, extension_invocations.len(), retained_grant)?;
    } else if more && !may_rearm(host_snapshot) {
        session.abandon_window_tick(window_id, retained_grant)?;
    }
    let publication = preview_eval_publication_for(session, host_snapshot, retained_eval);
    if let (Some(started_us), Some(finished_us)) = (started_us, started_us.and_then(|_| semio_framework_job::default_now_us())) {
        semio_framework_os_flow::record_flow_eval_step(finished_us.saturating_sub(started_us));
    }
    Ok(FlowEvalTickOutcome { extension_invocations, publication, census_moved: node_census_digest(session.status_json()) != census_before })
}

/// 📨️ The original evaluation request carries business identity and address; its caller lends execution authority through the actual context.
pub fn evaluate_invocation(pending: semio_framework_artifact_flow_flow::neural::PendingExtensionEval, resume: bool, dependency_json: String, window_id: &str, window_kind_id: &str) -> ExtensionInvocation {
    let request_json = semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([
        ("neuronId".to_string(), semio_framework_value::DslValue::String(pending.neuron_id)),
        ("operatorId".to_string(), semio_framework_value::DslValue::String(pending.operator_id.clone())),
        ("inputJson".to_string(), semio_framework_value::DslValue::String(if resume { String::new() } else { pending.input_json })),
        ("dependencyJson".to_string(), semio_framework_value::DslValue::String(dependency_json)),
        ("operatorVersion".to_string(), semio_framework_value::DslValue::String(format!("registry:{};geometry:1;policy:1", semio_framework_os_flow::flow_extension_registry_generation()))),
        ("nodeHash".to_string(), semio_framework_value::DslValue::uint(pending.node_hash)),
        ("resume".to_string(), semio_framework_value::DslValue::Bool(resume)),
        ("windowId".to_string(), semio_framework_value::DslValue::String(window_id.to_string())),
        ("windowKindId".to_string(), semio_framework_value::DslValue::String(window_kind_id.to_string())),
        ("extensionId".to_string(), semio_framework_value::DslValue::String(pending.extension_id.clone())),
    ]));
    ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve")
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
pub fn resolve_eval(payload:&FlowEvalResolve,receiver:&mut semio_framework_os_flow::FlowEvaluationReplyReceiver,session:&mut FlowEvalSession,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let address=semio_framework_os_flow::FlowEvaluationReplyAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id};receiver.step(session,&address,cx)
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
pub fn release_invocations_for(payload: &FlowEvalRelease, address: Result<String, semio_framework_os_flow::FlowExtensionAddressMiss>) -> Vec<ExtensionInvocation> {
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
pub fn resolve_tessellate_cancel_for(payload: &FlowTessellateCancelResolve, session: &mut FlowEvalSession, address: Result<String, semio_framework_os_flow::FlowExtensionAddressMiss>) -> Vec<ExtensionInvocation> {
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
pub fn resolve_tessellate(payload:&FlowTessellateResolve,receiver:&mut semio_framework_os_flow::FlowPreviewCompletedReceiver,session:&mut FlowEvalSession,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let address=semio_framework_os_flow::FlowPreviewCompletedAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id,handle:&payload.handle,tolerance:payload.tolerance};session.resolve_preview_tessellate(receiver,&address,cx)
}
//#endregion ⏱️Tick
