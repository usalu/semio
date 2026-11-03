//! 🧱️ Row units: the shared scope and unit builder every artifact uses to split its evaluation into one unit per entity plus one unit for everything that is not row-bound.

use super::*;

/// 🎯️ Which part of an evaluation runs: everything, every non-row (global) part, one named global part (for evaluations whose verdicts are interleaved with their rows), or the checks of the row `index` of the collection `tag`; rows are addressed by position so duplicate ids stay exact.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scope<'a> {
    #[default]
    All,
    Globals,
    Part(&'a str),
    Row(&'a str, usize),
}

impl Scope<'_> {
    pub fn admits(self, tag: &str, index: usize) -> bool {
        match self {
            Scope::All => true,
            Scope::Globals | Scope::Part(_) => false,
            Scope::Row(t, i) => t == tag && i == index,
        }
    }

    pub fn globals(self) -> bool {
        matches!(self, Scope::All | Scope::Globals)
    }

    pub fn part(self, name: &str) -> bool {
        match self {
            Scope::All | Scope::Globals => true,
            Scope::Part(n) => n == name,
            Scope::Row(..) => false,
        }
    }
}

/// 🚪️ A document-wide verdict that every row unit of one unit list needs (input validation, referential integrity): computed lazily by the first unit that asks and shared by all others, so a cold pass over `n` rows costs one gate instead of `n`. A gate belongs to the unit list of ONE document revision (create it in `units`); it never outlives the revision whose unit closures hold it, and its inputs must be among the reads every unit that uses it declares.
pub struct Gate<G>(Arc<std::sync::OnceLock<G>>);

impl<G> Gate<G> {
    pub fn new() -> Self {
        Self(Arc::new(std::sync::OnceLock::new()))
    }

    /// 🚦️ The verdict, computed by `compute` on the first call only.
    pub fn get(&self, compute: impl FnOnce() -> G) -> &G {
        self.0.get_or_init(compute)
    }
}

impl<G> Clone for Gate<G> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<G> Default for Gate<G> {
    fn default() -> Self {
        Self::new()
    }
}

/// 🏗️ Ordered unit list builder: `global` then `row`s in the exact order of the full evaluation; repeated ids get a positional suffix so memo keys stay unique.
pub struct Rows<D> {
    shared: Vec<String>,
    units: Vec<CheckUnit<D>>,
    taken: HashSet<UnitId>,
}

impl<D: 'static> Rows<D> {
    pub fn new<I: IntoIterator<Item = P>, P: Into<String>>(shared: I) -> Self {
        Self { shared: shared.into_iter().map(Into::into).collect(), units: Vec::new(), taken: HashSet::new() }
    }

    /// 🧭️ Shared reads = every top-level field of `document` except the row collections in `rows` (each row declares its own entry), so a misspelled field name cannot hide a read; the field names are asked per index (`value_key_at_path`, answered by the derive without converting any value), never through a whole value tree.
    pub fn except(document: &D, rows: &[&str]) -> Self
    where
        D: ToValue,
    {
        Self::new((0..).map_while(|index| document.value_key_at_path(&[], index).ok()).filter(|key| !rows.contains(&key.as_str())))
    }

    fn push(&mut self, family: &str, id: &str, subjects: Vec<EntityRef>, reads: ReadSet, tier: CostTier, run: UnitRun<D>) {
        let mut unit_id = UnitId::new(family, id);
        let mut repeat = 1;
        while !self.taken.insert(unit_id.clone()) {
            repeat += 1;
            unit_id = UnitId::new(family, &format!("{id}#{repeat}"));
        }
        self.units.push(CheckUnit { id: unit_id, subjects, reads, tier, run });
    }

    /// 🌐️ The unit of everything that is not row-bound (integrity, gates, global verdicts); it reads the whole document.
    pub fn global(mut self, run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        self.push("global", "document", Vec::new(), ReadSet::new(["*"]), CostTier::T0, Arc::new(move |document: &D, _: &mut dyn Progress, _: &CancelToken| run(document)));
        self
    }

    /// 🌐️ A further global unit placed between rows (a verdict that sits between two row groups in the report); it reads the whole document.
    pub fn part(mut self, name: &str, run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        self.push("part", name, Vec::new(), ReadSet::new(["*"]), CostTier::T0, Arc::new(move |document: &D, _: &mut dyn Progress, _: &CancelToken| run(document)));
        self
    }

    /// 🌐️ The global unit with declared reads: it re-runs only when one of `reads` (document paths, a row collection name stands for all its rows) changes, instead of on every edit.
    pub fn global_reading(mut self, reads: &[&str], run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        self.push("global", "document", Vec::new(), ReadSet::new(reads.iter().copied()), CostTier::T0, Arc::new(move |document: &D, _: &mut dyn Progress, _: &CancelToken| run(document)));
        self
    }

    /// 🌐️ The named part unit with declared reads, see [`Rows::global_reading`].
    pub fn part_reading(mut self, name: &str, reads: &[&str], run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        self.push("part", name, Vec::new(), ReadSet::new(reads.iter().copied()), CostTier::T0, Arc::new(move |document: &D, _: &mut dyn Progress, _: &CancelToken| run(document)));
        self
    }

    /// 🌐️ The global unit with the shared reads of the row units (every field outside the row collections) plus `extra`: the row-bound paths it needs (an id column, a reference field, a whole collection). A drag of any other row field leaves it clean.
    pub fn global_sharing(self, extra: &[&str], run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        let reads: Vec<String> = self.shared.iter().cloned().chain(extra.iter().map(|path| (*path).to_owned())).collect();
        self.global_reading(&reads.iter().map(String::as_str).collect::<Vec<_>>(), run)
    }

    /// 🌐️ The named part unit with the shared reads plus `extra`, see [`Rows::global_sharing`].
    pub fn part_sharing(self, name: &str, extra: &[&str], run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        let reads: Vec<String> = self.shared.iter().cloned().chain(extra.iter().map(|path| (*path).to_owned())).collect();
        self.part_reading(name, &reads.iter().map(String::as_str).collect::<Vec<_>>(), run)
    }

    /// 📄️ The unit of one row: reads the shared paths, its own row `collection.index` and `extra`.
    pub fn row(self, family: &str, collection: &str, index: usize, id: &str, tier: CostTier, extra: &[&str], run: impl Fn(&D) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        self.slow(family, collection, index, id, tier, extra, move |document, _, _| run(document))
    }

    /// 🐢️ A row unit that reports progress and polls the cancel token (run on the worker for [`CostTier::T2`]).
    pub fn slow(mut self, family: &str, collection: &str, index: usize, id: &str, tier: CostTier, extra: &[&str], run: impl Fn(&D, &mut dyn Progress, &CancelToken) -> Vec<CheckResult> + Send + Sync + 'static) -> Self {
        let reads = ReadSet::new(self.shared.clone()).with(format!("{collection}.{index}"));
        let reads = extra.iter().fold(reads, |reads, path| reads.with(*path));
        self.push(family, id, vec![EntityRef::new(collection, id)], reads, tier, Arc::new(run));
        self
    }

    pub fn finish(self) -> Vec<CheckUnit<D>> {
        self.units
    }
}
