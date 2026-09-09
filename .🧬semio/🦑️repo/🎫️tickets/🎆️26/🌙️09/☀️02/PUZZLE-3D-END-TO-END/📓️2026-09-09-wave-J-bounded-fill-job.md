# 🪣️ Wave J — Bounded Fill Job (2026-09-09)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-J. Written incrementally.

## 0. Conditions

- Repo `/Users/ueli/Documents/semio`, branch `HEAD` (detached), concurrent devs (W-S on the native `settle` stall; the precompute unit-test file changed under me once mid-session).
- Repo MCP failed to connect this session (`repo (-32602): invalid initialize params`), so the ticket folder is managed on disk; the ticket is NOT closed by this wave.
- Private cargo target dir: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`.
- Build env: `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728`.
- 🚧️ The scratch volume hit `ENOSPC` partway through the run (a peer's build filled the disk). Which
  laws had actually executed before that point is recorded verbatim in §4/§5 — nothing is claimed
  green that was not observed green.

## 1. Defect and host stepping path

### 1.1 The guest-side defect (fixed here)

`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:71` registered the fill job as `.job(FILL_JOB_KIND, fill_job)` — a plain
async `JobFn`. In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`:

- `spawn_job` (:349) resolves `BOUNDED_KIND_REGISTRY` FIRST (:353); only if the kind has no
  `BoundedJobFactory` does it fall through to `KIND_REGISTRY` (:364).
- On a non-`cfg(test)` build — which every production wasm guest is — that fallthrough files the job
  as `JobBody::ExplicitStateMachineRequired` (:368-372) and discards both `run` and `restored`.
- `step_job` (:424) then takes the `running == None` branch (:459) and answers
  `Failed(job.explicit-state-machine-required)` (:464, :481).

So every `Effect::SpawnJob { kind: FILL_JOB_KIND }` died on its first step, `ready` stayed 0, and the
120 ms `fillBuildTick` loop kept turning. Confirmed by a negative-control law (see §3, law b): with a
plain `JobFn` registration the runtime really does return that refusal.

### 1.2 The host stepping path — the React/browser target never steps an isolated job

Traced end to end (`file:line`):

| hop | what happens |
| --- | --- |
| `⚛️reactor/🔄️turn/🦀️.rs:1096` | `push_admitted_effect` — the single admission funnel |
| `⚛️reactor/🔄️turn/🦀️.rs:1104-1105` | binds `(instance, job)` into `JOB_RENDER_BINDINGS`; a capacity collision becomes a `plugin.job-render-binding-capacity` shell fault and the effect is DROPPED |
| `🌐host/🦀️.rs:617` | the Poll-world source: `registry.request(move |req| Effect::SpawnJob { job: req.0, … })` — the job id IS the request id |
| `🧬️schema/📜️.wit:479-484,593` | `record spawn-job-effect { job, kind, input, placement }`, `spawn-job(spawn-job-effect)` in `variant effect` |
| `🧬️schema/📜️.wit:748-753,800-801` | `job-progress-event { job, progress }`, `job-completed-event { job, outcome: completion-result }` |
| `🧬️schema/📜️.wit:1313-1323` | the guest exports the host is meant to drive: `job-budget`, `variant job-step`, `start-job`, `step-job`, `cancel-job` |
| `🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:189-222` | **`wireEffectToFriendly` has NO `spawn-job` case** — the effect falls to `default:` and is dropped with `console.warn("[DEBUG] … unmapped effect \"spawn-job\" dropped …")` |
| `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:790-835` | the renderer's own copy of `wireEffectToFriendly` — same omission, same `default:` drop (:833) |
| `🌐️browser-bundle/🌐️host/🟦️.ts:167` | `spawnJob: (job, kind, input, placement) => request("spawn-job", …)` — the `host-async` import, i.e. the ASYNC world's route, not the poll world's |
| `📮️shard-client/🟦️.ts:2047-2073` | `handleEffectRequest` has no per-effect switch at all; every name goes to `onHostEffect` |
| `📮️shard-client/🟦️.ts:2056-2058` | with no handler installed it replies `effect-error "no host effect handler installed"`. Repo-wide there is **no production `onHostEffect`** — only the option passthrough and test harnesses |
| `📮️shard-client/🟦️.ts:1941-1957` | `ShardClient.startJob` / `stepJob` / `cancelJob` exist… |
| `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:403-408` | …and the shard worker has `case "startJob"` / `case "stepJob"` routes (but NO `cancelJob` case — it would hit `default: throw unknown shard worker message kind`) |
| — | **no production caller.** `\.startJob(` / `\.stepJob(` and the message kinds `"startJob"`/`"stepJob"` appear only at those definition sites (plus `🌉️bridge.js` guest shims and `/dist/`). `runJobToCompletion` has zero JS/TS hits; `jobProgress`/`job-progress`/`jobCompleted` have zero production hits |

