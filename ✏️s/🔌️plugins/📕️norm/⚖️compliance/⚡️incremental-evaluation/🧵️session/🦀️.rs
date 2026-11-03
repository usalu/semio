//! 🧵️ Evaluation session: retained memo per unit, merged report, progress and cancellable runs (sync and async).

use super::*;

/// 📊️ In-flight evaluation state for the transient lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EvalProgress {
    pub running: Vec<UnitId>,
    pub done: u32,
    pub total: u32,
    pub cancelled: bool,
    pub running_permille: u16,
}

/// ✅️ One finished plan entry; `rerun` is false when the read hash proved the memo still valid.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitDone {
    pub id: UnitId,
    pub rerun: bool,
    pub checks: Vec<CheckResult>,
}

/// 📣️ Events a run emits while it works.
#[derive(Clone, Debug, PartialEq)]
pub enum SessionEvent {
    Tick { unit: UnitId, fraction: f32 },
    Done(UnitDone),
    Progress(EvalProgress),
}

/// 🏁️ How a run ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunOutcome {
    Completed,
    Cancelled,
}

enum Advance {
    Continue,
    Finished,
    Cancelled,
}

struct Tick<'a> {
    unit: &'a UnitId,
    sink: &'a mut dyn FnMut(SessionEvent),
}

impl Progress for Tick<'_> {
    fn report(&mut self, fraction: f32) {
        (self.sink)(SessionEvent::Tick { unit: self.unit.clone(), fraction });
    }
}

struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}


/// 📦️ One unit run detached from the session lock: the worker takes it, runs it without holding the session, and hands the results back through `complete`.
pub struct Job<D> {
    unit: CheckUnit<D>,
    hash: u64,
    document: D,
    token: CancelToken,
}

impl<D> Job<D> {
    pub fn id(&self) -> &UnitId {
        &self.unit.id
    }
}

enum Fetched<D> {
    Job(Job<D>),
    Done(RunOutcome),
}

/// 🧠️ Retained incremental evaluator for one open document: `update` plans, `run`/`run_async` execute, `report` is always the ordered merge of all memos.
pub struct EvaluationSession<S: UnitSource> {
    document: S::Document,
    value: Arc<DslValue>,
    hashes: ReadHashes,
    units: Vec<CheckUnit<S::Document>>,
    graph: EntityGraph,
    memo: HashMap<UnitId, (u64, Vec<CheckResult>)>,
    stale: HashSet<UnitId>,
    handle: CancelHandle,
    report: Arc<CheckReport>,
    merged: u64,
    revision: u64,
    serial: u64,
    in_flight: bool,
    progress: EvalProgress,
    source: PhantomData<fn() -> S>,
}

impl<S: UnitSource> EvaluationSession<S> {
    pub fn open(document: S::Document) -> Self {
        let mut session = Self {
            value: Arc::new(DslValue::Null),
            hashes: ReadHashes::default(),
            document,
            units: Vec::new(),
            graph: EntityGraph::default(),
            memo: HashMap::new(),
            stale: HashSet::new(),
            handle: CancelHandle::default(),
            report: Arc::new(CheckReport::default()),
            merged: 0,
            revision: 0,
            serial: 0,
            in_flight: false,
            progress: EvalProgress::default(),
            source: PhantomData,
        };
        session.rebuild();
        session
    }

    fn rebuild(&mut self) {
        self.value = shared_value_tree(&self.document);
        self.hashes = ReadHashes::default();
        self.units = S::units(&self.document);
        self.graph = EntityGraph::from_edges(S::references(&self.document));
        let live: HashSet<&UnitId> = self.units.iter().map(|unit| &unit.id).collect();
        self.memo.retain(|id, _| live.contains(id));
        self.stale.retain(|id| live.contains(id));
        let missing: Vec<UnitId> = self.units.iter().filter(|unit| !self.memo.contains_key(&unit.id)).map(|unit| unit.id.clone()).collect();
        self.stale.extend(missing);
        self.merge();
    }

