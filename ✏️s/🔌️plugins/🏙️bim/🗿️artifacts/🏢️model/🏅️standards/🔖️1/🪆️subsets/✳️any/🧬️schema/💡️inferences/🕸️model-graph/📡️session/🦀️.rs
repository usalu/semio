//! 📡️ `ModelInferenceSession`: the ONE inference path of editor, viewer, imports and exports. It holds the enabled `InferenceCache`, the last node values and the `ModelInference` projected from
//! them. A run is stepped and cancellable: [`ModelInferenceSession::begin`] opens a [`SessionRun`], [`ModelInferenceSession::step`] spends a fuel budget of node computes on
//! `protocol::step_field_update` (nodes the diff and their parents did not move are carried, not walked) and reports a monotonic progress fraction, [`SessionRun::cancel`] stops it and [`ModelInferenceSession::finish`] adopts a finished run. A cancelled or failed
//! run changes nothing the consumers read (the held inference stays the last complete one) but every node it finished stays in the cache, so the next run only computes what is left.
//! `update`/`refresh`/`sync` are the same run driven to the end; faults are values (`try_*` returns them, the plain calls keep the last inference and report the fault).

use super::compute::take_computed;
use super::projection::{apply, project, rebuild_diagnostics, retract};
use super::{kinds, ModelGraph, ModelNode, ModelValue, READS};
use crate::{ModelDiff, ModelInference, ModelSnapshot};
use protocol::{step_field_update, DiffRegions, MutationDiff, FieldChange, FieldDelta, FieldUpdate, InferenceCache, InferenceCacheConfig, InferenceError, InferenceFault, InferenceSession, TouchedPaths};
use std::collections::BTreeMap;

/// 📊️ What the last run did: the proof an edit recomputed only what it touched.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UpdateReport {
    pub gated: bool,
    pub cancelled: bool,
    pub fault: Option<InferenceError>,
    pub nodes: usize,
    pub computed: usize,
    pub reused: usize,
    pub carried: usize,
    pub confirmed: usize,
    pub computed_by_kind: BTreeMap<&'static str, usize>,
}

/// 📈️ Where a run stands: finished nodes of the plan and a fraction that never decreases.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RunProgress {
    pub completed: usize,
    pub total: usize,
    pub fraction: f32,
    pub done: bool,
}

struct Everything;

impl DiffRegions for Everything {
    fn touches(&self) -> TouchedPaths {
        TouchedPaths::new(READS.iter().copied())
    }
}

struct Touches(TouchedPaths);

impl DiffRegions for Touches {
    fn touches(&self) -> TouchedPaths {
        self.0.clone()
    }
}

type Update = FieldUpdate<ModelNode, ModelValue>;
type Delta = FieldDelta<ModelNode, ModelValue>;

/// 🧷️ One resumable update of a session: the engine update (the held node values with their dependency links, moved into the run while it walks the plan, carrying every node the diff and its parents did not move) and the counters of what it
/// computed. It borrows nothing, so a job can hold it across steps.
pub struct SessionRun {
    touches: Touches,
    update: Option<Update>,
    cancelled: bool,
    gated: bool,
    fault: Option<InferenceError>,
    computed_by_kind: BTreeMap<&'static str, usize>,
    epoch: u64,
}

impl SessionRun {
    fn new(gated: bool, touches: TouchedPaths, epoch: u64) -> Self {
        Self { touches: Touches(touches), update: None, cancelled: false, gated, fault: None, computed_by_kind: BTreeMap::new(), epoch }
    }

    /// 🛑️ Stops the run: later steps refuse with `InferenceError::Cancelled`; the nodes finished so far stay in the session's cache.
    pub fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(update) = self.update.as_mut() {
            update.cancel();
        }
    }

    /// 🛑️ Whether the run was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// 🧮️ The nodes this run finished (computed, served by the cache, confirmed or carried).
    pub fn finished(&self) -> usize {
        self.update.as_ref().map_or(0, Update::completed)
    }

    /// 📈️ Where the run stands.
    pub fn progress(&self) -> RunProgress {
        if self.gated {
            return RunProgress { completed: 0, total: 0, fraction: 1.0, done: true };
        }
        match self.update.as_ref() {
            None => RunProgress::default(),
            Some(update) => {
                let done = update.is_done();
                RunProgress { completed: update.completed(), total: update.total(), fraction: if done { 1.0 } else { update.fraction() }, done }
            }
        }
    }
}

/// 🧠️ The inference state of one open document.
pub struct ModelInferenceSession {
    cache: InferenceCache,
    engine: InferenceSession,
    nodes: usize,
    inference: ModelInference,
    report: UpdateReport,
    previous: Option<ModelSnapshot>,
    pending: Option<ModelDiff>,
    stale: bool,
    epoch: u64,
}

