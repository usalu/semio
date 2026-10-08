//! 🚂️ The instance-owned geometry engine: the typed inference cache, the retained base snapshot and the stepped run.

use super::value::WidgetEvaluation;
use super::{Generation3dGeometry, GeometryInput};
use crate::standards::v1::subsets::any::schema::catalogue::Catalogue;
use crate::Generation3dSnapshot;
use protocol::{infer_field_step, InferenceCache, InferenceCacheConfig, InferenceCacheStats, InferenceCursor, InferenceError};
use std::collections::BTreeMap;
use std::sync::Arc;

/// ⚖️ The bytes the engine's inference cache may hold.
pub const GEOMETRY_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// 🗃️ How a run uses the cache: `Incremental` reads and writes it, `Cold` clears it first, `Bypass` neither reads nor writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheMode {
    Incremental,
    Cold,
    Bypass,
}

/// 🚫️ Why an engine call could not advance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineError {
    NotStarted,
    Busy,
    Inference(InferenceError),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotStarted => write!(formatter, "the geometry engine has no run"),
            Self::Busy => write!(formatter, "the geometry engine is busy"),
            Self::Inference(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for EngineError {}

/// 📋️ What one engine step did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EngineStep {
    pub done: bool,
    pub fuel_used: usize,
    pub computed: usize,
    pub hits: usize,
    pub completed: usize,
    pub total: usize,
    pub fraction: f32,
}

/// 📊️ Lifetime counters of one engine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EngineTotals {
    pub runs: u64,
    pub computed: u64,
    pub hits: u64,
}

struct Base {
    digest: String,
    snapshot: Generation3dSnapshot,
}

struct Run {
    cursor: InferenceCursor<String>,
    values: BTreeMap<String, Arc<WidgetEvaluation>>,
    bypass: bool,
    cancellation_id: String,
}

impl Drop for Run {
    fn drop(&mut self) {
        self.cursor.cancel();
    }
}

/// 🚂️ The geometry engine of one artifact instance.
pub struct GeometryEngine {
    catalogue: Arc<Catalogue>,
    computes:Arc<dyn super::compute::GeometryComputeContext>,
    cache: InferenceCache,
    base: Option<Base>,
    run: Option<Run>,
    previous: BTreeMap<String, Arc<WidgetEvaluation>>,
    totals: EngineTotals,
}

impl Drop for GeometryEngine {
    fn drop(&mut self) {
        self.run = None;
        if let Some(base) = self.base.take() {
            base.snapshot.retire_cold();
        }
    }
}

impl GeometryEngine {
    /// 🏗️ An engine whose cache holds at most `budget_bytes`; a zero budget disables the cache.
    pub fn new(catalogue: Arc<Catalogue>, computes:Arc<dyn super::compute::GeometryComputeContext>, budget_bytes: usize) -> Self {
        let config = InferenceCacheConfig { enabled: budget_bytes > 0, budget_bytes, record_stats: true, ..Default::default() };
        Self { catalogue, computes, cache: semio_framework_async::poll::resolve_ready(InferenceCache::new(config)), base: None, run: None, previous: BTreeMap::new(), totals: EngineTotals::default() }
    }

    /// 🧷️ Whether the retained run of the request with this digest is open: neither finished nor replaced. A cancelled run stays open, so continuing it reports the cancellation.
    pub fn is_running(&self, digest: &str) -> bool {
        self.base.as_ref().is_some_and(|base| base.digest == digest) && self.run.as_ref().is_some_and(|run| !run.cursor.is_done())
    }

    /// 🧷️ Whether the retained base belongs to the request with this digest.
    pub fn holds(&self, digest: &str) -> bool {
        self.base.as_ref().is_some_and(|base| base.digest == digest)
    }

    /// 🚀️ Starts a run over `snapshot` (ownership moves in; it is retired when replaced), addressable for cancellation by `cancellation_id`. A run in flight is cancelled. The cache survives unless `mode` is `Cold`.
    pub fn start(&mut self, digest: String, snapshot: Generation3dSnapshot, mode: CacheMode, cancellation_id: &str) {
        self.run = None;
        if mode == CacheMode::Cold {
            semio_framework_async::poll::resolve_ready(self.cache.clear());
        }
        if let Some(displaced) = self.base.replace(Base { digest, snapshot }) {
            displaced.snapshot.retire_cold();
        }
        self.run = Some(Run { cursor: InferenceCursor::new(), values: BTreeMap::new(), bypass: mode == CacheMode::Bypass, cancellation_id: cancellation_id.to_string() });
        self.totals.runs += 1;
    }

