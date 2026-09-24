"""🎚️ Raster standalone control credits come from an explicit pool. Production retirements claim from the process
pool (`RASTER_STANDALONE_PROCESS_CONTROLS`); the two saturation laws own a private pool each, so sibling tests that
claim and return process credits on other test threads can no longer interleave with a law that must hold its pool
exactly full (T11 §6: `raster_standalone_control_max_plus_one…` re-acquired a sibling's returned credit). A snapshot
root retirement hands its pool to the inner owned retirement it builds, so a saturated private pool stays the one pool
the whole retirement chain claims from. `--dry` reports without writing."""
import sys

root = "/Users/ueli/Documents/semio/"
base = root + "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/"
dry = "--dry" in sys.argv
problems = []


def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new, count in pairs:
        if text.count(old) != count:
            problems.append(f"{path[len(root):]}: expected {count} of {old[:90]!r}, found {text.count(old)}")
            continue
        text = text.replace(old, new)
    return text


production = edit(base + "🦀️.rs", [
    ("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n\nstruct RasterStandaloneControlCredit {\n    held_items: usize,\n    held_bytes: usize,\n}",
     """/// 🎚️ One bounded pool of standalone control credits. Production retirements claim from the process pool
/// ([`RASTER_STANDALONE_PROCESS_CONTROLS`]); a law that must hold a pool exactly full owns a private one, because the
/// process pool is shared with every other retirement in the process.
struct RasterStandaloneControlPool {
    claimed: std::sync::atomic::AtomicUsize,
    capacity: usize,
}

impl RasterStandaloneControlPool {
    const fn new(capacity: usize) -> Self {
        Self { claimed: std::sync::atomic::AtomicUsize::new(0), capacity }
    }

    fn claimed(&self) -> usize {
        self.claimed.load(std::sync::atomic::Ordering::Acquire)
    }
}

static RASTER_STANDALONE_PROCESS_CONTROLS: RasterStandaloneControlPool = RasterStandaloneControlPool::new(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);

struct RasterStandaloneControlCredit {
    pool: &'static RasterStandaloneControlPool,
    held_items: usize,
    held_bytes: usize,
}""", 1),
    ("""    fn try_claim() -> Result<Self, &'static str> {
        let current = RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire);
        let next = current.checked_add(1).ok_or("raster-store.standalone-control-overflow")?;
        if next > RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
            return Err("raster-store.standalone-control-capacity");
        }
        if RASTER_STANDALONE_PROCESS_CONTROLS.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err("raster-store.standalone-control-capacity");
        }
        Ok(Self { held_items: 1, held_bytes: RASTER_CONTROL_BACKING_BYTES })
    }""", """    fn try_claim(pool: &'static RasterStandaloneControlPool) -> Result<Self, &'static str> {
        let current = pool.claimed();
        let next = current.checked_add(1).ok_or("raster-store.standalone-control-overflow")?;
        if next > pool.capacity {
            return Err("raster-store.standalone-control-capacity");
        }
        if pool.claimed.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err("raster-store.standalone-control-capacity");
        }
        Ok(Self { pool, held_items: 1, held_bytes: RASTER_CONTROL_BACKING_BYTES })
    }""", 1),
    ("""        let current = RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire);
        let next = current.checked_sub(1).ok_or("raster-store.standalone-control-underflow")?;
        if RASTER_STANDALONE_PROCESS_CONTROLS.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {""",
     """        let current = self.pool.claimed();
        let next = current.checked_sub(1).ok_or("raster-store.standalone-control-underflow")?;
        if self.pool.claimed.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {""", 1),
    ("""    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    depth: usize,
}

impl RasterOwnedRetirement {
    fn new(owner: RasterRetirementOwner) -> Self {
        let control = RasterStandaloneControlCredit::try_claim().ok();
        Self {""", """    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    pool: &'static RasterStandaloneControlPool,
    depth: usize,
}

impl RasterOwnedRetirement {
    fn new(owner: RasterRetirementOwner) -> Self {
        Self::new_in(&RASTER_STANDALONE_PROCESS_CONTROLS, owner)
    }

    fn new_in(pool: &'static RasterStandaloneControlPool, owner: RasterRetirementOwner) -> Self {
        let control = RasterStandaloneControlCredit::try_claim(pool).ok();
        Self {""", 1),
    ("""            control: std::mem::ManuallyDrop::new(control),
            depth: 1,
        }
    }""", """            control: std::mem::ManuallyDrop::new(control),
            pool,
            depth: 1,
        }
    }""", 1),
    ("""        if self.control.is_some() {
            return Ok(true);
        }
        match RasterStandaloneControlCredit::try_claim() {""", """        if self.control.is_some() {
            return Ok(true);
        }
        match RasterStandaloneControlCredit::try_claim(self.pool) {""", 1),
    ("""    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    control_returned: bool,
}""", """    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    control_returned: bool,
    pool: &'static RasterStandaloneControlPool,
}

impl RasterSnapshotRootRetirement {
    fn new_in(pool: &'static RasterStandaloneControlPool, snapshot: std::sync::Arc<RasterSnapshot>) -> Self {
        Self {
            owner: std::mem::ManuallyDrop::new(Some(snapshot)),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            control: std::mem::ManuallyDrop::new(RasterStandaloneControlCredit::try_claim(pool).ok()),
            control_returned: false,
            pool,
        }
    }
}""", 1),
    ("""        if self.control.is_none() && !self.control_returned {
            match RasterStandaloneControlCredit::try_claim() {""", """        if self.control.is_none() && !self.control_returned {
            match RasterStandaloneControlCredit::try_claim(self.pool) {""", 1),
    ("""        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, value));""",
     """        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new_in(self.pool, RasterRetirementOwner::Snapshot(value))));""", 1),
    ("""    fn retire(&self, snapshot: std::sync::Arc<RasterSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let control = RasterStandaloneControlCredit::try_claim().ok();
        Box::new(RasterSnapshotRootRetirement {
            owner: std::mem::ManuallyDrop::new(Some(snapshot)),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            control: std::mem::ManuallyDrop::new(control),
            control_returned: false,
        })
    }""", """    fn retire(&self, snapshot: std::sync::Arc<RasterSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(RasterSnapshotRootRetirement::new_in(&RASTER_STANDALONE_PROCESS_CONTROLS, snapshot))
    }""", 1),
])