impl Default for ModelInferenceSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelInferenceSession {
    /// 🆕️ A session with an enabled cache of 256 MiB.
    pub fn new() -> Self {
        Self::with_cache_budget(256 * 1024 * 1024)
    }

    /// 🆕️ A session whose cache holds at most `bytes`; evicted nodes are recomputed, never wrong.
    pub fn with_cache_budget(bytes: usize) -> Self {
        let cache = ::semio_framework_async::poll::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: bytes, ..InferenceCacheConfig::default() }));
        let engine = ::semio_framework_async::poll::resolve_ready(InferenceSession::new());
        Self { cache, engine, nodes: 0, inference: ModelInference::default(), report: UpdateReport::default(), previous: None, pending: None, stale: false, epoch: 0 }
    }

    /// 💡️ What the last complete run produced (the default inference before the first).
    pub fn inference(&self) -> &ModelInference {
        &self.inference
    }

    /// 📊️ What the last run did, finished or not.
    pub fn report(&self) -> &UpdateReport {
        &self.report
    }

    /// 📝️ Adds the concrete diff of mutations emitted since the last run to the pending sum that [`sync`](Self::sync) trusts.
    pub fn record(&mut self, diff: ModelDiff) {
        match self.pending.as_mut() {
            Some(pending) => pending.absorb(diff),
            None => self.pending = Some(diff),
        }
    }

    /// 🚪️ Opens a run whose diff is `diff`: a diff that touches none of the graph's reads on a settled session is gated, the stored result stands and the plan is not walked.
    pub fn begin(&self, diff: &impl DiffRegions) -> SessionRun {
        take_computed();
        let touches = diff.touches();
        SessionRun::new(!self.stale && self.nodes > 0 && !touches.intersects_any(READS), touches, self.epoch)
    }

    /// 🚪️ Opens a run that always walks the plan, so it is correct without any diff (the cache keeps it incremental).
    pub fn begin_full(&self) -> SessionRun {
        self.begin(&Everything)
    }

    /// 🚪️ Opens the run that brings the session to `snapshot` from the diffs [`record`](Self::record)ed since the last run: the same snapshot is gated, recorded diffs are trusted only
    /// when applying them to the held snapshot yields exactly `snapshot`, any other change (undo, a peer's edit, a load, a cancelled run) walks the plan.
    pub fn begin_sync(&mut self, snapshot: &ModelSnapshot) -> SessionRun {
        let pending = self.pending.take();
        match (self.previous.as_ref(), pending) {
            (Some(previous), _) if !self.stale && previous == snapshot => self.begin(&ModelDiff::default()),
            (Some(previous), Some(pending)) if !self.stale && protocol::apply_diff(&pending, previous).is_ok_and(|applied| &applied == snapshot) => self.begin(&pending),
            _ => self.begin_full(),
        }
    }

    /// ⏩️ Spends at most `fuel` node computes of `run` on `snapshot` (cache hits are free) and answers where the run stands. `snapshot` must be the same for every step of one run.
    pub fn step(&mut self, run: &mut SessionRun, snapshot: &ModelSnapshot, fuel: usize) -> Result<RunProgress, InferenceError> {
        if run.gated {
            return Ok(run.progress());
        }
        if run.cancelled {
            run.fault = Some(InferenceError::Cancelled);
            return Err(InferenceError::Cancelled);
        }
        let update = run.update.get_or_insert_with(|| self.engine.begin_update::<ModelSnapshot, ModelGraph<{ kinds::ALL }>, _>(&run.touches, true));
        let outcome = step_field_update::<ModelSnapshot, ModelGraph<{ kinds::ALL }>>(snapshot, &mut self.cache, update, fuel);
        for (kind, count) in take_computed() {
            *run.computed_by_kind.entry(kind).or_insert(0) += count;
        }
        match outcome {
            Ok(_) => Ok(run.progress()),
            Err(error) => {
                run.fault = Some(error.clone());
                Err(error)
            }
        }
    }

    /// 🏁️ Ends `run`: a finished run is adopted and the held inference moves to `snapshot`; a cancelled or failed run changes nothing the consumers read, the session stays unsettled
    /// and the nodes the run finished stay cached.
    pub fn finish(&mut self, run: SessionRun, snapshot: &ModelSnapshot) -> Result<&ModelInference, InferenceError> {
        let SessionRun { update, cancelled, gated, fault, computed_by_kind, epoch, .. } = run;
        let computed: usize = computed_by_kind.values().sum();
        let ended = if gated || update.as_ref().is_some_and(Update::is_done) {
            None
        } else {
            Some(fault.unwrap_or_else(|| if cancelled { InferenceError::Cancelled } else { InferenceError::Compute { key: "model-graph".into(), fault: InferenceFault::new("model-session-unfinished", "the run was finished before it was done") } }))
        };
        if let Some(error) = ended {
            self.stale = true;
            self.engine = ::semio_framework_async::poll::resolve_ready(InferenceSession::new());
            self.report = UpdateReport { cancelled: matches!(error, InferenceError::Cancelled), fault: Some(error.clone()), nodes: self.nodes, computed, reused: update.as_ref().map_or(0, Update::completed).saturating_sub(computed), computed_by_kind, ..UpdateReport::default() };
            return Err(error);
        }
        if epoch != self.epoch {
            self.report = UpdateReport { nodes: self.nodes, computed, reused: self.nodes.saturating_sub(computed), computed_by_kind, ..UpdateReport::default() };
            return Ok(&self.inference);
        }
        self.epoch += 1;
        let delta = update.map(|update| self.engine.finish_update(update)).filter(|delta| !delta.gated);
        let (carried, confirmed) = delta.as_ref().map_or((0, 0), |delta| (delta.carried, delta.confirmed));
        if let Some(delta) = delta {
            self.adopt(delta);
        }
        self.previous = Some(snapshot.clone());
        self.stale = false;
        self.report = UpdateReport { gated, cancelled: false, fault: None, nodes: self.nodes, computed, reused: self.nodes.saturating_sub(computed), carried, confirmed, computed_by_kind };
        Ok(&self.inference)
    }

    /// 🔁️ Brings the held inference up to `snapshot`, which `diff` produced from the snapshot of the previous run, driving the run to the end.
    pub fn try_update(&mut self, snapshot: &ModelSnapshot, diff: &ModelDiff) -> Result<&ModelInference, InferenceError> {
        let run = self.begin(diff);
        self.complete(run, snapshot)
    }

    /// 🔁️ The same without a diff: always walks the plan, so it is always correct.
    pub fn try_refresh(&mut self, snapshot: &ModelSnapshot) -> Result<&ModelInference, InferenceError> {
        let run = self.begin_full();
        self.complete(run, snapshot)
    }

    /// 🔁️ The same from the recorded diffs (see [`begin_sync`](Self::begin_sync)).
    pub fn try_sync(&mut self, snapshot: &ModelSnapshot) -> Result<&ModelInference, InferenceError> {
        let run = self.begin_sync(snapshot);
        self.complete(run, snapshot)
    }

    /// 🔁️ [`try_update`](Self::try_update) that keeps the last complete inference when the run fails; the fault is in [`report`](Self::report).
    pub fn update(&mut self, snapshot: &ModelSnapshot, diff: &ModelDiff) -> &ModelInference {
        let _ = self.try_update(snapshot, diff);
        &self.inference
    }

    /// 🔁️ [`try_refresh`](Self::try_refresh) that keeps the last complete inference when the run fails; the fault is in [`report`](Self::report).
    pub fn refresh(&mut self, snapshot: &ModelSnapshot) -> &ModelInference {
        let _ = self.try_refresh(snapshot);
        &self.inference
    }

    /// 🔁️ [`try_sync`](Self::try_sync) that keeps the last complete inference when the run fails; the fault is in [`report`](Self::report).
    pub fn sync(&mut self, snapshot: &ModelSnapshot) -> &ModelInference {
        let _ = self.try_sync(snapshot);
        &self.inference
    }

    /// 🔭️ The kinds `WANT` selects of a model that is not the document (a probe), over this session's cache; the held state is untouched.
    pub fn probe<const WANT: u32>(&mut self, snapshot: &ModelSnapshot) -> Result<ModelInference, InferenceError> {
        protocol::try_infer_field::<ModelSnapshot, ModelGraph<WANT>>(snapshot, Some(&mut self.cache)).map(project)
    }

    fn complete(&mut self, mut run: SessionRun, snapshot: &ModelSnapshot) -> Result<&ModelInference, InferenceError> {
        while run.fault.is_none() && !run.progress().done {
            if self.step(&mut run, snapshot, usize::MAX).is_err() {
                break;
            }
        }
        self.finish(run, snapshot)
    }

    fn adopt(&mut self, delta: Delta) {
        let Some(values) = self.engine.field_values::<ModelSnapshot, ModelGraph<{ kinds::ALL }>>() else { return };
        self.nodes = values.len();
        if self.stale || self.previous.is_none() {
            self.inference = project(values.clone());
            return;
        }
        let mut diagnostics_moved = false;
        for FieldChange { key, old } in delta.changes {
            let Some(value) = values.get(&key) else {
                diagnostics_moved |= matches!(key, ModelNode::Diagnostics(_));
                if let Some(old) = old {
                    retract(&mut self.inference, &old);
                }
                continue;
            };
            if old.as_ref().is_some_and(|old| old.same(value)) {
                continue;
            }
            if let Some(old) = old.filter(|_| matches!(key, ModelNode::Room(_))) {
                retract(&mut self.inference, &old);
            }
            diagnostics_moved |= matches!(key, ModelNode::Diagnostics(_));
            apply(&mut self.inference, value.clone());
        }
        if diagnostics_moved {
            self.inference.diagnostics = rebuild_diagnostics(values.values());
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
