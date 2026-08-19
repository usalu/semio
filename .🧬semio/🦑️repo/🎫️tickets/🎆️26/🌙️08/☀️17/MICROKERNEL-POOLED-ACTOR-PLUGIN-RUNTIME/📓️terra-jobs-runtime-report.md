# 📓️ terra-jobs-runtime report

Packet: make jobs authorable — the compute model. Owned paths: `⚛️reactor/💼️jobs/🦀️component.rs`
(rewrite), `🔌️plugin/🏗️builder/🦀️component.rs`. No cargo run (rule 4) — every command below is
**UNRUN**, pasted only as the exact command the coordinator should run.

## delivered

- **`⚛️reactor/💼️jobs/🦀️component.rs`** — full rewrite (170 → 704 lines). The old closed
  `match record.kind.as_str() { JOB_KIND_IO_RUN => .., JOB_KIND_IO_SNIFF => .., other => Failed }`
  is deleted. In its place: a `kind → JobFn` registry (`register_job_kind`/`KIND_REGISTRY`), a
  `JobCtx` (`tick`/`progress`/`checkpoint`/`budget`/`host`) sliced across `step_job` calls on a
  **dedicated** `JOBS_EXECUTOR: ⚛️reactor/🧵️executor::LocalExecutor` instance (separate from the
  reactor turn loop's own `EXECUTOR`), a stall guard, and checkpoint pack support
  (`checkpoint_jobs`/`restore_job`). `semio.io-run`/`semio.io-sniff` are preserved as ordinary
  registry entries (`job_io_run`/`job_io_sniff`), bodies byte-identical to the pre-rewrite
  `run_io_run`/`run_io_sniff` (only the return type moved from `JobOutcome` to
  `Result<Vec<u8>, Fault>`), pre-registered so every plugin gets them without calling `.job(...)`.
- **`🏗️builder/🦀️component.rs`** — added `PluginBuilder::job(kind, run)` next to
  `.plugin_command(...)`, a `jobs: Vec<(&'static str, JobFn)>` field threaded through the same
  4 struct-literal sites `commands` already touches (`new()`, `label()`, `version()`,
  `try_build()`'s destructure), and a fold loop in `try_build()` that calls
  `crate::reactor::jobs::register_job_kind(kind, run)` for each declared entry — this IS "bundle
  install" (the same `try_build()` call that installs every other builder registration), so no
  separate install hook was needed.
- `JobBudget`/`JobStep` are now real (a plain-Rust mirror of `jobs.wit`'s `record job-budget`/
  `variant job-step` — this module isn't gated to `component-guest`, so it can't name the
  WIT-generated type directly); `step_job` reads and stores the budget instead of discarding it.

## registry + slicing mechanics

`start_job(job, kind, input)` looks up `kind` in `KIND_REGISTRY`, builds the job's future, and
spawns it **parked** (ready but not yet polled) on `JOBS_EXECUTOR` — deliberately NOT the
reactor's own `EXECUTOR`, so a job slice never competes with UI-turn tasks for one `poll`'s
`run_until_idle` iterations (the architecture's stated split: async is the waiting model, jobs are
the compute model).

`step_job(job, budget)`:
1. Records `budget` on the job's shared `JobState`, clears this slice's `progress`, and grants
   exactly one more `JobCtx::tick()` resolution (`tick_budget += 1`).
2. Wakes the job's task by id and runs `JOBS_EXECUTOR.run_until_idle(64)` — only that one task is
   ready, so no other job's task runs.
3. Reads back `outcome`/`progress` from the shared state and returns `Done`/`Failed`/`Running`.

`JobCtx::tick()`'s own future (`JobTick`) needs **no waker bookkeeping**: it just compares
`ticks_consumed` against `tick_budget` and returns `Ready`/`Pending` — `LocalExecutor::wake(task)`
re-queues the task **by id**, so a fresh `Waker` per poll (which `LocalExecutor` already
constructs) is enough. A `JobFn` that awaits a real host future (`ctx.host()...await`, async-world
only) DOES get a real captured `Waker` at that await point, executor-agnostic by construction
(`⚛️reactor/📮️requests::RequestFuture` stores whatever `Waker` last polled it — confirmed by
reading that file, not assumed), so `RequestRegistry::resolve` correctly wakes the task back onto
`JOBS_EXECUTOR` even though it was never awaited on the reactor's own executor.

Stall guard: `step_job` compares the incoming `budget` against the previous call's; if it's
unchanged AND (after running) no `progress` bytes were set this slice, `stall_count` increments —
any slice with new progress OR a changed budget resets it to zero. Reaching `STALL_LIMIT` (3) fails
the job as a typed `job.stalled` fault and frees the slot, instead of returning `Running` forever.
This is a deliberate, documented judgment call (see `## honest gaps`) — there is no real fuel
metering in this wave (same as the pre-rewrite file: "job-budget is accepted but not metered yet"),
so "budget consumed" is approximated as "the host passed a different `JobBudget`."

## host-await restriction rationale

`JobCtx::host()` is `#[cfg(feature = "component-guest-async")]`-gated, with a docstring on the
field AND the accessor telling a future reader not to "helpfully" ungate it. Read
`🖥️host/🦀️component.rs`'s `PluginInstanceHandle::run_job_to_completion` (~line 1435) to confirm
this is real, not assumed: it loops `start_job` then `step_job` in a tight relay **strictly
POST-TURN**, by its own doc comment, "never re-entrant into an in-flight turn's own `Store`" — it
never calls `poll` again for this job. A poll-world (`world actor`) job that awaited a host effect
would park on an `⚛️reactor/📮️requests::RequestFuture` whose resolution only happens inside
`poll`'s `Event::Completed` routing step — which this relay loop never triggers — so the loop would
observe `Running` forever and spin. `world actor-async`'s `runner::run` has no such gap (the host
interleaves completions into the same long-lived call), which is exactly the world this accessor is
scoped to. Poll-world jobs therefore receive every input they need up front (`input`/`restored`)
and drive progress through `JobCtx::tick()` alone — never `host()`.

Note: the `component-guest-async` Cargo feature does not exist yet in
`🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (grepped repo-wide, zero hits before this packet) — that
Cargo.toml is not in this packet's owned paths, so the `#[cfg(feature = "component-guest-async")]`
gate compiles to "always off" until whichever packet wires `world actor-async`'s guest bindings
(`sdk-async`/`async-worlds` per `important.md`'s W5+ packet-id list) adds the feature. This is by
design, not a gap to fix here — the accessor and its restriction exist now so that packet only has
to add the feature flag, not design the gate.

## line ranges edited

- `⚛️reactor/💼️jobs/🦀️component.rs` — entire file rewritten, lines 1–704 (was 170 lines).
- `🏗️builder/🦀️component.rs` — was 929 lines, now 957:
  - Struct field: `jobs: Vec<(&'static str, crate::reactor::jobs::JobFn)>` (~line 56–62).
  - `new()`/`label()`/`version()` carry-through: one line each (~110, ~143, ~178).
  - `.job(kind, run)` method: ~line 248–256 (right after `.plugin_command`).
  - `try_build()` destructure (`jobs,` added ~line 501) and fold loop (~line 599–606, right after
    the `commands` fold).

## tests written (names)

All in `⚛️reactor/💼️jobs/🦀️component.rs`'s `#[cfg(test)] mod tests`:

- `step_job_on_an_unknown_id_fails_without_panicking` (kept from pre-rewrite, updated call shape)
- `cancel_job_removes_a_pending_record_so_a_later_step_fails` (kept)
- `step_job_on_an_unknown_kind_fails_with_a_named_fault` (kept)
- `io_run_dispatches_through_the_registry_and_keeps_its_decode_fault_code` (new — proves the
  registry reaches the builtin, not `job.unknown-kind`)
- `io_sniff_dispatches_through_the_registry_and_keeps_its_decode_fault_code` (new, same proof)
- `a_three_slice_job_returns_running_running_done_with_progress_each_slice` (mission-required:
  `Running, Running, Done` + progress bytes each slice)
- `the_budget_a_tick_observes_is_whatever_step_job_most_recently_passed` (mission-required: budget
  the body sees changes when the host passes a different one)
- `cancelling_a_job_mid_slice_frees_its_slot_for_the_id` (mission-required: cancel mid-slice frees
  the slot)
- `checkpoint_restore_resumes_and_matches_an_uninterrupted_run` (mission-required: checkpoint/
  restore resumes from packed state and produces the same final output as an uninterrupted run —
  runs a job to completion uninterrupted, separately runs an identical job through one real slice,
  extracts `checkpoint_jobs()`, cancels, `restore_job`s, finishes it, and asserts byte-identical
  final output)
- `the_stall_guard_fires_after_repeated_no_progress_static_budget_slices` (mission-required: stall
  guard fires)

10 tests total, all natively runnable (this module is not `#[cfg]`-gated to wasm32).

## commands + exit codes (all UNRUN — coordinator owns acceptance builds per rule 4/23)

```
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target \
  cargo test -p semio-framework-plugin --lib reactor::jobs::tests -- --nocapture
```
UNRUN — exit code not observed.

```
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target \
  cargo check -p semio-framework-plugin --lib
```
UNRUN — exit code not observed. This is the one that matters most: it will surface any signature
mismatch between my rewrite and the (not-yet-applied) lease edits below, since `🔌️plugin/🦀️component.rs`
currently still calls the OLD `crate::reactor::jobs::step_job(job)` (one arg) and matches on
`crate::reactor::jobs::JobOutcome` (a type this rewrite deletes) — **the crate will not compile
until the lease below lands**. Flagging this loudly rather than silently: this is expected,
sequenced breakage, not a defect in this packet's own files.

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-framework-plugin --lib --features component-guest --target wasm32-wasip2
```
UNRUN — exercises the WIT-bridge `JobsGuest` impl (the leased file); will also fail until the lease
lands, for the same reason.

## lease-requests

Two hooks, in files owned by the sibling packet. Exact diff text below — please apply verbatim
(or tell me if the surrounding code has moved since I read it).

### 1. `🔌️plugin/🦀️component.rs` — `JobsGuest` impl (currently lines ~39–57)

Replace:
```rust
    impl JobsGuest for ComponentGuest {
        fn start_job(job: u64, kind: String, input: Vec<u8>) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            crate::reactor::jobs::start_job(job, &kind, &input);
            Ok(())
        }

        fn step_job(job: u64, _budget: JobBudget) -> Result<JobStep, PluginError> {
            ensure_plugin_initialized();
            Ok(match crate::reactor::jobs::step_job(job) {
                crate::reactor::jobs::JobOutcome::Done(bytes) => JobStep::Done(bytes),
                crate::reactor::jobs::JobOutcome::Failed(bytes) => JobStep::Failed(bytes),
            })
        }

        fn cancel_job(job: u64) {
            crate::reactor::jobs::cancel_job(job);
        }
    }
```

With:
```rust
    impl JobsGuest for ComponentGuest {
        fn start_job(job: u64, kind: String, input: Vec<u8>) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            crate::reactor::jobs::start_job(job, &kind, &input);
            Ok(())
        }

        fn step_job(job: u64, budget: JobBudget) -> Result<JobStep, PluginError> {
            ensure_plugin_initialized();
            let budget = crate::reactor::jobs::JobBudget { fuel: budget.fuel, deadline_ms: budget.deadline_ms };
            Ok(match crate::reactor::jobs::step_job(job, budget) {
                crate::reactor::jobs::JobStep::Running(progress) => JobStep::Running(progress),
                crate::reactor::jobs::JobStep::Done(bytes) => JobStep::Done(bytes),
                crate::reactor::jobs::JobStep::Failed(bytes) => JobStep::Failed(bytes),
            })
        }

        fn cancel_job(job: u64) {
            crate::reactor::jobs::cancel_job(job);
        }
    }
