# P1f — Epoch Ticker + Process-Transport Threads: Closing the Last Semio-Owned Plugin-Host Threads

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/` (crate root `🦀️component.rs`, `⏳️runtime.rs`,
`⚡️effects/🦀️component.rs`, `🧵️shard/🚚️process-transport/🦀️component.rs`, `🧵️shard/👶️child/🦀️main.rs`),
plus a small, necessary addition to `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs` (a new `ThreadRole`
variant the task explicitly required and the current API could not express).

## 1. Epoch ticker — arrangement and what the owned interpreter must take over

### Before
`EpochTicker::start(engine: &Engine)` (`🦀️component.rs`, was ~line 181) spawned a dedicated
`"semio-epoch-ticker"` OS thread: `loop { sleep(1ms); engine.increment_epoch(); }`, joined on `Drop`.
Two call sites: `WasmtimeRuntime::new` (crate root) and `AsyncEngineHandle::new` (`⏳️runtime.rs`).

### After
`EpochTicker::start(engine: &Engine, pool: &WorkerPool) -> Self` now drives the tick through a new
`PeriodicPoolTimer` helper (`🦀️component.rs`, region `⏲️PeriodicPoolTimer`) submitted onto the pool's
`Lane::Timer`. Both call sites now pass `&plugin_host_worker_pool()` — a new crate-local
`OnceLock<WorkerPool>` singleton (region `🧵️PluginHostWorkerPool`), same idiom
`semio-framework-os-services`' `global_worker_pool`/`🌎️hub`'s `hub_worker_pool` already established
for this exact "the repo hasn't converged on ONE pool handle per process yet" situation — chosen
specifically so `WasmtimeRuntime::new`'s and `AsyncEngineHandle::new`'s PUBLIC signatures stay
unchanged (no ripple into `🏃️run`, `🌉️mcp/🏠️workspace`, `📺️renderer` — the sibling packet's own
territory — or the `semio-shard` child binary's `main`).

**`PeriodicPoolTimer` mechanism — NOT a copy of the services crate's `spawn_driver`/`spawn_refill_driver`
shape**, deliberately: those two submit ONE job that `block_on`-loops forever, permanently pinning a
`WorkerPool` worker for the process's entire lifetime — an accepted cost their own `🚨️ HONEST GAP` doc
documents, because each is a true process-wide SINGLETON (exactly one `TimerWheel`/`HttpPool` ever
exists). `EpochTicker` is NOT a singleton — `WasmtimeRuntime::new` runs once per caller, and this
crate's own test suite alone builds dozens of concurrent `WasmtimeRuntime` instances across parallel
`#[semio_framework_async_macros::async_test]`s, all sharing `plugin_host_worker_pool()`. A first draft
copied the loop-forever shape and it is a genuine correctness bug in that context: once the number of
concurrently-alive `EpochTicker`s exceeds the pool's `worker_count`, every ticker beyond that count
would queue forever behind ones that never release their worker — silent, permanent epoch-interruption
failure for those instances, not merely a slower tick. The landed design instead RESUBMITS a fresh,
short-lived job for every single tick (`PeriodicPoolTimer::schedule`): one job = one `block_on(pool
.timer().sleep_until(deadline))` wait, one `tick()` call (`engine.increment_epoch()`), one resubmission,
then the job returns — releasing its worker every ~1ms instead of holding it forever. Under contention
(many concurrent tickers on a small pool) this degrades tick cadence gracefully instead of starving
outright. `EpochTicker::drop` sets an `AtomicBool` the next scheduled tick observes (before AND after
its wait) and stops resubmitting — bounded by one `EPOCH_TICK_INTERVAL_MS` (1ms), fire-and-forget since
`WorkerPool` has no job-join primitive to wait on synchronously (unlike the old thread's
`JoinHandle::join`).

