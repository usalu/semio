"""W2A-3: deferred reprojection turns are bounded by a wall deadline first, an operation cap second (one compile-atomic wave)."""
import pathlib, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
DEFERRED = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs"
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TIME_TRAVEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"
TIME_TRAVEL_TESTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"


def patch(path, edits):
    text = path.read_text(encoding="utf-8")
    for old, new, count in edits:
        found = text.count(old)
        if found != count:
            sys.exit(f"{path.name}: expected {count} of {old[:80]!r}, found {found}")
        text = text.replace(old, new)
    return text


store = patch(STORE, [
    ("""/// ⏩️ Where one advance of a deferred reprojection left it.
enum ReprojectionAdvance {
    Pending(ReplayProgress),
    Adopted(Vec<crate::os_spr::EditMessages>),
}
""", """/// ⏩️ Where one advance of a deferred reprojection left it.
enum ReprojectionAdvance {
    Pending(ReplayProgress),
    Adopted(Vec<crate::os_spr::EditMessages>),
}

/// ⏱️ How far one turn of a deferred reprojection replays (design §7: ≤ 4 ms slices): until the turn's wall deadline on
/// `now_us` — `wall_us` after the turn began, or the runtime's own turn deadline ([`ArtifactStore::step_reprojection`]) —
/// and at most `operations` operations, whichever comes first; every turn replays at least one operation. `now_us` is the
/// job clock ([`semio_framework_job::default_now_us`]); laws inject their own.
#[derive(Clone, Copy, Debug)]
pub struct ReplayTurnBudget {
    pub wall_us: u64,
    pub operations: usize,
    pub now_us: fn() -> Option<u64>,
}

impl ReplayTurnBudget {
    /// 🕓️ `wall_us` of the job clock per turn, with `operations` as the secondary cap.
    pub fn wall(wall_us: u64, operations: usize) -> Self {
        Self { wall_us, operations, now_us: semio_framework_job::default_now_us }
    }

    /// 🔢️ `operations` per turn and no wall limit: what a law that counts operations replays with.
    pub fn operations(operations: usize) -> Self {
        Self { wall_us: u64::MAX, operations, now_us: semio_framework_job::default_now_us }
    }

    /// ⌛️ The turn's deadline on `now_us`: the runtime's `deadline_us`, else `wall_us` from now (now when the clock cannot
    /// be read); `None` without a wall limit.
    fn turn_deadline(&self, deadline_us: Option<u64>) -> Option<u64> {
        deadline_us.or_else(|| (self.wall_us != u64::MAX).then(|| (self.now_us)().map_or(0, |now| now.saturating_add(self.wall_us))))
    }

    /// 🛎️ Whether a turn that replayed `replayed` operations ends: its cap is reached or its deadline passed on `now_us`
    /// (an unreadable clock ends it).
    fn turn_ends(&self, replayed: usize, deadline_us: Option<u64>) -> bool {
        replayed >= self.operations || deadline_us.is_some_and(|deadline| (self.now_us)().is_none_or(|now| now >= deadline))
    }
}
""", 1),
    ("""    /// 🐢️ Operations one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    replay_budget: Option<usize>,
    /// ⏱️ Operations one turn of a deferred local history step replays; `None` replays inside the dispatch.
    local_replay_budget: Option<usize>,
""", """    /// 🐢️ How far one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    replay_budget: Option<ReplayTurnBudget>,
    /// ⏱️ How far one turn of a deferred local history step replays; `None` replays inside the dispatch.
    local_replay_budget: Option<ReplayTurnBudget>,
""", 1),
    ("""    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// in budgets of at most `operations` per turn ([`Self::step_reprojection`], also every [`Self::tick`]) instead of inside
    /// the ingest that admitted it, so a long downstream replay never freezes this replica. Until the change is adopted the
    /// replica shows the history before it, and the adoption is exactly the one an ingest without a budget makes. `None`
    /// replays inside the ingest.
    pub fn defer_remote_replays(&mut self, operations: Option<usize>) {
        self.replay_budget = operations.filter(|operations| *operations > 0);
    }
""", """    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// one `budget` per turn ([`ReplayTurnBudget`]: a wall deadline first, an operation cap second;
    /// [`Self::step_reprojection`], also every [`Self::tick`]) instead of inside the ingest that admitted it, so a long
    /// downstream replay never freezes this replica. Until the change is adopted the replica shows the history before it,
    /// and the adoption is exactly the one an ingest without a budget makes. `None` replays inside the ingest.
    pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }
""", 1),
    ("""    /// 🐌️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — in budgets
    /// of at most `operations` per turn instead of inside its dispatch, which replays the first budget and answers. Until""", """    /// 🐌️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — one
    /// `budget` per turn ([`ReplayTurnBudget`]) instead of inside its dispatch, which replays the first turn and answers. Until""", 1),
    ("""    pub fn defer_local_replays(&mut self, operations: Option<usize>) {
        self.local_replay_budget = operations.filter(|operations| *operations > 0);
    }
""", """    pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.local_replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }
""", 1),
    ("""    /// ⏭️ Advances the deferred reprojection by one budget of operations and adopts it once its replay finished, announcing
    /// a local step's transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection().await? {""", """    /// ⏭️ Advances the deferred reprojection by one turn — until `deadline_us` on the budget's clock when the runtime names
    /// its turn deadline, else one [`ReplayTurnBudget`] — and adopts it once its replay finished, announcing a local step's
    /// transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection(deadline_us).await? {""", 1),
    ("""    /// 🧮️ One budget of the deferred reprojection: (re)starts its Report replay when the history it started from changed,
    /// steps it, and adopts the change with the finished replay; a change that needs no replay is adopted at once. A
    /// refused adoption drops the change — a refused local step only, its remote transitions wait again — and answers the
    /// refusal.
    async fn advance_reprojection(&mut self) -> Result<Option<ReprojectionAdvance>, VcsError> {""", """    /// 🧮️ One turn of the deferred reprojection (until `deadline_us`, else one [`ReplayTurnBudget`]): (re)starts its Report
    /// replay when the history it started from changed, steps it, and adopts the change with the finished replay; a change
    /// that needs no replay is adopted at once. A refused adoption drops the change — a refused local step only, its remote
    /// transitions wait again — and answers the refusal.
    async fn advance_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReprojectionAdvance>, VcsError> {""", 1),
    ("""        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(usize::MAX);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let mut left = budget;
        let step = replay.step(&self.envelope.vcs.edits, &mut || {
            left = left.saturating_sub(1);
            left == 0
        });
""", """        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
        let deadline = budget.turn_deadline(deadline_us);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let mut replayed = 0usize;
        let step = replay.step(&self.envelope.vcs.edits, &mut || {
            replayed += 1;
            budget.turn_ends(replayed, deadline)
        });
""", 1),
    ("""        drop(pending.replay.take());
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),""", """        drop(pending.replay.take());
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection(None).await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),""", 1),
    ("""        self.clock = clock;
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Adopted(_)) => self.bump().map(|()| None),""", """        self.clock = clock;
        match self.advance_reprojection(None).await? {
            Some(ReprojectionAdvance::Adopted(_)) => self.bump().map(|()| None),""", 1),
    ("""        let pending = self.pending_reprojection.is_some();
        let progress = self.step_reprojection().await?;""", """        let pending = self.pending_reprojection.is_some();
        let progress = self.step_reprojection(None).await?;""", 1),
])