    /// 🔁️ Restarts the retained base from its first widget; the warm cache makes unchanged widgets free.
    pub fn restart(&mut self, mode: CacheMode, cancellation_id: &str) {
        self.run = None;
        if mode == CacheMode::Cold {
            semio_framework_async::poll::resolve_ready(self.cache.clear());
        }
        self.run = Some(Run { cursor: InferenceCursor::new(), values: BTreeMap::new(), bypass: mode == CacheMode::Bypass, cancellation_id: cancellation_id.to_string() });
        self.totals.runs += 1;
    }

    /// 🪜️ Spends at most `fuel` units on the run. A finished run becomes the retained evaluation.
    pub fn step(&mut self, fuel: usize) -> Result<EngineStep, EngineError> {
        let (Some(base), Some(run)) = (self.base.as_ref(), self.run.as_mut()) else { return Err(EngineError::NotStarted) };
        let cache = if run.bypass { None } else { Some(&mut self.cache) };
        let report = infer_field_step::<GeometryInput<'_>, Generation3dGeometry>(&GeometryInput::new(&base.snapshot, Arc::clone(&self.catalogue),Arc::clone(&self.computes)), cache, &mut run.cursor, &mut run.values, fuel).map_err(EngineError::Inference)?;
        self.totals.computed += report.computed as u64;
        self.totals.hits += report.hits as u64;
        if report.done {
            self.previous = run.values.clone();
        }
        Ok(EngineStep { done: report.done, fuel_used: report.fuel_used, computed: report.computed, hits: report.hits, completed: run.cursor.completed(), total: run.cursor.total(), fraction: report.progress })
    }

    /// 🛑️ Cancels the run in flight: the in-flight widget job is cancelled and finished widgets stay valid.
    pub fn cancel(&mut self) {
        if let Some(run) = self.run.as_mut() {
            run.cursor.cancel();
        }
    }

    /// 🛑️ Cancels the run in flight when it was started under `cancellation_id`; answers whether it was.
    pub fn cancel_run(&mut self, cancellation_id: &str) -> bool {
        match self.run.as_mut().filter(|run| run.cancellation_id == cancellation_id && !run.cursor.is_done()) {
            Some(run) => {
                run.cursor.cancel();
                true
            }
            None => false,
        }
    }

    /// 📦️ The evaluation of a widget: from the current run when it got that far, else from the last finished run.
    pub fn evaluation(&self, widget: &str) -> Option<&Arc<WidgetEvaluation>> {
        self.run.as_ref().and_then(|run| run.values.get(widget)).or_else(|| self.previous.get(widget))
    }

    /// 📦️ Every evaluation of the current run, else of the last finished run.
    pub fn evaluations(&self) -> &BTreeMap<String, Arc<WidgetEvaluation>> {
        match &self.run {
            Some(run) if !run.values.is_empty() => &run.values,
            _ => &self.previous,
        }
    }

    /// 🔗️ The dependency hash of a finished widget of the current run, hex encoded.
    pub fn dependency_hash(&self, widget: &str) -> Option<String> {
        self.run.as_ref().and_then(|run| run.cursor.hash(&widget.to_string())).map(|hash| hash.0.iter().map(|byte| format!("{byte:02x}")).collect())
    }

    /// 🧮️ Finished and total widgets of the current run.
    pub fn progress(&self) -> (usize, usize) {
        self.run.as_ref().map_or((0, 0), |run| (run.cursor.completed(), run.cursor.total()))
    }

    /// 📊️ The cache's hit, miss and eviction counters.
    pub fn cache_stats(&self) -> InferenceCacheStats {
        semio_framework_async::poll::resolve_ready(self.cache.stats())
    }

    /// 📊️ Lifetime counters of the engine: runs started, widgets computed and cache hits served.
    pub fn totals(&self) -> EngineTotals {
        self.totals
    }

    /// 📏️ The bytes the cache currently holds.
    pub fn cache_bytes(&self) -> usize {
        self.cache.used_bytes()
    }

    /// 📸️ The base snapshot of the current run.
    pub fn base(&self) -> Option<&Generation3dSnapshot> {
        self.base.as_ref().map(|base| &base.snapshot)
    }
}

