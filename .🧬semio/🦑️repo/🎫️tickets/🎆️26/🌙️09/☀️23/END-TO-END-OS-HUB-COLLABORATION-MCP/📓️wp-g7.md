# WP-G7: artifact_create Job, Demand-Driven Epoch, Clock Attribution, Suite Reds (Session 10)

Slice G7 · session 10 · 2026-09-24. Ports: serves 6340–6349, hubs 7860–7869. Continues G6 §9.
Status: DONE on the host side. All five items are landed and measured. The live over-MCP guest clock count waits on W1's wfc rebuild (§8.1).

## 0. Headline

| | G6 | G7 | capture |
|---|---|---|---|
| `artifact_create` | synchronous, no progress, cold compile uninterruptible | **job** (`jobId`, `creating: <phase>` progress, `job_get`/`job_cancel`/`notifications/cancelled`); a cancel mid cold compile is **answered in 1 ms**, the isolated compile worker is killed, nothing is persisted | `g7-create-cancel-live.txt` |
| cold compile under cancel | finished in the background into the cache | killed when its **last** requester cancels; shared by concurrent requesters (one worker), a detaching requester answered in ≤ 29 ms, the rest get the code | `compile-law1.txt` |
| idle gateway with a live engine | ~8 % of a core (1 ms epoch keeper) | **0.00 s CPU in 30 s** | `g7-idle-engine.txt` |
| solve: process CPU vs solving-thread CPU | 20.3–22.5 s vs 17.5–18.1 s | 16.88–17.02 s vs 16.84–16.97 s (keeper gone) | `g7-solve-pre*.txt` |
| over-MCP / native CPU factor (3 runs) | 1.50–1.66× process, 1.29–1.33× thread | **1.28–1.30× process** (with the G6-era guest) | §5.2 |
| framework clock reads per wfc solve | 2.72 M (live) | **0.49 M natively** (2 per step, −82 %); the live count needs W1's guest rebuild | `clock-attr1/3.txt`, `g7-clocks-oldguest.txt` |
| suite reds | async 2, shard 4, job (parallel races), wfc-engine 1 | **async 76/76, shard 69/69 ×3, job 30/30 in 1 000 loops, wfc-engine 301/301** | §4 |

Gates: client-e2e **38/38**, stdio transcript **17/17**, live-agent-loop **20/20**, crossing-grant law green, binding-cancellation law **1/1** (+ the new artifact-create and compile cancellation laws).


## 1. artifact_create As A Job (progress, cancel, compile abort / detach)

### 1.1 Cold compiles are cancellable (`🏠️workspace/🦀️.rs` `CompileFlight`, plugin-host `🧊️IsolatedCompile`)

