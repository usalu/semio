"""🩺 Stage finite mounted MediaIn worker diagnostics under the existing runtime switch."""
from difflib import unified_diff
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
os = next(root.glob("🧰*/🛍*/💻*/🔨*"))
plugin = os / "🔌️plugin"
scope = ticket / "🗑️generated/native-small-pdf-open"
scope.mkdir(parents=True, exist_ok=True)

job = plugin / "📜️declaration/📥️natural-file/🦀️.rs"
before = job.read_text()
anchor = "impl<E: ArtifactEditor> NaturalFileImportJob<E> {\n    fn step_original<'a>"
assert before.count(anchor) == 1
after = before.replace(anchor, """impl<E: ArtifactEditor> NaturalFileImportJob<E> {
    fn trace_original_file_worker(&self, cx: &semio_framework_job::StepContext<'_>) {
        if !semio_framework_job::runtime_diagnostics_enabled() { return; }
        thread_local! {
            static WITNESS: std::cell::Cell<(u64, u64, u8, u64)> =
                const { std::cell::Cell::new((0, 0, 255, 0)) };
        }
        let id = (cx.operation().0, cx.generation().0, self.stage as u8);
        WITNESS.with(|witness| {
            let (operation, generation, stage, calls) = witness.get();
            let calls = if (operation, generation, stage) == id { calls.saturating_add(1) } else { 1 };
            witness.set((id.0, id.1, id.2, calls));
            if calls.is_power_of_two() {
                let demand = self.pack_export.as_ref().map(|export| (export.ready(), export.advance_demands()));
                eprintln!("[DEBUG] natural-file worker operation={} phase={:?} calls={calls} bytes={}/{} pack_work={} pending_checkpoint={} checkpoint_delivered={} next_pack_demand={demand:?} copy_left={} fuel_left={} yield={}",
                    id.0, self.stage, self.byte_cursor, self.byte_length, self.publication_work, self.pending.is_some(),
                    self.outcomes.is_delivered(), cx.retained_grant().maximum_copy_bytes, cx.fuel_remaining(), cx.should_yield());
            }
        });
    }

    fn step_original<'a>""", 1)
anchor = "        use semio_plugin_decl_app::tool_contract::ReservedOutcomeKind;\n        if cx.is_cancelled()"
assert after.count(anchor) == 1
after = after.replace(anchor, """        use semio_plugin_decl_app::tool_contract::ReservedOutcomeKind;
        self.trace_original_file_worker(cx);
        if cx.is_cancelled()""", 1)
relative = job.relative_to(root)
patch = unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}")
(scope / "worker-phase-probe.patch").write_text("".join(patch))

witness = plugin / "🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs"
before = witness.read_text()
anchor = "                    let child_result_pending = operation.owned_child_result_pending;"
assert before.count(anchor) == 1
after = before.replace(anchor, '''                    if operation.stage == MountedTypedCommandFullOperationStage::Worker {
                        let session = operation.session.as_ref().map(|session|(
                            session.poll(), session.last_step_end_us(), session.checked_out_outcome_descriptor().is_some(),
                            session.checked_out_continuation().is_some(), session.callback_verdict().is_some(),
                        ));
                        semio_plugin_reactor_app::debug_runtime_line(format_args!(
                            "[DEBUG] worker ownership operation={} session={session:?} checkpoint={:?} semantic={} outcome={} resume={} terminal={}",
                            after.0, operation.progress, operation.worker_semantic_pending, operation.worker_outcome_pending,
                            operation.worker_resume_pending, operation.terminal_seen,
                        ));
                    }
''' + anchor)
relative = witness.relative_to(root)
with (scope / "worker-phase-probe.patch").open("a") as target:
    target.writelines(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}"))
