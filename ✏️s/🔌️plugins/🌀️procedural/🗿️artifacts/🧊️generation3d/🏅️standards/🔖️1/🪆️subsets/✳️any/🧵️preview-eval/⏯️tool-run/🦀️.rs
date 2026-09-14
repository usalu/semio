//! ⏯️ The read-only `previewEval` tool run (`📋️tool-run-contract.md` §2.4, §3.7): its declaration, the
//! observation and scheduling laws over the retained [`FlowEvalSession`], the surface-owned run link and
//! the run job. The hop and status transport it drives lives in the parent `🧵️preview-eval` module.

use super::*;

/// ⏯️ The tool id of the read-only preview evaluation run on both surfaces.
pub const PREVIEW_EVAL_TOOL_ID: &str = "previewEval";

/// 📜️ Source of record of the run's declaration (`🔣️.json` beside this file).
pub const PREVIEW_EVAL_RUN_RECORD_JSON: &str = include_str!("../🔣️.json");

/// 🪜️ The two stages of one preview evaluation, in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalRunStage {
    Evaluate,
    Tessellate,
}

impl PreviewEvalRunStage {
    pub const ALL: [Self; 2] = [Self::Evaluate, Self::Tessellate];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Evaluate => "evaluate",
            Self::Tessellate => "tessellate",
        }
    }
}

/// 🔢️ The run's counters, in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalRunCounter {
    Evaluated,
    Failed,
    Blocked,
    Tessellated,
    Hops,
}

impl PreviewEvalRunCounter {
    pub const ALL: [Self; 5] = [Self::Evaluated, Self::Failed, Self::Blocked, Self::Tessellated, Self::Hops];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Evaluated => "evaluated",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::Tessellated => "tessellated",
            Self::Hops => "hops",
        }
    }
}

/// 🗒️ Why a node carries its verdict, and the settle steps; `code` is the declaration index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreviewEvalRunReason {
    Queued,
    Computing,
    Evaluated,
    Blocked,
    Failed,
    Tessellating,
    Tessellated,
    MeshDiagnostics,
    MeshMissing,
    Settled,
    SettledWithFailures,
    ExtensionFault,
}

impl PreviewEvalRunReason {
    pub const ALL: [Self; 12] = [Self::Queued, Self::Computing, Self::Evaluated, Self::Blocked, Self::Failed, Self::Tessellating, Self::Tessellated, Self::MeshDiagnostics, Self::MeshMissing, Self::Settled, Self::SettledWithFailures, Self::ExtensionFault];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Computing => "computing",
            Self::Evaluated => "evaluated",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::Tessellating => "tessellating",
            Self::Tessellated => "tessellated",
            Self::MeshDiagnostics => "meshDiagnostics",
            Self::MeshMissing => "meshMissing",
            Self::Settled => "settled",
            Self::SettledWithFailures => "settledWithFailures",
            Self::ExtensionFault => "extensionFault",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Queued | Self::Computing | Self::Tessellating => ToolRunVerdict::Testing,
            Self::Evaluated | Self::Tessellated | Self::Settled => ToolRunVerdict::Success,
            Self::Blocked | Self::MeshDiagnostics | Self::MeshMissing | Self::SettledWithFailures => ToolRunVerdict::Warning,
            Self::Failed | Self::ExtensionFault => ToolRunVerdict::Danger,
        }
    }
}

fn preview_eval_run_record() -> serde_json::Value {
    serde_json::from_str(PREVIEW_EVAL_RUN_RECORD_JSON).expect("the preview evaluation run record is valid JSON")
}

/// ⏯️ The run declaration, deserialized from the source of record: read-only; a document or settings
/// change restarts it, because an evaluation of a moved graph is a different result and the session's
/// incremental baseline makes the restart cost only the dirty nodes.
pub fn preview_eval_run_definition() -> ToolRunDefinition {
    serde_json::from_value(preview_eval_run_record()["definition"].clone()).expect("the preview evaluation run definition matches the tool run schema")
}

/// 🛠️ The `previewEval` tool: declared on the app, referenced by no mode, started by the surface.
pub fn preview_eval_tool_definition() -> ToolDefinition {
    let record = preview_eval_run_record();
    let label: LocalizedLabel = serde_json::from_value(record["label"].clone()).expect("the preview evaluation tool label is a localized label");
    let icon = record["iconId"].as_str().unwrap_or_default().to_string();
    ToolDefinition { run: Some(preview_eval_run_definition()), ..semio_framework_plugin::resolve_ready(ToolDefinition::new(PREVIEW_EVAL_TOOL_ID, label, icon.as_str())) }
}

