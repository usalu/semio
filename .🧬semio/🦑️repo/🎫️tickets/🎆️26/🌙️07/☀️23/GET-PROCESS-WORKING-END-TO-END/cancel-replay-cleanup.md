# Cancelled replay poisons the runtime instance (prior-outcome)

## Root cause
- `historyEditCancelReplay` while a whole-document load runs does not touch a history edit: `cancel_reprojection_turns` calls `request_document_archive_cancel`, which cancels the load and its awaited store replacement (own cancel roots).
- `plugin.internal.prior-outcome` is produced only by the runtime live-cleanup pump (`runtime/instances/cleanup-turns`, `runtime_live_cleanup_pump_one`). When the pumped cleanup job reaches a terminal outcome, the pump set `faulted` for `JobOutcomeView::Fault` and also for `JobOutcomeView::Cancelled`. The drained session then stored `Fault(PriorOutcome)` in `maintenance_status`, and every later reactor turn (`plugin_step_live_cleanup_cell`) returned that fault again. That is the permanent poisoning of instance 1 (every later action, role switch, `shell.documentTransfer`).
- A cancelled cleanup job outcome (`advance_source` answers `CleanupTurn::Cancelled` when its context is cancelled) is a legitimate outcome: the job carries no state, the cell owns it, a fresh session is created on the next turn. It must not be a fault.
- Separate finding, not fixed here: a real `Fault` outcome (an `Err` from `mounted_owner_maintenance_step`, e.g. the receipt mismatch of `cancel-receipt.md`) is also reported only as `prior-outcome`; the underlying cause is printed only with runtime diagnostics (`debug_cleanup_continuation`). In the browser run the cancel hit at "87455 of 87461" records; if the hub in use predates the `cancel-receipt.md` fixes, that fault class is the more likely trigger and a rebuilt hub is needed to tell them apart.

## Fix
`runtime/instances/cleanup-turns/🦀️.rs`, live pump: `faulted` is now `matches!(view, JobOutcomeView::Fault { .. })`. A cancelled outcome drains the session and the pump returns its pending status (Ready); the next turn admits a new cleanup session. The close pump is unchanged (a cancelled close step stays terminal).

## Law
`a_cancelled_replay_leaves_the_runtime_instance_live_and_accepting_actions` in `plugin/tests/folder-reload-route` (crate `semio-plugin-tests-07-3`; dev-dependencies `semio-plugin-reactor-lifecycle`, `semio-plugin-runtime-instances` added). It wraps a toy program in a real `PluginRuntime` / `RuntimeAppCell`, admits a 24-edit load, pumps `plugin_step_live_cleanup` N turns (N = 0..987), cancels through `historyEditCancelReplay` via `verb`, then asserts: no live-cleanup turn is refused, `maintenance_status` is not a Fault, the load retires and the host poll finds Cancelled, the document equals the pre-load one, a further edit and an `undo` are accepted, more live turns succeed, and the instance retires through `plugin_destroy_app` plus `plugin_step_close_cleanup`.
Also extended the existing `a_cancel_at_any_point_...` law: after each cancelled run a further edit, a maintenance turn and an `undo` must be accepted (passes, app level).

## Results
- `a_cancel_at_any_point_of_a_load_restores_...` with the extension: ok (103 s).
- The new runtime-level law: compiled and ran in earlier attempts with the pump fix reverted; the only findings were test-harness artefacts that I removed (cancelled foreign callers). A cancelled host context makes the pump skip the job (no cancelled outcome), so this harness does not hit the cancelled-outcome path; the fault needs the cancellation to appear between the pump check and the job step. Final run with the fix on the current tree: see the last section.
- Several runs were lost to disk-full, memory-grant refusals of the build scheduler (cold build dir) and another agent's broken `semio-os-spr-cli` / `testing-laws` edits.

## Changed files (under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`)
- `⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`
- `🧪️tests/🧪️folder-reload-route/🦀️.rs`
- `🧷️companions/🫑️tests-07-3/📦️packages/🦀️rust/Cargo.toml`
- ticket: `cancel-replay-cleanup.md`; generated outputs under `🗑️generated/cancel-replay-*.txt` to delete.

## Final run
Not completed: the final `cargo test -j 2 a_cancelled_replay_leaves` run (output `🗑️generated/cancel-replay-16.txt`) was still compiling after about 2 h in a cold build dir (build scheduler memory-grant retries, other agents building). The fix and the law are therefore compiled and checked only up to the earlier runs above; the law has not yet been seen green with the fix, and a red-without-fix reproduction was not obtained. Next step: rerun that command once the build finishes, then delete `🗑️generated/cancel-replay-*.txt`.
