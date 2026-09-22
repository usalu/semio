# GJ1 — the guest inference job trap, root-fixed; wfc solves end to end

Slice owner: GJ1, session 8 (2026-09-22, 22:1x → ). Predecessor: `📓️wi1-wfc-inference-end-to-end.md`
(`client-e2e` 37/38; §7.2 the last red). Context read in full: `📓️worker-preamble.md`,
`📓️jb1-builtin-jobs-in-production.md` (the bounded `InteractiveInferenceJob` / `TwoPhaseBoundedJob`),
`📓️fp11-plugin-lib-zero-red-and-lanes.md` §0 + §3.1 (the async-task lane is production since 11:34:
`TASK_EXECUTOR` driven per turn, `PluginApp::resume_task_command`),
`📓️tc3d-guest-genesis-and-note-creation.md` §1 (retained guest stderr in the owned interpreter),
memory `project-to-dsl-value-self-call-trap`, `project-guest-contiguous-request-ceiling`.

Target: `client-e2e` **38/38** and a wfc bitmap SOLVED through `inference_run` with progress rows and
a result the user can see.

## 0. Headline

1. **The trap was never a memory fault. It is the host's own epoch deadline, set from the guest's
   cooperative grant.** `WasmtimeRuntime::step_job` armed the wasmtime store with
   `set_epoch_deadline(budget.deadline_ms)`, and the only budget that reaches it is the host constant
   `RELAY_JOB_BUDGET`, whose `deadline_ms` is `USER_VISIBLE_LANE_WALL_US / 1_000` = **2**. With
   `EPOCH_TICK_INTERVAL_MS = 1` that is a **2-millisecond hard kill per `step-job` crossing** —
   the host killing the guest at the exact instant the guest was supposed to yield cooperatively,
   with zero margin, inside a JIT, on a machine running a whole fleet. An epoch cut lands on
   whatever instruction the guest happens to be on, which is precisely why the same call answered
   `alloc::Global::deallocate` under `drop_glue::<wit_bindgen…FutureState>` on one run (WI1 §7.2)
   and `[async-lift]semio:framework/jobs@1.0.0#step-job` on the next
   (`🗑️generated/gj1-trap-probe.txt`), and why WI1 measured the trap address moving.
2. **It was invisible because the job exports never classified their traps.** `execute_turn` has
   classified off `Debug` (the source chain) since the `rust_oom` hunt — `start-job`/`step-job`/
   `cancel-job` used `error.to_string()`, which is only a wasmtime error's FIRST line. One
   classifier for all four exports turned the same call's answer from an unattributed backtrace
   into **`epoch deadline exceeded`** (`gj1-trap-probe-2.txt`, 17 s) with no other change.
3. **The trap was only the FIRST of two faults on the same lane.** With the watchdog in place the
   guest no longer dies — and no longer finishes either: `s.wfc.bitmap.solve` ran **907 s** and then
   **423 s with a 6 000 000-unit work grant** without answering. `InteractiveInferenceJob::pump`
   SUBMITS a step to the guest's process worker pool and, on wasm, that pool has no threads and runs
   a submitted step only inside `WorkerPool::pump` — which the reactor turn calls, and **no reactor
   turn runs during a `step-job` crossing**. The lane pumps its own pool now (§4.4). This is the
   third instance of the law in `project-wasm-pool-pump-starves-interactive-jobs`.
4. **`inference_run` was refused outright for the whole of a shell-resolved session.**
   `ShellRoutedArtifactChannel` sent `AppCommand::Infer` down the shell branch, where
   `ShellArtifactChannel` refuses it by design ("the gateway owns its own inference guest").
   Measured live on the first probe of this slice (`gj1-trap-probe.txt`: `PLUGIN_UNAVAILABLE`).
   That is exactly the session an MCP client driving a live shell has — outcome 4's own session.

## 1. Inherited state

| source | measurement |
| --- | --- |
| WI1 §7.2 | `client-e2e` **37/38**; `inference_run` red with `SIDE_EFFECT_REJECTED … guest trapped`, innermost frame `<alloc::Global as Allocator>::deallocate` under `drop_glue::<wit_bindgen::rt::async_support::FutureState>` → `Arc<FutureWaker>::drop_slow`; reproducible with a HAND-WRITTEN `payload.snapshot` that never touches the new document carrier |
| WI1 §7.2 | with `artifactId` bound the innermost frame is `<semio_framework_deflate::component::Inflater>::advance`; the bound bytes are proven byte-exact (405 B pack + 211 B spr, law `a_bound_document_round_trips_through_the_guest_json_decoder`) |
| WI1 §7.2 | the trap address moves between runs (`0x338e0c4`, `0x338e0e9`, `0x338e124`) |
| memory `project-guest-contiguous-request-ceiling` | dlmalloc never returns memory; single contiguous guest requests must stay < 64 KiB; **wit-bindgen's ~189 KB turn-task box per poll** was already named as over the ceiling on 2026-09-10 |
| JB1 §2.2 | `InteractiveInferenceJob` — `Dispatch`/`Pump`/`OutcomeClose`/`SessionClose`, one state action per `step_job` |
| FP11 §3.1 | `TASK_EXECUTOR` (cold) is driven per turn since 11:34; `REACTOR_EXECUTOR` is the bounded one |