**Verdict (state BEFORE this wave): on the React/browser target an isolated job is never started and
never stepped — not once.** Two independent breaks stack: the poll-world `spawn-job` effect is
dropped in `wireEffectToFriendly`, and even if it were mapped, nothing drives `startJob`/`stepJob`.
§2 fixes both for the React host; the wgpu host is still in this state (§5).

Additional wire defects found in the dead JS job path (all would need fixing before it can work):

- `ShardClient.startJob/stepJob/cancelJob` type `job` as `number`; the worker forwards `msg.job`
  straight into jco, whose `u64` lowering needs a `bigint` (`📮️shard-client/🟦️.ts:363-365`).
- `ShardJobBudget` is `{fuel: number, deadlineMs: number}` (`📮️shard-client/🟦️.ts:71-74`) but WIT
  `job-budget.fuel` is `u64` → also needs `bigint`.
- `ShardJobStep` is declared as `{status:"running"|"done"|"failed", …}`
  (`📮️shard-client/🟦️.ts:156`) while the worker returns jco's raw `{tag, val}` variant unchanged
  (`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:408`).
- `cancelJob` has no worker case.

### 1.3 Native contrast

Natively the driver is `ShardLoop::pump` (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs`): `running_jobs` (:359) is
stepped exactly once per `pump()`; `Effect::SpawnJob` admission at :1842; budget from the actor grant
at :1705 / :146-147; one guarded `step_job` at :1714-1717; `Done`/`Failed` deferred as
`Event::JobCompleted` at :1721-1765 and merged into the spawning actor's NEXT pump (:378-383).
`run_job_to_completion` no longer exists (doc-comment references only).

Note even natively: `Event::JobProgress` has no construction site outside the wire decode
(`⚛️reactor/🦀️.rs:1427`) and the kernel→WIT encode (`🖥️host/🦀️.rs:2671`) — `ShardLoop` turns
`JobStep::Running(Some(preview))` into a renderer-facing `ShardOutcome::Job` publication
(`🖥️host/🧵️shard/🦀️.rs:1770-1773`), not an event back into the guest. So only `JobCompleted` re-enters
the actor. The guest side is ready for both (`⚛️reactor/🔄️turn/🦀️.rs:317`, :327-344).

### 1.4 Why the guest ran out of memory

The registry-level guard in `enqueue_fill_job` stops a re-spawn only while the envelope's authority
is still live. A job that fails, or an envelope terminalized as `Fault`, is closed by
`pump_fill_terminal_step`, which clears `fill_job` — and the very next 120 ms tick then re-runs
`start_fill_preparation(true)` and a whole fresh measurement. That measure→admit→spawn→fault→close
cycle is unbounded and allocates a new `FillBuilder` preparation per cycle; with the fill tool active
it runs forever, at zero planned objects. That is the growth item 3 asked for, and the fault latch in
§2 is what stops it. (The tick's own `tool_operations` registry is an `ArtifactFixedRegistry` and is
NOT a growth source.)

## 2. Changes

All paths relative to the repo root.

### Guest side — `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/⏳️precompute/🦀️.rs`

- **imports** — dropped `std::future::Future` / `std::pin::Pin` (only the deleted async body used
  them); added
  `use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};`.
- **`//#region 💼️SharedPluginJob`** — the async `pub fn fill_job(JobCtx, Vec<u8>, Option<Vec<u8>>)
  -> Pin<Box<dyn Future<…>>>` is DELETED (no compatibility path kept) and replaced by:
  - `FILL_JOB_FAULT_{DECODE,IDENTITY,STALE,OWNER,TERMINAL}` — fixed `&[u8]` fault payloads, matching
    `🏗️fem`/`🔋️energy`'s bounded convention (a terminal path must not allocate a growable page).
  - `enum FillJobStage { Admitting(FillEnvelopeJobEntryCursor), Driving { request, guard }, Terminal }`
    — one `step` per token-cursor field while admitting (the same granularity `JobCtx::tick()` gave),
    one `step` per `drive_fill_envelope` while driving.
  - `pub(crate) struct Puzzle3dFillBoundedJob` + `impl BoundedJob`:
    `step` → `Running(None)` for a cursor field or a `Blocked` drive, `Running(Some(token))` when the
    slice advanced the observation, `Done(token)` on `slice.done` (after `guard.disarm()`),
    `Failed(<fixed code>)` for decode / identity / stale / missing-owner / post-terminal;
    `cancel` → `authority.cancel.cancel_now()`; `checkpoint` → the live envelope token;
    `terminal_drop_is_shallow` → `true` (the guard's `Drop` is three atomics).
  - `fn fill_job_factory(job, input)` — deliberately never `Err`: the fault guard must be armed from
    the RAW input before the token is validated, so a malformed token still terminalizes the envelope
    it names. The decode verdict is the first `step`'s, exactly as the async body took it on its first
    `tick`.
  - `pub fn initialize()` — `register_bounded_job_kind(FILL_JOB_KIND, fill_job_factory)`.
- **`Puzzle3dPrecomputeSession`** — two new fields: `fill_faulted: bool` (latch that stops
  re-admission after a FAULT terminal) and `fill_fault_notice: bool` (one pending user notice).
- **`supersede_admitted_fill`** — clears `fill_faulted` before its early return, so any edit that
  invalidates the plan also re-arms planning.
- **`take_fill_fault_notice` / `fill_is_faulted`** (new, `//#region 💼️FillJobBridge`) — the tick's hooks.
- **`enqueue_fill_job`** — returns `None` immediately while `fill_faulted`.
- **`observe_fill_terminal_reason`** (new) — latches the notice at terminal CHECKOUT (the authority is
  gone by the time `close_step` reports `Complete`, so `reason()` must be read then); called from both
  mount points in `pump_fill_terminal_step`. The `take_closed()` mount drops the registry lock first,
  because `reason()` re-locks.

