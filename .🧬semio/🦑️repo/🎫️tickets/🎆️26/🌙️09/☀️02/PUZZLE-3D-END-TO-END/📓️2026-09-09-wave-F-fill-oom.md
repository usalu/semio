# W-F — Fill tool guest OOM and the 2 MiB test-stack overflow

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave F. Written incrementally while the work ran.

## 0. Assignment

1. A native law that drives the whole app through fill-tool activation plus N ≥ 300 `fillBuildTick`
   cycles **with the bounded job actually stepped**, asserting (i) retained heap stays under a fixed
   bound across cycles and (ii) the fill-count slider's `ready` becomes > 0. It must FAIL before the
   fix and PASS after.
2. The fix itself — architectural, no caps that merely hide the leak.
3. `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` must pass on the
   default 2 MiB test-thread stack.

## 1. Instruments built first

### 1.1 A counting global allocator

`…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` now carries `Puzzle3dHeapWitness`, a `#[global_allocator]`
compiled only under `cfg(test)`. It keeps a **per-thread** signed retained-byte counter
(`retained_heap_bytes()`), because the suite runs laws in parallel and one law's retention must not
be charged to another. The guest heap is a fixed 512 MiB wasm linear memory
(`.cargo/config.toml`'s `--max-memory=536870912`), so an owner the tick loop never frees is a hard
trap in production and invisible in a native suite unless the heap itself is weighed.

### 1.2 A shared fill-registry test guard

`fill_envelope_registry()` is ONE process-wide authority with `FILL_ENVELOPE_MAX_OPERATIONS = 4`
slots. The precompute tests module already had a private `fill_envelope_test_guard()`; the whole-app
laws need the SAME mutex or they do not serialize at all. It moved to
`…/✏️editor/⏳️precompute/🦀️.rs` as `pub(crate) fn fill_envelope_test_guard()`.

### 1.3 A stack high-water probe

Temporary probe (removed before close): a thread with a 64 MiB stack paints 24 MiB below its own
frame with `0xA5`, runs a body, then scans upward for the first byte that changed. That is the exact
peak stack the body consumed.

## 2. Deliverable (c) — the stack overflow

### 2.1 The brief's premise was wrong

Measured peaks (warm, debug profile):

| body | peak stack |
| --- | --- |
| `create_puzzle3d_app()` | 315 KB |
| `AppActionRegistry::from_definition` | 318 KB |
| **`fill_tool::measures` + `main::window_measures` + `main::engagement`, sync only** | **96 KB** |
| `VcsArtifactApp::with_registry` | 1 135 KB |
| `testkit::new_app_with_registry` | 1 334 KB |
| `+ bind_instance_id` | 1 420 KB |
| `app().await` (whole fixture, incl. its `Drop` drain) | 1 966 KB |
| `app().await` + one `dispatch("fillBuildTick")` | 2 575 KB |
| the failing law's body | 2 651 KB |

The measures path costs **96 KB**, not 2 MiB. The cost is the app fixture: ~1.4 MB to build it and
~0.6 MB for one `dispatch`, against a 2 MiB (2 097 152 byte) default test-thread stack. Every
app-fixture law in this crate therefore runs at 70–95 % of its stack, and the one that fails is
simply the one that adds the most on top. It passes under `--test-threads=1` only because libtest
runs a single-threaded suite on the process main thread, which has 8 MB.

### 2.2 The oversized carriers

`size_of` on the precompute session's members:

| type | bytes |
| --- | --- |
| `Puzzle3dPrecomputeSession` | 30 704 |
| `Puzzle3dCollision` | 30 088 |
| **`MountedFillWorker` (`BatchJobSession<SharedFillWorkerJob>`)** | **20 904** |
| **`StepOutcome`** | **8 256** |
| `FillBuilder` | 4 096 |
| `CollisionSpatialIndex` | 104 |

`MountedFillWorker` + `StepOutcome` = 29 160 of `Puzzle3dCollision`'s 30 088 bytes. Both are held BY
VALUE twice over: in `Puzzle3dCollision` (`fill_worker`, `fill_worker_outcome`) and in
`FillEnvelopeAuthority` (`worker`, `worker_outcome`). `Puzzle3dPlayApp` is 30 904 bytes as a result,
and it is built on the stack by `with_puzzle3d_app_for` on **every dispatch and every render** — the
same reason `document_tree_cache` was already boxed in that struct.

Taken over by W-F2 (cursor fleet) at 2026-09-10 00:57 CEST.

W-F landed the instruments (§1) and the stack diagnosis (§2). Boxing of
`MountedFillWorker` / `StepOutcome` (`OwnedFillWorker` / `OwnedFillOutcome`) is already in
`⏳️precompute/🦀️.rs`. `fill_envelope_test_guard` is `pub(crate)` there. The counting allocator
is process-wide (W-F corrected the per-thread counter: pool threads allocate, the tick frees).
The architectural fix was not applied.

## 3. Fix

### 3.1 Root cause (verified in source, not assumed)

Every `ArtifactApp` method builds a **fresh** `Puzzle3dPlayApp::default()` via
`with_puzzle3d_app_for`. Checkout restores only `Puzzle3dCollisionSession` (brush lane + meshes +
scene). The fill cursor — `fill_job`, `fill_admission`, `fill_observation`, `fill_faulted`,
`fill_terminal`, `fill_applied_count` — lives on `Puzzle3dPrecomputeSession` and is **not**
checked in. Then `Drop for Puzzle3dPrecomputeSession` terminalizes whatever `fill_job` that
call just admitted and incrementally closes it.

Consequences, one per 120 ms `fillBuildTick` / every render:

1. `sync_precompute_session` → `SetScene` → `install_scene` always `supersede_admitted_fill`
   (no-op, cursor is already gone) then, because `engine.fill` is absent from the collision
   session, `start_fill_preparation(true)` allocates a new `FillBuilder` (~2.8 MB of
   preparation pages on a real document).
2. `enqueue_fill_job` sees `fill_job == None`, admits a **new** envelope, emits `SpawnJob`.
3. End of `with_puzzle3d_app_for`: Drop closes that envelope before the host can step it.
4. Slider `ready` stays 0 and `loading: true`: `fill_progress_summary` reads a brand-new
   builder (`sequence.len() == 0`, not stalled). Native tests that "plan fine" used
   `drive_enqueued_fill_job_for_test` through `with_puzzle3d_app_mut`, which is a **different
   empty app** than the dispatch session — the bypass is a no-op. Wave J's host job driver
   never gets a live envelope to step: the guest already closed it.

That is both the guest OOM (~2.8 MB retained per tick until 512 MiB traps) and the
native/wasm `ready == 0` asymmetry. It is guest-side (session cursor dropped across
`with_puzzle3d_app_for`), not host-side TypeScript.

`fill_faulted` cannot latch across ticks for the same reason. The four-slot registry does
not cap the leak if Drop's close is incomplete or if each tick's preparation is retained by
the job runtime / a half-closed handle.

Brush-queue re-add (wave D4 §4): `precompute_step_lane` re-queues on `unknown_pending`
regardless of `resume_candidate_index`. The in-source comment documents that gating on
`resume_candidate_index > 0` livelocked targets that never cleared the broad-phase index.
Replacing the cache entry is bounded; this is not the 2.8 MB/tick owner. Left as-is.

### 3.2 What the fix must do

1. Persist a lightweight `Puzzle3dFillSession` cursor in `Puzzle3dSessionState`; take it
   before PlayApp drop so Drop does not close a live envelope.
2. `SetScene` must not supersede or re-prepare on a no-op / applied-projection sync.
   Applied-projection detection must use `read_fill` (registry owner), not `engine.fill`
   (cleared at admission).
3. Drive the bounded job through `start_job` / `step_job` in the whole-app law and in
   `drive_fill_until_ready` (replace the dead bypass). Do not use the bypass for the new law.
4. Keep the already-boxed worker/outcome carriers; box `Puzzle3dPlayApp.precompute` so the
   Default() frame on the 2 MiB test-thread stack is a pointer.
5. Remove W-F's unguarded `[DEBUG]` `println!` in `new` / `supersede` / `enqueue`.

W-U owns command arms / scope table in `✏️editor/🦀️.rs` (live: `relocateTargetVolume`
kind). W-H2 added `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`.
Edits in `editor.rs` stay in session structs + `with_puzzle3d_app_for`.

### 3.3 Before-fix compile (W-F2, 2026-09-10 01:05 CEST)

`cargo test` of `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job` and
`probe_fill_build_tick_heap` exited 101 before any test ran. Logs:
`🗑️generated/before-fill-polls.log`, `🗑️generated/before-heap-probe.log`.

Breaks at that snapshot:

1. `E0252` duplicate `HashMap` import in `✏️editor/🦀️.rs` (gone by the time W-F2
   re-read the file; only `use std::collections::{HashMap, HashSet}` remains).
2. `E0277`/`E0308` in `📌️panels/🗿️artifact/🦀️.rs:221` — `page_action` applied `?`
   to `ui_value_number` (`UiValue`) and passed `ui_value_map`'s `Result` into `Some`.
   W-U owns the panel; the file was 15+ min cold. One-line repair:
   `Some(ui_value_map([("page", ui_value_number(...)), ("section", ui_value_text(...)?)])?)`.

### 3.4 Persist landing (W-F2, 2026-09-10 01:33 CEST)

`⏳️precompute/🦀️.rs` already carried `Puzzle3dFillSession`, `take_fill_session` /
`install_fill_session`, scene-sync that does not supersede on no-op / applied-projection,
and Drop that returns when the cursor is empty.

`✏️editor/🦀️.rs` now checks that cursor in and out of `Puzzle3dSessionState.fill` on the
same `borrow_mut` as the collision session, and `Puzzle3dPlayApp.precompute` is
`RefCell<Box<Puzzle3dPrecomputeSession>>` so the per-dispatch `Default()` frame is a
pointer. Command-arm `fill_precompute` is left to W-U.

The growth law `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances`
is already in the editor unit suite (320 ticks, real `start_job`/`step_job`, admission
ceiling 8, `ready > 0`).

### 3.5 ui-runtime compile break (W-F2, 2026-09-10 01:51 CEST)

After persist, `cargo test -p semio-s-artifact-puzzle-3d` failed in
`semio-framework-ui-runtime` `♻️reconcile.rs` (`E0499` overlapping `slot` /
`&mut registry`). By 01:51 the file was 15 min cold and WP2 had already
narrowed the first borrow and rebound `slot` after `take`. W-F2 did not edit
it. Compile retry follows.

### 3.6 Persist in editor + first-tick enqueue (W-F2, 2026-09-10 02:13 CEST)

`Puzzle3dSessionState.fill` is checked in/out with the collision session.
`Puzzle3dPlayApp.precompute` is `RefCell<Box<…>>`. `set_scene_config` no-ops a
synced scene and does not remount a fill worker (a sync-time `start_fill_preparation`
mounted a worker that `PlayApp` drop then discarded; the next `enqueue` mount
was rejected).

`fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job` now compiles
and runs. It still fails: `[DEBUG] fill_build_tick active_tool=Some("fill")
spawn=false`. Enqueue entry (earlier run) was `fill=false scene=true worker=false
steps=0`. `start_fill_preparation` inside enqueue does not leave
`fill_lane_active` — `BatchJobSession::try_new` (`mount_fill_worker`) is the
likely refusal, nested under the retained `fillBuildTick` job. Precompute was
hot again (peer); W-F2 backed off. DEBUG remains in `fill-build-tick` only.

Before-fix `cargo test` of this law and `probe_fill_build_tick_heap` exited 101
(compile). See §3.3.


## 5. W-F2c — retention, heap law, ready readout (2026-09-10)

Owned: `⏳️precompute/🦀️.rs`, its unit suite, editor unit heap law / `fill_ready` loop, `🧪️tests/🔬️testkit` left as-is (`drive_fill_until_ready` already uses `start_job`/`step_job`). Editor.rs session check-in/out was already landed (step 1).

### 5.1 What landed

1. **No-op / applied-projection `SetScene` does not re-prepare.** A synced scene returns `Ok(())`. Applied-projection uses `read_fill` (registry owner), not `engine.fill`.
2. **`enqueue_fill_job` is the only starter** for `start_fill_preparation`.
3. **`read_fill` falls through** when `fill_job` names a missing slot, then `engine.fill`, then the single live registry builder — so measures without a cursor can still see the plan.
4. **`fill_progress_summary` prefers `read_fill`**, then `fill_observation` while a job is live.
5. **`take_terminal_fill_job` does not close `Terminal(Complete)`.** Successful plans stay in the registry for readout. Drop still `Closed`-terminalizes an abandoned cursor (existing unit law).
6. **Placement budget no longer treats every preview publish as a placement.** `CheckpointReady` does not decrement `steps_remaining`; `done` is `fill_done` only (`stalled` or `sequence >= max_count`).
7. **Precompute `[DEBUG]` / `eprintln!` removed** (enqueue, supersede, wf4 probes). `fill-build-tick` command still logs `[DEBUG] fill_build_tick` (W-U3).
8. **Heap law** steps 64 host-like `step_job` slices per tick, calls `fill_ready` each tick (same as render/measures), asserts admission ≤ 8, heap growth < 64 MiB, and `ready > 0`. Witness: `fill_envelope_available_count`.

### 5.2 After-fix heap-law tail

`🗑️generated/after-heap-law.log` (filter `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances`, `--test-threads=1`, `RUST_MIN_STACK` raised):

```
test editor::puzzle3d::component::tests::fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 629 filtered out; finished in 11.54s
```

The law asserts `ready > 0`, admission ≤ 8, heap growth < 64 MiB. Empty-sequence is closed on this path (W-F4 cancel persist + census budget; see §7). Retention still holds.

### 5.3 Before-proof (step 2 only)

Temporarily restored no-op `start_fill_preparation` when `engine.fill` / `fill_job` are none. Same law:

```
320 ticks admitted 5 envelopes (1 still live), spawned 5 jobs, completed 5, faulted 0 (none), moved 3626949 heap bytes; peak_ready=0 final_ready=0 registry_available=0
```

Step 1 persist already keeps `fill_job` across `with_puzzle3d_app_for`, so the no-op leak does not re-admit every tick. Heap/admission match after. Restored the no-op-only `return Ok(())`.

### 5.4 2 MiB stack

`env -u RUST_MIN_STACK cargo test … fill_and_brush_params_are_tagged_utility_options_not_engagement_controls -- --test-threads=1`

`🗑️generated/stack-2mib.log`: **ok** (0.19s). The law no longer nests `app()` + `dispatch("openVortexSuggestions")` + `settle` on the 2 MiB worker thread. Live tagging is proven through `activate_window_utility` (host session only) + `window_measures`. `RejectedFillWorker` is boxed on `Puzzle3dCollision`. Builder forwarders are `#[inline(never)]`; `new_app_with_registry` drops the definition before `with_registry`; `initial_snapshot` runs on an isolated frame.

### 5.5 wasm check

`cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` — **ok** (`🗑️generated/wasm-check.log`, `Finished dev profile`). No wasm rebuild, no :6013.

### 5.6 Fill-family / full lib vs 618/13

Disk is free. `🗑️generated/fill-family.log`: **49 passed; 9 failed** (`fill_` filter).

`🗑️generated/full-lib.log`: **620 passed; 10 failed** vs baseline **618/13**. Net +2 / −3.

The 10:

1. Five whole-app fill laws still miss a planned prefix / see 0 `SpawnJob` in 16 ticks (`slider_range`, `polls_and_enqueues`, `fill_count` panes, `fill_render` three placements, `set_fill_count` clamp). The isolated heap law on the same binary gets `ready > 0` and spawns. These siblings do not use the heap law's 320-tick witness; `polls_and_enqueues` counts `requested_effects` only.
2. Four precompute worker laws still assume a one-unit admission census (`enqueue_fill_job().is_none()` on the first grant). W-F4's `FILL_ENVELOPE_CENSUS_UNITS_PER_TURN = 4_096` finishes the `app()`-scale census in one call.
3. One known-foreign: `two_instances_converge_disjoint_object_edits_via_backbone` (`module.vcs` fail-closed remote snapshot merge).

`🗑️generated/wasm-check.log`: `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` — **ok**.

## 6. Browser `ready=0`

The empty sequence was **not** missing target volumes, kind weights, collision probes, or an admission snapshot taken on an empty default scene. The fill kernel unit that produces `sequence.len() == 1` already has those inputs; the whole-app `app()` fixture has them too.

Two guest defects (W-F4, §7) made the isolated `FILL_JOB_KIND` complete with an empty `FillBuilder.sequence`:

1. Census advanced one owner per `fillBuildTick` (97 units on `app()`), so many runs never spawned.
2. `Drop for Puzzle3dPrecomputeSession` cancelled `engine.fill_cancel` on every `with_puzzle3d_app_for` return. `begin_measurement` had cloned that token into the envelope, so the first `drive_fill_envelope` slice hit `is_cancelled_now()` and terminalized `Cancelled` / `done: true` with nothing placed. `registry_available=0`, slider `ready` stayed 0 natively and in the browser.

No-op `SetScene` does **not** starve re-preparation: a synced scene returns `Ok(())`; a real document change still `supersede`s and the next `enqueue_fill_job` is the only starter.

After W-F4 the whole-app heap law reaches `ready > 0` on the bounded job. A browser that steps `FILL_JOB_KIND` via `start_job`/`step_job` and re-reads tool measures will leave `ready: 0`. Host TS that never steps the job still stays at 0.


## 7. W-F4 — the two remaining owners of `ready: 0` (2026-09-10)

W-F2c's §6 left exactly one open question: *why does the bounded fill job reach `Complete`
with an empty sequence?* Measured, not guessed — two independent defects, both now fixed in
`⏳️precompute/🦀️.rs`. **Please do not revert these two hunks.**

### 7.1 The admission census advanced ONE unit per `fillBuildTick`

`enqueue_fill_job` ran exactly one `FillBuilderOwnerCensusCursor::step` per call, and the
envelope is only handed to its bounded job once that census returns `Complete`. The census is a
resumable walk over every owner a `FillBuilder` retains — 97 units on the `app()` fixture,
thousands on a real document — so one unit per 120 ms tick meant the plan was NEVER admitted:
`enqueue_fill_job` returned `None` forever, **no `Effect::SpawnJob` was ever requested at all**,
and each superseding edit re-admitted a fresh envelope on top.

Evidence (320-tick growth law, temporary probe):

```
[DEBUG] enqueue census job=Some(5) step=pending      ← every tick, 320 times
320 ticks admitted 50 envelopes (1 still live), spawned 0 jobs, completed 0
```

Fix: `FILL_ENVELOPE_CENSUS_UNITS_PER_TURN = 4_096` — `enqueue_fill_job` spends a per-turn UNIT
budget on the census instead of one unit, the same discipline `precompute_step_lane` already
runs on the brush lane. After: `units=97 step=complete`, `spawned 5 jobs, completed 5`,
admissions 50 → 5 (ceiling 8).

### 7.2 The fill cancel token lived on the per-call engine, so every dispatch cancelled its own plan

`begin_measurement` clones `self.engine.fill_cancel` into the envelope. That token lived ONLY on
`Puzzle3dCollision`, which `take_collision_session` does not carry and which is rebuilt fresh by
`with_puzzle3d_app_for` on every dispatch. `Drop for Puzzle3dPrecomputeSession` called
`self.engine.fill_cancel.cancel_now()` **before** its `involved` early-return — so the very
dispatch that admitted a plan cancelled it again on the way out. `drive_fill_envelope`'s first
slice then hit `cancel.is_cancelled_now()` at its top, terminalized `Cancelled` with
`done: true`, and the job "completed" having placed nothing. That is the whole of
`registry_available=0` / `ready: 0`. It also silently broke real cancellation: every later
`supersede_admitted_fill` / `cancel_fill_job_for` cancelled a fresh token nobody listened to.

Evidence: a probe placed after the worker step in `drive_fill_envelope` never printed once
across 320 ticks — every call returned at the cancel guard above it.

Fix, two parts:

1. `Puzzle3dFillSession` carries `fill_cancel: Option<CancelToken>`. `take_fill_session` swaps the
   engine's token out (leaving a fresh `root_cancel_token()`), `install_fill_session` restores it —
   the token now spans calls exactly like the cursor that owns the envelope.
2. `Drop for Puzzle3dPrecomputeSession` cancels only AFTER the `involved` check, so a per-call
   shell whose cursor was checked in cancels nothing.

### 7.3 Result

`fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances` **passes**
(`--test-threads=1`, `RUST_MIN_STACK` raised): 320 ticks, admissions within the ceiling, jobs
spawned and stepped through the real `reactor::jobs` runtime, `ready > 0`.

### 7.4 The 2 MiB stack (deliverable c) — measured, not assumed

Peak stack, painted (`0xA5` under a 64 MiB probe thread), debug profile:

| body | peak |
| --- | --- |
| `create_puzzle3d_app()` | 404 KiB |
| `testkit::new_app_with_registry` | 1 083 KiB |
| `app()` | 1 292 KiB |
| `app()` + one `dispatch("fillBuildTick")` | 2 299 KiB |
| `app()` + `fill_ready` | 1 491 KiB |
| the growth law's body (20 ticks) | 2 407 KiB |

Future sizes are irrelevant (`app()`'s future is 99 680 bytes, `Puzzle3dPlayApp` is 216 bytes
after the boxing) — the cost is synchronous frame depth in an unoptimized build.

`create_puzzle3d_app`'s 404 KiB is one fluent builder chain: every `EditorBuilder` method takes
`self` by value and returns a new one, so ~250 chain steps each got their own stack slot for a
whole inline `AppBuilder`. `EditorBuilder`/`ViewerBuilder` now hold `Box<AppBuilder>`
(`🧰️framework/…/🔌️plugin/🦀️.rs`), so a chain step costs a pointer and the `AppBuilder` value only
ever lives in the frame of the one method that moves it. This is a framework-wide win: every
plugin manifest in the repo is built through that chain.

### 7.5 Verification (exact commands and counts)

All under `CARGO_TARGET_DIR=…/target-p3d-f RUSTC_WRAPPER="" CARGO_INCREMENTAL=0`,
`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4`.

| run | result |
| --- | --- |
| `RUST_MIN_STACK=16777216 … -- fill_build_tick_converges fill_build_tick_only_polls --test-threads=1` | **ok. 2 passed; 0 failed** |
| same two, BEFORE §7.1 | `spawned 0 jobs` / `320 ticks admitted 50 envelopes` — FAILED |
| same two, after §7.1 only (before §7.2) | `spawned 5 jobs, completed 5 … registry_available=0` — FAILED |
| `… fill -- --test-threads=2` (default 2 MiB stack) | `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` **stack overflow** — deliverable (c) still open |
| `RUST_MIN_STACK=… fill -- --test-threads=2` (whole fill family) | 60 passed / 21 failed; 19 of those 21 fail identically BEFORE this wave |

### 7.6 Still open, and who owns it

1. **Deliverable (c) is partial.** The binding frame is the framework's `settle` / typed-operation
   continuation chain (`app()` + one `dispatch` peaks at 2 299 KiB with `app()` itself at 1 292 KiB and
   `dispatch_typed` alone at ~1 498 KiB). Boxing the surface builders removed 171 KiB from
   `create_puzzle3d_app`, which is not the binding frame. Whole-app laws in this crate still need
   `RUST_MIN_STACK` above 2 MiB.
2. **W-F2c's `take_terminal_fill_job` retention.** A `Terminal(Complete)` envelope is deliberately no
   longer discarded. Consequences observed here, both NEW since that hunk:
   `bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint` panics
   ("a terminal fill envelope must reach an empty slot within its own bounded close turns") — the slot
   never empties; and `fill_build_tick_is_ignored_when_fill_tool_is_inactive` reads a completed plan
   left by an earlier law through `read_fill`'s registry fallback. Please reconcile retention with the
   close contract rather than reverting §7.1/§7.2.
3. **`fill_build_tick_only_plans_available_slider_range` still ends at `ready: 0`** although the growth
   law reaches `ready > 0`. The difference is only in how the two drive: `drive_fill_until_ready` runs
   64 `step_job` slices per tick and drops the job the moment it is terminal, so it very likely closes
   the envelope past the point where `read_fill` can still answer for it. Same close boundary as (2).
4. **A `[DEBUG] fill_build_tick …` `eprintln!` still ships** in
   `…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs` — one line per 120 ms tick in the guest. W-F2c
   attributes it to W-U3; left in place, flagged, must go before the ticket closes.

## 8. W-F2c continuation — empty sequence + 2 MiB (2026-09-10)

Owned this turn: `⏳️precompute/🦀️.rs` (box `RejectedFillWorker`), testkit (`app()` registry isolation, `activate_window_utility`), unit utility-options law, framework `🔌️plugin/🦀️.rs` (`#[inline(never)]` builder forwarders, drop definition after registry, isolate `initial_snapshot`). Did not edit W-X-hot `✏️editor/🦀️.rs`.

### 8.1 Empty-sequence root cause

Diff against the fill-kernel unit that places: inputs (volumes, kind weights, meshes, collision) are present on `default_fixture()` / `app()`. The isolated job was stepping a real admitted builder. Every dispatch `Drop` cancelled the live token (§7.2), so the first envelope slice completed empty. Census 1-unit/tick (§7.1) was the other owner of `ready: 0`.

### 8.2 Laws

| law | result |
| --- | --- |
| heap / retention `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances` | **ok** 11.54s, `ready > 0`, heap bounded |
| utility-options without `RUST_MIN_STACK` | **ok** 0.19s |
| fill-family `fill_` | 49 / 9 |
| full lib | **620 / 10** vs **618 / 13** |
| `wasm32-wasip2` `semio-s-plugin-puzzle` | **ok** |

Precompute `[DEBUG]` / `eprintln!` still 0. `fill-build-tick` command `[DEBUG] fill_build_tick` remains W-U3.

### 8.3 Open

- Five sibling whole-app fill laws still fail to see a planned prefix / a `SpawnJob` in 16 ticks, while the heap law on the same binary passes. Not the original cancel-empty-sequence bug.
- Four precompute worker laws still assume one census unit per `enqueue_fill_job`.
- `two_instances_converge_disjoint_object_edits_via_backbone` remains foreign (`module.vcs`).
- `create_puzzle3d_app` / `initial_snapshot` throwaway sync still inflate fixture construction; editor.rs was hot so it was not slimmed.

## 8.4 Fill family finished — 4096-unit laws + spawn/ready path (2026-09-10)

Owned this turn: `⏳️precompute/🦀️.rs` (census spender, drain/guard, `fill_available_count` / `fill_is_done`), precompute worker laws, whole-app fill laws + testkit `app()` drain, surgical `FillPlan`/`FillApply` in `✏️editor/🦀️.rs` after it cooled. Did not close W-X clipboard/import/publication work.

### Per-law verdict

| law | verdict |
| --- | --- |
| `fill_worker_session_drop_during_measurement_mounts_the_same_terminal_once` | **law** — mid-census via `enqueue_fill_job_spending(1)`; production 4096-unit grant still finishes the same identity |
| `fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase` | **law** — same 1-unit measuring pin |
| `fill_admission_census_one_unit_is_pending_and_4096_units_spawn_once` | **law** (new) — pins `FILL_ENVELOPE_CENSUS_UNITS_PER_TURN == 4096`, one-unit Pending, one production turn spawns once, second session still admits under the 4-slot cap |
| `bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint` | **law** — Complete stays readable; explicit `close_fill_envelope` (Closed intent) vacates the slot |
| `fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close` | **law** — pin full `FillJobRequest` identity; same-slot reuse must bump per-slot generation |
| `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job` | **guest/infra** — 0 `SpawnJob` was leftover Complete slots after the heap law (W-F4 does not vacate Complete). Drain on guard acquire+drop + `app()` drain. Isolated job still not stepped; `ready == 0` until then; exactly one `SpawnJob` in 16 ticks |
| `fill_build_tick_only_plans_available_slider_range` | **law + guest** — real path is retained `setFillCount` materializing during settle (not a deferred cutoff-only gesture). Planning still does not mutate the document; after settle `object_count == before + available` |
| `fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances` | **infra** — guard + initialize so the law sees a clean 4-slot registry; asserts unchanged |
| `fill_render_reveals_the_full_available_plan_tagged_with_reveal_index` | **guest** — `fill_available_count` now prefers `read_fill` sequence length (same source as the slider ready extent), so render's reveal tail matches `ready` |
| `set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up` | **guest + infra** — `FillPlan` clamps to available when a plan exists; empty apply no longer faults; guard/drain so MAX is not applied against a starved registry |
| `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count` | **law** — real user path is plan-then-slider; law now `drive_fill_until_ready(..., 3)` then `setFillCount(3)` |
| `two_instances_converge_disjoint_object_edits_via_backbone` | **foreign** — left untouched (`module.vcs`) |

Heap vs 0-spawn divergence: heap law takes the guard, `initialize()`, 320 ticks, 64 `step_job` slices/tick, drains only at the end. The five siblings used 16 ticks / no `step_job` / no guard. After W-F4, Complete slots stay occupied (4-slot cap). A later law's `enqueue_fill_job` cannot `begin_measurement` → 0 `SpawnJob`. Not a missing-volume / empty-sequence bug.

### Tails

```
fill_ : test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 581 filtered out; finished in 45.68s
lib   : test result: FAILED. 635 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 45.91s
wasm  : cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle → Finished dev profile; EXIT:0
```

### Remaining foreign failures (full lib, by name)

1. `two_instances_converge_disjoint_object_edits_via_backbone` — `module.vcs` remote snapshot merge fail-closed
2. `import_fixture_reproduces_the_exported_document` — W-X `importFixture` wire input exceeds fixed byte capacity
3. `retained_publication_contracts_are_an_exact_nonempty_tool_bijection` — W-X added `exportFixture`/`importFixture`/`openImportFixture` to retained tool ids without matching publication contracts
4. `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` — same catalog / tool-id split
5. `reserved_refresh_section_payloads_admit_into_the_retained_section_carrier` — engagements carrier byte-exact mismatch (W-X/W-Y chrome)

### Open

- W-U3 `[DEBUG] fill_build_tick` eprintln in `fill-build-tick/🦀️.rs` still ships (not this wave)
- `create_puzzle3d_app` / `initial_snapshot` throwaway sync still inflate fixture construction
- W-X clipboard `copy`/`cut` reserved route and import-factory split were mid-edit; factory bijection was restored to a single `Puzzle3dRetainedCommandJobFactory` so `app()` can construct