/// 🪪️ The trace entity of one flow node: FNV-1a 64 of its id's UTF-8 bytes, so a node graph in any
/// language highlights the node a trace record names without a lookup table.
pub fn preview_eval_node_entity(node_id: &str) -> u64 {
    node_id.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

/// 🧊️ What the session holds for one preview geometry handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalMeshState {
    Ready,
    Diagnostics,
    Pending,
}

/// 🔎️ One observation of the evaluation: every node's reason plus the preview mesh census.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalObservation {
    pub nodes: BTreeMap<String, PreviewEvalRunReason>,
    pub meshes: u64,
    pub tessellated: u64,
}

impl PreviewEvalObservation {
    pub fn count(&self, reason: PreviewEvalRunReason) -> u64 {
        self.nodes.values().filter(|candidate| **candidate == reason).count() as u64
    }

    pub fn stage(&self) -> PreviewEvalRunStage {
        if self.nodes.values().any(|reason| matches!(reason, PreviewEvalRunReason::Queued | PreviewEvalRunReason::Computing)) || self.meshes == self.tessellated {
            PreviewEvalRunStage::Evaluate
        } else {
            PreviewEvalRunStage::Tessellate
        }
    }

    /// 📊️ Settled nodes plus tessellated meshes of all nodes plus all known meshes.
    pub fn progress(&self) -> (u64, u64) {
        let settled = self.nodes.values().filter(|reason| !matches!(reason, PreviewEvalRunReason::Queued | PreviewEvalRunReason::Computing)).count() as u64;
        (settled + self.tessellated, self.nodes.len() as u64 + self.meshes)
    }
}

/// 🔎️ Reads one observation off the session's published per-widget status and evaluation — pure, so
/// the verdict law is a fixture table. A preview widget that evaluated refines into its meshes:
/// diagnostics warn, all ready succeed, otherwise it is still tessellating, or missing once `settled`.
pub fn observe_preview_eval(status_json: &str, eval_json: &str, preview_widget_ids: &[String], mesh_state: impl Fn(&str) -> PreviewEvalMeshState, settled: bool) -> PreviewEvalObservation {
    let mut observation = PreviewEvalObservation::default();
    let Ok(status) = dsl::json::parse(status_json) else { return observation };
    let Some(widgets) = status.as_object() else { return observation };
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    for (widget_id, entry) in widgets.iter() {
        let mut reason = match entry.get("status").and_then(dsl::json::Value::as_str) {
            Some("ok") => PreviewEvalRunReason::Evaluated,
            Some("computing") => PreviewEvalRunReason::Computing,
            Some("error") => PreviewEvalRunReason::Failed,
            Some("blocked") => PreviewEvalRunReason::Blocked,
            _ => PreviewEvalRunReason::Queued,
        };
        if reason == PreviewEvalRunReason::Evaluated && preview_widget_ids.iter().any(|id| id == widget_id) {
            let states: Vec<PreviewEvalMeshState> = preview_channel_items_for_widget(&eval, widget_id).into_iter().filter(|item| !item.handle.is_empty()).map(|item| mesh_state(&item.handle)).collect();
            observation.meshes += states.len() as u64;
            observation.tessellated += states.iter().filter(|state| **state == PreviewEvalMeshState::Ready).count() as u64;
            reason = if states.is_empty() {
                reason
            } else if states.contains(&PreviewEvalMeshState::Diagnostics) {
                PreviewEvalRunReason::MeshDiagnostics
            } else if states.iter().all(|state| *state == PreviewEvalMeshState::Ready) {
                PreviewEvalRunReason::Tessellated
            } else if settled {
                PreviewEvalRunReason::MeshMissing
            } else {
                PreviewEvalRunReason::Tessellating
            };
        }
        observation.nodes.insert(widget_id.to_string(), reason);
    }
    observation
}

fn preview_eval_mesh_state(session: &FlowEvalSession, handle: &str) -> PreviewEvalMeshState {
    if session.preview_diagnostics(handle).is_some() {
        PreviewEvalMeshState::Diagnostics
    } else if session.preview_mesh_pack(handle).is_some() {
        PreviewEvalMeshState::Ready
    } else {
        PreviewEvalMeshState::Pending
    }
}

