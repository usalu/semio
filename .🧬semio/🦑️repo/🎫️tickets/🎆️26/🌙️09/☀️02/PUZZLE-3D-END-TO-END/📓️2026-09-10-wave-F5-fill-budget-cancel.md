# W-F5 — The host's fill-job lifetime cap, and the cancel it turned into a failure

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave F5, following W-F4
(`📓️2026-09-09-wave-F-fill-oom.md` §7). Written incrementally while the work ran.

## 0. Assignment

Three defects, all observed on release build #28 (2026-09-10 10:05) after W-F4's fixes made the
Fill plan actually produce readiness (`fillBuildTick` 280–450 ms/tick, `ready: 12–15`,
`Cancel fill` visible):

1. ~76 ticks (~40 s) in, the React host printed
   `[DEBUG] plugin job semio.puzzle3d.fill#490 exceeded its 65536-step host budget — cancelling`
   and cancelled a healthy plan.
2. The cancel that followed trapped the guest — wasm `unreachable` → `shard 0 lost` →
   `actor-activation.revoked` on every later call.
3. A per-tick `[DEBUG] fill_build_tick … changed= faulted=` `eprintln!` flooded the browser
   console at ~7 000 lines/min.

## 1. Defect (1) — a host may not decide a plan is over

### 1.1 What the two hosts did

`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (the isolated bounded-job
driver W-J added) carried

```ts
const PLUGIN_JOB_STEP_LIMIT = 1 << 16;
…
while (step < PLUGIN_JOB_STEP_LIMIT) { … }
console.warn(`[DEBUG] plugin job ${kind}#${job} exceeded its ${PLUGIN_JOB_STEP_LIMIT}-step host budget — cancelling`);
await serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job));
```

That is a **lifetime** bound expressed as a host constant. The native host does not have one.
`ShardLoop::pump` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1671-1760`)
selects one `running_jobs` entry per pump turn, computes
`job_budget_from_grant(self.granted_budget(actor_id))` — a `JobBudget { fuel, deadline_ms }`, i.e.
a **per-slice** bound — steps the job once, and routes only what the guest itself answered
(`JobStep::Running` / `Done` / `Failed`). There is no step counter anywhere in that walk; a job ends
when its owner says so, when the actor is lost, or when something explicitly cancels it.

### 1.2 Why 65 536 is reachable in production

`PUZZLE3D_FILL_COUNT_MAX = 1000`. One `Puzzle3dFillBoundedJob::step` is one
`drive_fill_envelope` call — one bounded worker slice over the Nakagin / Concrete Forest census, not
one placement. The browser drives 32 slices per `yieldPluginUiContinuation`, and the trace shows the
cap hit at ~76 `fillBuildTick` ticks ≈ 40 s. So the cap is not a safety net that only a runaway job
reaches; it is squarely inside the working range of the feature.

### 1.3 The fix

`PLUGIN_JOB_STEP_LIMIT` is deleted. `driveSpawnedJob`'s loop is now `while (live())` — the same
three-way end condition the native walk has: the guest's own terminal, the instance going away, or an
explicit `Effect::CancelJob`. The per-slice `jobStepBudget`
(`{ fuel: DEFAULT_SHARD_BUDGET.fuel, deadlineMs: DEFAULT_SHARD_BUDGET.wallMs }`, already the web twin
of `job_budget_from_grant`) is unchanged and is now the ONLY budget the driver imposes.

The reactor half of the same contract is pinned natively — see §4.

## 2. Defect (2) — cancelling a stepping fill job

### 2.1 What reproduced natively, and what did not

Two new whole-app laws drive the real thing: `app()`, Fill activated, `fillBuildTick` ticked with the
bounded job started and stepped through `semio_framework_plugin::reactor::jobs` until the count slider
reports readiness, then cancelled — once through the user's own path (`cancelFillBuild` →
`Effect::CancelJob` → `jobs::cancel_job`) and once through the host's own (`jobs::cancel_job` alone,
which is exactly what `PLUGIN_JOB_STEP_LIMIT` did).

**No native panic reproduced.** What did reproduce, on the first run, was a mis-classification:

```
thread '…::cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run' panicked at …:2239:
a run the user cancelled is not a failure — the fault guard must not outlive a deliberate cancel:
["Fill planning stopped — the background job failed"]
```