    fn merge(&mut self) {
        let mut report = CheckReport::default();
        for unit in &self.units {
            if let Some((_, results)) = self.memo.get(&unit.id) {
                report.extend(results.iter().cloned());
            }
        }
        let summary = S::summarise(&self.document, &report);
        report.extend(summary);
        S::OWNERS.file(&self.value, &mut report);
        self.report = Arc::new(report);
        self.merged = self.revision;
    }

    pub fn cancel_handle(&self) -> CancelHandle {
        self.handle.clone()
    }

    pub fn document(&self) -> &S::Document {
        &self.document
    }

    pub fn report(&self) -> &CheckReport {
        &self.report
    }

    /// 🔗️ The merged report as one shared handle: every reader of the same revision holds the same allocation, nothing is copied per paint or tick.
    pub fn shared_report(&self) -> Arc<CheckReport> {
        self.report.clone()
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn progress(&self) -> &EvalProgress {
        &self.progress
    }

    pub fn stale(&self) -> &HashSet<UnitId> {
        &self.stale
    }

    pub fn is_stale(&self, id: &UnitId) -> bool {
        self.stale.contains(id)
    }

    /// 📥️ Adopts the mutated document, supersedes any in-flight run (latest wins) and returns the plan for the changed set.
    pub fn update(&mut self, document: S::Document, changed: &ChangedSet, focus: &[EntityRef]) -> Plan {
        let token = self.handle.supersede();
        self.serial += 1;
        self.in_flight = false;
        self.document = document;
        self.rebuild();
        self.plan(changed, focus, token)
    }

    /// 🗺️ Plans without adopting a new document (initial full run: `ChangedSet::all()`).
    pub fn plan(&self, changed: &ChangedSet, focus: &[EntityRef], token: CancelToken) -> Plan {
        plan_units(&self.units, &self.graph, changed, &self.stale, focus, token)
    }

    pub fn token(&self) -> CancelToken {
        self.handle.token()
    }

    fn advance(&mut self, plan: &Plan, cursor: &mut usize, sink: &mut dyn FnMut(SessionEvent)) -> Advance {
        if plan.token.is_cancelled() {
            return Advance::Cancelled;
        }
        let Some(planned) = plan.units.get(*cursor) else {
            return Advance::Finished;
        };
        let current = &self.units[planned.index];
        let hash = self.hashes.of(&self.value, &current.reads);
        let valid = !self.stale.contains(&current.id) && self.memo.get(&current.id).is_some_and(|(known, _)| *known == hash);
        let mut done = UnitDone { id: current.id.clone(), rerun: !valid, checks: Vec::new() };
        if !valid {
            let unit = current.clone();
            self.progress.running = vec![unit.id.clone()];
            sink(SessionEvent::Progress(self.progress.clone()));
            let results = (unit.run)(&self.document, &mut Tick { unit: &unit.id, sink }, &plan.token);
            if plan.token.is_cancelled() {
                return Advance::Cancelled;
            }
            done.checks = results.clone();
            self.memo.insert(unit.id.clone(), (hash, results));
            self.stale.remove(&unit.id);
            self.revision += 1;
        }
        *cursor += 1;
        self.progress.done = *cursor as u32;
        self.progress.running.clear();
        sink(SessionEvent::Done(done));
        sink(SessionEvent::Progress(self.progress.clone()));
        Advance::Continue
    }

    fn begin(&mut self, plan: &Plan) {
        self.progress = EvalProgress { running: Vec::new(), done: 0, total: plan.units.len() as u32, cancelled: false, running_permille: 0 };
    }

    fn finish(&mut self, outcome: RunOutcome, sink: &mut dyn FnMut(SessionEvent)) -> RunOutcome {
        self.progress.running.clear();
        self.progress.cancelled = outcome == RunOutcome::Cancelled;
        if self.merged != self.revision {
            self.merge();
        }
        sink(SessionEvent::Progress(self.progress.clone()));
        outcome
    }


    fn begin_cheap(&mut self, plan: &Plan, cursor: &mut usize, sink: &mut dyn FnMut(SessionEvent)) -> Option<RunOutcome> {
        self.begin(plan);
        while plan.units.get(*cursor).is_some_and(|planned| planned.tier != CostTier::T2) {
            match self.advance(plan, cursor, sink) {
                Advance::Continue => {}
                Advance::Finished => return Some(RunOutcome::Completed),
                Advance::Cancelled => return Some(RunOutcome::Cancelled),
            }
        }
        None
    }

    fn fetch(&mut self, plan: &Plan, cursor: &mut usize) -> Fetched<S::Document> {
        loop {
            if plan.token.is_cancelled() {
                return Fetched::Done(RunOutcome::Cancelled);
            }
            let Some(planned) = plan.units.get(*cursor) else {
                return Fetched::Done(RunOutcome::Completed);
            };
            let current = &self.units[planned.index];
            let hash = self.hashes.of(&self.value, &current.reads);
            if !self.stale.contains(&current.id) && self.memo.get(&current.id).is_some_and(|(known, _)| *known == hash) {
                *cursor += 1;
                self.progress.done = *cursor as u32;
                continue;
            }
            let unit = current.clone();
            self.progress.running = vec![unit.id.clone()];
            self.progress.running_permille = 0;
            return Fetched::Job(Job { unit, hash, document: self.document.clone(), token: plan.token.clone() });
        }
    }

    fn complete(&mut self, job: &Job<S::Document>, results: Vec<CheckResult>, cursor: &mut usize) -> bool {
        if job.token.is_cancelled() {
            return false;
        }
        self.memo.insert(job.unit.id.clone(), (job.hash, results));
        self.stale.remove(&job.unit.id);
        self.revision += 1;
        *cursor += 1;
        self.progress.done = *cursor as u32;
        self.progress.running.clear();
        self.merge();
        true
    }

    /// 🔄️ Whether every unit is current, so the merged report is exactly the full evaluation of the document.
    pub fn is_settled(&self) -> bool {
        self.stale.is_empty() && self.progress.running.is_empty()
    }

    /// ▶️ Runs the plan to completion or until its token is cancelled; finished units are kept, the rest stay stale.
    pub fn run(&mut self, plan: &Plan, sink: &mut dyn FnMut(SessionEvent)) -> RunOutcome {
        self.begin(plan);
        let mut cursor = 0;
        let outcome = loop {
            match self.advance(plan, &mut cursor, sink) {
                Advance::Continue => {}
                Advance::Finished => break RunOutcome::Completed,
                Advance::Cancelled => break RunOutcome::Cancelled,
            }
        };
        self.finish(outcome, sink)
    }

    /// ⏩️ Async variant of [`Self::run`]: yields to the executor between units so a runtime stays responsive.
    pub async fn run_async(&mut self, plan: &Plan, sink: &mut (dyn FnMut(SessionEvent) + Send)) -> RunOutcome {
        self.begin(plan);
        let mut cursor = 0;
        let outcome = loop {
            match self.advance(plan, &mut cursor, sink) {
                Advance::Continue => YieldNow(false).await,
                Advance::Finished => break RunOutcome::Completed,
                Advance::Cancelled => break RunOutcome::Cancelled,
            }
        };
        self.finish(outcome, sink)
    }
}


struct SharedTick<'a, S: UnitSource>(&'a std::sync::Mutex<EvaluationSession<S>>);

