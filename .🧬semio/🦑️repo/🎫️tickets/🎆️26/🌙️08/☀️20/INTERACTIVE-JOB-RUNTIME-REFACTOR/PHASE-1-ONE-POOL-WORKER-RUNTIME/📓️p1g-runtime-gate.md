# Phase 1 Runtime Gate Closure Audit

**Date:** 2026-08-21  
**Scope:** Remaining Phase 1 runtime defects and the native thread/process census. Phase 3/5 UI isolation and pack/schema redesign are explicitly excluded.

## Runtime Defects Closed

- `WorkerPool::shutdown()` now advances the shared timer wheel to `u64::MAX` before waking and joining workers. Existing timer waiters are woken, and timers registered during shutdown resolve immediately instead of parking a worker forever.
- `ManualRuntime` now uses task-specific retained wakers backed by an epoch/condition variable. It polls futures outside the task mutex, observes genuine cross-thread completion, preserves concurrent spawns, and still reports deterministic sleeps/pure pending futures as stalled rather than spinning.
- Low-priority admission is now a single atomic compare-exchange reservation. The RAII permit is acquired while the selected queue is locked and is held until the job returns, eliminating the prior check-then-increment race and ensuring the reserved interactive worker is preserved in release builds too.
- The plugin-host epoch-ticker regression test now calls `pool.shutdown()`; its former explanatory shutdown-gap comment is gone.

Regression coverage added in `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`:

- `manual_runtime_drive_observes_a_real_cross_thread_wake`
- `worker_pool_admission_reserves_one_worker_for_interactive_work`
- `worker_pool_shutdown_wakes_an_in_flight_timer_waiter_before_joining`

## Verification

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/framework-async-rs:test` | PASS — 43/43 debug |
| `bun nx run @semio-tech/framework-async-rs:test -- long --release` | PASS — 43/43 release, including release permit/admission checks |
| `bun nx run @semio-tech/os-services-rs:test -- long` | PASS — 30/30 debug |
| `bun nx run @semio-tech/os-services-rs:test -- long --release` | PASS — 30/30 release |
| `cargo test -p semio-framework-plugin-host --lib` | PASS — 141 passed, 0 failed, 1 ignored |
| `cargo test -p semio-framework-plugin-host --lib --release` | PASS — 141 passed, 0 failed, 1 ignored |
| `cargo test -p semio-framework-plugin-host epoch_ticker_starts_and_stops_cleanly_around_a_deadline_bearing_store --lib` | PASS — 1/1 debug |
| `cargo test -p semio-framework-plugin-host epoch_ticker_starts_and_stops_cleanly_around_a_deadline_bearing_store --lib --release` | PASS — 1/1 release |
| `cargo clippy -p semio-framework-async --all-targets -- -D warnings` | PASS |
| `cargo check -p semio-framework-async --target wasm32-unknown-unknown` | PASS |
| `cargo check -p semio-framework-async --target wasm32-wasip2` | PASS |
| `cargo check -p semio-framework-os-services --target wasm32-unknown-unknown` | BLOCKED upstream — 15 existing `semio-framework-actor/📦️glue.rs` errors (`await`/`read_opt`/`wasm_bindgen_futures`); none mention the Phase 1 files |
| `bun ./📜️script.ts verify dependencies` | PASS — baseline 238, current 238 |
| `bun ./📜️script.ts verify interactivity` | WARN/exit 0 — 180 total; 124 blocking bridges, 36 sync-fs, 6 sync-clipboard, 6 sync-process, 8 thread-pool |

The targeted plugin-host tests emit pre-existing warnings in upstream crates; there are no test failures.

## Native Thread and Process Census

The source census used:

```text
rg -n --glob '*.rs' --glob '!target/**' --glob '!**/.🧬semio/**' 'std::thread::spawn|thread::spawn|std::thread::Builder|thread::Builder|tokio::runtime::Builder::new_(multi_thread|current_thread)|rayon::ThreadPoolBuilder|available_parallelism' 🧰️framework
bun ./📜️script.ts verify interactivity
```

### Pool and explicit architecture boundaries

| Site | Cardinality | Classification |
| --- | ---: | --- |
| `⏳️async/🦀️component.rs:1293` `semio-pool-worker-{index}` | `max(1, N-1)` native interactive | The one Semio CPU worker pool |
| renderer wgpu `📦️glue.rs:384` `semio-kernel` | one per native renderer process | Explicit kernel/UI entry thread, not a pool |
| process transport `🦀️component.rs:130` `semio-process-shard-reader` | one per child process | Blocking stdout I/O boundary; trace-registered |
| process transport `🦀️component.rs:256` `semio-shard-stdin-reader` | one in the child process | Blocking stdin I/O boundary; trace-registered |
| process transport `🦀️component.rs:114` `Command::spawn` | one child per `ProcessTransport` | Intentional plugin process boundary, not a CPU pool |

No production `tokio::runtime::Builder`, Rayon pool, shard executor thread, shard forwarder thread, epoch ticker thread, DB submit-bridge thread, or per-request HTTP fetch thread remains in the re-hosted Phase 1 runtime/services/actor/plugin-host paths. The `tokio::runtime::Builder` hit is confined to the `asyncprobe` fixture.

### Residual Semio-owned production threads outside `WorkerPool`

These prevent the literal Phase 1 gate, “exactly UI thread + pool workers,” from closing:

| Site | Cardinality | Finding |
| --- | ---: | --- |
| `🎒️pack/🌐️http/🦀️component.rs:189` | one per armed retry sleep | Unbounded timer thread; reported by the interactivity audit |
| renderer Shell `🧊️component.rs:3305` | one per identity bootstrap | Background identity HTTP thread; reported by the interactivity audit |
| renderer Shell `🧊️component.rs:3363` | one per open directory stream | Long-lived stream driver thread; reported by the interactivity audit |
| store sync `🦀️component.rs:1544` | one lazy supervisor per `ArtifactHost` | Dedicated `LocalSet` supervisor for `!Send` document actors |
| DB artifact `🦀️component.rs:1184` | one per `ArtifactAuthority` | Dedicated `!Send` document actor thread |
| DB actor `🦀️component.rs:730` | one per default spawner invocation, when native `thread` feature is enabled | Production fallback still creates an OS thread per actor |

The audit additionally reports the repo CLI test harness at `⌨️cli/…/📦️glue.rs:759` and four `asyncprobe` fixture sites. Those are not native interactive runtime paths. Test-only thread spawns in the async, services, DB-actor, renderer, and shard-executor test modules are likewise excluded from the production count.

## Gate Decision

The Phase 1 runtime defect gate is met: shutdown, cross-thread manual wakeups, atomic admission, release permit checking, the shared pool, and removal of subsystem CPU pools are verified in debug and release.

The full Phase 1 exit gate **cannot honestly close yet**. The literal census requirement is false because six Semio-owned production thread sites remain outside `WorkerPool`, in addition to the explicit kernel and blocking-I/O boundaries. Three of the six are already machine-reported by `verify interactivity`. Re-hosting them would broaden this packet into the Phase 3 renderer/identity work or unresolved `!Send` DB/store architecture, so they are recorded as blockers rather than silently reclassified.

`ComputePool::run_blocking` also retains the already-recorded opaque `FnOnce` API. It submits onto `WorkerPool` rather than creating a pool/thread, but its capability-token redesign remains Phase 3 work and is not a new blocker introduced here.