That is the law failing, not the guest — but it names a real defect, below. The browser's `unreachable`
is therefore NOT attributable to a panic on this path from the evidence I have; what IS established is
that the host had no business issuing that cancel at all (§1), and that the cancel path it used
reported a healthy user-visible failure. The honest statement: defect (1) removes the browser's trigger,
and the cancel path is now covered by laws in both cancel orders; whether a wasm-only frame remains
cannot be settled without a browser repro, and build #28's trace does not localise one.

One narrowing that IS free: `[profile.wasm-release]` inherits `[profile.release]` and neither sets
`debug-assertions`, so the served component runs with them OFF. Every `debug_assert!` guard on this
path — `retire_rejected_fill_worker`'s terminal-emptiness check in `⏳️precompute/🦀️.rs`, the `Drop`
guards in `🧰️framework/🔨️modules/🧵️job/🦀️.rs` — is inert in the browser and cannot be the
`unreachable`. A recurrence is therefore a plain `assert!`/`expect`/`panic!` or an overflow check, and
`FillEnvelopeTerminalHandle::close_step`'s `checked_sub(…).expect("reserved fill byte credit")` is the
one such site on the envelope's own close path.

### 2.2 Defect — a deliberate cancel was reported as a fault

`Puzzle3dFillBoundedJob::cancel` (`✏️editor/⏳️precompute/🦀️.rs`) only tripped the envelope's cancel
token. But `jobs::cancel_job` calls `cancel()` and then **drops the owner** — no later `step` will ever
reach `drive_fill_envelope` to observe that token. The drop released a still-armed
`FillEnvelopeWorkerFaultGuard`, whose `Drop` calls
`request_fill_envelope_terminal(request, FillEnvelopeTerminalReason::Fault)`. The intent register takes
`fetch_max` over the reason codes (`Complete=1 < Cancelled=2 < Fault=3 < Closed=4`), so `Fault` won
outright. `Puzzle3dPrecomputeSession::observe_fill_terminal_reason` then latched `fill_faulted`, and the
next `fillBuildTick` raised the `fill_failed` notice — *"Fill planning stopped — the background job
failed"* — on every single deliberate cancel.

Fix: `cancel()` now owns the run's last word. It trips the token, requests
`FillEnvelopeTerminalReason::Cancelled` itself, and **disarms** the fault guard. A cancel that arrives
while the job is still in its `Admitting` stage takes the same route through
`FillEnvelopeJobEntryCursor::cancel`.

### 2.3 Defect — `Cancel fill` outlived the run it named

With 2.2 fixed the law's next assertion failed: the `Cancel fill` affordance was still published 16
ticks after the cancel. `fill_tool::cancel_measure` gates on `precompute.fill_job_identity()`, and that
answered `Some` for as long as `self.fill_job` was set — which is until the envelope's close cursor has
given the slot back, one retained owner per tick, ~100 ticks (≈12 s at 120 ms) on the `app()` fixture.
So the panel kept offering to cancel a run that was already over.

Fix: `fill_job_identity` now answers only for a run this session can still advance — no checked-out
terminal handle, and a registry authority whose phase is `Measuring`/`Admitted`
(`FillEnvelopeRegistry::is_live`). `cancel_fill_job_for`'s identity guard rides the same predicate, so a
cancel against an already-terminal run is a no-op instead of a second cancel.

## 3. Defect (3) — the per-tick console flood

`✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`'s `eprintln!("[DEBUG] fill_build_tick …")` ran once per
120 ms tick per instance. Removed.

## 4. Laws

### 4.1 Native — `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, region `🪣️FillJobLifetime`

| law | states |
| --- | --- |
| `a_plan_larger_than_one_host_slice_completes_without_a_host_imposed_cancellation` | A registered `BoundedJob` (`puzzle3d.test.long-plan`) that returns `Running` for `LONG_PLAN_SLICES = 70_000` slices under an UNCHANGED per-slice `JobBudget` reaches `Done` on exactly slice 70 000. Nothing in `reactor::jobs` fails it for length, and the constant budget is not a stall — which is what lets a host grant the same budget every slice. |
| `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run` | The user's path end to end: readiness reached through the real bounded job, `Cancel fill`'s published `(job, operation, generation)` read off the measure, `cancelFillBuild` dispatched, its `Effect::CancelJob` asserted to name exactly that job, `jobs::cancel_job` run, then 16 more ticks. No `fill_failed` notice, the affordance stops naming the cancelled run, the slot comes back. |
| `a_host_initiated_cancel_of_a_stepping_fill_job_leaves_the_guest_serving_further_ticks` | The host's path: `jobs::cancel_job` with no `cancelFillBuild` before it — exactly what `PLUGIN_JOB_STEP_LIMIT` did, and what an instance going away mid-plan does. Every later tick still dispatches, no `fill_failed`, the slot comes back. |