tests = edit(base + "🧪️tests/🔬️unit/🦀️.rs", [
    ("standalone: RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire),", "standalone: RASTER_STANDALONE_PROCESS_CONTROLS.claimed(),", 1),
    ("assert!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire) <= RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY, \"{what}: the standalone control pool never exceeds its declared capacity\");",
     "assert!(RASTER_STANDALONE_PROCESS_CONTROLS.claimed() <= RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY, \"{what}: the standalone control pool never exceeds its declared capacity\");", 1),
    ("""fn saturate_standalone_controls() -> (Vec<RasterOwnedRetirement>, RasterOwnedRetirement, *const u8) {
    let mut held: Vec<RasterOwnedRetirement> = Vec::with_capacity(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
    loop {
        assert!(held.len() <= RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY, "the standalone control pool refuses within its declared capacity");""",
     """fn saturate_standalone_controls(pool: &'static RasterStandaloneControlPool) -> (Vec<RasterOwnedRetirement>, RasterOwnedRetirement, *const u8) {
    let mut held: Vec<RasterOwnedRetirement> = Vec::with_capacity(pool.capacity);
    loop {
        assert!(held.len() <= pool.capacity, "the standalone control pool refuses within its declared capacity");""", 1),
    ("""        let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| RasterOwnedRetirement::new(RasterRetirementOwner::String(owner))));""",
     """        let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| RasterOwnedRetirement::new_in(pool, RasterRetirementOwner::String(owner))));""", 1),
    ("""    let (mut saturated, mut plus_one, plus_one_pointer) = saturate_standalone_controls();
    assert!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire) <= RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);""",
     """    static POOL: RasterStandaloneControlPool = RasterStandaloneControlPool::new(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
    let (mut saturated, mut plus_one, plus_one_pointer) = saturate_standalone_controls(&POOL);
    assert_eq!(POOL.claimed(), POOL.capacity, "the law's own pool is exactly full");""", 1),
    ("""    assert!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire) < RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY, "returning one standalone control frees one slot in the process pool");""",
     """    assert_eq!(POOL.claimed(), POOL.capacity - 1, "returning one standalone control frees exactly one slot in the law's own pool");""", 1),
    ("""    let (mut saturated, mut refused_probe, _) = saturate_standalone_controls();""",
     """    static POOL: RasterStandaloneControlPool = RasterStandaloneControlPool::new(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
    let (mut saturated, mut refused_probe, _) = saturate_standalone_controls(&POOL);""", 1),
    ("""    let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| store::SnapshotRetirementFactory::retire(&RasterSnapshotRetirementFactory, producer)));""",
     """    let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Box<dyn store::ErasedSnapshotRetirement> { Box::new(RasterSnapshotRootRetirement::new_in(&POOL, producer)) }));""", 1),
])
if problems:
    for p in problems: print("PROBLEM", p)
    sys.exit(1)
if not dry:
    open(base + "🦀️.rs", "w", encoding="utf-8").write(production)
    open(base + "🧪️tests/🔬️unit/🦀️.rs", "w", encoding="utf-8").write(tests)
print(("would edit" if dry else "edited") + " the raster binary module and its unit tests")
