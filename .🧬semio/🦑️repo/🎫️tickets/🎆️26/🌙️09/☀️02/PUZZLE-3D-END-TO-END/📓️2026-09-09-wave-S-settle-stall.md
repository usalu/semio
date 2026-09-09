# Wave S — `settle` stall: `pending typed operations outlived the settle budget`

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-S, 2026-09-09.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, branch detached at `9b605a4550`, working tree carries many peers' uncommitted edits.
- Crate under test: `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`.
- Private target dir (seeded, 39 GB): `/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`.
- Command envelope used everywhere below:
  `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=<private target> cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly …`
- Baseline handed over: `516 passed; 85 failed` at 18:17; green at 18:02 before the mesh-upload wave landed.

## 1 Reproduction + trace

1. `camera_actions_are_view_actions_that_emit_no_artifact_mutations` **passes alone** (0.28 s, quiesces at
   turn 256 with `2:Retiring:true:true`). So do `duplicate_selection_reselects_the_created_clones`,
   `close_vortex_suggestions_clears_the_menu`, `add_object_kind_honors_drop_origin`,
   `gumball_inactive_when_every_handle_flag_is_off` — every settle-stall example handed over is
   order-dependent, not intrinsically broken.
2. `accept_suggestion_closes_menu_even_when_placement_fails` **fails alone**, in 0.41 s, and NOT with the
   settle panic:

   ```
   [DEBUG] reserved job 'interactionHover' poll 1..256: Submitted
   interactionHover: Fault { code: FaultCode("interactive-job.worker-pump"),
     message: "framework reserved mounted worker transition was rejected" }
   ```

3. A `sample(1)` of the full-suite process while a stalled test was in `settle` showed the test thread
   inside `settle` → `advance_typed_operation_publication` → `advance_typed_operation_publication_one`,
   dominated by `_platform_memmove` in `ArtifactFixedRegistry::insert_admitted`/`remove` — i.e. the settle
   loop really does spin 1 048 576 turns re-admitting the operation. All nine `semio-pool-worker-*`
   threads were parked in `Condvar::wait_timeout` / contending on their queue mutexes: **no worker was
   pinned by an infinite job step**, so the coordinator's "a step never returns" hypothesis is REFUTED.
4. Temporary instrumentation of the `map_err` in `run_framework_reserved_job` named the exact rejection:

   ```
   [DEBUG] reserved job 'interactionHover' pump fault after 265 polls: Submit(Pool(Contended))
   ```

5. Fast ordered reproduction of the settle stall itself (no full-suite run needed):
   `<test binary> editor::puzzle3d::component::tests::fill_ --test-threads=1` — 9 tests, 5 failures,
   144 s, three of them the settle panic.

## 2 Root cause

### 2.1 Reserved routes faulted on transient scheduler weather

`WorkerPool::try_submit` (native, `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1846`) admits a step by
round-robin `next_submit` into ONE worker's per-lane queue and takes that queue's lock with `try_lock`.
A worker that happens to be popping its own queue at that instant makes the submission fail with
`WorkerSubmitErrorKind::Contended`. This says nothing about the job — it is scheduler weather, and the
mounted typed-operation path has always answered it with
`PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits transient scheduler authority" }`
(`🔌️plugin/🦀️.rs:15737`).

`run_framework_reserved_job` (same file, ~17 989) was the one pump site that mapped **every** `Err` from
`MountedWorkerJobSession::pump_one` to a hard `interactive-job.worker-pump` fault. So every framework
reserved route — `interactionHover`, `interactionSelect`, `setActiveTool`, `undo`, … — was a coin flip
against pool load: under a busy pool the very first transient `Contended` killed the verb, and the test
that dispatched it failed.

### 2.2 The settle stall itself — a leaked, process-wide worker-job session admission

That was NOT the settle stall. Extending the existing `[DEBUG] typed-operation publication` trace with the
mounted operation's own fields showed the frozen operation is in **`Publishing`**, not `Worker`:

```
["600:Publishing:true:true:pub=false:completion=true:artifact=false:child=false:ui=false:fault=false:page=false:seq=0"]
```

`seq=0` — the operation never published a single page; `pub=false` with `completion=true` — the completion
OWNER is mounted but its value slot is empty. `publish_mounted_typed_operation_unit` answers that state with
`let Some(publication) = completion.take()? else { return Ok(()) }` — a silent, progress-free `Ok`, forever.

Why is there no completion value? A temporary probe in `reserve_worker_job_retirement_slot` answered it:

```
[DEBUG] worker job retirement slots exhausted: 256 total, 0 reserved-without-node
```

