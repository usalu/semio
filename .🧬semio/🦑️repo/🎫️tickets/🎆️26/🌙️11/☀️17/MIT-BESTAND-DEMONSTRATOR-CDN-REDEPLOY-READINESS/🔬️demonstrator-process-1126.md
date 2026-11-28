# Demonstrator and Process pair, 2026-11-26

## State at start (05:09 Pacific)
- 12 of 33 modules confirmed. `demonstrator` and `process` are staged but unconfirmed.
- Source fixes already present per earlier notes: covering-grant publication law (publication-stall-native-law), held-fuel quote, retained CAD addObject, separated Process editor/viewer units, sixth cell-edit locale field (XLSX E0609 cause: producer admitted before the locale field; consumer after).
- Open, unproven: `interactive-job.publication-stalled` for Koordinator/Aussuchen/Bearbeiten/Verfolgen (cleanup stage 24 archive load, phase not witnessed), command-authority rejection of Koordinator/Aggregator catalogue insertion, six-surface Demonstrator library split.
- Completed fault native reference: fixture and ABI receipt check exist but native executable is not yet admitted to a shared Cargo scope.

## Action
- Launched `JOBS=2 bash native-queue.sh --pairs demonstrator process` (log `🗑️generated/queue-demonstrator-process-1126.log`, driver log `🗑️generated/browser-pair-demonstrator+process.log`). No source edits were made while it runs, to avoid source-drift refusals at admission.
- The step waited behind the exclusive product Cargo lane (another agent's `native-queue.sh puzzle playbook ...` run is active).

## Results
(see below, updated as the queue progresses)

### 05:20 Pacific
The pair build is queued, not running. The exclusive product Cargo lane `q53457c` is held by pid 31153, the `native-queue.sh puzzle playbook stdio sourcing energy cad gis flow` run started by the same session at 05:09 (log `browser-puzzle.log`). The pair step resumes automatically when that holder releases the lane. Confirmed count is still 12 of 33; no Demonstrator/Process confirmation exists yet. A background waiter runs until its 9-minute window ends; the queue process itself (`queue-demonstrator-process-1126.log`) keeps running.
Remaining after the build: if `[queue FAIL]`, read `browser-pair-demonstrator+process.log` and the `.prev` log for the first compiler error; then check the pair with `release-receipts.py status`; then rerun the Chromium `publication-stalled` probes against the new bytes and add the archive-load phase witness (`active.phase`, `replacement.state`) to the `[DEBUG] FEM cleanup semantic failure` line if the stall persists.

### 05:50 Pacific, coordinator items
1. Six-surface split: source verified. `semio-hub-demonstrator` (router target `semio_hub_demonstrator_router`) depends on exactly six separate crates (playground, sourcing, process; editor and viewer each), all six declared in `🌎️hub/Cargo.toml` and the hub package. Graph/closure resolution is covered by the earlier locked WASI graph; a fresh dry run happens with the pair step.
2. Completed fault fixture: the isolated manifest failed with `profile prototype not valid ... trim-paths`; `completed-fault-native-fixture.py` now emits `cargo-features = ["trim-paths"]`. Rerun in progress (`🗑️generated/completed-fault-native-run.log`).
3. Stage-24 stall: read `advance_document_archive_load` and the replacement driver. Every waiting phase (`AwaitingMembers` waiting for the replacement, `AwaitReplacement`, `FillMember` with no page, replacement `AwaitingMembers` with `member_ingress`) and every unfunded unit returns `Progress(empty)`, which `runtime_live_cleanup_nonterminal_status` counts as zero progress. The live fault needs 256 consecutive fruitless readings plus stall credit, so two units that both wait on each other, or one unfunded unit, fault with `publication-stalled`. The per-unit loan (64 items, 32 KiB copy) versus indivisible demands is the already-fixed class (`covering_grant`). No second deterministic cause can be named from source alone; the existing native law `every_example_loads_through_the_member_less_archive_door` (Process editor unit tests) drives a real archive load through replacement under the small `INTERACTIVE_COMPONENT` maintenance loan and prints `document_load_diagnostic` on a stall; it is the red/green witness to run. No new red test written: the part-4 test host still does not compile (see publication-stall-native-law).
4. Final link: `semio-hub-demonstrator-component` and `semio-hub-process-component` added to `leanCompositions.release` in the Cargo profile schema (size optimization, 16 codegen units, LTO off, as for FEM, GIS, Procedural, Sourcing, Stdio; Sourcing fell from 5,519 to 2,523 MiB). Three consumption rows added first (demonstrator, process, pair). 27 of 27 profile consumption tests pass. The running pair step exited 75 because the profile source changed under it; queue killed and re-queued at 05:55.

### 06:15 Pacific, second coordinator round
- Native test lane runs (not the release lane): Completed-fault fixture is compiling (`🗑️generated/completed-fault-native-run.log`, 104 crates, through the job closure). `run-door.sh` waits for it and then runs `cargo test --lib every_example_loads_through_the_member_less_archive_door` for `semio-s-artifact-process-process3d-editor` (`🗑️generated/archive-door-run.log`). That test lives in the editor lib, so it does not need the part-4 host (`semio-plugin-tests-14-2`, 115 compile errors from cfg(test)-only helpers and now-private fields).
- Part-4 host root fix and the stall red test/fix depend on those results and are not done. The scheduling principle to apply: an unfunded or mutually-waiting unit must return an explicit funding decision (fault naming demand versus grant) instead of `Progress(empty)`.
- No cad/puzzle/flow-closure files touched.

### Stage-24 / archive door stall: root cause and fix (2026-11-27)
- Reproduced natively on the carved target `process3d_example_archive_door` (`every_example_loads_through_the_member_less_archive_door`): archive load `AwaitingMembers`, replacement `Initializing` with its store-replay initializer in `BindGenesis`.
- `[DEBUG] birth` lines: the registry birth quotes 2,368 B copy and 2,384 B capacity, but the quoted native receiving scope is served `maximum_capacity_bytes: 0`, so `SnapshotReadRegistryBirth::advance` returns zero progress forever (livelock burning copy stock each turn).
- Cause: `OriginalReturnScopePlan::quote` caps a held scope's capacity at the held credit's capacity; only a `floor` stops it. `with_native_receiving_quoted` and `_quoted_work` passed `Default::default()` as the floor.
- Fix (semio-framework-job-context only): `StepContext::quote_floor(quote)` in `⏱️context/🛬️owned/🦀️.rs`; both quoted variants in `⏱️context/🦀️.rs` pass it as the floor.
- Results: door test red before, green after (21 s). `original-native-progress`: 15 of 15 pass. The new framework test `a_quoted_receiving_scope_over_a_held_scope_is_never_served_less_capacity_than_it_quotes` passed before the fix too, so it is a guard, not a reproduction.
- Open: editor lib-test carve (old unit still 4.4 to 6.2 GB; the admission carving entry for `semio_s_artifact_process_process3d_editor|host` stays until the unit fits), duplicate door test still in `contributions-and-examples`, part-4 test host (scheduled by the coordinator), Completed-fault fixture rerun.

### Regression run after quote_floor (2026-11-27, -j 1)
- Door test on `process3d_example_archive_door` without debug lines: pass (18.7 s).
- `semio-framework-job-context` (all targets): `original-native-progress` 15 of 15 and the other targets pass except `original-native-parent-retirement::native_parent_survives_foreign_context_cancellation_and_return` (`result.unwrap()` at `🎟️source/🏠️native/🧪️tests/🦀️.rs:31`; the test never calls a quoted receiving scope).
- `semio-framework-job-worker`: 8 pass, 23 fail (close and retained-ownership accounting, e.g. "an allocation of this turn was not receipted in it", "original effect close performed more work than it settled"). The worker crate contains no call to `with_native_receiving_quoted`, so `quote_floor` cannot reach them; the files involved (`⏱️context/🧪️testing/🎟️caller`, close paths) carry other agents' uncommitted edits. Not bisected: reverting `quote_floor` would change what the release rebuild snapshots.
- Item 2 (cold measurement of the editor lib-test unit with the admission entry removed) runs by itself via `🗑️generated/cold-measure.sh` once no `native-queue.sh` is running; result in `🗑️generated/cold-measure.txt`; backup of the schema in `🗑️generated/admission-schema.backup.json`.