deferred = patch(DEFERRED, [
    ("""async fn replica(id: &str, n: Option<i32>, log: &[crate::os_spr::MutationEnvelope], budget: Option<usize>) -> ArtifactStore<DemoSnapshot, DemoMutation> {""",
     """async fn replica(id: &str, n: Option<i32>, log: &[crate::os_spr::MutationEnvelope], budget: Option<usize>) -> ArtifactStore<DemoSnapshot, DemoMutation> {""", 1),
    ("""    store.defer_remote_replays(budget);
    store
}""", """    store.defer_remote_replays(budget.map(ReplayTurnBudget::operations));
    store
}""", 1),
    ("store.step_reprojection().await", "store.step_reprojection(None).await", 5),
    ("deferred.step_reprojection().await", "deferred.step_reprojection(None).await", 1),
    ("""    store.defer_local_replays(budget);
    store
}""", """    store.defer_local_replays(budget);
    store
}""", 1),
    ("""async fn long_history(id: &str, budget: Option<usize>) -> ArtifactStore<DemoSnapshot, CountedOp> {""",
     """async fn long_history(id: &str, budget: Option<ReplayTurnBudget>) -> ArtifactStore<DemoSnapshot, CountedOp> {""", 1),
    ("long_history(\"local-deferred\", Some(LOCAL_BUDGET))", "long_history(\"local-deferred\", Some(ReplayTurnBudget::operations(LOCAL_BUDGET)))", 1),
    ("long_history(\"local-discard\", Some(LOCAL_BUDGET))", "long_history(\"local-discard\", Some(ReplayTurnBudget::operations(LOCAL_BUDGET)))", 1),
    ("long_history(\"local-blocked\", Some(LOCAL_BUDGET))", "long_history(\"local-blocked\", Some(ReplayTurnBudget::operations(LOCAL_BUDGET)))", 1),
])

plugin = patch(PLUGIN, [
    ("store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));", "store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));", 2),
    ("store.defer_local_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));", "store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));", 2),
])

time_travel = patch(TIME_TRAVEL, [
    ("""/// 📡️ Operations one reactor turn replays of a history change before the document store adopts it — a remote change or
/// this replica's own interior undo/redo, checkout or alternative switch (design §16.6, gap N17;
/// `ArtifactStore::defer_remote_replays`, `ArtifactStore::defer_local_replays`).
pub const TIME_TRAVEL_REPLAY_OPERATIONS: usize = 256;
""", """/// 📡️ Most operations one reactor turn replays of a history change before the document store adopts it — a remote change
/// or this replica's own interior undo/redo, checkout or alternative switch (design §16.6, gap N17) — the cap behind the
/// turn's wall budget ([`time_travel_replay_turn_budget`]).
pub const TIME_TRAVEL_REPLAY_OPERATIONS: usize = 256;

/// ⏱️ One reactor turn of a deferred reprojection: [`TIME_TRAVEL_TURN_WALL_US`] of the job clock first, at most
/// [`TIME_TRAVEL_REPLAY_OPERATIONS`] operations second (`ArtifactStore::defer_remote_replays`,
/// `ArtifactStore::defer_local_replays`).
pub fn time_travel_replay_turn_budget() -> store::ReplayTurnBudget {
    store::ReplayTurnBudget::wall(TIME_TRAVEL_TURN_WALL_US, TIME_TRAVEL_REPLAY_OPERATIONS)
}
""", 1),
    ("let stepped = self.store.step_reprojection().await;", "let stepped = self.store.step_reprojection(None).await;", 1),
])

time_travel_tests = patch(TIME_TRAVEL_TESTS, [
    ("remote.store.defer_remote_replays(Some(1));", "remote.store.defer_remote_replays(Some(store::ReplayTurnBudget::operations(1)));", 1),
])

for path, text in ((STORE, store), (DEFERRED, deferred), (PLUGIN, plugin), (TIME_TRAVEL, time_travel), (TIME_TRAVEL_TESTS, time_travel_tests)):
    path.write_text(text, encoding="utf-8")
print("W2A-3 wave applied")