impl<S: UnitSource> Progress for SharedTick<'_, S> {
    fn report(&mut self, fraction: f32) {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).progress.running_permille = (fraction.clamp(0.0, 1.0) * 1000.0) as u16;
    }
}

/// 🧵️ Handle of a background evaluation: the slow (T2) units run on a worker thread, the editor never waits for them; `cancel` supersedes the run, `join` waits for its outcome.
pub struct Worker(std::thread::JoinHandle<RunOutcome>);

impl Worker {
    pub fn join(self) -> RunOutcome {
        self.0.join().unwrap_or(RunOutcome::Cancelled)
    }

    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
}

/// 🪢️ Single evaluation path shared by editor and viewer: every read of a document goes through `report_for`, which re-evaluates incrementally when the retained session is behind, so no manual evaluate step exists.
pub struct SharedEvaluation<S: UnitSource>(std::sync::Mutex<EvaluationSession<S>>);

impl<S: UnitSource> SharedEvaluation<S>
where
    S::Document: PartialEq,
{
    pub fn new(document: S::Document) -> Self {
        Self(std::sync::Mutex::new(EvaluationSession::open(document)))
    }

    fn settle(session: &mut EvaluationSession<S>, plan: &Plan) -> Arc<CheckReport> {
        session.run(plan, &mut |_| {});
        session.shared_report()
    }

    /// ✍️ Committed edit: supersedes in-flight work, re-runs only the affected units and returns the settled report.
    pub fn commit(&self, document: S::Document, changed: &ChangedSet, focus: &[EntityRef]) -> Arc<CheckReport> {
        let mut session = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let plan = session.update(document, changed, focus);
        Self::settle(&mut session, &plan)
    }

    /// 🖼️ Report for exactly `document`: served from the memo when the session is current, otherwise caught up with a full-change plan whose unchanged units are skipped by read hash.
    pub fn report_for(&self, document: &S::Document) -> Arc<CheckReport> {
        let mut session = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let plan = if session.document() == document {
            let token = session.token();
            session.plan(&ChangedSet::default(), &[], token)
        } else {
            session.update(document.clone(), &ChangedSet::all(), &[])
        };
        Self::settle(&mut session, &plan)
    }


    fn lock(&self) -> std::sync::MutexGuard<'_, EvaluationSession<S>> {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🧵️ Non-blocking edit: cheap units settle inline, T2 units continue on a worker thread that holds the session only between units; `report_now` serves the partially settled report meanwhile and the next `commit*`/`report_for`/`cancel` supersedes the worker (latest wins).
    pub fn commit_background(self: &Arc<Self>, document: S::Document, changed: &ChangedSet, focus: &[EntityRef]) -> Worker
    where
        S: 'static,
        S::Document: 'static,
    {
        let (plan, serial) = {
            let mut session = self.lock();
            let plan = session.update(document, changed, focus);
            session.in_flight = true;
            (plan, session.serial)
        };
        let shared = self.clone();
        Worker(std::thread::spawn(move || {
            let mut cursor = 0;
            let early = {
                let mut session = shared.lock();
                (session.serial == serial).then(|| session.begin_cheap(&plan, &mut cursor, &mut |_| {})).flatten()
            };
            let outcome = early.unwrap_or_else(|| loop {
                let fetched = shared.lock().fetch(&plan, &mut cursor);
                let job = match fetched {
                    Fetched::Done(outcome) => break outcome,
                    Fetched::Job(job) => job,
                };
                let results = (job.unit.run)(&job.document, &mut SharedTick(&shared.0), &job.token);
                if !shared.lock().complete(&job, results, &mut cursor) {
                    break RunOutcome::Cancelled;
                }
            });
            let mut session = shared.lock();
            if session.serial == serial {
                session.in_flight = false;
                session.finish(outcome, &mut |_| {})
            } else {
                RunOutcome::Cancelled
            }
        }))
    }

    /// 🧵️ Background catch-up for exactly `document`: `None` when the session is already settled on it or a worker is already settling it (repeated paints never restart a running worker).
    pub fn background(self: &Arc<Self>, document: &S::Document) -> Option<Worker>
    where
        S: 'static,
        S::Document: 'static,
    {
        let current = {
            let session = self.lock();
            session.document() == document && (session.is_settled() || session.in_flight)
        };
        (!current).then(|| self.commit_background(document.clone(), &ChangedSet::all(), &[]))
    }

    /// 👀️ The merged report as it stands, without running anything (units still pending keep their previous results).
    pub fn report_now(&self) -> Arc<CheckReport> {
        self.lock().shared_report()
    }

    /// ✅️ Whether the retained report is the full evaluation of the latest document.
    pub fn settled(&self) -> bool {
        self.lock().is_settled()
    }

    pub fn cancel_handle(&self) -> CancelHandle {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).cancel_handle()
    }

    /// 📊️ Progress of the latest run, for the transient lane.
    pub fn progress(&self) -> EvalProgress {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).progress().clone()
    }

    /// 🔢️ Number of unit runs the session has finished.
    pub fn revision(&self) -> u64 {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).revision()
    }
}
