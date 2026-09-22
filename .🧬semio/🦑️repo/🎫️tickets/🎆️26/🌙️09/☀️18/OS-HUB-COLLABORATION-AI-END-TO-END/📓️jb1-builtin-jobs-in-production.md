# JB1 — builtin job kinds run in PRODUCTION builds

Slice owner: JB1, session 7 (2026-09-21, 15:00 → ). Spec: `📓️pz1-catalog-zero-diagnostics.md` §3.

Inherited measurement (PZ1 §3, re-read at the source and confirmed line for line): `spawn_job`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:368-372` before this slice)
dropped every builtin `JobFn` on the floor under `#[cfg(not(test))]` and parked the slot as
`JobBody::ExplicitStateMachineRequired`, so `semio.infer`, `semio.io-run`, `semio.io-sniff`,
`semio.mutation-plan` and `semio.migrate` were refused with `job.explicit-state-machine-required`
in EVERY production build, for EVERY plugin. `🀄️wfc`'s red inference row in `client-e2e` was one
visible symptom of a framework-wide dead route: the unit suite exercised an executor that only
existed under `#[cfg(test)]`, so the admitted set in a test build and in a shipped component were
different sets and nothing said so.

## 1. Design chosen — PZ1 §3 option (1), taken to its root

PZ1 offered two designs: (1) a framework-level bounded `semio.infer`, (2) a plugin-local bounded
`semio.infer` in `🀄️wfc`. **(1), and not only for `semio.infer`** — (2) leaves the other four
builtins dead in every shipped component and makes every plugin re-author the same machine.

The brief's own bar ("no `cfg(test)` difference between test and production", "no cfg-gated product
paths") is not met by moving the five builtins into `BOUNDED_KIND_REGISTRY` and leaving the
`JobFn` door beside it: that door would STILL be `#[cfg(test)]`-only, and `PluginBuilder::job(kind,
run)` would still be a public API that silently does nothing in a shipped build. So the whole
opaque-future path is **deleted, not gated**:

- `JobFn`, `JobCtx`, `JobTick`, `JobState`, `run_two_phase`, `SLICE_MAX_ITERATIONS`, the per-jobs
  `TEST_JOBS_FUTURE_EXECUTOR` (`ColdFutureExecutor`), `JobBody::Running`,
  `JobBody::ExplicitStateMachineRequired` and `register_job_kind` are gone.
- One registry remains: `KIND_REGISTRY: HashMap<&'static str, BoundedJobFactory>`, seeded by
  `builtin_registry()`, extended by `register_bounded_job_kind`.
- `BoundedJobFactory` gained the third parameter `JobFn` always had and the bounded path used to
  drop: `fn(u64, &[u8], Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>>`. Without it a
  checkpoint restore could not resume a bounded job at all, which would have been a regression
  against the `PHASE_DECODED` resume the three cold kinds already had.
- **`💼️jobs/🦀️.rs`, its three submodules and their tests now contain no `#[cfg(test)]` product
  branch of any kind.** The only `cfg(test)` left in the module is the `mod tests` mount.

Grep proof of the shape: `grep -c 'cfg(test)' ⚛️reactor/💼️jobs/🦀️.rs` → the module's own test
mount only; `JobBody` has three variants (`Bounded`, `AdmissionFailed`, `UnknownKind`) and none is
gated.

### Budget semantics, declared rather than accepted-and-ignored

The pre-rewrite file's own doc admitted the old hard-coded match "accepted `job-budget` without
reading it". Now `JobBudget::fuel` is the **work-unit grant for one step**, and every state
declares its price:

| constant | value | what it prices |
| --- | ---: | --- |
| `WORK_UNITS_VALIDATE` | 1 | one decode/validate action (a bounded parse of `input`) |
| `WORK_UNITS_RETIRE` | 1 | one bounded `close_step` page grant |
| `WORK_UNITS_PUMP` | 64 | one `MountedWorkerJobSession` step submitted to the worker pool |
| `WORK_UNITS_EXECUTE` | 1 024 | one whole unchunked native dispatch |