`fill_cancel_identity` in `🧪️tests/🔬️testkit/🦀️.rs` reads the triple off the published `Toggle`
measure's own `on_change` args — the same bytes the host dispatches — rather than reaching into the
session.

### 4.2 Vitest — `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`

`bounds a job per slice and never ends one the guest is still running`: the driver is given a job that
answers `running` for `(1 << 16) + 1024 = 66 560` slices. It must step all 66 560, call `cancelJob`
never, and pass ONE distinct budget — `${DEFAULT_SHARD_BUDGET.fuel}/${DEFAULT_SHARD_BUDGET.wallMs}` —
on every slice, then feed the guest's `done` back as `job-completed`. The harness's `stepJob` stub now
forwards the budget so the law can see it.

Fails before the fix, exactly where it should:

```
AssertionError: expected 65536 to be 66560 // Object.is equality
```

### 4.3 One order-dependence repaired in passing

`fill_build_tick_is_ignored_when_fill_tool_is_inactive` took no fill-registry guard, and
`fill_progress_summary` falls back to the PROCESS-WIDE envelope registry when the app has no plan of
its own — so it read a concurrent law's admitted plan as its own progression (W-F4 §7.6 (2) named this
leak). It now takes `fill_envelope_test_guard()` like every other fill law in the file.

## 5. Verification

All native runs under
`CARGO_TARGET_DIR=…/target-p3d-f RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864`,
`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4`.