### `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`

- `plugin()` calls `semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::initialize();` before
  `Plugin::builder`, the same place `🏗️fem` (`✏️s/🔌️plugins/🏗️fem/🦀️.rs:38-39`) and `🔋️energy` call
  theirs. The `.job(FILL_JOB_KIND, fill_job)` builder line is DELETED.

### `…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs` and `…/✏️editor/🗣️terminology/🦀️.rs`

- `fill_build_tick` takes `precompute.take_fill_fault_notice()` and, when set, raises
  `ctx.notice(|labels| labels.fill_failed.as_str())` and dirties the fill scope.
- New label `fill_failed`, all four locale×terminology cells authored (en/de × native/reuse).

### Host side — the React/browser target now really starts and steps isolated jobs

- **`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`** — `ShardJobBudget.fuel` is a `bigint`
  (WIT `u64`); `startJob`/`stepJob`/`cancelJob` and their `OutboundMessage` variants carry `job:
  bigint`; `cancelJob` became an awaited request with a `requestId` (it used to post a message the
  worker had no case for, which would have hit `default: throw unknown shard worker message kind`).
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`** — the generated
  shard-worker/adapter source gains `normalizeJobStep`, converting jco's raw `{tag, val}` `job-step`
  variant into the `{status, …}` `ShardJobStep` every caller declares, plus the missing
  `case "cancelJob"` worker route.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`** —
  `routeDocumentEffects` is renamed `routeHostEffects(instanceId, effects, port)` (all five call
  sites updated) and is now the single choke point that also intercepts `spawn-job`/`cancel-job`.
  New: `drivingJobs` (one drive per `actorId#job`), `jobStepBudget` (the actor grant reshaped, the
  same thing `ShardLoop::pump` does natively), `PLUGIN_JOB_STEP_LIMIT` / `PLUGIN_JOB_STEPS_PER_YIELD`,
  `deliverJobCompletion` (submits `{kind:"job-completed", payload:{job, outcome}}` on the Background
  lane — the event `⚛️reactor/🔄️turn/🦀️.rs:327-344` uses to resolve the guest's parked `spawn_job`
  request, since `RequestId(job)` IS the job id), and `driveSpawnedJob` (start once, then one
  `step-job` per iteration until `done`/`failed`, then the completion event; over budget it cancels
  and reports `plugin.job-step-budget`). Every `startJob`/`stepJob`/`cancelJob` call goes through
  `serializeCommandIngressForActor`, so a job slice never re-enters the guest while a turn is in
  flight. `job-progress` is deliberately not delivered per slice: the guest ignores its payload
  (`⚛️reactor/🔄️turn/🦀️.rs:317` only marks the surface dirty) and one actor turn per slice costs more
  than the repaint is worth.

### Checkpoint / restore contract