/// 🦶️ What the run does next with the attached preview windows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalHop {
    /// ⏱️ The window at this roster index owes an evaluation hop nothing is chasing.
    Dispatch(usize),
    /// ⏳️ A dispatched hop or an extension answer is still outstanding.
    Wait,
    /// 🏁️ No window owes or awaits anything.
    Settled,
}

/// 🦶️ The scheduling law, read off the per-window latches the hops and answers maintain.
pub fn next_preview_eval_hop(session: &FlowEvalSession, windows: &[(String, &'static str)]) -> PreviewEvalHop {
    if let Some(index) = windows.iter().position(|(window_id, _)| session.window_tick_owed(window_id)) {
        return PreviewEvalHop::Dispatch(index);
    }
    if windows.iter().any(|(window_id, _)| session.window_tick_is_armed(window_id) || session.window_extensions_in_flight(window_id) > 0) {
        PreviewEvalHop::Wait
    } else {
        PreviewEvalHop::Settled
    }
}

/// 🔗 The surface-owned half of the run, held by the instance operation owner beside the session: the
/// attached preview roster the job schedules over, the live job's port its folds wake, and the run
/// generation the last evaluation settled under.
#[derive(Default)]
pub struct PreviewEvalRunLink {
    pub(crate) windows: Vec<(String, &'static str)>,
    pub(crate) port: Option<ToolRunJobPort>,
    pub(crate) settled: Option<(u64, u32)>,
    pub(crate) restart_owed: bool,
    /// ⏯️ The run action this link has asked for and not yet seen answered, with the run identity it
    /// named. A run action is a REQUEST — the host takes several `pending_effects` polls to turn one
    /// into a run state — so without this latch a surface re-reads the same unpaid debt on every
    /// refresh and asks again, which is how one quiet boot spent three starts in 500 ms and one busy
    /// session spent 1 053. The identity is what keeps the latch from becoming a deadlock: an action
    /// that named a generation the ledger has already moved past is refused as stale, and only a
    /// request addressed to the CURRENT identity is still worth waiting for
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
    pub(crate) requested: Option<(PreviewEvalRunRequest, Option<(u64, u32)>)>,
    /// 🧹️ The run whose job currently OWNS this link. A start retires the previous entry, so the old
    /// job's `close_step` runs AFTER the new job has already installed its own port and reset
    /// [`Self::settled`] — and a closing job that reads the new run's markers as its own quiesces the
    /// session it was never running, which paints the fresh preview `cancelled` and empties it. The
    /// closing job compares its own run id against this before touching anything
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
    pub(crate) job_run: Option<u64>,
}

/// ⏯️ A run action a surface has asked the host for, and the reading of the document's `ToolRunView`
/// that answers it. Deliberately not "the view changed": an unrelated generation bump would clear a
/// latch whose request is still outstanding, and the point of the latch is to survive exactly that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewEvalRunRequest {
    /// ▶️ Answered once the view reports a run that is no longer startable.
    Start,
    /// 🏁️ Answered once the view reports the complete run it named is complete no longer.
    Finalize,
}

impl PreviewEvalRunRequest {
    fn answered_by(self, state: Option<ToolRunState>) -> bool {
        match self {
            Self::Start => !matches!(state, None | Some(ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted)),
            Self::Finalize => state != Some(ToolRunState::Complete),
        }
    }
}

impl PreviewEvalRunLink {
    pub fn windows(&self) -> &[(String, &'static str)] {
        &self.windows
    }

    /// ⏰️ Wakes the live run job, if any — every fold that changed the session calls it.
    pub fn wake(&self) {
        if let Some(port) = self.port.as_ref() {
            port.wake();
        }
    }

    /// 🧹️ Whether the job of `run` is still the one this link belongs to.
    pub(crate) fn owned_by(&self, run: u64) -> bool {
        self.job_run == Some(run)
    }

    fn attach_windows(&mut self, windows: &[(&str, &'static str)]) {
        if self.windows.len() != windows.len() || self.windows.iter().zip(windows).any(|(held, attached)| held.0 != attached.0 || held.1 != attached.1) {
            self.windows = windows.iter().map(|(window_id, kind)| ((*window_id).to_string(), *kind)).collect();
        }
    }
}

/// 🧳️ What the run needs from a surface's retained instance operation owner.
pub trait PreviewEvalRunOwner: std::any::Any + Send {
    fn preview_eval_parts(&mut self) -> Option<(&mut FlowEvalSession, &mut PreviewEvalRunLink)>;
}

pub(crate) fn run_action_effect(action: &str, args: dsl::DslValue) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(PREVIEW_EVAL_HOP_REQUEST), action: action.into(), args: Some(args), delay_ms: 0 }
}

/// 🚦️ What a surface's `pending_effects` owes the run for the attached `windows`: start a run once a
/// window owes an evaluation, finalize a complete run whose document, settings or contributions moved
/// on (the next poll then starts the fresh one), and wake a live run so it sees a changed roster. An
/// aborted run stays down until a gesture owes its windows again.
pub fn preview_eval_run_effects(session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink, windows: &[(&str, &'static str)], run: Option<&ToolRunView>, servable: bool) -> Vec<Effect> {
    session.retain_window_tick_latches(&windows.iter().map(|(window_id, _)| *window_id).collect::<Vec<_>>());
    link.attach_windows(windows);
    link.wake();
    if windows.is_empty() || !servable {
        return Vec::new();
    }
    let owed = windows.iter().any(|(window_id, _)| session.window_tick_owed(window_id));
    let run = run.filter(|run| run.tool_id == PREVIEW_EVAL_TOOL_ID);
    // ⏯️ A start and a finalize are REQUESTS, not state changes: the document's run view keeps
    // answering what it answered before until the host has turned one into a run state. Asking once
    // per REFRESH instead of once per ANSWER is what spent three starts in 500 ms on a quiet boot and
    // 1 053 in one busy session — so the link holds its request until the view ANSWERS it, and a
    // landed gesture (`owe_attached_previews`) is the one other thing that releases it, which bounds
    // the ask at one per gesture and keeps a dropped request from wedging the preview for good
    // (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
    let state = run.map(|run| run.state);
    let identity = run.map(|run| (run.identity.id.run, run.identity.generation));
    // 🔓️ A request the ledger has already moved past is refused as stale, so waiting on it forever is a
    // deadlock; a request that named NO identity (a gesture's own start, taken before any poll) is
    // released by its answer alone.
    if link.requested.is_some_and(|(requested, asked)| requested.answered_by(state) || asked.is_some_and(|asked| Some(asked) != identity)) {
        link.requested = None;
    }
    if link.requested.is_some() {
        return Vec::new();
    }
    let (request, effects) = match state {
        Some(ToolRunState::Complete) => {
            let run = run.expect("a complete run is present");
            // 🏁️ A complete run is FINALIZED as soon as it is seen, owed or not. `Complete` means "the job
            // is done and the run awaits its finalize", and a run left standing in it makes every later
            // `toolRunStart` a no-op against the run already there — which is exactly how an inspector
            // edit could owe two windows a perfect debt and still never re-evaluate. Pairing the finalize
            // with the start in one poll does not help: the host schedules a poll's effects
            // independently, so the start lands while the run is still complete and is dropped again
            // (measured on 6018, `📓️preview-rearm-after-inspector-edit-2026-09-14.md`). Finalizing
            // promptly leaves the surface in the ONE state from which a single start — a gesture's own,
            // or the next poll's — really starts a run.
            link.restart_owed = owed;
            (Some((PreviewEvalRunRequest::Finalize, identity)), vec![run_action_effect(TOOL_RUN_FINALIZE_ACTION_ID, dsl::DslValue::object([(TOOL_RUN_ARG_RUN_ID.to_string(), dsl::DslValue::String(run.identity.id.run.to_string())), (TOOL_RUN_ARG_GENERATION.to_string(), dsl::DslValue::uint(u64::from(run.identity.generation)))]))])
        }
        None | Some(ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted) if owed || link.restart_owed => {
            link.restart_owed = false;
            (Some((PreviewEvalRunRequest::Start, identity)), vec![run_action_effect(TOOL_RUN_START_ACTION_ID, dsl::DslValue::object([(TOOL_RUN_ARG_TOOL_ID.to_string(), dsl::DslValue::String(PREVIEW_EVAL_TOOL_ID.into()))]))])
        }
        _ => (None, Vec::new()),
    };
    link.requested = request;
    effects
}

/// 🧯️ Whether the session still holds work a closing run must release.
fn preview_eval_work_outstanding(session: &FlowEvalSession, windows: &[(String, &'static str)]) -> bool {
    session.extensions_in_flight() > 0 || windows.iter().any(|(window_id, _)| session.window_tick_is_armed(window_id)) || session.preview_tessellate_status().is_cancellable() || session.preview_eval_status().is_cancellable()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PreviewEvalRunPhase {
    Running,
    Complete,
}

/// ⏯️ The `previewEval` run job (contract §3.7). One unit of fuel is one evaluation hop handed to the
/// host; between hops the job waits on its port, so a paused run's single step advances exactly one
/// hop and a run waiting on the geometry kernel costs no turn. Each step re-observes the session and
/// upserts a trace record for every node whose reason changed; the run completes once no attached
/// window owes or awaits anything. Closing quiesces the session and releases outstanding kernel work.
pub struct PreviewEvalRunJob<O: PreviewEvalRunOwner> {
    owner: ArtifactInstanceOperationOwnerHandle,
    port: ToolRunJobPort,
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    preview_widget_ids: Vec<String>,
    observed: BTreeMap<String, PreviewEvalRunReason>,
    observation: PreviewEvalObservation,
    hops: u64,
    faulted: bool,
    phase: PreviewEvalRunPhase,
    closing: bool,
    released: bool,
    owner_type: std::marker::PhantomData<fn() -> O>,
}

impl<O: PreviewEvalRunOwner> PreviewEvalRunJob<O> {
    /// 🧳️ Attaches the job's port to the owner's link and owes every attached window one evaluation, so a
    /// fresh or restarted run always evaluates the document it was started on.
    pub fn new(owner: ArtifactInstanceOperationOwnerHandle, port: ToolRunJobPort, identity: ToolRunIdentity, preview_widget_ids: Vec<String>) -> Result<Self, Fault> {
        owner.with_mut::<O, _>(|held| {
            let (session, link) = held.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-preview-eval-session-closing"))?;
            for (window_id, _) in &link.windows {
                session.note_window_tick_outcome(window_id, true);
            }
            link.port = Some(port.clone());
            link.settled = None;
            link.job_run = Some(identity.id.run);
            Ok(())
        })?;
        Ok(Self {
            owner,
            port,
            identity,
            writer: ToolRunTickWriter::new(identity),
            preview_widget_ids,
            observed: BTreeMap::new(),
            observation: PreviewEvalObservation::default(),
            hops: 0,
            faulted: false,
            phase: PreviewEvalRunPhase::Running,
            closing: false,
            released: false,
            owner_type: std::marker::PhantomData,
        })
    }

    fn record(&mut self, observation: PreviewEvalObservation) {
        for key in self.observed.keys().filter(|widget_id| !observation.nodes.contains_key(*widget_id)).map(|widget_id| preview_eval_node_entity(widget_id)).collect::<Vec<_>>() {
            self.writer.retire(key);
        }
        for (widget_id, reason) in &observation.nodes {
            if self.observed.get(widget_id) != Some(reason) {
                let entity = preview_eval_node_entity(widget_id);
                self.writer.upsert(entity, reason.verdict(), reason.code(), ToolRunTraceSubject::Entity { entity });
            }
        }
        self.observed.clone_from(&observation.nodes);
        self.observation = observation;
    }

    fn progress(&self, state: ToolRunState) -> ToolRunProgress {
        let (completed, total) = self.observation.progress();
        let values = [
            self.observation.count(PreviewEvalRunReason::Evaluated) + self.observation.count(PreviewEvalRunReason::Tessellated) + self.observation.count(PreviewEvalRunReason::Tessellating) + self.observation.count(PreviewEvalRunReason::MeshDiagnostics) + self.observation.count(PreviewEvalRunReason::MeshMissing),
            self.observation.count(PreviewEvalRunReason::Failed),
            self.observation.count(PreviewEvalRunReason::Blocked),
            self.observation.tessellated,
            self.hops,
        ];
        ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: self.observation.stage().index(),
            completed,
            total: Some(total),
            counters: PreviewEvalRunCounter::ALL.iter().zip(values).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        }
    }

    fn settle(&mut self) {
        let failed = self.observation.count(PreviewEvalRunReason::Failed) + self.observation.count(PreviewEvalRunReason::Blocked) + self.observation.count(PreviewEvalRunReason::MeshMissing) + self.observation.count(PreviewEvalRunReason::MeshDiagnostics);
        let stage = self.observation.stage().index();
        let nodes = self.observation.nodes.len() as u64;
        let _ = if failed > 0 {
            self.writer.step(ToolRunStepKind::Warning, stage, PreviewEvalRunReason::SettledWithFailures.code(), None, &[ToolRunStepArg::Unsigned(failed), ToolRunStepArg::Unsigned(nodes)])
        } else {
            self.writer.step(ToolRunStepKind::Success, stage, PreviewEvalRunReason::Settled.code(), None, &[ToolRunStepArg::Unsigned(nodes), ToolRunStepArg::Unsigned(self.observation.tessellated)])
        };
        self.phase = PreviewEvalRunPhase::Complete;
    }

    fn emit(&mut self, cx: &mut StepContext<'_>, state: ToolRunState) -> StepOutcome {
        self.writer.progress(self.progress(state));
        let payload = self.writer.finish().and_then(|tick| tick.encode().ok()).and_then(|bytes| cx.payload_from_bytes(JobPayloadStream::Preview, &bytes).map_err(|rejected| drop(rejected.into_source())).ok());
        payload.map_or_else(|| StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }), StepOutcome::PreviewReady)
    }
}

impl<O: PreviewEvalRunOwner> InteractiveJob for PreviewEvalRunJob<O> {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.phase == PreviewEvalRunPhase::Complete {
            return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) });
        }
        let (port, preview_widget_ids, identity) = (self.port.clone(), &self.preview_widget_ids, self.identity);
        let turn = self.owner.with_mut::<O, _>(|held| {
            let (session, link) = held.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-preview-eval-session-closing"))?;
            let hop = next_preview_eval_hop(session, &link.windows);
            match hop {
                PreviewEvalHop::Dispatch(index) => {
                    let (window_id, kind) = &link.windows[index];
                    if session.arm_window_tick(window_id) {
                        port.dispatch(tick_effect(window_id, kind));
                    }
                }
                PreviewEvalHop::Wait => port.wait(),
                PreviewEvalHop::Settled => link.settled = Some((identity.id.run, identity.generation)),
            }
            let observation = observe_preview_eval(session.status_json(), session.eval_json(), preview_widget_ids, |handle| preview_eval_mesh_state(session, handle), hop == PreviewEvalHop::Settled);
            Ok((hop, observation, session.extension_evaluate_fault().is_some()))
        });
        let (hop, observation, extension_fault) = match turn {
            Ok(turn) => turn,
            Err(fault) if fault.code.0 == "interactive-job.instance-owner-busy" => return StepOutcome::Yield,
            Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
        };
        self.record(observation);
        if extension_fault && !self.faulted {
            let _ = self.writer.step(ToolRunStepKind::Danger, self.observation.stage().index(), PreviewEvalRunReason::ExtensionFault.code(), None, &[]);
        }
        self.faulted = extension_fault;
        match hop {
            PreviewEvalHop::Dispatch(_) => {
                self.hops += 1;
                cx.consume_fuel(1);
                self.emit(cx, ToolRunState::Running)
            }
            PreviewEvalHop::Settled => {
                self.settle();
                self.emit(cx, ToolRunState::Complete)
            }
            PreviewEvalHop::Wait if self.writer.is_empty() => StepOutcome::Yield,
            PreviewEvalHop::Wait => self.emit(cx, ToolRunState::Running),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        if self.released {
            return InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let (port, identity) = (self.port.clone(), self.identity);
        let released = self.owner.with_mut::<O, _>(|held| {
            let Some((session, link)) = held.preview_eval_parts() else { return Ok(()) };
            if !link.owned_by(identity.id.run) {
                return Ok(());
            }
            link.port = None;
            link.job_run = None;
            if link.settled.is_some() {
                return Ok(());
            }
            let outstanding = preview_eval_work_outstanding(session, &link.windows);
            let first = link.windows.first().cloned();
            session.cancel_preview_evaluation(first.as_ref().map_or("", |(window_id, _)| window_id.as_str()));
            if let (true, Some((window_id, kind))) = (outstanding, first) {
                port.dispatch(release_effect(&window_id, kind));
            }
            Ok(())
        });
        match released {
            Err(fault) if fault.code.0 == "interactive-job.instance-owner-busy" => InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
            _ => {
                self.released = true;
                InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.released
    }
}
