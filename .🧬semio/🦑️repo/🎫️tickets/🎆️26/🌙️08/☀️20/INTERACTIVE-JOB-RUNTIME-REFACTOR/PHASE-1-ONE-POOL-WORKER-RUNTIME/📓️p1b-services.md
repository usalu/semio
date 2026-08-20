# P1b — Re-hosting `semio-framework-os-services` onto the `WorkerPool`

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` (~2440 lines after this packet)
and its `📦️packages/🦀️rust/Cargo.toml`. No file outside this crate was edited.

## What was re-hosted

### `TokioHostRuntime` — no thread pool of its own
- Deleted `RuntimeBuildError`, the `tokio::runtime::Builder::new_multi_thread()` construction, and the
  `plan: ThreadPlan, budget: &ThreadBudget` constructor. `TokioHostRuntime` now holds `scopes:
  ScopeTable, pool: WorkerPool` — **no `tokio::runtime::Runtime` object exists anywhere in this crate
  any more**, in any flavor.
- Two constructors: `TokioHostRuntime::new()` (uses `global_worker_pool()`, the crate-wide singleton —
  see below) and `TokioHostRuntime::with_pool(pool: WorkerPool)` (explicit dependency injection, used
  throughout this crate's own tests for deterministic sizing).
- `now_ms`/`sleep_until` delegate straight to `pool.now_ms()`/`pool.timer().sleep_until(..)` (the
  async crate's `TimerWheel` primitive) — no `epoch: tokio::time::Instant` field needed any more, no
  `tokio::time` call anywhere in the trait impl.
- `TokioHostRuntime::block_on` (the R4-sanctioned executor-entry bridge) now delegates to
  `semio_framework_async::block_on` directly instead of `self.runtime.block_on(f)`.

### `global_worker_pool` — the compromise this packet had to make
`ComputePool::new(capacity: u32)`, `HttpPool::new(..)`, `StorageScheduler::new(runtime, scope,
max_in_flight, byte_quota)` and `TimerWheel::new(quota_per_plugin)` are all called from **outside this
packet's boundary** with signatures this packet cannot change (`🔌️plugin/🖥️host`,
`📇️directory/🔌️client` — confirmed by reading their call sites; none of the four take a pool
parameter). Since "the pool becomes the ONLY owner of CPU threads in the process" cannot be achieved by
threading an injected `WorkerPool` through every one of those frozen constructors, this packet
introduces one crate-private lazy static, `global_worker_pool()` (`OnceLock<WorkerPool>`,
`ProcessKind::InteractiveNative`, `available_parallelism().max(4)` workers — the `.max(4)` floor is
deliberate: this crate's dispatched work is I/O-and-blocking-bound, not CPU-bound, so a small
constrained host still gets enough real workers to make progress). Every one of those four
constructors, plus `TokioHostRuntime::new()`'s convenience path, resolves this SAME pool, so "one
process-wide pool" still holds even though it isn't literally injected everywhere. `TokioHostRuntime`
itself — the one constructor this packet fully owns — additionally supports real injection via
`with_pool`. **Honest gap for a follow-up packet**: whoever next touches
`📺️renderer/…/Shell/🧊️component.rs` (already broken by P1a's `ThreadPlan`/`ThreadBudget` deletion, so
already needs a rewrite) should inject a single real, externally-owned `WorkerPool` there instead of
ever falling through to this lazy default.

### `ScopeTable` — same finished/cancelled/leaked semantics, backed by pool jobs
- `ScopeRecord.tasks` is now `Vec<tokio::sync::oneshot::Receiver<TaskOutcome>>` instead of
  `tokio::task::JoinSet<TaskOutcome>` — `tokio::task::JoinSet` needs a `tokio::runtime::Handle` to
  spawn onto, which no longer exists.
- `ScopeTable::spawn_scoped` submits ONE `WorkerPool` job (on `Lane::from_context_lane(ctx.lane)`)
  whose body calls `semio_framework_async::block_on(wrapped)` — the async crate's own module doc
  names this exact pattern as P1b's job ("polling Futures to completion remains the future-polling
  executor's job... built ON TOP of this pool"). The `await_live_or_cancelled` park-gate is unchanged
  in shape, only its sleep source moved from `tokio::time::sleep` to `pool.timer().sleep_until`.
- `ScopeTable::cancel_scope` drains each scope's outcome receivers with a bounded
  `try_recv`-then-`pool.timer().sleep_until(tick)` poll loop (`CANCEL_DRAIN_POLL_MS = 5`) instead of
  `tokio::time::timeout(remaining, tasks.join_next())` — `finished`/`cancelled`/`leaked` accounting is
  identical to before.
- **Honest gap, documented inline on `ScopeTable::spawn_scoped`**: a task that never resolves (an
  infinite loop) occupies its `WorkerPool` worker for the process's entire lifetime — `block_on`
  parks/unparks the SAME worker thread rather than yielding it back to the pool between suspension
  points. This is a real behavior change from tokio's cooperative multitasking. Bounded/short-lived
  tasks (everything this crate's own tests exercise) are unaffected; a long-lived detached task is not
  something this crate currently spawns in production.

### `TimerWheel`/`WheelCore` — pure arithmetic kept, driven by a pool job, not a dedicated thread
- `WheelCore` (arm/disarm/pop_expired/next_expiry_ms, the per-plugin quota accounting) is **completely
  unchanged** — it was already pure, clock-free, tokio-free.
- `TimerWheel::spawn_driver` no longer takes `runtime: &Arc<R>, scope, ctx` (dropped the
  `HostAsyncRuntime` dependency entirely) — signature is now `spawn_driver(&self, pool: &WorkerPool,
  sink: Arc<dyn CompletionSink>)`, submitting ONE job onto `Lane::Timer` (exempt from
  `WorkerPool`'s interactive-admission reserve per the async crate's own `is_low_priority` doc — a
  stalled timer lane would itself block whatever's waiting on it). Internally still races
  `pool.timer().sleep_until(next_expiry)` against `wake.notified()` (the pre-existing
  `tokio::sync::Notify` for "wake early on a fresh `arm()`") via `tokio::select!` — the macro itself
  needs no runtime context, only the future being polled decides that, and neither does now.
- Confirmed via grep that no production caller outside this crate's own tests calls `spawn_driver` (a
  comment in `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:885` explicitly says `SetTimer` uses
  `arm`/`disarm`/`armed_count` directly, NOT this driver), so the signature change carries zero
  external blast radius.
- **Same `block_on`-drives-forever honest gap as `ScopeTable::spawn_scoped`, called out explicitly on
  this method's doc**: on a pool forced to `worker_count == 1`, running BOTH this driver and
  `HttpPool::spawn_refill_driver` would starve the pool completely. Flagged for Phase 9 (real
  cooperative multi-task-per-thread scheduling replacing `block_on`-per-job).

### `HttpPool::spawn_refill_driver` — same shape, now takes `interval_ms`
Same `block_on`-drives-forever-on-`Lane::Maintenance` shape as the timer driver. Signature gained an
explicit `interval_ms: u64` parameter (production callers pass `HTTP_BUCKET_REFILL_INTERVAL_MS =
60_000`, now `#[allow(dead_code)]` since nothing in this crate calls it with that value — the real
caller is a future process bootstrap, out of this packet's boundary) so this crate's own test doesn't
have to wait 60 real seconds for one tick.