| run | result |
| --- | --- |
| the three new laws, `--test-threads=1`, BEFORE the §2.2 fix | 2 passed / 1 failed — `["Fill planning stopped — the background job failed"]` |
| after §2.2, before §2.3 | 2 passed / 1 failed — `the Cancel fill affordance must disappear once the run it names is gone` |
| the three new laws, `--test-threads=1`, after both | **ok. 3 passed; 0 failed** (9.18 s) |
| whole lib `--test-threads=2`, before §4.3 | 641 passed / 3 failed (114 s) |
| whole lib `--test-threads=2`, final | see §5.1 |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` | **`Finished dev profile` in 5m 03s, EXIT 0** |

### 5.1 The whole-lib number is load-bound, and has to be read as such

A `--test-threads=2` lib run taken while peer sessions were saturating the box (17 concurrent `rustc`,
`load average 56–70`) reported **636 passed / 8 failed in 330 s** — against 641/3 in 114 s an hour
earlier on the same tree. Five of those eight are timing-driven fill laws (`… only_polls …` saw
`spawns 0` in its 16 ticks; `… only_plans_available_slider_range` and `fill_count_is_shared …` saw no
planned prefix) and one — `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` — is a
brush-lane law this wave does not touch at all. Every one of them budgets its work on the wall clock
(`BatchDriveConfig { step_budget_us: 2000 }`, `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US = 500`), so a thrashing
machine makes a bounded tick do nothing and the law starve. The number to trust is the one taken on a
quiet box, below.

On a quiet box (71.5 s) the same run reports **640 passed / 4 failed**. All three new laws pass. Of the
four failures, two (`a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh`,
`window_options_are_local_to_the_window_instance_not_shared_across_split_panes`) are other waves' laws
this one does not touch, and `two_instances_converge_disjoint_object_edits_via_backbone` is the standing
foreign `module.vcs` failure.

The fourth, W-F4's `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances`, needs
naming precisely because it is a fill law:

| run | result |
| --- | --- |
| whole lib `--test-threads=2` before the §4.3 guard | **passed** |
| whole lib / fill family `--test-threads=2` after it | fails: `320 ticks admitted 2 envelopes … spawned 1 jobs, completed 0, faulted 1 (puzzle3d.fill-job.stale)` |
| **alone, `--test-threads=1`** | **ok. 1 passed; 0 failed** (14.93 s) |
| **whole fill family `-- fill_ --test-threads=1`** | **ok. 61 passed; 0 failed** (62.03 s) |

Passing alone on the identical binary means the production path this wave changed is not what breaks
it — the same code runs in both runs. It is a test-isolation interaction: `FILL_ENVELOPE_MAX_OPERATIONS`
is four process-wide slots, this wave added three more laws that admit into them, and the law's own
`Stale` verdict is a superseded envelope, not a broken plan. W-F4 verified this law under
`--test-threads=1` for the same reason. The clean fix is to give the fill family a real per-law registry
rather than a shared mutex plus drains — out of this wave's remit, and recorded in §8. Run
single-threaded the whole fill family is green, this wave's three laws included: **61 passed, 0
failed**, against W-F4's 59/0 plus the two new fill laws.

Vitest, `cd 🧰️framework/…/🎯️targets/⚛️react && bun ./📜️script.ts test long --run '🔌️PluginRuntime'`:

| run | result |
| --- | --- |
| whole suite, after | **90 passed (90)**, 12.58 s |
| the new law alone, after | **1 passed \| 89 skipped**, 62 ms |
| the new law alone, with the 65 536 cap restored | **1 failed** — `expected 65536 to be 66560` |

`bun ./📜️script.ts typecheck`: **824 errors, none in either file this wave touched**
(`🧱️elements/🔌️PluginRuntime/🟦️.tsx`, `🧪️tests/🔌️plugin-runtime/🟦️.tsx` — zero hits each), against the
≈820 pre-existing elsewhere.

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `PLUGIN_JOB_STEP_LIMIT` deleted; `driveSpawnedJob` loops `while (live())` on the per-slice budget alone; the cap's cancel + fault completion and its `[DEBUG]` lines gone |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | budget forwarded into the `stepJob` hook; new per-slice law |
| `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/⏳️precompute/🦀️.rs` | `Puzzle3dFillBoundedJob::cancel` terminalizes `Cancelled` and disarms the fault guard; `FillEnvelopeJobEntryCursor::cancel`; `FillEnvelopeRegistry::is_live`; `fill_job_identity` answers only for a live run |
| `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs` | the per-tick `[DEBUG] fill_build_tick` `eprintln!` removed |
| `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `fill_cancel_identity` |
| `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | three new laws + the `🪣️FillJobLifetime` region; guard on the inactive-tick law |

## 7. Served wasm

Rust changed in the puzzle plugin (`⏳️precompute/🦀️.rs`, `🪣️fill-build-tick/🦀️.rs`), so **the served
wasm must be rebuilt** before §2 or §3 is visible in the browser. The §1 fix is the host half and is
TypeScript — it reaches the browser on a plain reload. `cargo check --target wasm32-wasip2
-p semio-s-plugin-puzzle` is green on this tree (5m 03s, EXIT 0), so the rebuild has nothing to fix
first.

## 8. Open

1. **The browser `unreachable` is not localised.** §2.1: the native reproduction of both cancel orders
   does not panic. The trigger that produced it on build #28 (the host cancelling a healthy plan) is
   gone, and the cancel path is now covered by laws, but if a trap recurs after a wasm rebuild it needs
   a guest backtrace, not another native pass.
2. `[DEBUG] spawn-job routed …` remains in `routeHostEffects` — once per spawned plan, not a flood;
   left for the debug-trace inventory to sweep with the rest.
3. `two_instances_converge_disjoint_object_edits_via_backbone` is still foreign (`module.vcs`).
4. **The fill family needs per-law registry isolation, not a shared mutex.**
   `fill_envelope_registry()` is one process-wide authority with four slots, guarded in tests by ONE
   `fill_envelope_test_guard()` mutex plus `drain_fill_envelope_registry_for_test()` on drop. That
   serializes the laws that take the guard but does not isolate them: a law's verdict still depends on
   what the previous holder left, which is why
   `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances` passes alone
   (`--test-threads=1`, 14.93 s) and fails in a `--test-threads=2` family run that this wave's three
   extra admitting laws now share slots with (§5.1). The durable fix is a registry the fixture owns
   per law; W-F4 hit the same wall and worked around it the same way.

