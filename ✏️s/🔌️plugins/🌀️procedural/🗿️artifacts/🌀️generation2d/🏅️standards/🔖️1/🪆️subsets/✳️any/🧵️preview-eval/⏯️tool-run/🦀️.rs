//! ⏯️ The read-only `previewEval` tool run (`📋️tool-run-contract.md` §2.4, §3.7): its declaration, the
//! observation and scheduling laws over the retained [`FlowEvalSession`]s, the surface-owned run link
//! and the run job. The hop transport it drives lives in the parent `🧵️preview-eval` module.

use super::*;

/// ⏯️ The tool id of the read-only preview evaluation run.
pub const PREVIEW_EVAL_TOOL_ID: &str = "previewEval";

/// 📜️ Source of record of the run's declaration (`🔣️.json` beside this file).
pub const PREVIEW_EVAL_RUN_RECORD_JSON: &str = include_str!("../🔣️.json");

/// 🪜️ The one stage of a generation2d preview evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalRunStage {
    Evaluate,
}

impl PreviewEvalRunStage {
    pub const ALL: [Self; 1] = [Self::Evaluate];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Evaluate => "evaluate",
        }
    }
}

/// 🔢️ The run's counters, in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewEvalRunCounter {
    Evaluated,
    Failed,
    Blocked,
    Hops,
}

impl PreviewEvalRunCounter {
    pub const ALL: [Self; 4] = [Self::Evaluated, Self::Failed, Self::Blocked, Self::Hops];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Evaluated => "evaluated",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
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
    Settled,
    SettledWithFailures,
    ExtensionFault,
}

impl PreviewEvalRunReason {
    pub const ALL: [Self; 8] = [Self::Queued, Self::Computing, Self::Evaluated, Self::Blocked, Self::Failed, Self::Settled, Self::SettledWithFailures, Self::ExtensionFault];

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
            Self::Settled => "settled",
            Self::SettledWithFailures => "settledWithFailures",
            Self::ExtensionFault => "extensionFault",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Queued | Self::Computing => ToolRunVerdict::Testing,
            Self::Evaluated | Self::Settled => ToolRunVerdict::Success,
            Self::Blocked | Self::SettledWithFailures => ToolRunVerdict::Warning,
            Self::Failed | Self::ExtensionFault => ToolRunVerdict::Danger,
        }
    }

    /// 🔀️ How far from settled a node reason is: when two previews observe the same node, the one
    /// further from done names it.
    fn unsettledness(self) -> u8 {
        match self {
            Self::Computing => 5,
            Self::Queued => 4,
            Self::Failed | Self::ExtensionFault => 3,
            Self::Blocked | Self::SettledWithFailures => 2,
            Self::Evaluated | Self::Settled => 1,
        }
    }

    fn is_settled(self) -> bool {
        !matches!(self, Self::Queued | Self::Computing)
    }
}

fn preview_eval_run_record() -> serde_json::Value {
    serde_json::from_str(PREVIEW_EVAL_RUN_RECORD_JSON).expect("the preview evaluation run record is valid JSON")
}

/// ⏯️ The run declaration, deserialized from the source of record: read-only, because it evaluates
/// into retained sessions and never authors a document mutation; a document or settings change
/// restarts it, because an evaluation of a moved graph is a different result and each session's
/// incremental baseline makes the restart cost only the dirty nodes.
pub fn preview_eval_run_definition() -> ToolRunDefinition {
    serde_json::from_value(preview_eval_run_record()["definition"].clone()).expect("the preview evaluation run definition matches the tool run schema")
}

/// 🛠️ The `previewEval` tool: declared on the app, referenced by the modes that mount a preview.
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

/// 🔎️ One observation of the evaluation: every node's merged reason and the settled/total pair summed
/// over every observed target.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalObservation {
    pub nodes: BTreeMap<String, PreviewEvalRunReason>,
    pub settled: u64,
    pub total: u64,
}

impl PreviewEvalObservation {
    pub fn count(&self, reason: PreviewEvalRunReason) -> u64 {
        self.nodes.values().filter(|candidate| **candidate == reason).count() as u64
    }