### `ComputePool::run_blocking` — this is where `run_blocking` actually got eliminated
- **Kept its exact external signature**: `run_blocking<T: Send + 'static, R: HostAsyncRuntime>(&self,
  runtime: &R, _scope: &ScopeHandle, ctx: OperationContext, work: impl FnOnce() -> T + Send + 'static)
  -> Result<T, ComputeError>` — unchanged from before P1a, because `🔌️plugin/🖥️host` and
  `📇️directory/🔌️client` call it with this exact shape and are outside this packet's boundary.
- Internally: no more `runtime.run_blocking(scope, ctx, work)` (deleted from the trait). Now builds a
  `tokio::sync::oneshot` result channel and calls `self.pool.submit(Lane::from_context_lane(ctx.lane),
  job)` where `self.pool = global_worker_pool()`. The caller `.await`s `result_rx` — **it never blocks
  a worker waiting for the result; only the worker that eventually RUNS `work` is occupied, for exactly
  `work`'s own duration**. This is the literal "opaque `FnOnce` closures the runtime cannot yield,
  cancel or inspect are being eliminated" outcome the packet brief asked for: the work now goes through
  a named `Lane`, is subject to `WorkerPool`'s DRR fairness and admission control, and is visible to
  `WorkerPool::active_workers()`/`occupancy()`.