## 2. Reproduction and the decisive measurement

Every run below is against this slice's own staged `semio-os-mcp`
(`🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`), `.mcp.json`'s real `semio` entry, folder mode,
with WI1's serve **6200** up (`/` → 200). Probe: `🐍️gj1-trap-probe.ts` — WI1's own hand-written
`payload.snapshot` (the crate's 4×4 stripes fixture → 6×4 periodic output), which never touches the
`document` carrier, plus a `context_resolve` first and the FULL refusal printed (WI1's probe sliced
it at 2 500 chars, which is why its capture ends mid-backtrace).

| # | binary | channel | wall | answer |
| --- | --- | --- | ---: | --- |
| 1 | WI1's 21:46 stage | **shell** (resolved lazily, no `context_resolve`) | 4 s | `PLUGIN_UNAVAILABLE`: `inference never travels the shell route …` — §4.3 |
| 2 | WI1's 21:46 stage | headless (pinned by `context_resolve`) | 31 s | `guest trapped: error while executing at wasm backtrace: 0: 0x3bf0a - semio_s_plugin_wfc.wasm![async-lift]semio:framework/jobs@1.0.0#step-job` — ONE frame, no trap code |
| 3 | + the classifier (§4.1) | headless | 17 s | **`epoch deadline exceeded`** (`gj1-trap-probe-2.txt`) |
| 4 | + the watchdog (§4.2) | headless | > 300 s | no trap at all; the guest runs on (§7) |

Run 2 next to WI1's §7.2 is the whole diagnosis: **two different innermost frames for the same
input on two runs of the same binary.** A deterministic decode fault cannot do that; an
asynchronous epoch cut can do nothing else.

Arithmetic on the source constants, which is the same fact without a run:
`RELAY_JOB_BUDGET = JobBudget { fuel: USER_VISIBLE_LANE_FUEL, deadline_ms: (USER_VISIBLE_LANE_WALL_US / 1_000) as u32 }`
(`🖥️host/🦀️.rs`), `USER_VISIBLE_LANE_WALL_US = 2_000` (`🧵️job/🦀️.rs:389`), `EPOCH_TICK_INTERVAL_MS = 1`
(`🖥️host/🦀️.rs:319`) → **2 epoch ticks ≈ 2 ms of wall clock for one `step-job` crossing**, against
`💼️jobs`' own `WORK_UNITS_EXECUTE = 1 024` priced as "one whole unchunked native dispatch".

## 3. Root cause, stated once

`JobBudget` carries two different quantities and the host spent them as one.

| quantity | whose | what it means | where it is read |
| --- | --- | --- | --- |
| `JobBudget.fuel` / `.deadline_ms` | the GUEST's | the cooperative grant a bounded job state machine prices its actions against and yields on (`💼️jobs`' work units; `BatchDriveConfig.step_budget_us`) | inside the guest, on `wit_budget` |
| the wasmtime store's fuel + epoch deadline | the HOST's | the hard kill after which the component instance is unrecoverable ("cannot enter component instance") | `store.set_fuel` / `store.set_epoch_deadline` |

