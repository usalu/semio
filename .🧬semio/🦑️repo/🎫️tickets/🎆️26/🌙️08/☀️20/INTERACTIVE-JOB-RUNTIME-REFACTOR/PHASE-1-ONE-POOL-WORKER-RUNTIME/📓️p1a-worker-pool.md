# P1a — WorkerPool: the One Process-Wide Worker Pool (async crate)

Scope: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` (owner file, ~1490 lines after this packet;
wired into the crate via `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📦️glue.rs`'s `#[path]`) and
`🧰️framework/🔨️modules/⏳️async/✨️macros/` (macros crate, one pre-existing clippy fix only). No file
outside these two module trees was edited — see "Broken call sites" below for what every OTHER crate
now needs.

## Public API landed

### `ProcessKind` / `worker_count_for`
```rust
pub enum ProcessKind { InteractiveNative, HeadlessBatch }
pub fn worker_count_for(process_kind: ProcessKind, cores: usize) -> usize
```
`max(1, cores-1)` for `InteractiveNative` (UI/OS thread keeps the remaining core), `cores` for
`HeadlessBatch`. Never zero. Process kind is passed explicitly at `WorkerPoolConfig::new` — never
inferred.

### `Lane`
```rust
pub enum Lane { Interactive, UserVisible, Background, Maintenance, Io, Timer }
```
`Interactive`/`UserVisible`/`Background`/`Maintenance` mirror `🎭️actor::Lane`'s discriminant order
and `weight()` VERBATIM (8/4/2/1) — read-only reference, no dependency added (actor → async stays
the only allowed direction). `Io`=4, `Timer`=3 are new lanes with no actor-crate analogue, covering
the OS-thread work Phase 0's census found with no actor equivalent (HTTP fetch threads, DB storage
blocking I/O, the epoch ticker's replacement). `Lane::from_context_lane(u8)` maps
`OperationContext.lane`'s existing bare-`u8` convention onto this enum for `0..=3`; out-of-range
values (this crate's own `Io`/`Timer`) fall back to `Background` since a caller wanting those submits
to `WorkerPool::submit` with an explicit `Lane` rather than via an `OperationContext`.

### `PermitLedger` (replaces `ThreadBudget` — deleted, no shim)
```rust
pub struct PermitLedger { .. }
pub fn checkout(&self, n: u32) -> Result<PermitGuard<'_>, PermitError>
pub fn remaining(&self) -> u32
pub fn occupancy(&self) -> u32
```
Checked compare-exchange loop — `Err(PermitError{requested, remaining})` on over-allocation, in
EVERY build profile including `--release` (test `permit_ledger_checked_in_release_too` runs under
both `cargo test` and `cargo test --release`, no `cfg(debug_assertions)` guard anywhere). This closes
the Phase 0 gate-report defect: `ThreadBudget::checkout` used `fetch_sub` + `debug_assert!`, so a
release build silently wrapped to a huge value on over-draw. `occupancy()` is backed by
`semio_framework_trace::PermitLedger` (new dependency, workspace-internal, not a new third-party
dep — `bun ./📜️script.ts verify dependencies` confirmed 238→238). `WorkerPool` owns one `PermitLedger`
sized to its `worker_count`; `WorkerPool::active_workers()` (backed by a separate
`semio_framework_trace::WorkerCounters`, incremented/decremented around actually running a job) and
`WorkerPool::occupancy()` (the ledger) are exposed as two distinct named signals per the packet spec,
numerically coupled today (1 job = 1 permit = 1 active worker) but backed by different trace types so
either can be read independently.

`ThreadPlan`, `thread_plan()`, `ThreadBudget`, `ThreadRole` are DELETED — no compatibility shim, per
the greenfield-repo rule.

### `TimerWheel` (the epoch-ticker thread's replacement)
```rust
pub struct TimerWheel { .. }
pub fn sleep_until(&self, deadline_ms: u64) -> TimerSleep<'_>   // a Future<Output=()>
pub fn next_deadline_ms(&self) -> Option<u64>
pub fn fire_due(&self, now_ms: u64) -> u32
```
Min-heap of `(deadline_ms, id)` guarded by one `Mutex`; a `sleep_until` future registers a `Waker`
on first poll (or resolves immediately if the wheel's last-known `now_ms` already reached the
deadline) and is woken only by `fire_due`. Never reads a clock itself — same "caller supplies
`now_ms`" discipline as `HostAsyncRuntime::now_ms`. On native, every idle-parked `WorkerPool` worker
calls `fire_due(pool.now_ms())` each time it wakes (park timeout capped at `MAX_IDLE_PARK_MS = 4ms`,
clamped by the wheel's own `next_deadline_ms()`), so no dedicated OS thread exists for timers — the
epoch-ticker `"semio-epoch-ticker"` 1ms-poll-loop thread the Phase 0 census found has a direct
replacement mechanism here, though the actual wasmtime-epoch-callback rewiring is NOT in this
packet's blast radius (plugin host crate — a sibling packet). On wasm, `WorkerPool::pump(now_ms)`
calls `fire_due` with the host-supplied time each call.

### `HostAsyncRuntime` trait surgery
- `run_blocking` REMOVED entirely (was: `fn run_blocking(&self, scope, ctx, work: Box<dyn FnOnce()+Send>) -> impl Future<Output=()>`).
  No replacement method — callers now submit directly to a `WorkerPool` on whichever `Lane` fits
  (typically `Lane::Io` or `Lane::Background`).
- `open_scope`/`spawn_scoped`/`sleep_until`/`cancel_scope`/`now_ms` unchanged.
- `ChannelPolicy` extended so every variant bounds items AND bytes:
  ```rust
  pub enum ChannelPolicy {
      LatestWins { max_bytes: u64 },                                  // item bound implicitly 1
      Coalesced { key: String, max_items: u32, max_bytes: u64 },
      LosslessBounded { max_items: u32, max_bytes: u64 },              // was `{ cap: u32 }`
      ByteCredit { max_items: u32, max_bytes: u64 },                   // was `{ bytes: u64 }`
  }
  ```

### `block_on` — gated behind `entrypoint` feature
```rust
#[cfg(any(test, feature = "entrypoint"))]
pub fn block_on<F: Future>(fut: F) -> F::Output
```
Same shape as the pre-existing `testkit`/`ManualRuntime` gate: always compiled under `cfg(test)` for
this crate's own suite, otherwise only when a downstream `Cargo.toml` opts in with
`features = ["entrypoint"]` (CLI binaries, test/testkit consumers). `semio_framework_async_macros::
async_test`'s generated `#[test]` harness is UNAFFECTED — it already carries its own self-contained
inline `block_on` copy (verified in `✨️macros/🦀️component.rs`, predates this packet) specifically so
the macro crate never links this one; nothing here changes that.

### `WorkerPool` — the one process-wide CPU substrate
Same public surface on every target (native, `wasm32-unknown-unknown`, `wasm32-wasip2`):
`new(WorkerPoolConfig)`, `submit(Lane, Job)`, `worker_count()`, `active_workers()`, `occupancy()`,
`permits()`, `timer()`, `shutdown()`; native additionally has `now_ms()`, wasm additionally has
`pump(now_ms) -> bool` and `has_pending_work()`.

**Native** (`#[cfg(not(target_arch = "wasm32"))]`): one `std::thread` per worker
(`worker_count_for`-sized), each with its own 6-lane array of `Mutex<VecDeque<Job>>` (no lock-free
Chase-Lev deque, no external crate — a `Mutex` per (worker, lane) pair, correct and cheap enough at
whole-closure job granularity). Own-queue selection is deficit-round-robin: `Lane::ALL` scanned in a
persistent rotating cursor, each present lane accrues `weight()` per scan, popped once its deficit
reaches `Lane::Interactive::weight()` (=8, the unit cost) — so `Interactive` runs every scan while
`Maintenance` (weight 1) accrues 1/8 as fast and is serviced roughly every 8 scans WITH pending work,
never zero (see `lane_weights_never_starve_the_lowest_lane`,
`worker_pool_lane_fairness_background_cannot_starve_interactive`). When a worker's own queues are
empty it steals from siblings' queues front-to-back in `Lane::ALL` order, starting just past its own
index (`worker_pool_work_stealing_moves_work_between_workers`). Idle workers park on a shared
`Condvar` with a timeout bounded by `MAX_IDLE_PARK_MS`/the timer wheel's next deadline, so a 1-worker
pool still wakes promptly and cooperatively rather than blocking the process
(`worker_pool_sizing_multi_core_and_forced_single_core` covers the forced-1-core case).

**Admission control** (`worker_pool_admission_control_keeps_an_interactive_slot_free`): a real runtime
constraint, not a comment. `PoolInner::admit_low_priority()` returns `false` — meaning
`Background`/`Maintenance` lanes are skipped in THIS scan (their deficit is not consumed, so they
still get serviced once a slot frees) — whenever `interactive_reserve` is on (default for
`ProcessKind::InteractiveNative`, off for `HeadlessBatch`), `worker_count >= 2`, and
`low_priority_active >= worker_count - 1`. Both the DRR selector and the steal scanner honor this
gate, so low-priority work can never occupy every worker while the reserve is active. `Io`/`Timer`
are deliberately EXEMPT from the reserve (a stalled I/O/timer lane can itself block the interactive
work waiting on it).

**WASM** (`#[cfg(target_arch = "wasm32")]`, covers both `wasm32-unknown-unknown` and
`wasm32-wasip2` — no OS-thread branch is attempted on either): a single-logical-worker cooperative
scheduler behind a `Mutex<SchedulerState>`. `pump(now_ms)` fires due timers then runs AT MOST one
DRR-selected job, returning whether more work remains, so the host (the browser Web Worker running
this WASM module — see `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`, NOT wired to this crate in
this packet) controls the cadence between pump calls. Admission control is a documented no-op here
(single worker — the spec's "when there are 2+ workers" clause). `catch_unwind` is called uniformly
in both branches; on `wasm32-wasip2`, which the workspace `Cargo.toml` already documents as
`panic = "abort"` by target-spec default, a panicking job aborts the whole module instance regardless
— `catch_unwind` is a no-op there, not a defect, and is exactly why the interactive job protocol
landing in Phase 2 should treat "job panics" as a `Fault` outcome rather than relying on real Rust
unwinding on that target.

## Test coverage (29 tests, `cargo test -p semio-framework-async`)
Sizing (multi-core + forced 1-core), work stealing across workers, lane fairness (saturated
background cannot starve interactive — bounded wait, not "never runs"), admission control (interactive
slot stays free under a maintenance-lane flood), permit ledger over-allocation `Err` in BOTH debug and
`--release`, timer-wheel ordering (`fire_due` order-independent of registration order) and
`sleep_until` resolving only after `fire_due` reaches the deadline, active-worker/occupancy reflecting
real running-job state, FIFO determinism within one worker's own lane, plus every pre-existing
`CancelToken`/`Scope`/`ManualRuntime`/`block_on` test unchanged.

## Verified commands (run 2026-08-20, this session)
| Command | Result |
|---|---|
| `cargo check -p semio-framework-async --all-targets` | clean, 0 warnings |
| `cargo clippy -p semio-framework-async --all-targets -- -D warnings` | clean |
| `cargo test -p semio-framework-async` | 29/29 passed |
| `cargo test -p semio-framework-async --release` | 29/29 passed (proves the release permit-checked test) |
| `cargo test -p semio-framework-async --features entrypoint,testkit` | 29/29 passed |
| `cargo test -p semio-framework-async --features typegen exports_typescript_bindings` | 1/1 passed |
| `cargo check -p semio-framework-async --target wasm32-unknown-unknown` | clean, 0 warnings |
| `cargo clippy -p semio-framework-async --target wasm32-unknown-unknown -- -D warnings` | clean |
| `cargo check -p semio-framework-async --target wasm32-wasip2` | clean, 0 warnings |
| `cargo clippy -p semio-framework-async --target wasm32-wasip2 -- -D warnings` | clean |
| `cargo test -p semio-framework-async-macros` | 8/8 passed (one pre-existing clippy fix: `syn::Error::new_spanned(&input.sig.fn_token, ..)` → drop the `&`, `needless_borrows_for_generic_args`) |
| `cargo fmt -p semio-framework-async -p semio-framework-async-macros` | applied, re-verified clean+green |
| `bun ./📜️script.ts verify dependencies` | clean — 238→238 (the new `semio-framework-trace` dependency is workspace-internal, not third-party) |
| `bun nx run @semio-tech/framework-async-rs:test` | 29/29 passed via nextest, nx target green |

## Broken downstream call sites (for the coordinator to dispatch — NOT fixed here, out of this packet's blast radius)

Confirmed via `cargo check -p <crate>` against the new API. Two crates (`semio-framework-os-kernel-db`
and its dependents) showed a much larger, FLUCTUATING error count across repeated runs a few minutes
apart (54 errors → 2 errors, all the extra ones about `.execute()`/`.prepare()`/`?` on
`impl Future<Output=MutexGuard<rusqlite::Connection>>` — nothing to do with any type this packet
touched) — that is a live concurrent session's in-progress edit to the sqlite storage bridge, not
this packet's breakage; only the `run_blocking`-shaped errors below are attributable to P1a.

**`semio-framework-os-services`** (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`) — 9 errors:
- `:43` — `use ...{ThreadBudget, ThreadPlan, ThreadRole}` unresolved import (also `:1548` imports `thread_plan` in the test module)
- `:235-269`, `:268` — `TokioHostRuntime::new(plan: ThreadPlan, budget: &ThreadBudget)` and its doc — the whole construction path needs redesigning onto `WorkerPool`/`PermitLedger` (this is P1b's job per the phase plan)
- `:299-320` — `impl HostAsyncRuntime for TokioHostRuntime` has a `run_blocking` fn (`:308`) that no longer satisfies the trait (E0407) — must be removed from the impl or turned into an inherent method that submits to a `WorkerPool` instead
- `:600-618` — `ComputePool::run_blocking` (the crate's own wrapper) calls `runtime.run_blocking(...)` (`:618`, E0599) — this is the ONE bridge function ~10 other call sites in this same file (`:811`, `:982`, `:1212`, `:1677`, `:1705`, `:2196`, `:2235`, plus the plugin-host crate's `:591`/`:995`) depend on; fixing this one fn fixes all its callers' source, they don't need individual edits
- `:1341-1344` — `ChannelPolicy` match arms destructure the old `cap`/`bytes` field names (E0533/E0026/E0027 ×4) — needs updating to `max_items`/`max_bytes`
- `:2011-2062` — test call sites constructing `ChannelPolicy::LatestWins`/`LosslessBounded{cap:..}`/`Coalesced{key:..}`/`ByteCredit{bytes:..}` need the new field shape
- ~30 more test call sites (`:1548-2316`) construct `ThreadBudget::from_plan(thread_plan(N))` — all need replacing with `PermitLedger`/`WorkerPool` test setup

**`semio-framework-os-kernel-db`** (db crate) — 2 errors once concurrent churn settled:
- `🗄️storage/🦀️component.rs:1576` — a test double `impl HostAsyncRuntime for InlineRuntime` has a `run_blocking` fn that no longer satisfies the trait (E0407) — delete it from the impl
- `🗄️storage/🦀️component.rs:140` (inside `run_blocking_op`, defined at `:131`) — calls `runtime.run_blocking(...)` (E0599) — this ONE bridge fn is what ~60 call sites throughout `db_storage`'s `🦀️component.rs` and `🪶️sqlite/🦀️component.rs` depend on; fixing `run_blocking_op` itself (submit to `WorkerPool` instead) fixes all of them, they do not need individual edits

**`semio-hub`** (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`) — not independently confirmed by a green build (its
db-crate dependency currently fails to compile for the unrelated reason above), but static grep
confirms `:1552` has a `fn run_blocking(&self, ..)` inside an `impl HostAsyncRuntime for ..` block —
same E0407 shape, same fix (delete from the impl).

**`semio-framework-os-renderer-wgpu`** (renderer) — not independently confirmed by a green build (its
`semio-s-plugin-puzzle` build-script dependency currently fails for an unrelated, pre-existing reason
— `build.rs` calling `.join()` on a `Future` and using `async fn main`, nothing to do with this
packet). Static grep confirms two more sites that will need fixing once that's resolved:
- `🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:50` — imports `thread_plan`/`ThreadBudget`; `:1318-1326` constructs its own `plan`/`budget` from them for GPU-thread sizing — needs redesigning onto `worker_count_for`/`WorkerPool`
- `🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:166` — `semio_framework_async::thread_plan(cores).shards as u16` — needs a replacement sizing call (likely `WorkerPool::worker_count()` or a fresh `Lane`-aware constant, this crate's call — P1b/renderer packet's decision)

**`🔌️plugin/🖥️host/⏳️imports.rs:591`** and **`⚡️effects/🦀️component.rs:995`** — call
`call.services.compute.run_blocking(...)` / `compute.run_blocking(...)`, i.e. `ComputePool::run_blocking`
— fixed transitively once the services-crate bridge above is fixed, no separate edit needed.

**`🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`** — `ChannelPolicy` construction sites needing the new
field shape: `:281` (`LosslessBounded { cap: .. }`), `:769` (`LatestWins` as a unit-like use — now a
struct variant), `:1369` (`Coalesced { key: .. }`).

**Call-site count for the coordinator: 3 crates independently confirmed broken
(`semio-framework-os-services`, `semio-framework-os-kernel-db`, and transitively the plugin-host
crate's two `run_blocking` call sites), 2 more statically confirmed but blocked on unrelated
pre-existing build failures upstream (`semio-hub`, `semio-framework-os-renderer-wgpu`). Total distinct
`run_blocking`-trait-impl sites to delete: 3 (`TokioHostRuntime`, db's `InlineRuntime`, hub's runtime
impl). Total `run_blocking`-bridge functions to redesign onto `WorkerPool`: 2 (`ComputePool::run_blocking`
in services, `run_blocking_op` in db_storage) — fixing each fixes all of ITS OWN callers for free.
Total `ChannelPolicy` construction/match sites needing the new field shape: 2 files
(`services/🦀️component.rs`, `plugin/host/⚡️effects/🦀️component.rs`).**

## What Phase 2's job protocol should know
- `WorkerPool::submit(Lane, Job)` is the ONE place CPU work enters the substrate now — the job
  protocol's `InteractiveJob::step` should be submitted as a `Job` closure on whichever `Lane` matches
  the operation's priority (`Lane::from_context_lane` bridges `OperationContext.lane` today).
- `TimerWheel::sleep_until` is the deadline primitive to build `StepContext.deadline` timeouts on.
- A worker-loop job that panics is caught (`catch_unwind`) and swallowed on native (unwind-panic
  targets) but ABORTS THE WHOLE PROCESS on `wasm32-wasip2`/most wasm configurations
  (`panic = "abort"`) — Phase 2 jobs must convert their own panics into a `Fault` `StepOutcome` rather
  than relying on this crate's `catch_unwind` to protect the pool on every target.
- `WorkerPool::now_ms()` (native only) is available as a ready-made monotonic clock source a future
  `HostAsyncRuntime` implementation can delegate `now_ms()` to, instead of owning a second clock.
- Admission control today only distinguishes `Background`/`Maintenance` from everything else; Phase 2
  may want a finer per-operation-priority reserve, but the current two-tier gate is the literal
  packet spec ("background/maintenance lanes must not occupy every worker while a UI is live").

## Files touched
- Rewrote: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
- Modified: `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-trace`
  dependency, added `entrypoint` feature, updated crate description)
- Modified: `🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️component.rs` (one pre-existing clippy fix,
  `needless_borrows_for_generic_args`, unrelated to the API change but blocked `-D warnings`)
- No other files edited (constraint on blast radius honored).
