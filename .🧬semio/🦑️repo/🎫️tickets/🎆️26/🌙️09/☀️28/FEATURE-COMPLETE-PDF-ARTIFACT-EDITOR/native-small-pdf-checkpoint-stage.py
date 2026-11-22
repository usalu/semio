"""📏 Stage natural-file checkpoint pacing and report actual worker checkpoint progress."""
from difflib import unified_diff
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
os = next(root.glob("🧰*/🛍*/💻*/🔨*"))
plugin = os / "🔌️plugin"
scope = ticket / "🗑️generated/native-small-pdf-open"
scope.mkdir(parents=True, exist_ok=True)
changes = {}


def edit(path, old, new):
    original, current = changes.get(path, (None, None))
    if original is None:
        original = current = path.read_text()
    if current.count(new) == 1:
        changes[path] = original, current
        return
    assert current.count(old) == 1, (path, old)
    changes[path] = original, current.replace(old, new, 1)


job = plugin / "📜️declaration/📥️natural-file/🦀️.rs"
edit(job, """    fn note_publication_work(&mut self, fuel_before: u64, cx: &semio_framework_job::StepContext<'_>) {
        let spent = fuel_before.saturating_sub(cx.fuel_remaining());
        self.publication_work = self.publication_work.saturating_add(spent);
        if spent != 0 || self.stage != NaturalFileImportStage::Encode { self.checkpoint(); }
    }
""", """    fn note_publication_work(
        &mut self, previous_stage: NaturalFileImportStage, fuel_before: u64, cx: &semio_framework_job::StepContext<'_>,
    ) {
        let previous_work = self.publication_work;
        let spent = fuel_before.saturating_sub(cx.fuel_remaining());
        self.publication_work = previous_work.saturating_add(spent);
        if publication_checkpoint_due(previous_work, self.publication_work,
            self.contract.max_work_units_per_step, previous_stage != self.stage) {
            self.checkpoint();
        }
    }
""")
edit(job, """                    if self.advance_pack_export(cx)? { self.stage = NaturalFileImportStage::AdoptGenesis; }
                    self.note_publication_work(before, cx);""", """                    if self.advance_pack_export(cx)? { self.stage = NaturalFileImportStage::AdoptGenesis; }
                    self.note_publication_work(NaturalFileImportStage::Encode, before, cx);""")
edit(job, """                if self.stage == NaturalFileImportStage::Publish || cx.fuel_remaining() < before {
                    self.note_publication_work(before, cx);
                }""", """                self.note_publication_work(NaturalFileImportStage::AdoptGenesis, before, cx);""")
edit(job, "impl<E: ArtifactEditor> ArtifactReservedJob for NaturalFileImportJob<E> {}",
     """/// 📶 Coalesces byte-level Pack work by the existing contract quantum and publishes stage boundaries immediately.
fn publication_checkpoint_due(previous: u64, current: u64, quantum: u64, changed_stage: bool) -> bool {
    let quantum = quantum.max(1);
    changed_stage || current / quantum != previous / quantum
}

impl<E: ArtifactEditor> ArtifactReservedJob for NaturalFileImportJob<E> {}""")

worker = plugin / "🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs"
edit(worker, "        Some((operation_id, stage, flags, operation.publication_progress))",
     """        let progress = match operation.stage {
            MountedTypedCommandFullOperationStage::Worker => operation.progress.unwrap_or(0),
            _ => operation.publication_progress,
        };
        Some((operation_id, stage, flags, progress))""")

diff = []
for path, (before, after) in changes.items():
    relative = path.relative_to(root)
    diff.extend(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}"))
(scope / "publication-checkpoint.patch").write_text("".join(diff))