A step granted less than the current state's price is refused with `job.<kind>.budget-exhausted`
BEFORE the call runs, rather than overrunning the actor grant it was called under. There is no
"zero means unmetered" exemption: an empty grant cannot pay for a validate action either. The host
always sends a real grant (`RELAY_JOB_BUDGET` = `USER_VISIBLE_LANE_FUEL` = 6 000 000;
`job_budget_from_grant` forwards the actor's own `Budget.fuel`), so the gate only bites a caller
whose grant is genuinely spent. `deadline_ms` is honoured by construction: one state action per
step.

### The async→sync boundary, named instead of hidden

`BoundedJob::step` is synchronous; the five native dispatches
(`io_run`, `io_identify`, `wire_artifact_infer`, `wire_artifact_mutation_plan`, `migrate_document`)
are `async` by SIGNATURE only — `io-async-signatures` made the signatures uniform without adding a
suspension point to any of those bodies. `settle_in_step(prefix, future)` polls such a future
exactly once. A `Pending` is NOT a hang and NOT a panic (unlike the framework's own
`resolve_ready`, which panics): it is a typed `job.<kind>.suspended` refusal naming the guarantee a
future edit broke. In the poll world a host-await inside a job could never resolve anyway — the
module doc's own "Host-await restriction" — so a loud typed refusal is strictly better than the
`Running` spin the old shape would have produced.

## 2. State machines per builtin kind

### 2.1 `semio.io-run`, `semio.io-sniff`, `semio.migrate`, `semio.mutation-plan`, and `semio.infer`'s registry route — `TwoPhaseBoundedJob`

| state | price | action | outcome |
| --- | ---: | --- | --- |
| `Decode` | 1 | parse + validate `input`; for io/migrate also parse both dialect coordinates | `Running(Some(identity bytes))`, next state `Execute`; a bad envelope is `Failed(<kind>.decode)` |
| `Execute` | 1 024 | the one unchunked native dispatch, settled in-step | `Done(bytes)` / `Failed(<the dispatch's own fault>)` |
| `Complete` | 0 | none | `Failed(<kind>.terminal)` |

- `checkpoint()` answers `PHASE_DECODED` exactly while the machine sits at `Execute`, so
  `restore_job` starts the replay at `Execute` and finishes on its FIRST `step_job`.
- `cancel()` marks the machine; the next `step` is `Failed(<kind>.cancelled)`.
- `terminal_drop_is_shallow()` is always true — the machine owns only `Vec<u8>` and two fn
  pointers.
- Fault codes are preserved exactly: `job.io-run.decode`, `job.io-sniff.decode`,
  `job.migrate.decode`, `job.migrate`, `job.mutation-plan.decode`, `job.infer.decode` all keep the
  codes their pre-bounded bodies raised, which is what the inherited laws assert.

### 2.2 `semio.infer`'s ActionBus route — `InteractiveInferenceJob`

This is the route `🀄️wfc` takes (`ToolFactoryKey::new("semio.infer", request.inference_schema)`
present in `ActionBus::production()`), and it is the literal state-by-state translation of the
former `run_interactive_inference` future: **every `ctx.tick().await` in that body is one state
boundary here.**

| state | price | action | next |
| --- | ---: | --- | --- |
| (admission) | — | `validate_wire_request_resources`, claim the cancellation slot (a drop guard held for the machine's whole life), mint the `Operation`, build the `InferenceBridge` | `Dispatch` |
| `Dispatch` | 1 024 | publish the identity preview; resolve `payload_schema_id`; `dispatch_wire`; `MountedWorkerJobSession::try_new` | `Pump`, or `RejectedClose` when admission is refused |
| `Pump` | 64 | re-read the host cancellation flag (cancelling the token on a hit); emit a scheduled bridge item; ONE `pump_one` on the `UserVisible` lane; on `Outcome`/`Terminal` take the checked-out outcome and turn `Yield`/`PreviewReady`/`CheckpointReady`/`Complete`/`Cancelled`/`Fault` into progress, checkpoint or result | stays in `Pump` while the worker is not ready, else `OutcomeClose` |
| `OutcomeClose` | 1 | one `outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES)` page | `SessionClose` when the outcome was terminal, else `resume()` → `Pump` |
| `SessionClose` | 1 | one `session.close_step(...)` page | `Complete` + `Done`/`Failed` with the recorded result |
| `RejectedClose` | 1 | one `rejected.close_step(...)` page | `Complete` + `Failed(job.infer.admission)` |
| `Complete` | 0 | none | `Failed(job.infer.terminal)` |

- **Progress on every `Running`.** Each retirement state emits a fresh `bridge.scheduled()` item
  (the bridge's monotonic `sequence` makes the bytes differ every time), so the stall guard never
  mistakes a bounded close walk for a wedged job — the one behavioural trap in turning a close loop
  into steps.
- **Cancellation is real.** `cancel()` sets the flag, calls `CancelToken::cancel_now()` AND drives
  `retire()`: outcome → session (`begin_close` first) → rejection, each through its own bounded
  close protocol under a `RETIRE_STEP_CEILING` of 4 096 page grants. This matters because
  `cancel_job` asserts `terminal_drop_is_shallow()` immediately after `cancel()`; a machine holding
  a live `MountedWorkerJobSession` would trip that assert.
- `terminal_drop_is_shallow()` = `session.is_none() && rejected.is_none() && outcome.is_none()`.
  Every early failure path calls `retire()` first (via `fail`), so a fault never leaks a worker
  session; if a retirement cannot finish inside the ceiling the wrapper stays deep and `step_job`
  reports `job.bounded-false-terminal` instead of dropping a live session silently.
- `checkpoint()` answers the last `CheckpointReady` state bytes the worker produced.
- The bounded budgets, bridge caps (`PREVIEW_MAX_BYTES`, `DIAGNOSTIC_MAX_ITEMS/BYTES`,
  `LOSSLESS_MAX_BYTES`) and the `BatchDriveConfig` (`fuel_per_step` clamped into
  `USER_VISIBLE_LANE_FUEL`, `step_budget_us = USER_VISIBLE_LANE_WALL_US`) are carried over
  unchanged.

## 3. Laws

All in `⚛️reactor/💼️jobs/🧪️tests/🔬️unit/🦀️.rs` unless noted. 39 tests run under the `jobs`
filter, 39 pass.

| law | what it pins |
| --- | --- |
| `the_admitted_builtin_set_is_exactly_the_declared_builtin_set` | `builtin_registry()` keys == `BUILTIN_JOB_KINDS`, and each resolves through `job_kind_is_admitted`. `builtin_registry()` is the only producer and carries **no `cfg` branch**, so the set a test build admits is by construction the set a `wasm32-wasip2` component admits. |
| `no_builtin_kind_is_refused_before_its_own_state_machine_runs` | for all five kinds, the first `step_job` is never `job.unknown-kind` and never `job.explicit-state-machine-required` — the exact refusal PZ1 measured. |
| `a_builtin_walks_decode_then_execute_to_a_terminal_outcome` | the full state walk: `Running(Some(hop identity))` → `Done(confidence rank)` → the id is released (`job.unknown`). |
| `a_builtin_checkpoint_restore_resumes_at_the_execute_state` | `checkpoint_jobs()` reports `PHASE_DECODED`; a restore finishes `Done` on its FIRST step with byte-identical output. |
| `a_state_action_granted_less_than_its_declared_price_is_refused` | budget exhaustion is a typed `job.io-sniff.budget-exhausted`, both for an execute state given a validate-sized grant and for a zero grant. |
| `cancelling_a_builtin_mid_walk_refuses_its_next_state_action` | cancellation mid-step: the owner's next state action is `Failed(job.io-sniff.cancelled)`, the wrapper is shallow, and through `cancel_job` the host's next `step_job` is `job.unknown`. |
| `a_registered_bounded_kind_advances_one_state_action_and_retains_its_admission_fault` | a plugin-declared kind walks one action per step and a refused admission surfaces its own bytes on the first step. |
| `a_registered_bounded_kind_is_restored_through_its_own_factory` | the new `restored` factory parameter really reaches a plugin's factory and resumes past the checkpointed state. |
| `the_stall_guard_fires_after_repeated_no_progress_static_budget_steps` | `job.stalled` after `STALL_LIMIT` no-progress static-budget steps, now measured on a bounded owner. |
| parity: `step_job_on_an_unknown_id…`, `…unknown_kind…`, `io_run/io_sniff …keeps_its_decode_fault_code`, `cancel_job_removes_a_pending_record…` | pre-rewrite behaviour unchanged. |
| `💡️infer`, `🧬️mutation-plan`, `🔀️migrate` `🔬️unit` suites (inherited, unchanged assertions) | each cold kind still reaches its REAL registry (a registered inference service, a registered contributed mutation, a registered `DialectMigration`) in two state actions, checkpoints `PHASE_DECODED`, and restores to a byte-identical result. |

One fixture bug found and fixed while landing: the two `💡️infer` tests shared
`cancellation_id: "jobtest-cancel-1"`, and the in-flight-inference registry is process-global, so
under the new (faster, executor-less) path they raced for the same slot
(`artifact-inference.in-flight inference "jobtest-cancel-1" is already active`). `request_bytes`
now takes the id.

## 4. Checks and captures

| check | result | capture |
| --- | --- | --- |
| `cargo check -p semio-framework-plugin --all-targets` (16:14) | **0 errors**, 378 warning lines, `Finished in 16.80s` | `🗑️generated/jb1-check-plugin-all-targets.txt` |
| `cargo test -p semio-framework-plugin --lib jobs` (16:07) | **39 passed, 0 failed**, 782 filtered out | `🗑️generated/jb1-test-plugin-lib-jobs.txt` |
| `cargo check -p semio-s-plugin-wfc -p semio-s-plugin-note -p semio-s-artifact-wfc-{bitmap,2d,3d} --target wasm32-wasip2 --features …/component-app-assembly` (16:10) | **0 errors**, 98 warnings, `Finished in 2m 01s` | `🗑️generated/jb1-check-wasm-wfc-note.txt` |
| `cargo check -p semio-s-artifact-fem-{2d,3d} --features …/component-app-assembly` (16:17) | **0 errors** — the two other `BoundedJobFactory` registrations take the new `restored` parameter | `🗑️generated/jb1-check-fem-factories.txt` |
| `cargo check -p semio-framework-plugin --lib` (16:33, after a coordinator red report) | **0 errors**, 45 warnings | — |
| `cargo check -p semio-framework-plugin --lib` (21:48, `CARGO_INCREMENTAL=0`, cold, 22m 00s) | **0 errors**, 48 warnings — the shared crate is green as this slice hands over | — |

Three separate red windows were observed and none of them survived:

1. 15:50–15:55, while this slice's own edits were in flight (before its first check) — the
   registration API and its five call sites landed in one pass and the 15:55 `--all-targets` run
   was already 0 errors.
2. 16:11, a concurrent peer's in-flight `maintenance_stage_step` edit in `🔌️plugin/🦀️.rs`
   (`E0407` at `:30531`, `E0599` at `:30516`) turned the crate red for both this slice's re-check
   and its `fem` check; `grep -c '💼️jobs'` over that capture is **0** — not one error named this
   slice's module. Kept as `🗑️generated/jb1-check-plugin-all-targets-peer-red.txt`. Green again at
   16:14 once the peer finished.
3. 16:30, the coordinator reported 6 errors naming a 2-arg `framework_reserved_job_factory` and an
   unresolved `reactor::jobs::JobFn`. Re-grepped at 16:33: every caller of the changed API
   (`🔌️plugin/🦀️.rs:17819`, `🏗️builder/🦀️.rs:710`, the two `🏗️fem` session modules, three unit-test
   registrations) is on the 3-arg `BoundedJobFactory`, no `JobFn` symbol exists anywhere in the
   tree, and `cargo check -p semio-framework-plugin --lib` is green. That report described window
   (1)/(2), not the tree as it stands.

### 4.1 The source-shape gate that pinned the broken design

`📜️script.ts`'s `toolJobOpaqueFutureProductionFailClosed` (`:4361`, reported at `:6749`) **required
the exact shape this slice removed**: a `#[cfg(test)] static TEST_JOBS_FUTURE_EXECUTOR:
ColdFutureExecutor`, the string `"job.explicit-state-machine-required"`, and a `spawn_job` body
whose `JobBody::ExplicitStateMachineRequired` arm precedes `let future = run(`. So the dead route
was not an oversight — it was load-bearing in a gate. Rewritten to assert the stronger property it
was reaching for: no `executor::ColdFutureExecutor`, no `pub type JobFn`, no `pub struct JobCtx`, no
`ExplicitStateMachineRequired` variant or fault string, a single
`KIND_REGISTRY: RefCell<HashMap<&'static str, BoundedJobFactory>>`, a declared
`BUILTIN_JOB_KINDS: [&str; 5]`, the 3-parameter `BoundedJobFactory`, and a `spawn_job` body with
**no `#[cfg(` at all**. `toolJobFem2dMountedSessionExact` (`:5123-5126`) carried the same two
clauses and now negates them; its synthetic fixture in
`🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts:195` was updated to match. The
negative self-test at that file's `:628` (an opaque `JOBS_EXECUTOR.spawn(future)` body) is still
rejected by the new oracle.

The module doc had to be worded around two of its own assertions (it names `ColdFutureExecutor` and
a `cfg(not(test))` gate when explaining what was deleted); the oracle now matches
`executor::ColdFutureExecutor` and the doc says "a `cfg(not(test))` gate" rather than the attribute
literal.

`bun ./📜️script.ts verify interactivity tool-jobs` **cannot be run to completion right now**: it
aborts much earlier on a pre-existing, unrelated failure in the wgpu shell
(`[verify interactivity tool-jobs shared-action-fixture] recordTutorial arms before the accepted
retained route`, `📜️script.ts:5864`), so `:6749` is never reached. Capture
`🗑️generated/jb1-verify-tool-jobs.txt`. Instead every clause of both rewritten oracles was
evaluated directly against the real `💼️jobs/🦀️.rs` — 13/13 true, including the `spawn_job` block
walk and the `UnknownKind`-before-`Bounded` ordering. The gate itself owes a run once the wgpu
shell fixture is fixed by whoever owns it.

## 5. `client-e2e` before / after

**Before** (16:24, the staged components are the 2026-09-20 22:29 `note` and 23:25 `wfc`):
`client-e2e: 15/17 steps green`, capture `🗑️generated/jb1-client-e2e-before.txt`. The two reds:

1. `os: capability catalog health` — 2 diagnostics, first `skipping plugin \`puzzle\`: … 🔣️.json did
   not decode as a PackageDescriptor: missing field \`artifact…\``. Not this slice's; PZ1/a peer owns
   the puzzle descriptor.
2. `os: artifact_create (a real plugin artifact kind)` — `kind=s.note.note`: `` `note` refused
   ReadArtifact (channel.not-wired): instantiate: wasmtime: no exported instance named
   `semio:framework/codec@1.0.0` ``.

**The 17 vs 36 step count is the measurement that matters here.** CE1 closed this gate at 34/36 and
PZ1 measured 30/32; it is 17 today because the os journey ABORTS at `artifact_create` — every step
after it (the discovered mutation through `action_prepare`/`action_invoke`, the snapshot, the
`history_undo`, and **`inference_run` with its job progress and cancel**) never runs. So the
`job.explicit-state-machine-required` row this slice exists to turn green is currently unreachable
from the gate: it is behind a `note` component that predates today's WIT by a day. That is exactly
CE1 §8 gap 5 (the freshness oracle catches age, not ABI) and PZ1 §0.5's new red, and it is why
rebuilding BOTH components — not just `wfc` — is the brief's own last step.

**Rebuild** (under the fleet wasm mutex; the `jb1` ticket was queued at 16:21 and reached the lock
at 19:56 after `pz1`/`rb1`/`play` — 3 h 35 min of queue, and the session was cut by the account
limit at ~18:20 while waiting, so the detached hold finished on its own):

| component | built | staged bytes | sha256 |
| --- | --- | ---: | --- |
| `semio_s_plugin_note.wasm` | 19:56:15 → 19:58:05 | 64 528 261 → **65 046 823** | `7cc1e0ed1125…` → `28f9ba635f35…` |
| `semio_s_plugin_wfc.wasm` | 19:58:05 → 20:00:11 | 125 434 240 → **126 899 875** | `04295417effa…` → `c5f82e5fdeb9…` |

rc=0, 0 errors, capture `🗑️generated/jb1-rebuild-components.txt`, script
`📜️jb1-rebuild-wfc-note.sh`, private uplift dir `⚡️cache/cargo/target-jb1-wasm`, staged into the
shared `wasm32-wasip2/wasm-dev` dir with rm+cp.

**After the rebuild, before the re-describe** (20:26): `client-e2e: 5/6 steps green`. This is the
predicted consequence of the warning above and it is the gate working, not a regression of the job
change: CE1's freshness step is the FIRST os step and **fails closed**, so the whole os journey
aborts before `tools/list`:

> `note`: the staged `wasm-dev/semio_s_plugin_note.wasm` (65 046 823 B, sha256 `28f9ba635f35…`) is
> NOT the build its committed descriptor describes (`f0add4106aa3…`) … re-describe it

**Re-describe** — `📜️jb1-describe-batch.sh 🗒️note 🀄️wfc` (PZ1's batched recipe, ONE mutex hold for
both owners), lock taken 20:28, ledger `🗑️generated/jb1-describe-ledger.txt`:

| owner | rc | wall | guest execute | outcome |
| --- | ---: | ---: | ---: | --- |
| `🗒️note` | **0** | 522 s | 91.6 s, 120 659 482 fuel | descriptor recommitted, `wasm_sha256 = e6dc23fc54a2…`, `🔣️.json` 197 882 B, pack 55 482 B (byte sizes unchanged, hashes new) |
| `🀄️wfc` | **1** | 2 829 s | **1 800 s — epoch deadline exceeded**, 373 368 646 fuel | `descriptor emitter exited with 1`; descriptor NOT refreshed |

**After the re-describe** (21:26): `client-e2e: 5/6 steps green`, capture
`🗑️generated/jb1-client-e2e-after.txt`. The freshness step now reads:

> `note`: `wasm-dev/semio_s_plugin_note.wasm` 65 046 823 B, sha256 `e6dc23fc54a2…` **matches its
> committed descriptor** | `wfc`: the staged … (126 958 189 B, sha256 `98246cccac67…`) is NOT the
> build its committed descriptor describes (`b0e311466809…`)

So `🗒️note` — the component that blocked the journey at `artifact_create` with
`no exported instance named semio:framework/codec@1.0.0` — is fixed and current. **`🀄️wfc` is not,
and that is a regression this slice caused and could not undo**: its staged component is new (a
`describe` run rebuilds into the shared dir itself, and the build is not reproducible, so the
09-20 binary that matched `b0e311466809…` cannot be restored), while its descriptor is old. The
gate therefore still aborts at step 3 and **no run in this slice has reached the `inference_run`
row**. `🀄️wfc`'s `describe` hitting the 1 800 s guest epoch is the same `🧩️puzzle`-class cliff PZ1
§2.3 measured (there: 5 872 116 097 fuel at the wall; here: 373 368 646 fuel at 207 fuel/ms, a
twelfth of `🗄️stdio`'s 2 568 fuel/ms — i.e. the guest is spending wall time outside fuel, which on a
fleet-loaded machine may simply be contention). One retry (`jb1w`) was queued on the mutex at 21:25
writing `🗑️generated/jb1-describe-wfc-retry.txt`; if it also dies on the epoch, `🀄️wfc` needs a
describe-cliff slice of its own before this gate can be green, exactly as `🧩️puzzle` did.

**`semio-os-mcp` was NOT re-staged.** The gateway's own source is untouched by this slice and the
staged binary is today's 11:04 build; the builtin-job change is guest-side (it compiles into the
component; the host's `run_job_to_completion` relay is unchanged), so a re-stage would not change
what the gate observes. If a later run shows a host/guest ABI mismatch, that is the first thing to
redo.

**What is and is not proven.** The framework fix is proven natively (39/39 laws, §3) and proven to
COMPILE into a `wasm32-wasip2` component (§4). It is NOT yet proven at runtime in a shipped
component, because the gate that would show it is blocked one step earlier on a descriptor refresh
that is a separate, measured cliff.

## 6. Honest gaps

1. **`🀄️wfc`'s committed descriptor is stale and this slice made it so** (§5). Its `describe` dies
   on the 1 800 s guest epoch, the staged component cannot be rolled back (the build is not
   reproducible), and CE1's freshness step fails closed on it, so `client-e2e` aborts at step 3 and
   the `inference_run` row this slice exists to turn green is still unreached. A retry was queued
   (`jb1w`); if it fails, `🀄️wfc` needs a describe-cliff slice like the one `🧩️puzzle` got.
   **This is the single most important handover item.**
2. `settle_in_step` polls once. If a future edit makes any of the five native dispatches genuinely
   suspend, that kind fails with `job.<kind>.suspended` rather than suspending. That is deliberate
   (a bounded step has no executor, and a poll-world host-await could never resolve), but it is a
   real constraint a later "real scheduler-driven awaiting" packet must lift at the state-machine
   level, not by re-introducing an opaque executor.
3. `WORK_UNITS_EXECUTE = 1 024` is a declared price, not a measured one. It is large enough to be a
   meaningful gate against a nearly-spent grant and far below the 6 000 000 the host actually
   sends; a later packet that measures real fuel consumption per dispatch should replace the
   constant with the measurement.
4. The interactive machine's `retire()` ceiling (4 096 page grants) is synchronous work inside
   `cancel()`. A session that needs more pages than that leaves the wrapper deep and the caller
   sees `job.bounded-false-terminal`. Bounded and loud, but a cancellation is still not itself
   sliced across `step-job` calls — the trait has no `cancel_step`.
5. `BUILTIN_JOB_KINDS` is an array the author keeps in step with `builtin_registry()`; the law
   compares them, so they cannot silently diverge, but neither is derived from the other.

## 7. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs` | rewritten: one bounded registry, `BUILTIN_JOB_KINDS`, `job_kind_is_admitted`, work-unit prices, `TwoPhaseBoundedJob`, `settle_in_step`, cfg-free `JobBody`/`spawn_job`/`step_job`/`checkpoint_jobs`; `JobFn`/`JobCtx`/`JobTick`/`run_two_phase`/the test executor deleted |
| `…/💼️jobs/💡️infer/🦀️.rs` | `job_infer` is a factory; `run_interactive_inference` became `InteractiveInferenceJob` (6 states + retirement); `decode_request` extracted |
| `…/💼️jobs/🧬️mutation-plan/🦀️.rs`, `…/🔀️migrate/🦀️.rs` | factories over `TwoPhaseBoundedJob`, sync phase slots |
| `…/💼️jobs/🧪️tests/🔬️unit/🦀️.rs` | rewritten laws (§3) |
| `…/💼️jobs/{💡️infer,🧬️mutation-plan,🔀️migrate}/🧪️tests/🔬️unit/🦀️.rs` | grants renamed to `FULL_GRANT` (the execute price); infer fixtures get distinct cancellation ids |
| `…/🔌️plugin/🏗️builder/🦀️.rs` | `.job(kind, factory)` takes a `BoundedJobFactory`, folded into `register_bounded_job_kind` |
| `…/🔌️plugin/🦀️.rs` | `framework_reserved_job_factory` takes `_restored` |
| `…/🔌️plugin/🖥️host/🦀️.rs` | one stale doc reference to `register_job_kind` |
| `📜️script.ts` | `toolJobOpaqueFutureProductionFailClosed` rewritten (it REQUIRED the dead route's shape); `toolJobFem2dMountedSessionExact`'s two `cfg`/`ExplicitStateMachineRequired` clauses negated; the `:6749` failure message updated |
| `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts` | the `femMountedJobs` fixture no longer carries the cfg-split shape |
| `✏️s/🔌️plugins/🗒️note/{🔣️.json,🛂️.descriptor.semio}` | recommitted by the plugin's own `describe` (§5) |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/…/✏️editor/🧵️session/🦀️.rs` | the two mounted-job factories take `_restored` |
| ticket scripts | `📜️jb1-rebuild-wfc-note.sh`, `📜️jb1-describe-batch.sh` |