    pub fn stage(&self) -> PreviewEvalRunStage {
        PreviewEvalRunStage::Evaluate
    }

    /// 📊️ Settled node evaluations of all node evaluations.
    pub fn progress(&self) -> (u64, u64) {
        (self.settled, self.total)
    }

    /// 🔎️ Folds one target's published per-widget status in — pure, so the verdict law is a fixture table.
    pub fn observe(&mut self, status_json: &str) {
        let Ok(status) = dsl::json::parse(status_json) else { return };
        let Some(widgets) = status.as_object() else { return };
        for (widget_id, entry) in widgets.iter() {
            let reason = match entry.get("status").and_then(dsl::json::Value::as_str) {
                Some("ok") => PreviewEvalRunReason::Evaluated,
                Some("computing") => PreviewEvalRunReason::Computing,
                Some("error") => PreviewEvalRunReason::Failed,
                Some("blocked") => PreviewEvalRunReason::Blocked,
                _ => PreviewEvalRunReason::Queued,
            };
            self.total += 1;
            self.settled += u64::from(reason.is_settled());
            self.nodes.entry(widget_id.to_string()).and_modify(|held| *held = if reason.unsettledness() > held.unsettledness() { reason } else { *held }).or_insert(reason);
        }
    }
}

/// 🔎️ Reads one observation off every target's published per-widget status, in order.
pub fn observe_preview_eval(status_jsons: &[&str]) -> PreviewEvalObservation {
    let mut observation = PreviewEvalObservation::default();
    for status_json in status_jsons {
        observation.observe(status_json);
    }
    observation
}

/// 🧳️ The two retained sessions a generation2d editor instance evaluates into.
pub struct PreviewEvalSessions<'a> {
    pub document: &'a mut FlowEvalSession,
    pub generation: &'a mut FlowEvalSession,
}

impl PreviewEvalSessions<'_> {
    pub fn get(&self, target: PreviewEvalTarget) -> &FlowEvalSession {
        match target {
            PreviewEvalTarget::Document => self.document,
            PreviewEvalTarget::Generation => self.generation,
        }
    }

    pub fn get_mut(&mut self, target: PreviewEvalTarget) -> &mut FlowEvalSession {
        match target {
            PreviewEvalTarget::Document => self.document,
            PreviewEvalTarget::Generation => self.generation,
        }
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

/// 🦶️ The scheduling law, read off the per-window latches of each window's own target session.
pub fn next_preview_eval_hop(sessions: &PreviewEvalSessions<'_>, windows: &[(String, &'static str, PreviewEvalTarget)]) -> PreviewEvalHop {
    if let Some(index) = windows.iter().position(|(window_id, _, target)| sessions.get(*target).window_tick_owed(window_id)) {
        return PreviewEvalHop::Dispatch(index);
    }
    if windows.iter().any(|(window_id, _, target)| sessions.get(*target).window_tick_is_armed(window_id) || sessions.get(*target).window_extensions_in_flight(window_id) > 0) {
        PreviewEvalHop::Wait
    } else {
        PreviewEvalHop::Settled
    }
}

