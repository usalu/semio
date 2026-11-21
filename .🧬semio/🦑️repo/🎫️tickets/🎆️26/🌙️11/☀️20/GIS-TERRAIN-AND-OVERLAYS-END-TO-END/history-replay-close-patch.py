from difflib import unified_diff
from pathlib import Path

store = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store")
history = store / "🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs"
admission = store / "🕰️history/📐️planning/🎟️admission/🦀️.rs"
fixture = store / "🕰️history/📐️planning/🧪️tests/🦀️.rs"
lift = store / "⚙️engine/📥️lifted/🏪️store/🦀️.rs"
hydration = store / "🧾️document/📜️history/💧️hydration/🧬️hydration-types-constructors-and-drive/🦀️.rs"
files = {file: file.read_text() for file in [history, admission, fixture, lift, hydration]}
updated = files.copy()

def replace(file, source, target, count=1):
    if updated[file].count(source) != count:
        raise RuntimeError(f"{file} no longer has {count} exact matches for {source!r}")
    updated[file] = updated[file].replace(source, target)

slot = """/// 🧹️ Keeps the replay in its paid frame through terminal in-place destruction.
pub struct HistoryReadReplaySlot<T> {
    original: std::mem::MaybeUninit<T>,
    initialized: bool,
}

impl<T> Default for HistoryReadReplaySlot<T> {
    fn default() -> Self {
        Self { original: std::mem::MaybeUninit::uninit(), initialized: false }
    }
}

impl<T> HistoryReadReplaySlot<T> {
    pub fn is_some(&self) -> bool { self.initialized }

    fn is_none(&self) -> bool { !self.initialized }

    pub fn as_ref(&self) -> Option<&T> {
        if self.initialized { Some(unsafe { self.original.assume_init_ref() }) } else { None }
    }

    fn as_mut(&mut self) -> Option<&mut T> {
        if self.initialized { Some(unsafe { self.original.assume_init_mut() }) } else { None }
    }

    pub fn install_original(&mut self, value: T) {
        assert!(!self.initialized, "history retirement already holds its original replay");
        self.original.write(value);
        self.initialized = true;
    }

    fn destroy_terminal(&mut self) {
        assert!(self.initialized, "terminal history replay is absent");
        self.initialized = false;
        unsafe { self.original.assume_init_drop() };
    }
}

impl<T> Drop for HistoryReadReplaySlot<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || !self.initialized, "history replay retains original ownership");
    }
}

"""
anchor = "pub struct ArtifactHistoryReadRetirement<P, Mu: Mutation<P>> {"
replace(history, anchor, slot + anchor)
replace(history, "pub replay: std::mem::ManuallyDrop<Option<ArtifactDerivedReplay<P, Mu>>>,", "pub replay: HistoryReadReplaySlot<ArtifactDerivedReplay<P, Mu>>,")
replace(history, "copy_bytes:size_of::<Option<ArtifactDerivedReplay<P,Mu>>>()+size_of::<SnapshotReadRegistryHandle>()", "copy_bytes:size_of::<bool>()+size_of::<SnapshotReadRegistryHandle>()")
replace(history, "drop(self.replay.take());Ok(Some(moved))", "self.replay.destroy_terminal();Ok(Some(moved))")
replace(admission, "*owner.replay=Some(original)", "owner.replay.install_original(original)")
replace(admission, "replay:ManuallyDrop::new(None),finished:", "replay:Default::default(),finished:", 2)
replace(hydration, "replay: ManuallyDrop::new(None), loaded_replay:", "replay: Default::default(), loaded_replay:")
replace(lift, "*retirement.replay=Some(replay)", "retirement.replay.install_original(replay)")
replace(fixture, "replay:ManuallyDrop::new(None),finished:", "replay:Default::default(),finished:", 3)
replace(fixture, '*owner.replay=Some(ArtifactDerivedReplay{', 'owner.replay.install_original(ArtifactDerivedReplay{')

for file, original in files.items():
    print("".join(unified_diff(original.splitlines(keepends=True), updated[file].splitlines(keepends=True), f"a/{file}", f"b/{file}")), end="")

