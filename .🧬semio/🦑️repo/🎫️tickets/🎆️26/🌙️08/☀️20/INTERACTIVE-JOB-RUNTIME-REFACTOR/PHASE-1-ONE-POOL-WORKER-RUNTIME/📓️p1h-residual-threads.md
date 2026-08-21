# P1h Residual Non-Shell Threads

Date: 2026-08-21

## Scope and outcome

P1h removed all four assigned Semio-owned production thread sites. The owned source files now contain zero production `std::thread`/`thread::Builder` spawns. The only literal spawn matches in those files are two DB actor tests and one pack HTTP historical doc reference.

This packet does not claim that the repository-wide Phase 1 thread gate is closed. A fresh global census still finds production Shell identity/bootstrap and directory-stream threads, plus separately classified renderer-kernel and process-I/O/platform boundaries and a repo CLI thread. Those sites were outside P1h ownership.

## Implemented architecture

### Pack HTTP retry timer

- Deleted the custom sleep future that created one OS thread per retry delay.
- Added injected `RetryRuntime { pool, now_ms }`; retry waits use the shared `WorkerPool` `TimerWheel`.
- `HttpPackSource::new` and `with_retry_policy` now require the runtime, and the pack root explicitly reexports it.
- Tests inject a bounded process pool, shut it down, and cover timer cardinality.

### Store sync supervisor

- Deleted the per-document supervisor thread and embedded `LocalSet` loop.
- Replaced the forever loop with finite `ActorRunner` turns: at most 32 commands plus one nonblocking hub poll, reconnect/filesystem deadlines, drain, and status publication.
- A single atomic scheduled flag prevents duplicate concurrent turns. Timer wakeups use the shared `WorkerPool` `TimerWheel`; no timer job pins a worker.
- WebSocket dialing remains on the ambient Tokio reactor as an explicit cancellable platform-I/O task and returns through a channel. It does not create a Semio-owned actor or timer thread.
- Direct host commands/presence/close schedule a new turn immediately. Socket writes are bounded by a 4 ms timeout.
- Repaired the compiler-identified stale `.await`/`resolve_ready` fallout in this module while preserving the concurrent synchronous contracts.

### DB artifact authority

- Replaced the dedicated authority thread with an injected-pool `ArtifactRunner`.
- Each scheduled turn consumes at most one mailbox message, then explicitly reschedules if work remains. The job never parks waiting for mailbox input.
- Preserved engine/build ownership, panic-to-failure delivery, terminal state, cancellation, readiness, and mailbox completion semantics.

### Feature-gated DB actor default spawner

- Removed `ThreadSpawner`, `StdThreadSpawner`, join-handle ownership, and the `thread` Cargo feature/default.
- `Supervisor` now requires an injected `Arc<WorkerPool>`.
- `ActorRunner` processes at most 32 messages or 4 ms per turn on `Lane::UserVisible`, with an atomic one-job scheduling gate, consumer wake callback, cancellation, restart, and drop handling.
- Converted the persistent map's shared nodes from `Rc` to `Arc` so the actor state is cleanly `Send`; no compatibility layer was added.

## ArtifactHost pool injection

`ArtifactHost::new` universally requires a shared `Arc<WorkerPool>`. All current callsite families are wired:

- OS host tests use their shared test pool.
- Renderer Shell injects `renderer_worker_pool()`.
- MCP workspace uses one process-wide `workspace_worker_pool()` and declares the direct async-framework dependency.
- Store tests use their test pool.
- The wasm store worker owns a zero-thread cooperative pool and pumps it per request.

The exact callsite census was:

```text
rg -n 'ArtifactHost::new\(' 🧰️framework
```

Every result passes a pool argument; no zero-argument constructor remains.

## Verification

Passing:

```text
cargo check -p semio-framework-pack --all-features --lib
Finished dev profile; exit 0.

cargo check -p semio-framework-pack --all-features --release --lib
Finished release profile in 10.93s; exit 0.

cargo check -p semio-framework-os-kernel --lib
Finished dev profile; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --release --lib
Finished release profile in 7.73s; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --features sync --lib
Finished dev profile; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --features sync --release --lib
Finished release profile in 18.86s; exit 0 (warnings only).
```

The DB library passed after the actor/pool conversion before the concurrent Phase 1.5 de-async wave changed its reachable dependencies. A later latest-tree DB recheck is not green due to the unrelated de-async cascade described below; it must be rerun after Phase 1.5 stabilizes.

## Thread census

Command:

```text
rg -n 'std::thread::(spawn|Builder)|thread::spawn|thread::Builder|StdThreadSpawner' \
  🧰️framework/🔨️modules/🎒️pack/🌐️http/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs
```

Exact residual matches:

```text
db/🎭️actor/🦀️component.rs:1073  test-only blocked-sender thread
db/🎭️actor/🦀️component.rs:1117  test-only channel sender thread
pack/🌐️http/🦀️component.rs:306  historical documentation reference
```

Therefore the P1h production count is zero. `StdThreadSpawner` has zero repository matches.

The repository-wide literal census still finds out-of-scope production thread sites, most importantly Shell identity bootstrap and Shell directory streaming. The renderer kernel thread and plugin process transport threads are explicitly classified platform/process boundaries. Repo CLI also retains a thread. Fixture and test spawns are not production runtime sites. Consequently the full Phase 1 gate cannot honestly close on P1h alone.

## Current unrelated blockers

- `cargo test -p semio-framework-pack --all-features --no-run` fails in concurrent Phase 1.5 code at pack IO line 301 (`write_varint_u64(...).await`) and pack format lines 944/1013 (`crc32c(...).await`). None is in the P1h HTTP retry implementation.
- Store test compilation is blocked by the broader concurrent DSL/store trait async-to-sync contract wave. Production OS kernel libraries compile in debug, release, native, and wasm.
- `cargo check -p semio-framework-os-mcp --lib` reaches through the repaired kernel and then fails in unrelated mesh-engine calls where `append_glb_node` and `append_glb_mesh` futures are used with `?` from synchronous code.
- The latest-tree DB check is blocked by the concurrent de-async cascade across engine, CLI, preview, storage, and reachable dependencies; the earlier focused DB pass is not represented as a latest-tree pass.

## Scratch policy

All captured diagnostics remain inside this Phase 1 ticket as `.txt` files. `find PHASE-1-ONE-POOL-WORKER-RUNTIME -type f -name '*.log'` returns no results.
