# P0a — Interactivity Tracing/Observability Module

## What was created

A new zero-dependency framework module, `semio-framework-trace`, following the exact structural
conventions of `🧰️framework/🔨️modules/⏳️async/` and `🧰️framework/🔨️modules/🎭️actor/`:

- `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs` — the pure domain logic (std-only, no I/O, no
  `wasm_bindgen`/`web_sys`/`tokio`/`std::thread` dependency).
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml` — package `semio-framework-trace`,
  lib name `semio_framework_trace`, `[lints] workspace = true`, **zero dependencies** (no
  `serde`/`thiserror`/`ts-rs`, per the packet's "ZERO new external dependencies" mandate).
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📦️glue.rs` — thin `#[path]` re-export, no
  wasm-bindgen wrapper needed (every type/fn is plain Rust, portable as-is).
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📋️project.json` — nx targets `test`/`test-quick`/
  `test-long`/`test-exhaustive`, mirroring `⏳️async`'s (no `typegen` target — this crate has no
  `ts-rs`, so there is nothing to mirror to TypeScript).
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📜️script.ts` — `bun ./📜️script.ts test` routes to
  `runCargoTestBudgeted(["semio-framework-trace"], …)`, same shape as `⏳️async`/`🎭️actor`'s scripts.
- Registered as a workspace member and as a `[workspace.dependencies]` alias
  (`semio-framework-trace`) in the root `Cargo.toml`, in the same cluster as
  `semio-framework-actor`/`semio-framework-async`.

**launch.json**: checked — neither `⏳️async` nor `🎭️actor` has any per-module entry in
`.vscode/launch.json` (only the repo-wide `⚖️gate🦀️zero-warnings*` gates mention `actor` by name, for
its wasm-target check, and those gates are global/crate-name-agnostic). There was therefore no
existing per-module launch.json pattern to extend, so none was added for `⏱️trace` either — this
matches "follow the existing order, grouping and naming" (there is none to follow at the module
level).

## Public API surface

### Latency instrumentation
- `StepTimer::start(site: &'static str) -> StepTimer` / `CallbackTimer::start(...)` — RAII guards;
  on `Drop` they record elapsed microseconds into that site's `PercentileRing`. No operation context,
  no overrun reporting.
- `INTERACTIVE_STEP_CEILING_US = 8_000` (hard), plus soft targets: `UI_EVENT_SOFT_TARGET_US = 1_000`,
  `UI_PRESENT_SOFT_TARGET_US = 2_000`, `INTERACTIVE_STEP_SOFT_TARGET_US = 1_000`,
  `USER_VISIBLE_SIM_STEP_SOFT_TARGET_US = 2_000`, `BACKGROUND_STEP_SOFT_TARGET_US = 4_000`, each
  addressable via `InteractiveStage::soft_target_us()`.
- `InteractiveStage` enum: `UiEvent | UiPresent | InteractiveStep | UserVisibleSimStep |
  BackgroundStep`.
- `Watchdog::start(site, operation: OperationId, generation: Generation, stage: InteractiveStage) ->
  Watchdog` — RAII guard layered on the same per-site ring; on overrun (`elapsed_us >
  INTERACTIVE_STEP_CEILING_US`) pushes a `ContractViolation { site, operation, generation, stage,
  elapsed_us }` into a bounded (128-entry) global ring. `Watchdog::violations() -> Vec<ContractViolation>`
  and `Watchdog::violation_count() -> u64` are queryable in every build (debug/test and release
  alike) and never panic, even on a poisoned lock. `Watchdog::clear()` resets the store.
- `PercentileRing` — fixed-capacity (64-sample) per-site ring with `record`/`p50`/`p95`/`p99`,
  mirroring `ActorMetrics::wall_us_ring`/`wall_us_p95` in `🎭️actor/🦀️component.rs` (array + sort-on-
  read; no dependency on that crate). `site_percentiles(site: &str) -> Option<(u32, u32, u32)>` reads
  the name-keyed registry `StepTimer`/`CallbackTimer`/`Watchdog` populate.