`step_job` set the second from the first. The gateway had already written the argument for keeping
them apart, one file away (`headless_inference_budget`: *"under the JIT a fuel yield is exactly as
unresumable as an epoch cut … this number is the point at which the guest is declared wedged and its
instance is thrown away, never a quantum the host resumes from"*) — and then passed `u64::MAX` /
30 000 ms for its own turns. The job lane never got that treatment.

Two smaller faults fall out of the same reading and are fixed with it:

- **`start_job` armed nothing at all.** It inherited whatever fuel and epoch deadline the preceding
  `execute_turn`/`step-job` left in the store, so a start after a fully spent turn began with an
  already-passed deadline.
- **`cancel_job` armed nothing either**, and an epoch deadline is ABSOLUTE: a cancellation arriving
  well after the step it cancels ran against that step's spent deadline and trapped the instance it
  was trying to wind down.

**Not the cause** (each ruled out by a measurement, not by reading): the `wit_bindgen`
`FutureState` drop glue (it is where run 2's cut happened to land, not why), the document carrier
(run 2 carries a hand-written `snapshot`), the bound bytes (WI1's byte-exact round-trip law), and
the contiguous-request ceiling (`project-guest-contiguous-request-ceiling` — the turn future is
53 872 B, under the 65 536 B ceiling its own law pins, and no allocation refusal appears in any
capture).

## 4. The fix

### 4.1 One trap classifier for all four async-lifted exports

`classify_guest_trap` (`🔌️plugin/🖥️host/🦀️.rs`) — the classification `execute_turn` had inline since
the `rust_oom` hunt, extracted and applied to `start-job`, `step-job` and `cancel-job`, whose six
`map_err(|error| TurnFault::Trapped(error.to_string()))` sites threw the source chain away. `Display`
on a wasmtime error is only its first line; the trap code lives in the chain, which `Debug` renders.
A fuel cut is `TurnFault::FuelExhausted`, an epoch cut `DeadlineExceeded`, anything else keeps the
whole `Debug` text instead of one line of it.

### 4.2 The host's watchdog, named apart from the guest's grant

`GUEST_JOB_WATCHDOG_MS = 30_000` + `arm_guest_job_watchdog(store)` (same file), called by
`start_job`, `step_job` and `cancel_job`. The guest still receives its own cooperative grant
unchanged on `wit_budget`, so `💼️jobs`' work-unit prices and `step_budget_us` are untouched — the
host simply stops killing it at the instant it is supposed to yield. Fuel is disarmed (`u64::MAX`)
for the reason the gateway's own cold-open budget already states: a fuel yield is as unresumable as
an epoch cut, so arming both only adds a second, harder-to-read way to say "wedged". 30 s is the
same "declared wedged" ceiling `headless_inference_budget` uses.

### 4.3 The guest's worker pool is pumped by the lane that waits on it

`InteractiveInferenceJob::pump` (`🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs`) now calls
`crate::reactor::turn::pump_process_worker_pool()` (made `pub(crate)`) when `pump_one` answers
`Submitted`, then re-polls once in the same state action. `pump_one` only SUBMITS to
`semio_framework_async::process_worker_pool`; on wasm that pool has no threads and executes a
submitted step only inside `WorkerPool::pump`. Every other wasm lane that waits on a pool step got
its own pump on 2026-09-09 (`drive_typed_operation_worker`, `pump_process_worker_pool` per reactor
turn); the `semio.infer` lane did not, and it is the only one that runs with NO reactor turn
in between its steps — the host is inside `step-job` the whole time. Natively the pool has real
workers and the call is a compiled-away no-op, so both hosts run the identical source.

Bounds are the turn's own (`PROCESS_POOL_PUMPS_PER_TURN` = 64, `PROCESS_POOL_WALL_MS` = 2), so one
crossing still costs at most one bounded pump window — which is exactly what the 30 s watchdog now
leaves room for and the 2 ms one did not.

### 4.4 An inference is not a shell command

`ShellRoutedArtifactChannel::exchange` (`🌉️mcp/🏠️workspace/🦀️.rs`) now routes `AppCommand::Infer` to
the headless lane whatever the session resolved. `ShellArtifactChannel` refuses `Infer` by design,
and `ensure_inference_route` drives its OWN guest instance on its own actor ordinal holding no
document the shell owns — so this splits no document owner; it does for the caller what the
refusal's own message asks them to do. Without it `inference_run` answers `plugin.unavailable` for
every session that resolved `shell`, which is every session an MCP client driving a live shell has.

## 5. Laws

`🔌️plugin/🖥️host/🧪️tests/🔬️wasmtime-runtime/🦀️.rs`, run with a private
`CARGO_TARGET_DIR=⚡️cache/cargo/target-gj1`, `CARGO_INCREMENTAL=0`
(`🗑️generated/gj1-test-host-laws.txt`, 23:20): **3 passed / 0 failed**, 237 filtered out.

| law | what it pins |
| --- | --- |
| `a_fuel_cut_and_an_epoch_cut_are_named_rather_than_reported_as_a_backtrace` | a wasmtime error whose CAUSE says "all fuel consumed" / "epoch deadline reached" classifies as `FuelExhausted` / `DeadlineExceeded` — the budget cut is never reported as an unattributed backtrace again |
| `a_real_trap_keeps_its_whole_source_chain_not_its_first_line` | a genuine `unreachable` keeps BOTH the trap code (which only `Debug` renders) and the backtrace |
| `the_host_watchdog_is_not_the_guests_cooperative_grant` | `RELAY_JOB_BUDGET.deadline_ms` stays `USER_VISIBLE_LANE_WALL_US / 1_000` (the guest's slice is unchanged) AND `GUEST_JOB_WATCHDOG_MS` is at least 1 000× it — the currency separation as a ratchet, so re-deriving one from the other fails a law instead of a live solve |

Compile gates: `cargo check -p semio-framework-plugin --lib` rc 0 / 59 warnings
(`gj1-check-plugin.txt`), `cargo check -p semio-framework-plugin-host --lib --profile test` rc 0 /
52 warnings (`gj1-check-plugin-host-test.txt`), `semio-os-mcp` staged twice rc 0
(`gj1-mcp-build-{1,2}.txt`, 53 s and 49 s).

## 6. Rebuild + describe (ordered wasm mutex)

(filling)

## 7. `client-e2e`

(filling)

## 8. The solved bitmap

(filling)

## 9. Honest gaps

(filling)

## 10. Files changed

(filling)