`WorkerJobSession::try_new` (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:2684`) admits every mounted worker session
out of `WORKER_JOB_RETIREMENT_SLOTS` — a **fixed, process-wide array of `WORKER_JOB_SESSION_SLOTS = 256`**.
`Drop for WorkerJobSession` (2893) does NOT free the slot when the session is not already `SESSION_EMPTY`:
it parks a retirement node in the slot and raises `WORKER_JOB_RETIREMENT_WAKE`. Only
`pump_worker_job_retirements` gives that slot back — and a repo-wide grep found **no caller in the plugin
host or the app runtime at all**; the only callers are the wgpu renderer's `present_step` and two tests.

So every app dropped with a live operation (a test that panics, a closed tab, a cancelled command) leaks one
of the 256 process-wide admissions permanently. Once 256 leaked, `MountedWorkerJobSession::try_new` was
refused for every later operation, `drive_worker_step`'s `session_rejected` branch walked the operation
straight to `Publishing` — and a refused session never ran the job, so no completion could ever arrive and
`settle` spun its full 1 048 576-turn budget. That is the whole order dependence: each test alone passes,
the binary dies once the array fills, and unrelated view actions (`camera_actions_…`) die with it.

## 3 Fix

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — the host now pumps the retirement array.**
   `MAINTENANCE_STAGES` 23 → 24, with a new stage 23 that runs exactly one bounded
   `semio_framework_job::pump_worker_job_retirements(1, …)` unit, and the `maintenance_step` idle early
   return now also requires `!semio_framework_job::worker_job_retirements_are_parked()` so an otherwise idle
   app still gives its parked slots back. This is the leak fix.
2. **Same file — a refused session admission is terminal, not a spin.** `drive_worker_step`'s
   `session_rejected` branch now stamps `terminal_fault = interactive-job.admission-capacity` and cancels
   the operation's lease as it moves to `Publishing`, so `reject_cancelled_publication` mints the exact
   fault page. Any future exhaustion surfaces as a named fault in one turn instead of an unbounded
   livelock.
3. **Same file — `run_framework_reserved_job` retries transient pump rejections.** New
   `reserved_pump_fault_is_transient` / `reserved_pump_fault_detail` helpers next to
   `retained_payload_eq_slice`; the loop `continue`s on `Submit(Contention)`,
   `Submit(Pool(Contended | Saturated))` and `Take(Pending)` — the exact set the mounted typed-operation
   path already treats as `AwaitingInput` — and faults only on a structural loss, now with the rejection
   named in the message. The `[DEBUG] reserved job … poll` eprintln is gone; its information lives in the
   fault message.
4. **`🧰️framework/🔨️modules/🧵️job/🦀️.rs`** — new `pub fn worker_job_retirements_are_parked()`, the
   predicate a host needs to know it is not idle while a dropped session still holds an admission.
5. **`✏️…/✏️editor/⏳️precompute/🦀️.rs`** — the five unguarded hot-loop `eprintln!` D4 flagged are removed,
   together with the three census counters (`no_preview`, `no_mesh`, `collided`) that existed only to feed
   them. No behaviour change: `unknown_pending` is still set on a missing mesh.

Not changed: the mesh-upload wave's `unknown_pending` re-queue law in `precompute_step_lane` /
`refresh_brush_candidates`. It was the coordinator's labelled lead, but it is NOT this defect — see §6.

## 4 Laws added

`✏️…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`:
`an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app`

Mounts a typed operation and drops the app before any continuation turn (through a new testkit primitive
`dispatch_unsettled`, which is `dispatch` minus `settle`), then drives a fresh app's cooperative maintenance
and asserts `worker_job_retirements_are_parked()` goes false and that the next app still quiesces on a plain
view action.

Negative check performed: with stage 23's pump replaced by `let advanced = 0;` the law FAILS
(`test result: FAILED. 0 passed; 1 failed`); with the pump restored it passes in 0.37 s.

## 5 Measurements

All runs: the prebuilt test binary
`<target-p3d>/debug/deps/semio_s_artifact_puzzle_3d-c42533047be4a4bc`, `RUST_MIN_STACK=134217728`,
`--test-threads=1`. Builds: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=<target-p3d> cargo test -p
semio-s-artifact-puzzle-3d --features component-app-assembly --no-run`.

| run | before | after |
| --- | --- | --- |
| full suite (`--test-threads=1`) | `516 passed; 85 failed` (handover, 601 tests) | **`581 passed; 30 failed`** in 103 s (611 tests, one added by this wave) |
| `never quiesced` panics in that run | many | **0** |
| `interactive-job.worker-pump` faults | present | **0** |
| `worker job retirement slots exhausted` | 256/256 from ~op 594 on | **0** |
| `precompute` filter | — | `ok. 146 passed; 0 failed` (7.76 s) |
| `editor::puzzle3d::component::tests` module | — | `130 passed; 15 failed` (101.8 s), 0 stalls |
| `editor::puzzle3d::component::tests::fill_` (fast ordered repro) | `4 passed; 5 failed` in 144 s, 3 settle stalls | `4 passed; 5 failed` in 50 s, 0 stalls |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | — | `Finished dev profile in 16.75s`, clean |
| `cargo check --target wasm32-wasip2 -p … --features component-app-assembly` | — | `Finished dev profile in 1m 10s`, clean |
| the new law alone | FAILS with the pump disabled | `ok` in 0.37 s |

The reproducing test `accept_suggestion_closes_menu_even_when_placement_fails` no longer faults on
`interactive-job.worker-pump`. It now fails on a DIFFERENT, concurrently-introduced defect from another
wave, in `render`, not in the job machinery:

```
ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
surface payload exceeds fixed capacity with 33527 bytes
```

### The 30 residual failures are not this defect

- **14** are `editor::puzzle3d::precompute::component::tests::fill_worker_*`. They reproduce ON THEIR OWN:
  `<binary> precompute::component::tests::fill_worker --test-threads=1` → `4 passed; 14 failed` in
  **0.07 s**. The first one fails with a plain `assertion left != right`, and its panic poisons the
  group's static `fill envelope test guard`, so the other 13 fail with `PoisonError`. Self-contained,
  no worker session, no settle — a separate defect in that test group (note: the whole `precompute`
  filter, which runs more of the module first, is green, so the group is order-sensitive within itself).
- **16** are ordinary assertion failures in `component::tests`, every one of which also fails when run
  alone: `ui.fixed-capacity` scene-surface overflow, `Object option`, `document tree objects section`,
  `expected a Partial ui_scope for fillBuildTick, got None`, `the fill slider ready extent must expose
  collision-free compatible placements`, `need a planned fill prefix …`, `fill planning must expose at
  least three ready placements`, `one undo restores the whole coalesced gumball drag`, and similar. They
  belong to the fill-planning, render-capacity and localisation waves, not to this one.

Triage method used throughout: every failing name from an ordered run was re-run with `--exact` alone.
Before the fixes, 10 of 19 sampled failures passed alone (this defect); after them, the residual set
reproduces alone.

## 6 Not verified

- The coordinator's labelled lead — the dropped `resume_candidate_index > 0` guard at
  `⏳️precompute/🦀️.rs:1677,1896` — is **not** the cause of the settle stall and was deliberately left as
  the mesh wave wrote it. It does mean a brush target whose pending is unsatisfiable (missing mesh, index
  not ready) is re-queued forever and keeps `brush_lane_active()` true, which is a real livelock risk for
  the brush lane's own idleness, but it never reaches the typed-operation publication path that stalls
  `settle`. Worth a separate wave: split `unknown_pending` into "owed more compute" (re-queue) and
  "waiting on an input" (park; `install_collision_mesh` already re-enqueues everything when the mesh
  lands).
- `sync_precompute_session`'s `while seed_one_precompute_mesh_fallback(session, envelope) {}`
  (`✏️editor/🦀️.rs:1371`) is an unbounded loop whose termination depends on `place_collision_mesh`
  succeeding; that returns `false` on a too-long URL, on `FILL_WORKER_MAX_MESHES` (64) overflow and on a
  degenerate mesh, each of which would spin it forever on whatever thread it runs on. Not observed in any
  run here, but it is a latent hang with no progress law.
- Nothing outside `semio-s-artifact-puzzle-3d` was rebuilt or tested. Fixes 1–3 are in the shared plugin
  host and the job module: **every other artifact app and every plugin gains maintenance stage 23 and the
  bumped `MAINTENANCE_STAGES`**, so a repo-wide `cargo check --keep-going` and the other artifacts' suites
  should be run by whoever picks this up. Any test that hardcodes 23 stages (none found by grep — every
  user derives from the constant) would need updating.
- The wasm guest path was type-checked (`--target wasm32-wasip2`, clean) but not RUN. The same leak exists
  there and the same stage now drains it, but no wasm runtime witness was taken.
- The peer `[DEBUG] typed-operation publication turn=…` trace in `🔌️plugin/🦀️.rs` was extended during this
  investigation and then restored verbatim; it is still in the tree and still prints on every power-of-two
  and every 4096th turn. The `LAST_MAINTENANCE_STAGE` debug static is likewise untouched.
- The `settle` budget in the puzzle3d testkit is still 1 048 576 turns. With the fix nothing reaches it,
  but a genuine future stall still costs ~30 s of memmove-bound spinning per test before it reports.

## 7 Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — retirement-pump maintenance stage (23),
  `MAINTENANCE_STAGES` 24, idle guard, terminal fault on refused session admission, transient-tolerant
  reserved-route pump + its two helpers.
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs` — `worker_job_retirements_are_parked()`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs` —
  removed five unguarded hot-loop `eprintln!` and their three dead census counters.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` —
  new `dispatch_unsettled`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
  new law `an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app` and its turn budget.
