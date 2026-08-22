# P1l Process-Wide Pool Enforcement

Date: 2026-08-21

## Outcome

The production pool migrations are now enforced as one exact process singleton instead of a convention. `process_worker_pool` records the first `WorkerPoolConfig`, returns handles to the same underlying pool, and rejects a later subsystem request whose process kind, core count, or interactive-reserve contract differs. The rejection is an `assert_eq!`, so it remains active in release builds.

The repository-wide thread/pool audit is now in `deny` mode and reports zero production thread/pool constructors outside the sanctioned async-runtime root. Its only current finding is the permanent native renderer process-entry `block_on` bridge.

## Production Callers

The following production owners resolve their pool through `process_worker_pool`:

- hub database process entry
- native OS host activation
- database CLI process entry
- renderer root and scale environment
- MCP workspace
- plugin host
- services runtime
- browser backbone worker

Each native interactive caller derives `cores` from `available_parallelism().map_or(1, NonZeroUsize::get)` and requests `ProcessKind::InteractiveNative`. Headless and browser-worker process entries establish their own process-local headless contract.

Direct `WorkerPool::new` calls outside the async-runtime crate remain only in test-gated sources. The deny audit strips those test items before applying the repository-wide constructor rule.

## WASM Clock Parity

The cooperative WASM pool now exposes `now_ms`, matching the native public surface used by services. Its value is the latest host time passed to `pump`, starts at zero before the first pump, and advances monotonically even if a host submits a regressing sample. The effective monotonic value also drives the timer wheel.

This fixes the seven WASM service diagnostics introduced when production services moved from private pools to the process singleton. It does not invent a WASM clock or read platform time inside the async crate.

## Finite Timer, Maintenance, and Future Turns

The process pool no longer spends a worker waiting for a delayed submission. `WorkerPool::submit_at` stores a callback in the owned timer wheel; only the due callback enqueues the finite closure. Native and WASM wheel pumping handles at most 32 timer actions per pool turn, including stale heap entries, so cancellation churn cannot turn a timer scan into an unbounded worker step.

The services timer driver and HTTP refill driver are finite chains on `submit_at`. The timer driver processes at most eight due entries per turn. HTTP refill advances one shared epoch in O(1); individual package buckets observe that epoch lazily, avoiding both a permanently occupied worker and an unbounded map scan.

`TokioHostRuntime::spawn_scoped` now uses `PoolFutureTask`: one future poll per finite worker closure. A pending future releases the worker, lane permit, and admission slot and is re-enqueued only when its waker fires. This replaces the former `block_on`-for-the-entire-future closure. The future contract still requires each individual `poll` implementation to return cooperatively; arbitrary code that never returns from `poll` cannot be preempted by Rust's `Future` ABI.

## Verification

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/framework-async-rs:test` | PASS — 49/49 debug |
| `SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun nx run @semio-tech/framework-async-rs:test -- --release` | PASS — 49/49 release |
| `cargo clippy -p semio-framework-async --all-targets -- -D warnings` | PASS |
| `bun nx run @semio-tech/os-services-rs:test` | PASS — 39/39 debug |
| `SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun nx run @semio-tech/os-services-rs:test -- --release` | PASS — 39/39 release |
| `cargo clippy -p semio-framework-os-services --all-targets -- -D warnings` | PASS |
| `cargo check -p semio-framework-os-services --target wasm32-unknown-unknown --lib` | PASS |
| `cargo check -p semio-framework-os-services --target wasm32-wasip2 --lib` | PASS |
| `bun ./📜️script.ts verify interactivity` | PASS — deny mode, zero thread/pool findings |
| `git diff --check -- <async and services packet files>` | PASS |

The first release async invocation used the fundamental target's default 15-second build budget and was killed during compilation. It was rerun with the explicit 120-second build/test budgets shown above and passed; this was a harness budget failure, not a test failure. The current post-finite-turn matrix is the 49-test result recorded above.

## Interactive Compute Boundary Closure

The remaining opaque-compute boundary identified by this report is closed. `ComputePool::run_blocking`
has been deleted, both production plugin-host consumers create explicit `InteractiveJob`s, and the
service submits one bounded job step per `WorkerPool` closure with cancellation/deadline/terminal
propagation. Blocking platform I/O remains separately named and classified. Current evidence and the
remaining repository-wide non-compute caveats are in `📓️p1m-interactive-compute-closure.md`.
