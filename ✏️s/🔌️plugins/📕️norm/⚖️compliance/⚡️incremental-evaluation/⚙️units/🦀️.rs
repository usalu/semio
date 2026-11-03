//! ⚙️ Check units: the smallest independently re-runnable verification, plus progress and cooperative cancellation.

use super::*;

/// 🪪️ Stable unit identity `(check family, entity id)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnitId(pub String);

impl UnitId {
    pub fn new(check_family: &str, entity_id: &str) -> Self {
        Self(format!("{check_family}:{entity_id}"))
    }
}

/// 🧷️ Reference to one document entity (`kind` is the collection, `id` the raw entity id).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityRef {
    pub kind: String,
    pub id: String,
}

impl EntityRef {
    pub fn new(kind: &str, id: &str) -> Self {
        Self { kind: kind.to_owned(), id: id.to_owned() }
    }
}

/// 💰️ Cost tier: T0 closed-form under 1 ms, T1 under 50 ms, T2 heavy with progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostTier {
    T0,
    T1,
    T2,
}

/// 📶️ Progress sink handed to a running unit.
pub trait Progress {
    fn report(&mut self, fraction: f32);
}

/// 🔇️ Progress sink that discards ticks.
pub struct NullProgress;

impl Progress for NullProgress {
    fn report(&mut self, _fraction: f32) {}
}

/// 🛑️ Owner side of cooperative cancellation: superseding or cancelling flips every token handed out before.
#[derive(Clone, Debug, Default)]
pub struct CancelHandle(Arc<AtomicU64>);

impl CancelHandle {
    pub fn token(&self) -> CancelToken {
        CancelToken { current: self.0.clone(), generation: self.0.load(Ordering::Acquire) }
    }

    pub fn cancel(&self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }

    pub fn supersede(&self) -> CancelToken {
        self.cancel();
        self.token()
    }
}

/// 🚦️ Observer side of cooperative cancellation; units poll it at their own checkpoints.
#[derive(Clone, Debug)]
pub struct CancelToken {
    current: Arc<AtomicU64>,
    generation: u64,
}

impl CancelToken {
    pub fn never() -> Self {
        CancelHandle::default().token()
    }

    pub fn is_cancelled(&self) -> bool {
        self.current.load(Ordering::Acquire) != self.generation
    }
}

/// 🏃️ Unit body: pure over the document, reporting progress and polling the token.
pub type UnitRun<D> = Arc<dyn Fn(&D, &mut dyn Progress, &CancelToken) -> Vec<CheckResult> + Send + Sync>;

/// 🧱️ One re-runnable verification keyed `(check family, entity id)`; it may read only what its `reads` declare.
pub struct CheckUnit<D> {
    pub id: UnitId,
    pub subjects: Vec<EntityRef>,
    pub reads: ReadSet,
    pub tier: CostTier,
    pub run: UnitRun<D>,
}

impl<D> Clone for CheckUnit<D> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), subjects: self.subjects.clone(), reads: self.reads.clone(), tier: self.tier, run: self.run.clone() }
    }
}

impl<D> CheckUnit<D> {
    pub fn new(id: UnitId, subjects: Vec<EntityRef>, reads: ReadSet, tier: CostTier, run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        Self::streaming(id, subjects, reads, tier, move |document, _, _| run(document))
    }

    pub fn streaming(id: UnitId, subjects: Vec<EntityRef>, reads: ReadSet, tier: CostTier, run: impl Fn(&D, &mut dyn Progress, &CancelToken) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        Self { id, subjects, reads, tier, run: Arc::new(run) }
    }

    pub fn execute(&self, document: &D) -> Vec<CheckResult> {
        (self.run)(document, &mut NullProgress, &CancelToken::never())
    }
}

/// 🔗️ A reference edge: `from` refers to `to` (a member refers to its section).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEdge {
    pub from: EntityRef,
    pub to: EntityRef,
}

/// 🧩️ Source of check units and entity references for one document type; full evaluation is the ordered fold of its units.
pub trait UnitSource {
    type Document: Clone + PartialEq + ToValue + FromValue + Send + 'static;

    /// 🧭️ Who owns the checks of this source: after the fold every check is filed under its owning entity, documented document-level checks under none (`Owners::file`).
    const OWNERS: Owners = Owners::ROWS;

    fn units(document: &Self::Document) -> Vec<CheckUnit<Self::Document>>;

    fn references(_document: &Self::Document) -> Vec<EntityEdge> {
        Vec::new()
    }

    /// 🧾️ Aggregate checks derived from the merged unit results (a governing-case summary); runs after the fold and is cheap by contract.
    fn summarise(_document: &Self::Document, _report: &CheckReport) -> Vec<CheckResult> {
        Vec::new()
    }
}

/// 🧮️ Full evaluation as the ordered fold of all units (the reference every incremental run must equal).
pub fn evaluate_units<S: UnitSource + ?Sized>(document: &S::Document) -> CheckReport {
    let mut report = CheckReport::default();
    S::units(document).iter().for_each(|unit| report.extend(unit.execute(document)));
    let summary = S::summarise(document, &report);
    report.extend(summary);
    S::OWNERS.file(&document.to_value(), &mut report);
    report
}

/// 🧭️ `report`, a full evaluation of `document`, with every check filed under its owner exactly as the units of `S` do.
pub fn owned<S: UnitSource + ?Sized>(document: &S::Document, mut report: CheckReport) -> CheckReport {
    S::OWNERS.file(&document.to_value(), &mut report);
    report
}