/// 🔗 The surface-owned half of the run, held by the instance operation owner beside the sessions: the
/// attached preview roster the job schedules over, the live job's port its folds wake, the run the last
/// evaluation settled under and the digest each target last evaluated.
#[derive(Default)]
pub struct PreviewEvalRunLink {
    pub(crate) windows: Vec<(String, &'static str, PreviewEvalTarget)>,
    pub(crate) port: Option<ToolRunJobPort>,
    pub(crate) settled: Option<(u64, u32)>,
    pub(crate) restart_owed: bool,
    /// ⏯️ The run action this link has asked for and not yet seen answered, with the run identity it
    /// named: a start or a finalize is a REQUEST the host takes several polls to turn into a run state,
    /// so without the latch every refresh asks again.
    pub(crate) requested: Option<(PreviewEvalRunRequest, Option<(u64, u32)>)>,
    /// 🧹️ The run whose job currently OWNS this link: a start retires the previous entry, so the old
    /// job's close runs after the new job installed its port, and must not quiesce the fresh evaluation.
    pub(crate) job_run: Option<u64>,
    /// 🔢️ The digest of the text each target's last hop evaluated ([`preview_eval_digest`]).
    pub(crate) evaluated: BTreeMap<PreviewEvalTarget, u64>,
}

/// ⏯️ A run action a surface has asked the host for, and the reading of the run view that answers it.
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
    pub fn windows(&self) -> &[(String, &'static str, PreviewEvalTarget)] {
        &self.windows
    }

    /// ⏰️ Wakes the live run job, if any — every fold that changed a session calls it.
    pub fn wake(&self) {
        if let Some(port) = self.port.as_ref() {
            port.wake();
        }
    }

    /// 🔢️ Records the digest of the text `target`'s hop just evaluated.
    pub fn note_evaluated(&mut self, target: PreviewEvalTarget, digest: u64) {
        self.evaluated.insert(target, digest);
    }

    /// 🧹️ Whether the job of `run` is still the one this link belongs to.
    pub(crate) fn owned_by(&self, run: u64) -> bool {
        self.job_run == Some(run)
    }

    fn attach_windows(&mut self, windows: &[PreviewEvalWindow<'_>]) {
        if self.windows.len() != windows.len() || self.windows.iter().zip(windows).any(|(held, attached)| held.0 != attached.0 || held.1 != attached.1 || held.2 != attached.2) {
            self.windows = windows.iter().map(|(window_id, kind, target)| ((*window_id).to_string(), *kind, *target)).collect();
        }
    }
}

/// 🧳️ What the run needs from the surface's retained instance operation owner.
pub trait PreviewEvalRunOwner: std::any::Any + Send {
    fn preview_eval_parts(&mut self) -> Option<(PreviewEvalSessions<'_>, &mut PreviewEvalRunLink)>;
}

pub(crate) fn run_action_effect(action: &str, args: dsl::DslValue) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(PREVIEW_EVAL_HOP_REQUEST), action: action.into(), args: Some(args), delay_ms: 0 }
}

/// 🔢️ Owes every attached window whose target moved since its last evaluated digest one evaluation.
pub fn owe_moved_targets(sessions: &mut PreviewEvalSessions<'_>, link: &PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>], current: &BTreeMap<PreviewEvalTarget, u64>) {
    for (window_id, _, target) in windows {
        if link.evaluated.get(target) != current.get(target) {
            sessions.get_mut(*target).note_window_tick_outcome(window_id, true);
        }
    }
}

/// 🚦️ What the surface's `pending_effects` owes the run for the attached `windows`: owe every window
/// whose target moved under it (`current` digests), start a run once a window owes an evaluation,
/// finalize a complete run as soon as it is seen (the next poll then starts the fresh one), and wake a
/// live run so it sees a changed roster. An aborted run stays down until a gesture or a moved document
/// owes its windows again.
pub fn preview_eval_run_effects(mut sessions: PreviewEvalSessions<'_>, link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>], current: &BTreeMap<PreviewEvalTarget, u64>, run: Option<&ToolRunView>, servable: bool) -> Vec<Effect> {
    for target in [PreviewEvalTarget::Document, PreviewEvalTarget::Generation] {
        sessions.get_mut(target).retain_window_tick_latches(&windows.iter().filter(|(_, _, window_target)| *window_target == target).map(|(window_id, _, _)| *window_id).collect::<Vec<_>>());
    }
    owe_moved_targets(&mut sessions, link, windows, current);
    link.attach_windows(windows);
    link.wake();
    if windows.is_empty() || !servable {
        return Vec::new();
    }
    let owed = windows.iter().any(|(window_id, _, target)| sessions.get(*target).window_tick_owed(window_id));
    let run = run.filter(|run| run.tool_id == PREVIEW_EVAL_TOOL_ID);
    let state = run.map(|run| run.state);
    let identity = run.map(|run| (run.identity.id.run, run.identity.generation));
    if link.requested.is_some_and(|(requested, asked)| requested.answered_by(state) || asked.is_some_and(|asked| Some(asked) != identity)) {
        link.requested = None;
    }
    if link.requested.is_some() {
        return Vec::new();
    }
    let (request, effects) = match state {
        Some(ToolRunState::Complete) => {
            let run = run.expect("a complete run is present");
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PreviewEvalRunPhase {
    Running,
    Complete,
}

/// ⏯️ The `previewEval` run job (contract §3.7). One unit of fuel is one evaluation hop handed to the
/// host; between hops the job waits on its port, so a paused run's single step advances exactly one
/// hop. Each step re-observes the attached targets' sessions and upserts a trace record for every node
/// whose reason changed; the run completes once no attached window owes or awaits anything. Closing
/// quiesces the sessions it was running.
pub struct PreviewEvalRunJob<O: PreviewEvalRunOwner> {
    owner: ArtifactInstanceOperationOwnerHandle,
    port: ToolRunJobPort,
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
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
    pub fn new(owner: ArtifactInstanceOperationOwnerHandle, port: ToolRunJobPort, identity: ToolRunIdentity) -> Result<Self, Fault> {
        owner.with_mut::<O, _>(|held| {
            let (mut sessions, link) = held.preview_eval_parts().ok_or_else(|| Fault::from("generation2d-preview-eval-session-closing"))?;
            for (window_id, _, target) in &link.windows {
                sessions.get_mut(*target).note_window_tick_outcome(window_id, true);
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
        let values = [self.observation.count(PreviewEvalRunReason::Evaluated), self.observation.count(PreviewEvalRunReason::Failed), self.observation.count(PreviewEvalRunReason::Blocked), self.hops];
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
        let failed = self.observation.count(PreviewEvalRunReason::Failed) + self.observation.count(PreviewEvalRunReason::Blocked);
        let stage = self.observation.stage().index();
        let nodes = self.observation.nodes.len() as u64;
        let _ = if failed > 0 {
            self.writer.step(ToolRunStepKind::Warning, stage, PreviewEvalRunReason::SettledWithFailures.code(), None, &[ToolRunStepArg::Unsigned(failed), ToolRunStepArg::Unsigned(nodes)])
        } else {
            self.writer.step(ToolRunStepKind::Success, stage, PreviewEvalRunReason::Settled.code(), None, &[ToolRunStepArg::Unsigned(nodes)])
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
        let (port, identity) = (self.port.clone(), self.identity);
        let turn = self.owner.with_mut::<O, _>(|held| {
            let (mut sessions, link) = held.preview_eval_parts().ok_or_else(|| Fault::from("generation2d-preview-eval-session-closing"))?;
            let hop = next_preview_eval_hop(&sessions, &link.windows);
            match hop {
                PreviewEvalHop::Dispatch(index) => {
                    let (window_id, kind, target) = &link.windows[index];
                    if sessions.get_mut(*target).arm_window_tick(window_id) {
                        port.dispatch(tick_effect(window_id, kind));
                    }
                }
                PreviewEvalHop::Wait => port.wait(),
                PreviewEvalHop::Settled => link.settled = Some((identity.id.run, identity.generation)),
            }
            let mut targets: Vec<PreviewEvalTarget> = link.windows.iter().map(|(_, _, target)| *target).collect();
            targets.sort_unstable();
            targets.dedup();
            let observation = observe_preview_eval(&targets.iter().map(|target| sessions.get(*target).status_json()).collect::<Vec<_>>());
            let extension_fault = targets.iter().any(|target| sessions.get(*target).extension_evaluate_fault().is_some());
            Ok((hop, observation, extension_fault))
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
        let identity = self.identity;
        let released = self.owner.with_mut::<O, _>(|held| {
            let Some((mut sessions, link)) = held.preview_eval_parts() else { return Ok(()) };
            if !link.owned_by(identity.id.run) {
                return Ok(());
            }
            link.port = None;
            link.job_run = None;
            if link.settled.is_some() {
                return Ok(());
            }
            for (window_id, _, target) in &link.windows {
                sessions.get_mut(*target).cancel_preview_evaluation(window_id);
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