`BoundedJobFactory` is `fn(u64, &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>>` — `spawn_job`'s
bounded branch (`⚛️reactor/💼️jobs/🦀️.rs:353-363`) never passes `restored` to it, so `restore_job`'s
checkpoint bytes are dropped for EVERY bounded kind (fem2d, fem3d, energy included). For puzzle 3d
this is harmless and now enforced rather than assumed: the fill kind's checkpoint IS its spawn input,
byte for byte (both are `FillEnvelopeAuthority::token_page`, written once by `finish_measurement` and
never rewritten), so a restored actor rebinds the identical envelope from `input` alone. The law
`bounded_fill_job_reaches_done_…` pins that equality via `checkpoint_jobs()`. **The general framework
gap is real and is NOT fixed by this wave** — see §5.

## 3. Laws

### Guest — `…/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`, region `💼️BoundedFillJob`

- **(b) `fill_job_kind_is_bounded_and_a_plain_job_fn_is_not`** — drives the REAL reactor job runtime
  (`reactor_jobs::start_job` / `step_job`) against `FILL_JOB_KIND` and asserts the first step is
  `Running`. `semio-framework-plugin` is not built in `cfg(test)` when this crate's tests run, so this
  exercises exactly the production `spawn_job` branch. The negative control registers
  `semio.puzzle3d.fill-law-explicit` as a plain `JobFn` and asserts its step is
  `Failed(… explicit-state-machine-required …)` — without it the positive assertion would prove nothing.
- **(a) `bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint`** —
  steps with the smallest possible budget (`JobBudget { fuel: 1, deadline_ms: 1 }`) until `Done`;
  asserts at least one `Running(Some(token))` progress slice, that every published progress equals the
  spawn input, that `Done` carries the same token, that `checkpoint_jobs()` offers a checkpoint equal
  to the spawn input, and that the registry publication the session reads is marked `done`. Adds
  `drain_fill_envelope` (the mounted counterpart of `drain_orphaned_fill_envelope`).
- **(c-i) `an_unstepped_fill_job_is_enqueued_once_across_five_hundred_ticks`** — 512
  `poll_fill_job`+`enqueue_fill_job` turns with the job never stepped (exactly what the React target
  used to do to an isolated job): exactly ONE spawn, and `aggregate_bytes` unchanged after admission.
- **(c-ii) `a_faulted_fill_envelope_latches_one_notice_and_never_silently_retries`** — terminalizes an
  admitted envelope as `Fault`, then 512 ticks: ZERO further spawns, the latch set, exactly one notice
  taken (and only once), and the whole process byte credit returned.

Fixture: the crate's own `fill_worker_session(seed)` (single-host / single-vortex fill-capable scene),
NOT the Concrete Forest example — the Concrete Forest fill laws live in the `dispatch`-driven editor
suite W-S is currently repairing (§5).

### Host — TypeScript

- **`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`,
  region `💼️JobWire`** — `carries u64 job identities as bigint and settles start, step and cancel
  against the worker`: asserts `job` and `budget.fuel` leave as `bigint`, that `stepJob` returns the
  declared `{status,…}` shape, and that `cancelJob` posts a `requestId`-bearing request that settles.
- **`…/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`** — `starts an isolated job, steps it to
  its terminal and feeds job-completed back into the actor`: a turn emits `spawn-job`; the law asserts
  `startJob` was called once with the exact `(job, kind, input)`, that `stepJob` was called repeatedly
  (three times, until the fake answers `done`), and that a `job-completed` event carrying
  `{job, outcome:{tag:"ok"}}` was submitted back to the actor. The `withRequester` harness gained an
  optional `hooks.jobs` and the fake shard client gained `startJob`/`stepJob`/`cancelJob`.
  **Proven to be a real guard**: with the `spawn-job` interception disabled the law fails with
  `started` empty (observed, then reverted; the file was diffed byte-identical afterwards).

## 4. Commands and tails