```
(`JobBudget`/`JobStep` unqualified here are the WIT-generated types already imported at this
file's top via `use exports::semio::framework::jobs::{Guest as JobsGuest, JobBudget, JobStep};` —
no import changes needed, field/variant names match my plain-Rust mirrors 1:1.)

### 2. `⚛️reactor/📸️checkpoint/🦀️component.rs` — `jobs:` section

Add an import:
```rust
use crate::plugin_runtime;
use semio_framework::Fault;
use serde::{Deserialize, Serialize};
```
→
```rust
use crate::plugin_runtime;
use crate::reactor::jobs;
use semio_framework::Fault;
use serde::{Deserialize, Serialize};
```

Add a field to `CheckpointPack`:
```rust
#[derive(Serialize, Deserialize)]
pub struct CheckpointPack {
    instances: Vec<InstanceCheckpoint>,
    timers: Vec<u64>,
    pending_requests: Vec<u64>,
}
```
→
```rust
#[derive(Serialize, Deserialize)]
pub struct CheckpointPack {
    instances: Vec<InstanceCheckpoint>,
    timers: Vec<u64>,
    pending_requests: Vec<u64>,
    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime) — every job this actor
    /// still has open, packed via `⚛️reactor/💼️jobs::checkpoint_jobs()`; `restore()` below replays
    /// each through `jobs::restore_job`.
    jobs: Vec<jobs::JobCheckpointEntry>,
}
```

In `checkpoint()`, change:
```rust
    let pack = CheckpointPack { instances, timers, pending_requests };