**`SEMIO_PLUGIN_HOST_WORKER_COUNT`**: `plugin_host_worker_pool()`'s sizing normally reads
`std::thread::available_parallelism()` (matching `global_worker_pool`'s own default), but this
crate-local env var (read once, at first construction) overrides it. `👶️child/🦀️main.rs` sets it to
`"1"` at the very top of `main`, before `WasmtimeRuntime::new` (the first thing that touches the pool)
runs. Reasoning: an out-of-process shard child's own `ShardLoop::pump` runs directly on that process's
main thread, never submitted to this pool — the pool's ONLY tenants there are the epoch ticker and
`StdioTransport`'s heartbeat sender (§2), two sub-millisecond periodic jobs. Sizing it to
`available_parallelism()-1` (potentially many cores on a real host) would spin up that many OS threads
PER SHARD CHILD PROCESS for work that needs exactly one worker — multiplying total host thread count by
however many shard processes are running, for zero benefit. No other caller (renderer, `🏃️run`, MCP
gateway) sets this variable, so they keep the full-parallelism default.

### `📓️p1c-actor-shards.md`'s proposed design vs. what actually landed
P1c's §3 proposed "a job that calls `engine.increment_epoch()` then re-registers
`wheel.sleep_until(now_ms + 1)` before returning" — evaluated, not followed verbatim: `TimerWheel::
sleep_until`'s returned `TimerSleep` cancels its own registration on `Drop` (removes the wheel's
`entries` map slot AND drops the pending `Waker`), and `TimerWheel`'s registration primitives
(`register`/`update_waker`/`forget`) are private to the async crate — reachable only through the public
`sleep_until` Future. Polling that Future once (`Pending`) and letting the local variable go out of
scope before the deadline fires immediately deregisters it (the tick would silently never fire again).
The only leak-free way to keep it alive across the wait, using ONLY the public API, is to hold it on an
active stack frame — i.e. `block_on`. So the "job returns immediately, wheel wakes it later" half of
P1c's proposal isn't achievable without either a leak (mem::forget, unbounded `TimerWheel::entries`
growth) or a new private-API surface in the async crate (out of this packet's boundary). What DID land
is a hybrid: `block_on` for the wait (correct, leak-free), but a FRESH job per tick rather than one
`block_on`-driven infinite loop (correct under concurrent multi-instance load — see above). `TimerWheel`/
`Lane::Timer` are exactly the primitives P1a intended for this, used as intended.

### What the owned WASM interpreter must take over
Wasmtime's epoch interruption today is the ENTIRE enforcement mechanism for `Budget.wall_ms`:
`store.set_epoch_deadline(budget.deadline_ms)` plus a periodic `engine.increment_epoch()` (now a pool
job instead of a thread, but the SAME mechanism) is the only thing that can preempt a guest call
mid-execution. Three properties a later, repo-owned WASM interpreter must reproduce or deliberately
replace:
- **Granularity**: epoch interruption only fires at wasm-bytecode safe points wasmtime itself chooses —
  coarse-grained, "eventually" preemption (bounded by the tick interval PLUS however long the current
  safe-point interval runs), never instruction-precise. It cannot interrupt a tight loop mid-instruction
  the way fuel metering's per-instruction check can.
- **What breaks if simply removed**: `store.set_epoch_deadline`/`config.epoch_interruption(true)` with
  no ticker means the epoch counter never advances — `WasmtimeRuntime::execute_turn`'s own trap-message
  sniffing (`lowered.contains("epoch") || lowered.contains("interrupt")` → `TurnFault::DeadlineExceeded`,
  `🦀️component.rs` ~line 1275) would never fire; a runaway/malicious guest call inside ONE `execute_turn`
  would run to actual completion (or the `consume_fuel` ceiling, a SEPARATE, coarser bound already also
  enforced) rather than being cut off at its wall-clock budget — the ONE thing standing between a slow
  guest turn and a frozen interactive process today.
- **What the interpreter replaces it with**: instruction-level fuel metering — the interpreter's own
  fuel-decrement-and-check loop (every instruction dispatch already checks) replaces the epoch ticker's
  periodic wakeup entirely; no OS-level timer is needed to force a check. `Budget.wall_ms`'s CURRENT
  role (an upper bound enforced from OUTSIDE the guest, via a ticking clock) shifts to `Budget.fuel`
  being the PRIMARY, precise ceiling, with wall-clock time becoming a secondary/derived signal. `Lane::
  Timer`/`TimerWheel`/`PeriodicPoolTimer` stay useful regardless — they are not interpreter-specific,
  only the epoch-ticker's specific 1ms-poll CONSUMER of them goes away (real host-side deadline/timeout
  primitives, e.g. Phase 2's `StepContext.deadline`, keep using them).

## 2. Process-transport threads — decision and thread count

`🧵️shard/🚚️process-transport/🦀️component.rs` had three `thread::Builder` sites (post-rename, still at
roughly the same relative positions): `ProcessTransport::spawn`'s reader (parent side, blocking read off
the child's stdout), `StdioTransport::new`'s reader (child side, blocking read off its own stdin), and
`StdioTransport::new`'s heartbeat sender (child side, periodic sleep+write).

**Decision, split by what each thread actually does:**

- **The two READER threads stay real OS threads — genuine PLATFORM I/O BOUNDARIES.** Both do a real,
  blocking `std::io::Read::read_exact` off an OS pipe (`ChildStdout`/`stdin()`), with no non-blocking
  alternative reachable without a new dependency — this crate deliberately owns no async-I/O crate
  (`tokio` here is `["sync", "rt"]` ONLY, no `net`/`process`/`io-util`, per its own Cargo.toml comment:
  "tokio itself stays OWNED by `semio-framework-os-services`... this crate must never construct a
  `tokio::Runtime`"). Reaching for `tokio::process`/`mio`/`async-process` to make these non-blocking
  would be a real new third-party dependency for two sites — out of proportion. Each is bounded: EXACTLY
  ONE per `ProcessTransport`/`StdioTransport` instance, i.e. one per child process (parent side) or one
  per process (child side) — never per-message, never unbounded by shard count alone. Both now call
  `semio_framework_trace::register_io_boundary_thread(site)` as their first statement — a NEW
  `ThreadRole::IoBoundary(&'static str)` variant added to the trace crate (§3) specifically because the
  existing `Ui | Worker(u32) | Unknown` roles couldn't express "a counted, justified I/O thread that is
  NOT a `WorkerPool` worker" without corrupting the pool's own worker-index accounting. Neither reader
  performs domain work — each only decodes a length-prefixed frame and pushes raw bytes into a
  `VecDeque`; no actor turn execution, no effect dispatch, nothing beyond "get bytes off the wire".

- **The heartbeat sender moved to `Lane::Timer` — it is a periodic sleep+write, not a blocking read.**
  `StdioTransport`'s `_heartbeat: JoinHandle<()>` field is now `_heartbeat: super::PeriodicPoolTimer`
  (the same mechanism §1's `EpochTicker` uses, reused via `super::` since `process_transport` is a
  submodule of the crate root `component` module — no API export needed). It calls `super::
  plugin_host_worker_pool()` internally (already constructed in this process by the time
  `StdioTransport::new` runs — `main` builds a `WasmtimeRuntime`, which starts its own `EpochTicker` on
  this same singleton, before ever opening the transport) and submits on `Lane::Timer` at the caller's
  own `heartbeat_interval_ms`.

**Resulting thread count per process, this crate's own contribution:**
- **Parent process** (hosting N out-of-process shard children): 1 `"semio-process-shard-reader"` thread
  PER CHILD (bounded by shard count, registered `IoBoundary`) — no change in count from before P1f,
  only now census-visible and documented as justified rather than silently present.
- **`semio-shard` child process** (one per out-of-process shard): main thread (unregistered — the
  process's own entry point, running `ShardLoop::pump` directly, never submitted to any pool) + 1
  `"shard-stdin-reader"` thread (registered `IoBoundary`) + exactly 1 `WorkerPool` worker (registered
  `Worker(0)`, sized via `SEMIO_PLUGIN_HOST_WORKER_COUNT=1` — hosting both the epoch ticker's and the
  heartbeat sender's `Lane::Timer` jobs, time-sliced). **Net: the dedicated `"semio-shard-heartbeat"`
  thread is GONE (3 threads → 3 threads, but now exactly "1 I/O boundary + 1 pool worker" instead of "1
  I/O boundary + 2 ad hoc periodic threads" — matching the exit gate's shape, not just its letter).**

## 3. `ChannelPolicy` field-shape fixes (P1a's `LosslessBounded`/`Coalesced`/`LatestWins` now bound both
items and bytes)

Three sites in `⚡️effects/🦀️component.rs`, confirmed at current line numbers (shifted from P1a's
`~281/769/1369` catalogue by other packets' concurrent edits landing in between):
- `EnvelopeCompletionSink::ensure_subscribed` (~line 281): `LosslessBounded { cap: COMPLETION_MAILBOX_CAP }`
  → `{ max_items: COMPLETION_MAILBOX_CAP, max_bytes: COMPLETION_MAILBOX_MAX_BYTES }` — new sibling
  constant `COMPLETION_MAILBOX_MAX_BYTES: u64 = 1_000_000`, same "generous safety bound, not a
  steady-state throttle" rationale as the existing item cap.
- `Effect::Subscribe`'s handler (~line 769, guest-controlled generic pub/sub): `LatestWins` (unit-like)
  → `LatestWins { max_bytes: 1_000_000 }`.
- A backbone test (~line 1369): `Coalesced { key: .. }` → `Coalesced { key: .., max_items: 100,
  max_bytes: 1_000_000 }` — values match `semio-framework-os-services`' own equivalent test constants
  for consistency (the underlying `Mailbox::Coalesced` doesn't yet enforce `max_items`, so the number
  itself is inert today, matching that crate's own precedent rather than inventing a new one).

Also found and fixed while getting the crate to compile (not in P1a's original catalogue — either
introduced or exposed by intervening changes): two `ShardKind::Thread` references in this crate's own
test module (`🦀️component.rs`, `Kernel::new(ShardKind::Thread, ..)` ×2) left over from P1c's `ShardKind
::Thread → ShardKind::Native` rename; and `crate::EpochTicker::start(&engine)` in `⏳️runtime.rs`'s
`AsyncEngineHandle::new` (a second call site P1a's/P1c's own catalogues didn't name), updated to the new
two-argument signature alongside the crate-root call site.

## 4. Watchdog wiring and guest turn paths exceeding 8ms

**Already landed by P1c** (`🧵️shard/🦀️component.rs`, `execute_turn_for`/the job-step loop) — verified
still correctly wired, not re-done here: both wrap their guest call in a `semio_framework_trace::
Watchdog::start(site, OperationId(actor_id), Generation(..), stage)` guard, `stage` derived from the
actor's `Lane` via `interactive_stage_for`. Nothing in P1f's own changes touches this mechanism.

**Which paths exceed 8ms** — re-confirmed by the same static reasoning P1c's report gave (lane budget
constants, `semio_framework_actor::lane_defaults::budget_for`, are UNCHANGED by this packet too):
`INTERACTIVE_STEP_CEILING_US` is a flat 8ms regardless of lane. `Lane::Interactive`'s own grant (4ms
wall) stays under the ceiling by construction. `Lane::UserVisible` (16ms grant, 2× the ceiling),
`Lane::Background` (50ms, 6.25×), and `Lane::Maintenance` (200ms, 25×) can each trip the watchdog once a
turn spends close to its full granted budget — a property of the EXISTING budget constants, not
something P1f (or P1c) caused. No live workload was run through this packet's crates either (see §6 —
this packet's own test run is the first time `semio-framework-plugin-host`'s full suite has compiled and
executed in this wave, but the specific `execute_turn`/`step_job` paths under real guest load were not
independently exercised here). **For Phase 2's job protocol**: `execute_turn`/`step_job` remain the only
two guest-call sites in this crate running under a lane-derived wall-clock budget; `UserVisible`/
`Background`/`Maintenance` are the three lanes to target for internal resumability, `Interactive` is
lower priority (already inside the ceiling).

## 5. Trace crate addition: `ThreadRole::IoBoundary`

`🧰️framework/🔨️modules/⏱️trace/🦀️component.rs`'s `ThreadRole` enum had exactly `Ui | Worker(u32) |
Unknown` — no way to register a bounded, justified, non-pool I/O thread distinctly from a real
`WorkerPool` worker (reusing `Worker(N)` for the process-transport readers would either collide with
real pool worker indices or corrupt whatever a census tool infers from `Worker(_)` counts as "pool
size"). Added `ThreadRole::IoBoundary(&'static str)` (the site name, e.g. `"process-shard-reader"`) plus
`register_io_boundary_thread(site)`/`is_io_boundary_thread()`, mirroring the existing `Ui`/`Worker`
pair's shape exactly. One new test (`io_boundary_thread_registers_distinct_from_worker_and_ui`); 13/13
tests pass (was 12/12); clippy clean on native + `wasm32-unknown-unknown` + `wasm32-wasip2`.

## 6. Test results — including a real, pre-existing bug this packet's fix exposed for the first time

`cargo check -p semio-framework-plugin-host --all-targets`: **clean, 0 errors** (this is itself notable
— P1a's and P1c's own reports both recorded this command as BLOCKED upstream by `semio-framework-os-
services`'s `ChannelPolicy` errors, "never reaches this packet's own files"; P1f's fixes are what
finally let it compile end to end, for the first time in this wave).

`cargo test -p semio-framework-plugin-host --lib`: **125 passed, 2 failed, 1 ignored — in `--debug`,
reproduced identically across two separate runs (once under the default parallel harness, once isolated
under `--test-threads=1`).** `cargo test -p semio-framework-plugin-host --lib --release`: **127 passed,
0 failed, 1 ignored — the SAME two tests pass in `--release`.** This split is itself the confirming
evidence for the root cause below: a genuine WALL-CLOCK RACE between a real background `WorkerPool`
thread finishing its work and `ManualRuntime::drive()`'s single, no-op-waker poll pass checking for it —
`--release`'s lower overhead apparently lets the background thread win that race more often, but a
race a build profile can flip is not a fix; it means CI reliability for these two tests depends on
`--release` vs `--debug`, not on anything deterministic. Neither failing test's code path touches P1f's
own edits (none of the three `ChannelPolicy` sites, the epoch ticker, or the process-transport threads).
Root cause, traced precisely:

- Both tests build their `AsyncEffectExecutor` over `semio_framework_async::testkit::ManualRuntime` (a
  deterministic `HostAsyncRuntime` test double, async crate) and drive it via `runtime.drive()`.
- `ManualRuntime::drive()`'s own doc says exactly what it does: "Polls every not-yet-finished task ONCE
  with a NO-OP WAKER, repeating until a full pass makes no further progress" (`⏳️async/🦀️component.rs`,
  `impl ManualRuntime`). It never re-polls a task whose Waker fires from OUTSIDE that single pass.
- Both failing tests exercise an `Effect` that routes through `dispatch_router_effect`
  (`⚡️effects/🦀️component.rs` ~line 984), which calls `compute.run_blocking(..)` —
  `semio_framework_os_services::ComputePool::run_blocking`, P1a/P1b's OWN redesign: it submits the real
  work onto `global_worker_pool()` (a REAL, multi-threaded `WorkerPool`, real OS threads) and `.await`s
  a `tokio::sync::oneshot::Receiver` that a DIFFERENT, real background thread eventually fires.
  `dispatch_router_effect`'s future is pushed via `spawn_scoped` into `ManualRuntime`'s task list, NOT
  awaited inline.
- `runtime.drive()` polls that task ONCE. The real background `WorkerPool` thread has not had time to
  actually run the closure and fire the oneshot sender yet (a genuine, ~always-true race on any real
  OS scheduler) — the poll returns `Pending` under a no-op waker, `drive()` sees "no progress" on that
  pass and returns immediately. The real thread DOES eventually complete the work and calls `.wake()` —
  on the no-op waker, which discards it. The task is now permanently abandoned: nothing ever polls it
  again, so the router handler recorded as "never called" (`left: 0`) and the completion sink never
  receives it (`left: 1` instead of the expected `2`).
- Confirmed this is specific to the `ManualRuntime` + real-`ComputePool` combination, not `ComputePool`
  itself: `semio-framework-os-services`'s OWN `ComputePool::run_blocking` tests
  (`run_blocking_never_exceeds_the_compute_bound_under_a_burst` etc.) use `TokioHostRuntime::with_pool`
  and a real `async_test`-driven `.await` chain — a genuinely capable executor, not `ManualRuntime`'s
  single-pass/no-op-waker double — and are unaffected by this class of bug.

**This is real, pre-existing debt from the P1a→P1b `ComputePool` redesign (replacing the old fully-
synchronous `HostAsyncRuntime::run_blocking` with a genuinely cross-thread, `WorkerPool`-backed one),
spanning `ManualRuntime` (async crate, P1a's boundary — NOT this packet's) and `ComputePool` (services
crate, P1b's boundary — NOT this packet's), surfaced for the first time by THIS packet's own fix making
`semio-framework-plugin-host`'s full test suite compile and run for the first time in this wave.** Not
fixed here — a real fix needs either `ManualRuntime` gaining a way to be re-driven when an external
Waker fires (a design change to a shared test double many other crates depend on), or `AsyncEffectExecutor
`'s own test suite gaining a fully-synchronous `ComputePool` test double instead of routing through
`global_worker_pool()`'s real threads at all. Flagged here precisely rather than silently patched or
silently left unmentioned.

`cargo clippy -p semio-framework-plugin-host --all-targets --no-deps -- -D warnings`: every finding in
P1f's OWN new/changed lines is clean (verified by grepping clippy's own line numbers against this
packet's diff after each fix). The invocation as a whole still reports ~108 pre-existing findings spread
across `🧵️shard/🦀️component.rs`, `🧵️shard/🏃️executor.rs`, and thousands of lines of `🦀️component.rs`
this packet never touched (`result_large_err` on `TransactionError`/`PluginHostError`/`Fault`,
`await_holding_lock` in `ShardTransport::send` impls, `SharedThreadTransport`/`LoopbackTransport`
visibility, etc.) — same shape P1a/P1c/P1d's own reports already documented for their own crates
("pre-existing... left as-is"). Full breakdown available via `cargo clippy -p semio-framework-plugin-host
--all-targets --no-deps -- -D warnings` if a future packet wants to work through it.

`bun ./📜️script.ts verify dependencies`: clean — 238 → 238.

`cargo check -p semio-framework-plugin-host --target wasm32-unknown-unknown`: **not applicable — this
crate cannot target wasm32 at all**, confirmed by attempting it (`errno` crate, a `wasmtime` transitive
dependency, hard-fails to compile for `wasm32-unknown-unknown`: "The target OS is unknown or none... 
unsupported by the errno crate"). This is correct and expected: `semio-framework-plugin-host` is the
NATIVE HOST that runs wasm components via wasmtime — it is host-only by design, not wasm-gated code, and
has no `#[cfg(target_arch = "wasm32")]` branches anywhere in the crate. `semio-framework-trace` (the one
crate this packet also touched) DOES build clean on both `wasm32-unknown-unknown` and `wasm32-wasip2`
(§5).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — `plugin_host_worker_pool`,
  `PeriodicPoolTimer`, `EpochTicker` rewrite (both call sites), `ChannelPolicy`-adjacent unnecessary-
  qualification cleanup, `ShardKind::Thread → Native` (2 test sites), the epoch-ticker test rewritten
  onto a `WorkerPool`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` — `AsyncEngineHandle::new`'s
  `EpochTicker::start` call site updated to the new signature.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` — three
  `ChannelPolicy` field-shape fixes + `COMPLETION_MAILBOX_MAX_BYTES` constant.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs` —
  both reader threads registered `IoBoundary`; heartbeat sender moved onto `Lane::Timer` via
  `PeriodicPoolTimer`; `now_ms()`'s pre-existing `map_unwrap_or` clippy finding fixed (shifted into view
  by this packet's own edits above it).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs` — sets
  `SEMIO_PLUGIN_HOST_WORKER_COUNT=1` before the pool is first touched.
- `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs` — new `ThreadRole::IoBoundary` variant +
  `register_io_boundary_thread`/`is_io_boundary_thread`, one new test.

## Cross-boundary findings for the coordinator (not fixed here)

- **`WorkerPool::shutdown()` can deadlock against an in-flight `TimerWheel::sleep_until` waiter**
  (`semio-framework-async`, P1a's boundary). Reproduced directly while writing this packet's own epoch-
  ticker test: `shutdown()` sets `inner.shutdown` then joins every worker thread; any OTHER worker that
  was idle-parked wakes, re-checks the `while !inner.shutdown` loop condition FIRST, and exits WITHOUT
  calling `TimerWheel::fire_due` again. A worker currently parked inside `block_on(sleep_until(..))`
  (waiting on exactly that `fire_due` call to wake it) can then never be woken — `shutdown()`'s own
  `.join()` on that thread hangs forever. This packet's own test worked around it by simply not calling
  `pool.shutdown()` (same "HONEST GAP" shape `semio-framework-os-services`' `spawn_driver`/
  `spawn_refill_driver` already document, for a sharper reason — see the test's own comment). A real fix
  belongs in the async crate: either `shutdown()` keeps calling `fire_due` on every worker until all are
  joined, or it fires every pending `TimerWheel` waker directly before joining.
- **`ManualRuntime` (async crate) is structurally incompatible with anything bridging through a real
  `WorkerPool`** — see §6. Affects any test, in any crate, that spawns a `ManualRuntime`-scoped task
  whose future eventually depends on a REAL cross-thread wakeup (not just this crate's two failing
  tests — any future caller of `ComputePool`/`HttpPool`/`StorageScheduler` from `ManualRuntime`-driven
  test code would hit the identical bug). Flagged for the coordinator to route to whichever packet owns
  `ManualRuntime`'s design.
- **`semio-framework-plugin-host`'s pre-existing clippy debt** (§6, ~108 findings under `--no-deps -D
  warnings`, none touched by this packet) — a real, separate cleanup task if the repo wants this crate
  clippy-clean under `-D warnings`, out of proportion to a threading-closure packet.