```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d \
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
```
→ `Finished dev profile [unoptimized] target(s) in 24.28s`; 1 pre-existing warning
(`restored_precompute_session` is never used — `✏️editor/🦀️.rs:1375`, not this wave's).

```
… cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1 fill_job
```
→ `running 4 tests` … `an_unstepped_fill_job_is_enqueued_once_across_five_hundred_ticks ... ok`,
`bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint ... ok`,
`fill_job_checkpoint_is_a_fixed_generation_token_not_a_whole_state_buffer ... ok` (pre-existing),
`fill_job_kind_is_bounded_and_a_plain_job_fn_is_not ... ok`
→ `test result: ok. 4 passed; 0 failed; 607 filtered out`

```
… cargo test … -- --test-threads=1 a_faulted_fill_envelope
```
→ `a_faulted_fill_envelope_latches_one_notice_and_never_silently_retries ... ok` — `1 passed; 0 failed`

```
… cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly precompute -- --test-threads=1
```
→ `test result: ok. 146 passed; 0 failed; 0 ignored; 465 filtered out; finished in 6.44s`

```
… cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
```
→ `Finished dev profile [unoptimized] target(s) in 14.83s` (production wasm build, warnings only, none
this wave's).

```
npx tsc --noEmit -p "🧰️framework/…/🎯️targets/⚛️react/tsconfig.json" | grep -E "PluginRuntime|shard-client|plugin-runtime|🔌️plugin/📦️packages"
```
→ no output (the workspace has many pre-existing errors in unrelated files; none in the files this
wave touched).

```
cd "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript" && npx vitest run "📮️shard-client"
```
→ `Test Files 1 passed (1)` · `Tests 151 passed (151)`

```
cd "🧰️framework/…/🎯️targets/⚛️react" && SEMIO_TEST_LEVEL=long npx vitest run "PluginRuntime"
```
→ `Test Files 1 passed (1)` · `Tests 82 passed (82)` (81 before this wave's law).

First attempt of law (a) FAILED at `close_fill_envelope`'s `take_terminal_fill_job().expect(…)`:
`poll_fill_job` had already checked the terminal out into the session. Fixed by draining through the
session's own cursor (`drain_fill_envelope`), then green as above.

## 5. Not verified

- **No browser run.** Every claim above is from native cargo tests, `tsc`, and vitest. The end-to-end
  proof — puzzle 3d's Fill tool reaching a non-zero planned count in the dev preview — was NOT
  performed by this wave and must be done before calling the defect closed.
- **`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript` has 11 failing tests** in
  `📤️return/`, `🚪️lifetime/`, `🪪️activation/` — all ajv `can't resolve reference … NonZeroU64`
  schema-registry failures in files this wave never touched, present alongside the 151/151 green
  shard-client suite. Not investigated, not caused here.
- **The wgpu renderer target still drops `spawn-job`.** The shared
  `🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:189-222` `wireEffectToFriendly` and the
  wgpu `🐚️plugin-bridge.ts:480` consumer have no `spawn-job` case and no job driver; an isolated job
  spawned under the wgpu host is still started zero times (it logs the `[DEBUG] unmapped effect`
  warning). Only the React/browser host was wired here.
- **`Event::JobProgress` is dead on both targets.** Natively it has no construction site outside the
  wire decode (`⚛️reactor/🦀️.rs:1427`) and the kernel→WIT encode (`🖥️host/🦀️.rs:2671`) — `ShardLoop`
  turns `JobStep::Running(Some(preview))` into a renderer publication
  (`🖥️host/🧵️shard/🦀️.rs:1770-1773`), not a guest event; this wave's browser driver deliberately does
  not synthesize it either. Puzzle 3d does not need it (its 120 ms tick reads the registry), but any
  plugin that does will find the variant unimplemented end to end.
- **The `BoundedJobFactory` restore gap is not fixed.** `restore_job`'s checkpoint bytes never reach a
  bounded factory. Puzzle 3d is immune (checkpoint == input, pinned by law) but fem2d/fem3d/energy are
  not: their `checkpoint()` returns identity bytes that differ from their `input`, so a
  checkpoint-restored actor silently rebuilds them from `input` only. Widening the factory signature
  touches three other plugins and was judged outside this wave's job-kind boundary.
- **`JobPlacement` is still advisory.** Natively `Isolated` only affects per-pump ordering
  (`🖥️host/🧵️shard/🦀️.rs:369-376`); this wave's browser driver likewise runs the job on the instance
  that spawned it. Nothing here makes an isolated job actually isolated.
- **The `dispatch`-driven component tests of this crate stall (W-S).** None of this wave's four Rust
  laws use `dispatch`, so none are blocked by it — but the existing editor fill laws
  (`fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`,
  `fill_build_tick_only_plans_available_slider_range`, …) were NOT re-run here, and the new
  `fill_failed` notice path in `fill_build_tick` therefore has no editor-level law.
- **The editor testkit's `drive_fill_until_ready` still bypasses the job runtime.** It calls
  `drive_enqueued_fill_job_for_test`, which drives `drive_fill_envelope` DIRECTLY
  (`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:596-605`). That stub is exactly why the native suite stayed green
  while production planned nothing; it was left in place (W-S owns the testkit this session) and the
  new laws in §3 cover the real path instead. Replacing it with a real `start_job`/`step_job` host
  loop is the obvious follow-up.
- **The scratch volume hit `ENOSPC` mid-session** (a peer's build filled the disk); everything above
  was re-run after it cleared.