- Kept its own `tokio::sync::Semaphore`-based admission gate (`capacity`) UNCHANGED — this is a
  logical, per-`ComputePool` concurrency bound independent of (and typically smaller than) the shared
  pool's total worker count; it is what makes `ctx.deadline_ms` racing meaningful and is what the
  `run_blocking_never_exceeds_the_compute_bound_under_a_burst` test still verifies (now against
  `global_worker_pool()`'s real execution, capacity=3, observed max asserted `<= 3` and `>= 2`).

### `StorageScheduler`/`storage_try_dispatch` — `resolve_ready` narrowed, not deleted
- `StorageState<R>` gained a `pool: WorkerPool` field (`global_worker_pool()`, set in
  `StorageScheduler::new` — that constructor's signature is also frozen by
  `🔌️plugin/🖥️host/⏳️imports.rs`).
- `storage_try_dispatch` no longer calls `resolve_ready(state.runtime.run_blocking(..))` (the whole
  reason `resolve_ready` existed) — dispatch is now a direct, synchronous
  `state.pool.submit(Lane::from_context_lane(job.ctx.lane), job_closure)`, since `WorkerPool::submit`
  is plain sync — this actually SIMPLIFIED the dispatch path.
- `resolve_ready` itself is KEPT, narrowed to one remaining use: synchronously reading
  `scope.cancel.is_cancelled()` before running a popped job (a plain atomic load wrapped in `async fn`
  syntax, never truly suspends — same justification the file's own `WheelCore` `E1-adjacent` tags
  already use). This is new behavior, not just a compile fix: previously scope cancellation was
  enforced by `ScopeTable::run_blocking`'s wrapper and never reached `StorageScheduler` at all; now a
  job popped after its scope was cancelled reports `StorageError::Closed` and releases its byte
  reservation, matching what `ScopeTable`'s old wrapper effectively did.

### `EventRouter`/`Mailbox` — compile-fixed for the new `ChannelPolicy` shape, enforcement NOT extended
`ChannelPolicy::LatestWins`/`Coalesced`/`LosslessBounded`/`ByteCredit` all gained `max_bytes` (P1a);
`Coalesced` also gained `max_items`. `Mailbox::new`'s match arms were updated to destructure the new
fields (`LosslessBounded.max_items` → `Mailbox::LosslessBounded.cap`, `ByteCredit.max_bytes` →
`Mailbox::ByteCredit.remaining`, matching the pre-P1a field meaning 1:1). **Deliberately did NOT wire
`max_bytes` into `LatestWins`/`Coalesced`/`LosslessBounded`'s enforcement, or `max_items` into
`Coalesced`'s** — flagged inline as an honest gap: P1b's scope is the async/`WorkerPool` substrate, not
`EventRouter`'s backpressure precision, and every existing test's asserted `PublishOutcome` sequence is
preserved exactly.

## tokio surface: what remains and why

Cargo.toml features narrowed from `["rt-multi-thread", "sync", "time", "macros"]` to `["sync",
"macros"]`. Confirmed by full-file `tokio::` grep (only doc-comment mentions remain):
- `tokio::sync::{oneshot, Semaphore, Notify}` — pure primitives that need **no entered `Runtime`
  context** to function (unlike `tokio::time`/`tokio::spawn`, which panic without one). Used for:
  compute-work result channels (`ComputePool`), the compute-admission semaphore, the storage-job result
  channels, and the timer-driver's early-wake notification.
- `tokio::select!` — a plain manual-polling macro, also runtime-context-free; used for every
  deadline-race (`ComputePool::run_blocking`, `StorageTicket::await_result`) and the timer driver's
  sleep-vs-notify race.
- **Zero `tokio::runtime::Runtime` objects, zero `tokio::task::spawn`/`spawn_blocking`/`JoinSet`, zero
  `tokio::time::*`** anywhere in this crate. `rt`/`rt-multi-thread`/`time` dropped from both
  `[dependencies]` and `[dev-dependencies]`.

## Test suite changes
- Every `thread_plan(N)`/`ThreadBudget::from_plan`/`TokioHostRuntime::new(plan, &budget)` call site
  (≈20) replaced with `TokioHostRuntime::with_pool(test_pool(N))`, where `test_pool` is a small new
  test helper building a `HeadlessBatch`-sized `WorkerPool` directly (deterministic sizing, independent
  of `global_worker_pool()`'s real-core-count default).
- `tokio::time::sleep`/`sleep_until` replaced by a new `sleep_ms(&runtime, ms)` test helper
  (`runtime.now_ms()` + `runtime.sleep_until`).
- `tokio::task::yield_now()` (unavailable without the `rt` feature) replaced by a tiny self-contained
  `Yield` future (wakes itself immediately) — same busy-cooperative-yield behavior.
- `tokio::spawn` (also unavailable) replaced by `std::thread::spawn(move || runtime.block_on(fut))` in
  the one test that used it (`http_pool_rejects_past_the_per_actor_outstanding_cap`).
- The two `ManualRuntime`-based tests (`timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink`,
  `http_pool_refill_driver_actually_refills_a_consumed_bucket_on_its_tick`) were rewritten against a
  REAL `WorkerPool` on the real wall clock (`spawn_driver`/`spawn_refill_driver` no longer accept a
  generic `HostAsyncRuntime` at all) — polling `sink.recorded()`/`remaining_package_budget()` on a 5ms
  tick with a 5s test timeout instead of `ManualRuntime::drive()`/`set_now_ms()`. The refill-driver test
  injects a 40ms `interval_ms` instead of the real 60-second production default. `testkit`/
  `ManualRuntime` dropped entirely from this crate's dev-dependencies (nothing constructs it any more).
- New test `tokio_host_runtime_with_pool_never_resizes_the_injected_pool` (the packet's core
  invariant: this type owns no thread pool of its own).
- Two `ChannelPolicy` field-shape updates in `EventRouter` tests (`LosslessBounded{cap}` →
  `{max_items, max_bytes}`, `ByteCredit{bytes}` → `{max_items, max_bytes}`, `LatestWins`/`Coalesced`
  gained their new required fields).
- Fixed two PRE-EXISTING clippy findings unrelated to this packet's own changes, surfaced only because
  the crate had never compiled clean enough to run clippy before (`AsyncHttpTransport::start`'s
  `type_complexity` — factored into a `StartedTransport` alias; `HttpPool::remaining_package_budget`'s
  `map(..).unwrap_or(..)` → `map_or`), since `cargo clippy --all-targets -- -D warnings` promotes the
  workspace's normally-`warn`-level `clippy::map_unwrap_or`/`clippy::type_complexity` to hard errors.

## Verified commands (this session)
| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-services` | clean, 0 warnings |
| `cargo check -p semio-framework-os-services --all-targets` | clean, 0 warnings |
| `cargo clippy -p semio-framework-os-services --all-targets -- -D warnings` | clean |
| `cargo test -p semio-framework-os-services` | 30/30 passed, re-run 3× stable (~0.13s each) |
| `cargo test -p semio-framework-os-services --release` | 30/30 passed |
| `bun ./📜️script.ts verify dependencies` | clean — 238→238 |
| `bun nx run @semio-tech/os-services-rs:test` | 30/30 passed via nextest, nx target green |

No wasm targets: this crate is native-only by construction (`📦️glue.rs`'s own doc: "no wasm target...
tokio's multi-thread runtime never targets wasm32" — pre-existing, unchanged by this packet), so
`wasm32-unknown-unknown`/`wasm32-wasip2` builds do not apply here.

## Environment note: intermittent file reversion during this session
While editing `🦀️component.rs`, large blocks of already-applied, already-verified edits (the entire
`mod tests` region once, `Mailbox::new`, `HttpPool::spawn_refill_driver`'s signature, two clippy fixes)
reverted to their pre-edit content between tool calls with no action by this agent — `git status`
showed `MM` (both staged AND unstaged modifications) on this file mid-session, consistent with this
repo's documented live/concurrent-dev auto-commit tooling periodically snapshotting and something
replaying stale content over it. Every edit was re-applied and re-verified via a fresh `Read`/`grep`
immediately before the final compile/test/clippy passes reported above all landed in one uninterrupted
sequence with no further reversion observed. Flagging this so a later session isn't surprised if it
recurs, and so nobody mistakes it for this packet's own regression.

## Cross-boundary breakage (confirmed, NOT fixed here — outside this packet's boundary)
- **`semio-framework-plugin-host`**: `cargo check -p semio-framework-plugin-host` fails with 2 errors,
  both `ChannelPolicy` field-shape mismatches P1a's report already flagged: `⚡️effects/🦀️component.rs:281`
  (`LosslessBounded { cap: COMPLETION_MAILBOX_CAP }`) and `:769` (`ChannelPolicy::LatestWins` used as a
  unit value). `:1369` (`Coalesced { key: .. }`) is also still unfixed per a static grep, just not yet
  reached by the compiler's first-error-per-file ordering. Also still has the `ComputePool::run_blocking`
  call sites P1a's report named (`imports.rs:591`, `⚡️effects/🦀️component.rs:995`) — these ARE now
  source-compatible with this packet's `ComputePool::run_blocking` (signature unchanged), so once the
  `ChannelPolicy` sites above are fixed by whoever owns that crate, this specific pair should resolve
  with no further edit needed.
- **`semio-framework-os-kernel-db`**, **`semio-hub`**, **`semio-framework-os-renderer-wgpu`**: not
  independently re-checked this session (P1a's report already catalogued these; nothing in this
  packet's changes touches `HostAsyncRuntime`'s public shape further beyond what P1a already changed,
  so their status should be unchanged from that report).
- **`📺️renderer/…/Shell/🧊️component.rs`**: still broken (pre-existing, from P1a's `ThreadPlan`/
  `ThreadBudget` deletion) AND now also needs a `TokioHostRuntime::new()`/`with_pool(..)` rewrite for
  its constructor call — this is the natural place a future packet should inject a real, externally
  owned `WorkerPool` instead of ever falling through to this crate's `global_worker_pool()` default (see
  the `## What was re-hosted` → `global_worker_pool` section above).

## Files touched
- Rewrote: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml` (tokio
  features narrowed to `sync`+`macros`, `entrypoint` feature added on `semio-framework-async`,
  `testkit` dropped, description updated)
- No other files edited (constraint on blast radius honored).