### Thread ownership
- `register_ui_thread()` / `register_worker_thread(index: u32)` — call once, from the thread itself.
- `current_role() -> ThreadRole` (`Ui | Worker(u32) | Unknown`), `is_ui_thread()` /
  `is_worker_thread()` — cheap, non-panicking, thread-local, available in every build.
- `assert_ui_thread()` / `assert_worker_thread()` — always callable; the check body is a
  `debug_assert!` (compiles away in release, so the assertion itself is debug-gated while the
  function stays available everywhere).

### Counters (lock-free atomics)
- `WorkerCounters` — `worker_started()`/`worker_finished()`/`active()`. `worker_finished` mirrors
  `ThreadBudget::checkout`'s overdraw tripwire in `⏳️async/🦀️component.rs` (`debug_assert!` +
  `wrapping_sub`, never panics in release).
- `PermitLedger` — `acquire()`/`release()`/`occupancy()`, same shape as `WorkerCounters`.
- `QueueCounter` — `enqueued(bytes)`/`dequeued(bytes)`/`snapshot() -> QueueCounterSnapshot { items,
  bytes }`. Per-queue counters are meant to be instantiated by the caller as one `static QUEUE:
  QueueCounter = QueueCounter::new();` per named queue (all three counter types have `const fn new()`
  for this) rather than this crate keeping a name-keyed registry, since that would require a lock and
  this section's requirement is lock-free throughout.