- Cranelift has no interruption point, so a cold compile now runs in an **isolated worker process**: `semio-os-mcp compile-component --engine <cfg> --out <cache.cwasm>` (component on stdin, no environment inherited, runs before the credential seal like `schemas`). Its whole body is `semio_framework_plugin_host::compile_component_isolated`, which rebuilds the host engine from `SharedEngineConfig::to_isolated_arg` (the gateway's unmetered, pooling-or-not config) and stores the result through the ordinary `.cwasm` cache.
- `store_compiled_component` now writes `<name>.<pid>.scratch` and renames it into place (`compiled_cache_scratch_path`), so no reader ever sees a partial `.cwasm` and a killed writer leaves only its own scratch file, which the host removes.
- `CompileFlight`: one flight per content hash, shared by every requester while it runs. A requester waits on `select2(answer, cancel.cancelled())`; a cancel answers that requester at once and detaches it. When the **last** requester detaches, the worker is killed and reaped under the flight lock, its scratch output removed and the flight dropped, so **nothing completes in the background after a cancel**. A requester still attached keeps the compile alive and receives the compiled code.
- `scoped_compiled_component` order is now: process map → `LoadingCompiledCode` (on-disk `.cwasm`, `GuestRuntimes::load_compiled`) → only then read the component (if not already read for hashing) → `CompilingComponent` (isolated flight for a JIT runtime, in-process for the owned interpreter). A warm start no longer reads the 125 MB component at all.
- Phases are neutral now: `ActivationPhase` (was `BindingPhase`) gained `compiling-component`; every phase is interruptible, so the `interruptible` flag is gone. Schema: `🏠️workspace/🧬️schema` `SessionActivationPhaseV1`/`SessionActivationPhaseRowV1`/`SessionActivationPhasesV1`; the inference binding law refers to them (its own phase defs are removed). Each job maps `ActivationPhase::at(fraction)` onto its own span (`inference_run` 0.15–0.25, `artifact_create` 0.05–0.95).

### 1.2 artifact_create is a job (`🗿️artifact/🦀️.rs`, `HeadlessWorkspace::create_plugin_artifact{,_cancellably}`)

- The handler mints `artifact.create` in the job registry (bound to the call's progress token, so `notifications/progress`, `job_get`, `job_cancel` and `notifications/cancelled` all work), binds the cancel hook, and runs the create on the pool's Io lane via `run_activation_job` (was `run_binding_job`). Progress rows read `creating: <phase>`.
- The create opens its channel scoped (`open_session_artifact_channel_scoped` → `open_plugin_artifact_channel_scoped`), activates the guest in scope (`ArtifactChannel::activate`), enters `reading-document` for the genesis `ReadArtifact`, and checks the cancel once more just before persisting. A cancel answers `status: CANCELLED`; nothing is persisted. The shell route (`ShellDirect`) goes through the same job, scope and checkpoints.
- Output schema (`artifact_create_output_shape`, mirror regenerated with `schema-mirror`): `jobId`, `status` (`SUCCEEDED`|`CANCELLED`), `pluginId`, `appId`, `sizeBytes`. `🗿️artifact/🧬️schema` (new) holds `ArtifactCreateJobStatusV1` and the create law's schema.

- Live over stdio (`g7-create-cancel-live.txt`, cold private `SEMIO_HOME`, the 125 MB wfc component): `artifact_create` reported `resolving → reading → hashing (×30 chunk rows) → loading-compiled-code → compiling-component`. A `notifications/cancelled` 2 s into the compile was **answered CANCELLED 1 ms later**. The compile worker (pid 68359) was gone at the next check, and the cancelled artifact does not exist. The next `artifact_create` compiled cold in a new worker (21 s), and the solve SUCCEEDED.

### 1.3 Laws

| law | fixture | replay | result |
|---|---|---|---|
| compile cancellation: sole requester cancels → worker killed, no `.cwasm`, no scratch; both of two cancel → same; one of two cancels → it is answered, the other gets the code from the **same single worker**; later request is warm with no worker | `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json` (`ComponentCompileCancellationLawV1`) | `[[test]] compile_cancellation_law` — real `semio-os-mcp compile-component` workers (`CARGO_BIN_EXE`), smallest staged component (`cad-extension-spatial-shape`, 19.6 MB), private empty `SEMIO_HOME` | **1/1**; cancels answered in 1.9 ms / 29.3 ms + 0.4 ms / 0.07 ms (ceiling 100 ms), `live_component_compile_workers() == 0` after every case (`compile-law1.txt`) |
| binding cancellation (7 phases, positions = Rust) | `💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json` | `binding_cancellation_law` | **1/1**, cancels 0.09–24 ms (`binding-law1.txt`) |
| artifact_create cancellation: cancel at resolving / opening-guest / reading-document → CANCELLED and nothing persisted; uncancelled → the guest's genesis document | `🗿️artifact/🧫️fixtures/⏱️create-cancellation-law.json` (`ArtifactCreateCancellationLawV1`) | same test binary, after the binding cases (warm wfc) | **pass**, cancels 0.17–0.61 ms |


## 2. Demand-Driven Epoch Ticker

`🔌️plugin/🖥️host/🦀️.rs` `EpochDeadlines` replaces `EpochTicker` (the 1 ms `PeriodicPoolTimer`, which remains only for the shard heartbeat):
- Every `Store` installs one epoch callback reading its own `EpochDeadlineCell` (a wall deadline on the pool clock): `Interrupt` once passed, `Continue(1)` otherwise.
- Every guest entry arms a deadline for exactly the call (`EpochDeadlines::arm` → an `EpochDemand` guard held across `run_concurrent`): `instantiate`, `execute_turn`, `start/step/cancel_job` (30 s watchdog), and now also `checkpoint`/`restore` (they used to inherit whatever the previous crossing left) and the four `codec` calls (armed per call, not from `instantiate`).
- One pool timer is registered for the earliest deadline not yet advanced through; when it fires the engine epoch advances **once** and every store whose own deadline is still ahead pays one callback and continues. No call armed → no timer, no advance. A withdrawn deadline's timer fires once at its instant and does nothing.
- The async actor runtime (`⏳️runtime`, test-only) timeslices with `Yield(1)`: each in-flight command task holds an `arm_timeslice()` demand, so its 1 ms slices exist only while a command runs.
- Live (debug `semio-os-mcp`, stdio): idle gateway with a live wfc engine and guest (after `artifact_create`) burns **0.00 s CPU in 30 s** (`g7-idle-engine.txt`; G6 measured ~8 % of a core whenever an engine existed). During a solve, whole-process CPU now equals the solving thread's: 17.01 s process vs 16.97 s thread (`g7-create-cancel-live.txt`). G6 measured 20.3–22.5 s vs 17.5–18.1 s.
- Laws (`🧪️tests/🔬️shared-wasmtime-engine`): `an_idle_engine_registers_no_timer_and_never_advances`, `an_armed_deadline_interrupts_the_guest_with_one_advance` (a `(loop br 0)` guest is cut at ≥ 40 ms by exactly 1 advance, 0 timers left), `a_far_watchdog_costs_one_registration_and_another_stores_deadline_does_not_cut_it` — **3/3**, plus shard/wasmtime-runtime/guest-cold-relay **115/115** (`host-epoch1.txt`).


## 3. Clock-Read Attribution In Framework Code

### 3.1 Attribution (manual counters; `dtrace` needs root under SIP and `sudo` wants a password)

A temporary `[DEBUG] g7` test (since removed) installed a counting clock with `semio_framework_job::install_microsecond_clock` and ran the relay-crossing replay natively (`run_genesis_crossings`, the gateway's exact `start_job`/`step_job` walk; same guest code compiled natively). It recorded a symbolized backtrace for ~1/256 of the reads. The first run used `count % 256` sampling, which aliased with the per-step read pattern once reads became periodic, so the second run uses hashed sampling. Captures: `clock-attr1.txt` (before), `clock-attr3.txt` (after).

**Before: 2 697 330 reads per solve** (matches G6's 2.72 M wasm host calls). Eleven sites, each about 245 k, which is **11 reads per worker step** over ~245 k steps. WFC ends a step at every preview, i.e. every 16 units:

| # | site | redundant with |
|---|---|---|
| 1 | `drive_worker_job_authority` reads the clock to derive the step budget | — (entry reading) |
| 2 | `drive_step_with_payload_ledger`'s admission check `now >= deadline` | #1 (the budget was just derived from it) |
| 3 | `Watchdog::start` | #1 |
| 4 | first `ClockStride` read in `WfcJob::step` ("a new budget reads first") | #1 |
| 5, 6 | `StepContext::set_stage` → `record_stage_changed` trace timestamp (bitmap job + WFC job) | the step's latest reading |
| 7 | `should_yield` in `RetainedJobPayloadWriter::write_slice_page` | per-unit clock (unstrided) |
| 8 | `should_yield` in WFC `Publication::poll` | per-unit clock (unstrided) |
| 9 | `Watchdog::finish` | — (exit reading) |
| 10 | `record_preview_published` trace timestamp | #9 |
| 11 | `semio.infer` `Pump`'s per-transition `clock_spent` | #9 |

### 3.2 Removed at the root (`🧵️job`, `⏱️trace`, `💡️infer`, `🀄️wfc` engine)

- A worker step reads the clock **once on entry and once on exit**. The entry reading is the budget start, the admission check, `Watchdog::start_at` and the step clock's first value (`ClockStride::begin`). The exit reading is `Watchdog::finish_at`, the outcome's trace event (`record_trace_event_at`), and `WorkerJobSession::last_step_end_us`/`MountedWorkerJobSession::last_step_end_us` for the caller.
- `StepContext::now_us` is now strided by a `ClockStride` the worker authority carries from step to step, so every in-step deadline check (`should_yield`, `deadline_exceeded`, a job's per-unit reads) shares ~64 µs reads. `ClockStride` lost its per-job `now_us(context)`/`should_yield(context)` API; `🀄️wfc` `WfcJob`/`WfcRestore` drop their own stride and read `context.now_us()`.
- `StepContext::set_stage` stamps its trace event with the step's latest reading (`StepContext::latest_us`, at most one stride old).
- `Pump` decides whether to run another transition from the session's last step exit reading.
- **After: 490 616 reads per solve** (native, same replay) = 2 per step + a few hundred strided in-step reads, **−81.8 %**. Relay walk unchanged: 41 crossings.
- Law: `🧫️fixtures/🪜️clock-stride-law.json` restated. `stepReadsOutsideTheJob: 2` is replayed by `a_worker_step_reads_the_clock_once_on_entry_and_once_on_exit` (and the exit reading is kept for the caller). `newStep.readsForTheFirstCall: 0` is replayed by `a_new_step_starts_at_its_drivers_reading_and_keeps_the_calibrated_stride`. The interval/overshoot cases are unchanged. Job lib **31/31** (`job-clock2.txt`). Also green: trace 49/49, wfc engine 301/301, wfc-bitmap 152/152 (+1 ignored), plugin `jobs::` 39/39 (`suite-*.txt`).
- What remains is the floor for this plugin's cadence. WFC publishes a preview every 16 units (`PREVIEW_UNIT_INTERVAL`), and every preview ends a step. Reusing a step's exit reading as the next step's entry would fold the pump's own absorb work into the watchdog sample, so I did not do it.
- Guest rebuild requested from W1 (`wp-w1/requests/g7.txt`) for the live count.


## 4. Suite Reds (async, host-shard, job, wfc-engine)

Baseline (`host-lib-baseline.txt`, this tree before my edits): plugin-host lib 232/243, 10 red (4 shard, 3 ui-patch, 2 schema-parity, 1 owned-codec).

### 4.1 Host shard (4 red → 69/69, three runs: `host-shard-fix2-{1,2,3}.txt`)

Pre-G6: `git diff HEAD` over `🧵️shard/` shows only G6's one-line heartbeat edit in `🚚️process-transport` (not on these tests' path, which use `LoopbackTransport`), and `🧵️shard/🦀️.rs` + its tests are unchanged since commit 46c3cb9 (09-11). The reds are deterministic count/order mismatches, not timing.

- Root cause A (3 tests): `consume_frame` answered a frame for an unregistered actor with a `Fault` outcome but reported it as frame-only admission, so `pump` returned 0 and went on to grant another authority in the same drive. Fix: `FrameAdmission::{Deferred, Answered}`; an answered rejection is an outcome on the wire and spends the drive's one opportunity (`pump` → 1).
- Root cause B (`permanently_over_capacity…`): the registration check ran before the capacity preflight, so a frame that can never fit was answered as "not registered" instead of being retained exact for terminal close. Capacity is a property of the frame, not of its target: preflight now runs first. The two now-unreachable per-arm registration checks are removed.
- Root cause C (`cancel_unregisters…`): the test predates paged replay-seed retirement (one bounded close page per opportunity), so the third pump drove the cancelled seed's close. The test now drains the seed lifecycle, asserts nothing is published and no step ran, then asserts the pump is idle.
- Exposed by the fix: `cancel_job_effect_stops_a_job_before_it_is_ever_stepped` read the process-wide replay page/ABI counters without holding `replay_test_authority()`, so it raced other replay tests once those ran further. It now holds the guard.

### 4.2 Async (2 red → 76/76, `async-lib2.txt`)

- `cooperative_maintenance_live_host_revisits_queued_owner` (red since the helper's call site changed to `pump_runtime_live_cooperative_turn(cell)` and doc comments with the word "while" landed between the helper and `plugin_step_live_cleanup`, 08-27…09-02, long before G5/G6): the source predicate now ignores comment lines and matches the current call; hostile variants follow. A stale `[DEBUG]` print in the same test file is removed.
- `worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release`: the neutral DRR law (`🤝️cooperative/🧫️fixtures/🔣️.json`) says a `UserVisible` job (weight 4 of unit cost 8) is selected on the second host turn; the test pumped once. The test now submits on `Interactive` (selected on the first turn), which is what "one pump runs it" means under the law.


### 4.3 Job (parallel races → 30/30, 0 failures in 1 000 back-to-back runs of the test binary: `job-loop1000.txt`)

G6's failure did not reproduce on the first run (`job-lib1.txt`, 30/30); looping the test binary did (13th run). Each fix below was followed by another loop, which surfaced the next race; none depends on G6's changes (all are test-side assumptions about process-wide or timing state):
1. `worker_session_slots_max_plus_one…` fills all 256 process-wide `WORKER_JOB_SESSION_SLOTS` while other tests hold slots (8/60 fail). Fix: `WORKER_SESSION_SLOTS_TEST_AUTHORITY` (`RwLock`): slot-admitting tests hold a shared guard, the exhaustion test the exclusive one, and it first drains retirements parked by earlier tests' drops (`worker_job_retirements_are_parked`).
2. `try_submit` is non-blocking and answers `Contended` when the pool's own worker holds the lane queue for a moment — documented behaviour. Three tests `expect`ed the first submit (10/60). Fix: `admitted`/`mounted_admitted` follow the owner protocol (`take_rejected().resume()`, or the mounted pump's `Rejected`) and retry only that answer, consuming the wake their own resume raised.
3. `wait_for` bounded a wait on a pool worker (which unwinds a hostile panic and captures a backtrace) by 4 096 `yield_now`s. Now a 30 s wall liveness bound (`WORKER_LIVENESS_BOUND`).
4. The quiet-wake test read the phase before the wake flag; the session publishes phase then flag (standard order), so the test now waits on the durable wake and then asserts the phase is terminal.
5. `microsecond_platform_clock_and_real_half_ms_worker_progress` asserted a real 500 µs grant admits a step on the first try; at load 28 the thread was descheduled past it (1/1000). Now up to 64 fresh attempts; a clock or budget defect fails every one.
Six `[DEBUG]` prints in `⏱️budget` tests are removed.

### 4.4 WFC engine (1 red → 301/301, `wfc-engine1.txt`)

`every_large_domain_unit_including_checkpoint_stays_below_watchdog` asserted the single worst wall sample < 8 ms. The framework's own watchdog law (`semio_framework_trace::SUSTAINED_OVERRUN_QUARANTINE_STEPS` = 4, documented as the deliberate replacement for a per-sample law because one descheduled sample is not an overrun) quarantines only 4 consecutive units over `INTERACTIVE_STEP_CEILING_US`. The test now asserts that law with the framework's constants; the p99 < 2 ms bound is unchanged. Wall-time-only, independent of G6 (G6 reproduced it with its change backed out).

### 4.5 Whole-suite regressions check (final tree)

- plugin-host lib: **238/244** (`host-lib-final.txt`). The baseline was 232/243 with 10 red. The 4 shard reds are fixed; the 6 left are the out-of-scope pre-existing ui-patch ×3 (`registered scale component path`), schema-parity ×2 and owned-codec ×1, the same names and assertions as in the baseline.
- os-mcp lib: 460/462 in the first full run (`mcp-lib-final.txt`). `routing_artifact_channel_routes_two_capabilities…` was mine: a unit-test binary cold-compiled `cad` and started *itself* as the compile worker, and libtest rejected `--engine`. Fix: under `cfg(test)` the worker is the test binary running exactly `workspace::quick::isolated_compile_worker_entry`, with the engine and output passed in its environment (it is a no-op otherwise). `transport::quick::terminal_public_fifo…` failed on an `EAGAIN` socket read under parallel load; it is not on any path I touched. Rerun: both **3/3** with the worker entry (`mcp-lib-retest1.txt`).

## 5. Gates And Factor (three runs)

### 5.1 Gates (final host tree; staged wfc guest = W1's G6-era build, see §8)

| gate | measured | capture |
|---|---|---|
| client-e2e | **38/38** (nx `build` rebuilt the dist binary from this tree; genesis `inference_run` SUCCEEDED) | `g7-client-e2e.txt` |
| stdio transcript probe | **17/17** (in-flight cancel answered 10 ms after `notifications/cancelled`) | `transcript-pre-console.txt`, `g7-probe-pre*.json` |
| live-agent-loop (note react serve :6340, local-only; f1 `artifact_create` goes through the shell route as a job) | **20/20** | `g7-live-agent-loop.txt` |
| crossing-grant law (wfc-bitmap lib, which contains it) | 152/152 (+1 ignored) | `suite-semio-s-artifact-wfc-bitmap.txt` |
| binding-cancellation law + artifact_create cancellation law | **1/1** | `binding-law1.txt` |
| compile-cancellation law | **1/1** | `compile-law1.txt` |
| epoch laws + shard/relay/wasmtime-runtime | **115/115** | `host-epoch1.txt` |

### 5.2 Factor, three back-to-back runs (load 12–14; debug `semio-os-mcp`, warm `.cwasm`; native = `the_artifact_bound_genesis_document_resolves_and_solves`, built from this tree)

| run | native wall / CPU | over-MCP `inference_run` wall | over-MCP process CPU (solving thread) | CPU factor | wall factor |
|---|---|---|---|---|---|
| 1 | 15.10 s / 13.22 s | 20.77 s | 16.97 s (16.93) | **1.28×** | 1.38× |
| 2 | 17.35 s / 13.22 s | 17.17 s | 16.88 s (16.84) | **1.28×** | 0.99× |
| 3 | 13.34 s / 13.14 s | 18.48 s | 17.02 s (16.97) | **1.30×** | 1.39× |

Captures: `g7-native-pre{1,2,3}.txt`, `g7-solve-pre{1,2,3}.txt`, `g7-load-pre*.txt`. **Whole-process CPU now equals the solving thread's** (G6: 1.50–1.66× process vs 1.29–1.33× thread), because the epoch keeper is gone. These runs use the G6-era guest, so the guest-side clock reductions of §3 are not in them yet (§8).


## 6. Pids

Note serve :6340 (pids 78584/78587/78589/79056, mine) was stopped after the live-agent-loop run. Every probe gateway exits with its probe. None of mine is left running except the detached `g7-suites.sh` cargo chain while it finishes (§4).


## 7. Files Changed

| path | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | `EpochDeadlines`/`EpochDeadlineCell`/`EpochDemand` replace `EpochTicker`; every guest entry is armed per call (`checkpoint`/`restore`/`codec` newly); `🧊️IsolatedCompile` (`IsolatedCompileTarget`, `SharedEngineConfig::{to,from}_isolated_arg`, `compile_component_isolated`, `GuestRuntimes::{load_compiled, isolated_compile_target}`); atomic `.cwasm` writes (`compiled_cache_scratch_path`); stale ticker docs |
| `…/🖥️host/⏳️runtime/🦀️.rs` | async actor runtime: per-command timeslice demands |
| `…/🖥️host/🧵️shard/🦀️.rs`, `…/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` | `FrameAdmission`, preflight before registration, dead checks removed; stale cancel test drains, missing replay guard |
| `…/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs`, `…/🧵️shard/👶️child/🦀️.rs` | docs only (no epoch ticker any more) |
| `…/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs`, `…/🔬️wasmtime-runtime/🦀️.rs` | three epoch laws; watchdog law in ms |
| `🧰️framework/🔨️modules/⏳️async/🤝️cooperative/{🦀️.rs,🧪️tests/🔬️standalone/🦀️.rs}`, `…/⏳️async/🧪️tests/🔬️wasm-pool-cooperative/🦀️.rs` | source predicate ignores comments / follows the call; stale `[DEBUG]` print removed; DRR test on `Interactive` |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs` | step clock: entry/exit readings, `ClockStride` in `StepContext` + authority, `latest_us`, `last_step_end_us`; `WORKER_SESSION_SLOTS_TEST_AUTHORITY` |
| `…/🧵️job/🧪️tests/🔬️{retained-ownership,clock-stride}/🦀️.rs`, `…/⏱️budget/🧪️tests/⏱️budget/🦀️.rs`, `…/🧫️fixtures/🪜️clock-stride-law.json` | race-free job tests; clock-stride law restated + step read law; `[DEBUG]` prints removed |
| `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` | `Watchdog::{start_at, finish_at}`, `record_trace_event_at` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs` | Pump's deadline check uses the step's exit reading |
| `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs`, `…/💼️job/🧪️tests/🔬️unit/🦀️.rs` | WFC reads `context.now_us()`; watchdog test asserts the quarantine law |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs` | `ActivationPhase` (+ `compiling-component`, `position`, `at`), `ACTIVATION_CANCELLED_FAULT_CODE` |
| `…/🌉️mcp/🏠️workspace/🦀️.rs` | `CompileFlight`, `set_component_compile_worker`, `prepare_plugin_component`, `live_component_compile_workers`, `component_compile_workers_started`; load-before-read order; scoped session channel; `create_plugin_artifact{,_cancellably}`; `run_activation_job`; orphan doc block removed |
| `…/🌉️mcp/🗿️artifact/🦀️.rs` | `artifact_create` as a job |
| `…/🌉️mcp/💡️inference/🦀️.rs` | activation progress mapped onto the binding span |
| `…/🌉️mcp/🏗️bootstrap/🦀️.rs` | `compile-component` worker subcommand |
| `…/🌉️mcp/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts}` | `artifact_create` output (`jobId`, `status`, …); mirror regenerated with `schema-mirror` |
| `…/🌉️mcp/🏠️workspace/🧬️schema/🔣️.json`, `…/🗿️artifact/🧬️schema/🔣️.json` (new), `…/💡️inference/🧬️schema/🔣️.json` | `SessionActivationPhase*V1`, `ComponentCompileCancellationLawV1`; `ArtifactCreate*V1`; inference refs the shared phases |
| `…/🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`, `…/🗿️artifact/🧫️fixtures/⏱️create-cancellation-law.json` (new), `…/💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json` | laws |
| `…/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs` (new), `…/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs`, `…/📦️packages/🦀️rust/Cargo.toml` | law replays, `[[test]] compile_cancellation_law` |
| ticket | `wp-g7/{g7-cargo.sh,g7-solve-probe.ts,g7-transcript-probe.ts,g7-native-cpu.sh,g7-idle-cpu.sh,g7-serve-note.sh,g7-suites.sh,g7-fixture-check.ts}`, `wp-w1/requests/g7.txt` |

A temporary `[DEBUG] g7` attribution test in `✏️s/…/🖼️bitmap/…/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` was added and removed (net zero). The AJV oracle (`g7-fixture-check.ts`) validates all three law fixtures against their schemas and rejects a corrupted one: 6/6 (`g7-fixture-check.txt`).


## 8. Honest Gaps

1. **Live guest clock count and the post-rebuild factor are not measured yet.** The §3 changes are guest-side, so they need W1's wfc rebuild. W1 folded it into its final all-component pass, which had not started by 10:47 (it was still bootstrapping catalog A). Baseline with the old guest and the counting host (`target/semio-os-mcp-count`, a temporary `[DEBUG] g7` counting WASI clock that is removed from the source; the file matched its pre-edit copy byte for byte): **~2.7 M `monotonic-clock.now` calls** per solve (`g7-clocks-oldguest.txt`). To finish: once `wp-w1/requests/g7.txt` shows the new sha, run `SEMIO_OS_MCP_BIN=…/wp-g7/target/semio-os-mcp-count G7_RUN=clocks-newguest bun wp-g7/g7-solve-probe.ts` (expect ≈ 0.5 M), then three `g7-native-cpu.sh` + `g7-solve-probe.ts` pairs.
2. The factor table in §5.2 therefore uses the G6-era guest. The native reference already includes the new job code, which makes the over-MCP factor conservative.
3. `EpochDeadlines` registers pool timers through `WorkerPool::callback_at`. R7 has since fixed a lost-timer bug in `⏳️async/🔔️worker-parking` (a firing-worker callback registering an earlier deadline did not re-arm a sleeping keeper), which is exactly the path a deadline armed from inside a timer callback takes. My epoch laws passed before that fix. A withdrawn deadline's timer cannot be cancelled on the pool, so it fires once at its instant and does nothing (at most one wake per watchdog interval).
4. Out of scope and still red, as in the baseline: plugin-host ui-patch ×3, schema-parity ×2, owned-codec ×1. os-mcp `transport::quick::terminal_public_fifo…` failed once with `EAGAIN` under parallel load and passed in isolation.
5. Trace timestamps inside a step (`set_stage`) are now as of the step's latest reading, at most one stride (~64 µs) old.
6. What remains per step is 2 reads, set by the WFC plugin's own preview cadence (a preview, and so a step, every 16 units).

