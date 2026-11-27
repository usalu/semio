# Live cleanup stall fix: the maintenance reading carries its cause

Root: `runtime_live_cleanup_nonterminal_status` counted `Blocked` and every empty `Progress` as stall debt per pump. Browser evidence: `stage=14`, empty `Progress`, `credit_us=43100`, i.e. a funded store-replacement or archive phase waiting on host or worker.

## Change (P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`)
- `RuntimeMaintenanceReading { Progress, Starved, Waiting, Fruitless }` plus `runtime_maintenance_reading(starved, step)` in `P⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs`. It reuses `PluginLifecycleStep` and the existing `contended` witness; no new step type.
- Ladder (`P🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`):
  - a funded stage 14 (store replacement) or 24 (archive load), also as a `Pressure` unit, that returns an empty `Progress` now reports `AwaitingInput` (waiting on host, worker or replacement owner);
  - yield, zero grant, the archive-reserve skip (stock 1..=16 while the archive is pending) and an unfunded unit mark starvation through `maintenance_starved_mark()` (thread-local in `reactor-app/🛠️runtime-helpers`), which the live job reads right after the step (`contended`).
- Live verdict (`🔁️cleanup-turns`): Starved buys no debt and does not clear; Waiting (`AwaitingInput`, `Blocked`) and Progress clear, consistent with the close verdict; only Fruitless (funded runnable step moved nothing) buys debt and still needs 256 readings and the stall credit.
- Per turn, not per pump: `PluginRuntime.live_cleanup_repeat` (set by the reactor turn loop for pumps 1..31, `plugin_mark_live_cleanup_repeat`) makes `cell.maintenance_repeat` classify repeat pumps' empty readings as starved. The pressure repeat stops when the last reading was starved (`cell.maintenance_starved`), so a stock that cannot fund a unit is not pumped again.
- The coordinator's `[DEBUG]` line is untouched.

## Tests
- Vectors: `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🩻️live-stall-reading/🔣️.json` (reading cases plus sequences: 2048 stage-14 waits over 500 ms never fault; blocked never faults; starved never faults; a funded fruitless run faults; the same count within 1 ms does not). Checked by an independent SQLite decision table in `🟦️.ts` and by Rust `live_readings_are_judged_by_their_cause_over_language_agnostic_vectors`.
- Existing laws updated to the new truth: former `Blocked` live-stall laws now use an empty `Progress` (genuine stall still faults after limit and credit); the `Blocked` entry was removed from the "cannot fault before credit" loop.

## Not done
- A per-wait deadline owned by the archive load (analysis item 6): a hung host now waits without a fault from this verdict. Needs its own ticket.

## Results
- `semio-plugin-tests-01-2 --lib` filter `live_`: 10 passed, 0 failed (includes `live_readings_are_judged_by_their_cause_over_language_agnostic_vectors`).
- `semio-s-artifact-fem-3d-editor` target `editor_live_cleanup`: `fem3d_live_cleanup_never_reaches_the_zero_progress_verdict` ok.
- `cargo check` of `semio-plugin-runtime-instances` and `semio-plugin-reactor-turn` clean.
- The SQLite vector test in the close-budget `🟦️.ts` was written but not run (bun not run here).

## Side fixes
- `SurfaceMutation` (`🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs`) now declares `artifact_persisted_encoded_binary_operation!`, fixing the stale surface-fixture build break.
- A duplicate `maintenance_starved_mark/take` definition in reactor-app runtime-helpers (added concurrently by someone else) was reduced to the single existing one.

## Follow-up: stage 14 hang after the verdict fix (browser, coordinator evidence)
Trace ends at `[DEBUG] store initializer call=449 terminal_target=Some(AwaitingMembers) session_poll=Some(Closing)`; call=481 never prints. `drive_initializer` returns before its counter once the state leaves `Initializing`, so the silence is consistent with the initializer having finished and the hang being later (archive load member phases, replacement member opens, or the closing worker session whose close returns `Blocked` while its `Arc` is shared or a waker is registered). Not determined from the traces; the verdict used to surface this as zero-progress, the Waiting reading now hides it.

Delivered:
- Per-wait deadline: `ActiveDocumentArchiveLoad.observe_wait/clear_wait` over a pure `StallWait` (`🗃️document/🗄️document-archive-load/🦀️.rs`); `drive_document_archive_original` faults a non-closing load whose witness (phase, count, fold, stage, member passes, decoded) stayed unchanged for `DOCUMENT_ARCHIVE_WAIT_CEILING_US` (120 s) as `plugin.document-load.failed` naming phase, stage, members and state.
- Vectors `🧫️fixtures/⏳️archive-wait/🔣️.json` + law `stall_wait_laws::a_wait_expires_only_while_its_witness_stays_unchanged` (semio-plugin-document-media --lib: ok).
- Temporary `[DEBUG] archive/replacement wait reading=N <document_load_diagnostic>` (every 512th stage 14/24 wait, maintenance-ladder file) to name the stuck phase in the next browser run. Remove when the cause is fixed.
- `semio-plugin-tests-01-2 --lib live_`: 10 passed after both changes.
Open: root cause of the stage 14 wait; needs the diagnostic line from one wasm build.

## Follow-up 2: old store never released (browser evidence `store_close=ReturnedReads ... leases_empty=false`)
Cause found by reading: a snapshot read lease on the displaced store (the FEM 3D and 2D live visual jobs hold one per render, `MountedState.snapshot`) is never returned because nothing tells the holder its store was replaced; the job only checks `commit_authority_matches` after a full solve. The displaced store waits in `ReturnedReads`, stage 14 reads empty and the archive stays in `AwaitReplacement`. The 120 s deadline never fired because the maintenance ladder skips the archive rung (24) while stock items < 16 (grant items=1), so a per-load check inside the archive driver never ran.

Changes (not yet compiled together or run natively, see Status):
- `SnapshotReadLeaseRegistry::revoke_authority()` (value/snapshot/read) + `ArtifactStore::revoke_snapshot_read_authority()`; called on the displaced store in `commit_store_replacement_candidate` (maintenance-ladder).
- FEM 3D and 2D mounted job step: cancel itself when its lease no longer matches the store authority (`💼️mounted-state`, `🔁️mounted-state-step`).
- Deadline moved to the ladder: `MaintenanceCursors.archive_wait` (`StallWait<()>`), cleared by any ladder progress, expiring all live archive loads as `plugin.document-load.failed` after `DOCUMENT_ARCHIVE_WAIT_CEILING_US` (120 s) of waits only (`expire_stalled_document_archive_loads`).
- Native law `fem3d_replacement_revokes_the_read_leases_of_the_displaced_store` (`editor_live_cleanup`), helpers `plugin_admit_document_load`, `plugin_poll_document_load`, `LiveCleanupFixture::runtime`.

Status: `stall_wait_laws` ok. `editor_live_cleanup` and the framework `live_` laws could not run: peers broke 2D `results-state` (missing `semio_os_store_test_support`) and `instance` ingress (`semio_framework_trace` unresolved), then the disk filled (C: 0 bytes free; `C:\t\wdb` 105 GB). My private build dirs were deleted to free space; rerun needs about 12 GB.