### Operation tracing
- `OperationId(pub u64)`, `Generation(pub u64)`, `TraceId(pub u64)` — local newtypes, deliberately
  NOT the same types as `semio_framework_async::TraceId`/`OperationContext` (same seam discipline as
  that crate's `CapabilityTokenId`). `allocate_operation_id() -> OperationId` hands out
  process-unique ids (never `0`).
- `TraceEvent { operation, generation, sequence, stage: TraceStage, at_us }` in a bounded
  (4096-entry) global ring. `TraceStage`: `Started | StageChanged { label: &'static str } |
  PreviewPublished | Checkpoint | Committed | Cancelled | Failed`.
- Recording fns: `record_operation_started`, `record_stage_changed`, `record_cancel_requested`
  (convenience over `record_stage_changed` with the reserved `CANCEL_REQUESTED_STAGE_LABEL`),
  `record_preview_published`, `record_checkpoint`, `record_committed`, `record_cancelled`,
  `record_failed` — each returns the `TraceEvent` it just recorded.
- `trace_snapshot() -> Vec<TraceEvent>` / `trace_snapshot_for(operation) -> Vec<TraceEvent>`.
- `preview_latency_us(operation) -> Option<u64>` (Started → first PreviewPublished),
  `cancellation_latency_us(operation) -> Option<u64>` (cancel-requested StageChanged → terminal
  Cancelled).

### Clock
- `now_us() -> u64` — monotonic microseconds. Native and `wasm32-wasip2` use an `Instant`-based
  source (same cfg split as `puzzle3d_now_ms`). Plain `wasm32-unknown-unknown` (no OS clock, and this
  crate has zero `js-sys`/`wasm-bindgen` to reach `Date.now()`) falls back to a monotonically
  increasing tick counter — order-correct, not wall-clock-accurate — until the host calls
  `install_clock(fn() -> u64)` once at startup.

## Design decisions later phases should know about

1. **No `async fn` anywhere in this crate**, deliberately breaking with the repo's universal-async
   convention. Rationale is documented at length in the module doc: every public fn is either reached
   from `Drop::drop` (a fixed sync external-trait signature, same E1 class as `CancelToken::fmt`) or
   is a cheap hot-path primitive meant to be callable from any context, including one with no executor
   running yet (the R9 "pure, zero-I/O" class generalized to "any caller", not one closure). This was
   the single highest-risk judgment call in this packet — flagging it explicitly so a later phase
   doesn't "fix" it into `async fn` without re-reading the rationale.
2. **Site lookup is name-keyed and Mutex-guarded** (`PercentileRing` registry, violation ring, trace
   ring) — NOT lock-free. Only the `Counters` section (§3 of the spec) was required to be lock-free,
   and it is (pure atomics, no registry). If a later phase needs lock-free latency recording on a
   truly hot path, that's a new requirement, not a bug in this packet.
3. **`Watchdog` vs `StepTimer`/`CallbackTimer` are deliberately different guards**: the plain timers
   have no operation context and never report violations; `Watchdog` adds `OperationId`/`Generation`/
   `InteractiveStage` and is the only thing that pushes `ContractViolation`s. Both record into the
   same per-site `PercentileRing`, so `site_percentiles()` sees samples from either.
4. **This crate is NOT wired into any other crate yet** (per the packet's explicit scope). No
   `Cargo.toml` outside this module's own and the workspace root references
   `semio-framework-trace`/`semio_framework_trace`.
5. Do not confuse `semio_framework_trace::{OperationId, Generation, TraceId}` with
   `semio_framework_async::{TraceId, OperationContext}` — same names/spirit, deliberately separate
   local types (see the module doc's seam note). Reconciling them (if ever) is a later-phase decision.

## Verified build/test commands and outcomes

All run from `/Users/ueli/Documents/semio` on 2026-08-20.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-trace` | clean, 0 warnings |
| `cargo clippy -p semio-framework-trace --all-targets -- -D warnings` | clean, 0 warnings |
| `cargo test -p semio-framework-trace` | 12/12 passed |
| `cargo nextest run -p semio-framework-trace` | 12/12 passed |
| `cargo check -p semio-framework-trace --target wasm32-unknown-unknown` | clean, 0 warnings |
| `cargo clippy -p semio-framework-trace --target wasm32-unknown-unknown -- -D warnings` | clean, 0 warnings |
| `cargo check -p semio-framework-trace --target wasm32-wasip2` | clean, 0 warnings |
| `cargo metadata --no-deps --format-version 1` (whole workspace) | resolves, includes `semio-framework-trace` once |
| `bun nx run @semio-tech/framework-trace-rs:test` | 12/12 passed via nextest, nx target green |
| `cargo fmt -p semio-framework-trace` | applied; re-verified clean+green after formatting |

Test coverage (all in `component.rs`'s `#[cfg(test)] mod tests`, 12 tests total):
- `percentile_ring_orders_samples_correctly`, `percentile_ring_wraps_past_capacity_keeping_newest`,
  `percentile_ring_empty_reads_as_zero` — ring percentile math.
- `watchdog_reports_contract_violation_on_overrun` (real sleep-based overrun, via the actual `Drop`
  path), `watchdog_stays_silent_under_ceiling` — watchdog overrun detection.
- `thread_role_registers_and_asserts`, `assert_ui_thread_panics_off_ui_thread` — thread-role
  assertions.
- `counters_snapshot_reflects_updates` — counter snapshots (`WorkerCounters`/`PermitLedger`/
  `QueueCounter`).
- `trace_follows_one_operation_start_to_preview_to_commit`,
  `cancellation_latency_measures_requested_to_observed`,
  `latency_helpers_are_none_before_their_events_land` — end-to-end trace by operation id.
- `clock_is_monotonically_non_decreasing` — clock sanity.

Tests avoid cross-test interference on shared global state by using `allocate_operation_id()` for a
fresh id per test (trace/violation queries are always filtered by operation id) and by not depending
on the thread-role default state (each thread-role test explicitly sets the role it then asserts).

## Files touched

- Created: `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs`
- Created: `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml`
- Created: `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📦️glue.rs`
- Created: `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📋️project.json`
- Created: `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📜️script.ts`
- Modified: `/Users/ueli/Documents/semio/Cargo.toml` (added workspace member +
  `[workspace.dependencies]` alias `semio-framework-trace`)
- Created (this file): `📓️p0a-trace-module.md`

## 2026-08-22 Regression Checkpoint

The 4,096-slot bounded trace ring now stores its fixed capacity in a boxed slice. This preserves the
same bounded semantics while avoiding a large lazy inline-array initialization on deeply nested
default-stack dispatch frames. The change fixed the shared Puzzle/Energy viewer-guard stack overflow.
`cargo test -p semio-framework-trace` was rerun after the change and passed **13/13**; the added
I/O-boundary thread-role test accounts for the count increase from the original packet.
