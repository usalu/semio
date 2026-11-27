# Live cleanup zero-progress fault at boot

## Cause
- The browser message `zero-progress ... (elapsed 0us, ceiling 8000us)` is the signature of the count-only live stall rule: 256 fruitless readings (`Progress(default)` or `Blocked`) faulted regardless of real time, and the message quoted the last turn's cost (0us on a coarse browser clock). A booting document load keeps the live pump Blocked/zero-progress for hundreds of microsecond-long pressure steps per turn (`LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN` = 32), so the instance faulted at boot and every later turn re-returned the stored fault.
- The tree now already prices the live verdict in both currencies (`runtime_live_cleanup_nonterminal_status`: 256 readings AND `RUNTIME_CLOSE_STALL_CREDIT_US` of real time; message quotes the credit spent). `cleanup-turns/🦀️.rs` was last edited 11:53 and `app-cell/🦀️.rs` 11:51, so the tested hub was built before that fix. With the fix, the fault message could not read `elapsed 0us` for a live ZeroProgress (it must be at least 8000us).
- Lead 1 (cancel restart loop) is ruled out: a `Cancelled` outcome records no reading (`outcome_progress = None`), which `runtime_live_cleanup_nonterminal_status` treats as "not zero progress" and clears both the count and the stall instant. A session restarted every turn after a cancel therefore never accumulates stall.
- Lead 2 (time-travel effects, snapshot read drains, CancelPreview) does not touch the live pump's stall counters; not implicated.

## Fix
No further production change needed beyond the credit rule already in the tree. Rebuild the hub from the current tree.

## Laws
- Existing: `live_zero_progress_cannot_fault_before_its_stall_credit_elapses`, `permanently_blocked_live_cleanup_faults_after_its_stall_credit_without_claiming_released_ownership` (a genuinely stuck pump still faults, quoting >= the credit).
- New: `cancelled_live_cleanup_outcomes_clear_the_stall_run_and_never_fault` (close-budget law file, run in crate `tests-01-2`): after 255 stalled readings, 1000 cancelled (None) readings leave count and instant at 0 and stay Ready.

## Results
`cargo test -j 2 -- cancelled_live_cleanup live_zero_progress_cannot_fault permanently_blocked_live_cleanup` in `tests-01-2`: 3 passed, 0 failed (new cancelled-outcome law, credit-before-fault law, permanently-blocked-still-faults law).

## Changed files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` (new law)
- ticket: `live-cleanup-zero-progress.md`

## Second round: Blocked behind a progressing load
The rebuilt hub still faulted (elapsed 14-27 ms): during boot the live pump is Blocked while the whole-document load legitimately advances, and 8 ms of wall time is far below a browser main thread's interleaving.

### Product rule
- `RuntimeAppCell` gained `instance_progress_epoch` (bumped by `plugin_begin_document_archive_load` and every `plugin_poll_document_archive_load`) and `maintenance_seen_progress_epoch`.
- `runtime_live_cleanup_nonterminal_status` takes `(instance_epoch, seen_epoch)`; if the epoch moved since the previous reading, the stall run is cleared (another owner of the instance advanced; the pump is waiting behind progress, not stalled).
- The wall credit is now `RUNTIME_LIVE_STALL_CREDIT_US` = turn ceiling x (zero-progress limit / pressure steps per turn) = 8000 us x 8 = 64 ms, judged only while the instance epoch is frozen. The 256-reading count still applies.

### Laws
- New `live_cleanup_blocked_behind_a_progressing_load_never_faults_but_a_frozen_instance_does` (1024 Blocked readings over many ceilings with a moving epoch never fault; a frozen epoch faults with ZeroProgress).
- Updated existing live laws to the new signature and `RUNTIME_LIVE_STALL_CREDIT_US`.

### Results
Not run: the tests-01-2 build fails in `semio-plugin-instance-app` on another agent's in-progress edit (`maintenance-ladder/🦀️.rs` calls `semio_plugin_reactor_app::maintenance_starved_mark`, which is not defined anywhere yet). `folder_reload_route` not run either. Rerun `cargo test -j 2 -- runtime_instances_test_hosts` in tests-01-2 and `folder_reload_route` in tests-07-3 once that edit lands.

### Changed files (under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`)
- `⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`, `⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🛎️app-cell/🦀️.rs`, `⚛️reactor/🏃️runtime/🧰️ops/📄️document/🦀️.rs`
- tests: `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs`, `⚛️reactor/🏃️runtime/📦️instances/🧪️tests/🦀️.rs`, `🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs`, `🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/📦️runtime-instances/🦀️.rs`

## Third round: the epoch was never bumped at boot (hub v17, elapsed 64000us)
### Diagnosis
Archive-load polls were the only bump, but the Process 3D boot is driven by other host crossings (render, projection, ingest, media). A host crossing also makes the pump `contended`, which neither counts a step nor clears the stall instant, so wall time accrued across legitimately busy host turns until the 64 ms credit was spent. The exact blocking reason at boot is still unobserved: the faulting message did not name it.

### Changes
1. Self-diagnosing fault: `runtime_cleanup_fault_with_reading` appends `(last reading: <blocked|awaiting-input: reason | zero-progress receipt>; last progressing owner: <owner>)`. The live pump stores its last reading in `RuntimeAppCell::maintenance_last_reading` (a `Blocked` reason, an `AwaitingInput` reason or a zero receipt), and `stall_reading()` joins it with the last owner.
2. Generalized epoch: `RuntimeAppCell::note_instance_progress(InstanceProgressOwner)` is now called by every successful `host_instance()` acquisition (render, projection, ingest, media, archive polls and admission all go through it), not only by archive loads. Owners: HostCrossing, ArchiveLoadAdmission, ArchiveLoadPoll.
3. Law `a_live_zero_progress_fault_names_its_blocking_reading_and_last_owner` (close-budget file).

### Status (honest)
- `folder_reload_route` (tests-07-3): 10 passed, 0 failed (run with the epoch rule, before the host-crossing generalization).
- tests-01-2 `runtime_instances_test_hosts`: last green run 20/20 before this round; the rerun with the host-crossing epoch and diagnostics is BLOCKED by peers: `semio-plugin-retained-command` exceeds its source-size boundary (262349 of 262144 bytes), and earlier a testing-laws fixture mismatch (I added `.map_err(Into::into)` in `🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`).
- NOT done: the native Process 3D repro (booting the real process3d editor in a `PluginRuntime`). That needs the process3d editor/host crates built natively, which the memory warden has been refusing; I did not attempt it.

### Changed files (additional)
- `⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary/🦀️.rs`, `.../🛎️app-cell/🦀️.rs`, `📦️instances/🔁️cleanup-turns/🦀️.rs`, `🧰️ops/📄️document/🦀️.rs`, `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs`, `🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`