```
to:
```rust
    let pack = CheckpointPack { instances, timers, pending_requests, jobs: jobs::checkpoint_jobs() };
```

In `restore()`, add before `Ok(pack)`:
```rust
    for job in &pack.jobs {
        jobs::restore_job(job.job, &job.kind, &job.input, job.checkpoint.clone());
    }
```

`JobCheckpointEntry` already derives `Serialize`/`Deserialize` on my side (`⚛️reactor/💼️jobs`), so
no further glue is needed — `CheckpointPack`'s own derive covers the new field directly.

## honest gaps

- **Cargo does not compile end-to-end until the lease lands.** By design (rule 3) — I own the two
  files that define the new shapes; the sibling's `JobsGuest` impl and checkpoint pack are the only
  call sites, both leased above, both minimal mechanical diffs.
- **`cancel_job`/re-`start_job`-over-an-existing-id do not free the task's slot inside
  `JOBS_EXECUTOR`.** `⚛️reactor/🧵️executor::LocalExecutor` (not in my owned paths — sibling/
  unclaimed, read-only for me) exposes `spawn`/`wake`/`run_until_idle`/`has_ready`/`has_pending`
  but no by-id removal. Cancelling drops my own `JOBS` bookkeeping slot (which is what the mission's
  "cancel mid-slice frees the slot" test checks, and what the pre-rewrite file's own semantics
  already meant by "the slot"), but the orphaned `Rc<RefCell<JobState>>` + boxed future sitting
  inside `JOBS_EXECUTOR`'s `Vec<Option<BoxedTask>>` is never reclaimed — a bounded, per-actor-
  lifetime memory overhead (not per-turn, not unbounded across a session, since an actor is torn
  down and restored from checkpoint on any trap), not a correctness bug. Real fix would be a
  `pub fn cancel(&self, id: TaskId)` on `LocalExecutor` — flagged here rather than added silently,
  since that file is outside my owned paths.
- **The stall guard's "budget consumed" signal is `budget != last_budget`, not real fuel
  metering.** There is no fuel-per-tick accounting anywhere in this wave (the pre-rewrite file's own
  doc note: "job-budget is accepted but not metered yet" — still true here, just no longer silently
  discarded). `STALL_LIMIT = 3` is a judgment call, documented at its definition; not measured
  against any real workload.
- **`JobCtx::host()` returns `&Host` (owned `Host` constructed once at `start_job`/`restore_job`
  time via `crate::reactor::host()`), not literally the brief's illustrative `&Host` built lazily
  per-call** — `Host` is `#[derive(Clone, Default)]` and cheap (an `Rc` bump under a
  `RequestRegistry`), so this is a one-line implementation choice, not a scope gap.
- **No `descriptor_is_fresh`/descriptor regeneration was touched** — `.job(...)` doesn't change any
  plugin's static descriptor surface (jobs aren't in `PackageDescriptor` per design-abi.md §3), so
  no plugin needs a descriptor re-emit for this packet alone.
- I did not attempt to make any real *first-party* plugin call `.job(...)` yet (e.g. the WFC solve
  named in the mission's motivation) — that's explicitly out of this packet's scope (design-abi.md
  §6 migration ordering: SDK first, then per-crate migration waves).
