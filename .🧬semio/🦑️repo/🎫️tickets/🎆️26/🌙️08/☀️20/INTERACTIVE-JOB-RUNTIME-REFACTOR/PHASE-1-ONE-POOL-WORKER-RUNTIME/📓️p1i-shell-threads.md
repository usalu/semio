# P1i Shell Identity and Directory Threads

Date: 2026-08-21

## Outcome

The two assigned Shell production threads are removed. Identity bootstrap and directory streaming now use the renderer's one injected `WorkerPool`; neither path creates an OS thread, builds a local executor, calls a blocking receive, nor pins a worker in a forever loop.

The owned production census is zero. The repository-wide literal census still contains separately owned production sites, so this packet alone cannot honestly close the literal Phase 1 gate; exact remaining sites are classified below.

## Architecture

### Identity bootstrap

- `ShellPoolFuture` stores a `Send` future and polls it once per `Lane::Io` pool turn with a retained `Wake` implementation.
- The 5-second deadline and the Shell cancellation root travel in `OperationContext`.
- `mint_or_restore` remains asynchronous and preserves cached/offline identity semantics. HTTP blocking work is admitted through `HttpPool`/`ComputePool::run_io` on `Lane::Io`; the UI observes only `try_recv`.
- `ShellState::drop` cancels the root and drops any retained bootstrap future.
- Shell construction no longer uses `block_on`: `TokioHostRuntime::open_scope_now`, `ComputePool::with_pool`, `HttpPool::new_now`, and `NativeDirectoryTransport::with_new_http_pool_now` provide pure setup paths over the renderer pool.

### Directory stream

- Native `DirectoryTransport`/`DirectoryWsConnection` are `Send`-capable; browser transports retain platform-local marker traits without weakening the native contract.
- WebSocket receive is synchronous nonblocking `try_recv_text`; native sockets are put in nonblocking mode after a deadline-aware `connect_timeout`/TLS dial.
- `DirectoryStream::turn` is an owned state machine returning `Dial`, `Message`, `ReconnectAt`, `Idle`, or `Closed`. It skips at most eight invalid/control frames per turn, tracks the last event/head sequence, and reconnects from that sequence.
- `ShellDirectoryRunner` executes at most 32 messages or 4 ms per turn, performs dials as separate `Lane::Io` jobs, and uses `TimerWheel` wakeups for 8 ms socket polling and reconnect deadlines.
- Output is ordered in a 256-entry bounded queue. When full, the runner stops consuming the socket; draining from the UI schedules forward progress, so events are backpressured rather than dropped.
- Shell drop cancellation closes the socket and makes every scheduled runner turn terminal.

### Shared pool wiring

`ComputePool::with_pool` was added because the old `ComputePool::new` selected the services singleton even when Shell had injected `renderer_worker_pool()`. Shell now constructs its directory runtime, scope, compute admission, HTTP pool, identity bootstrap, socket dials, stream turns, and timers over the same renderer pool.

`tokio-tungstenite` enables `rustls-tls-webpki-roots` so the synchronous finite-dial implementation preserves `wss://` support.

## Tests added or updated

- Directory reconnect resumes from the latest sequence after a short outage.
- Ordered event delivery across finite stream turns.
- A stream turn completes below the 8 ms ceiling in the deterministic fake transport.
- Already-cancelled and in-flight request cancellation.
- Sticky stream closure after cancellation.
- Retained-waker future forward progress without changing worker cardinality.
- Sleeping future cancellation and bounded pool shutdown.

The directory tests were updated from the deleted forever-stream API to explicit finite turns. Their latest-tree test binary is currently masked by unrelated Phase 1.5 stale-await errors in store/DSL tests before these tests execute.

## Verification

Passing:

```text
cargo check -p semio-framework-os-kernel --features sync,ureq --lib --message-format=short
Finished dev profile; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --lib
Finished dev profile in 6.40s; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --features sync,ureq --release --lib --message-format=short
Finished release profile in 18.45s; exit 0 (warnings only).

cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --release --lib --message-format=short
Finished release profile in 8.89s; exit 0 (warnings only).

bun './🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts' check
Finished dev profile in 1m 20s; exit 0 (warnings only).
```

Attempted, currently blocked outside P1i:

```text
cargo test -p semio-framework-os-kernel --features sync,ureq directory::client::tests --lib
Blocked during unrelated store/DSL test compilation by stale `.await` on synchronous Phase 1.5 APIs.

cargo check -p semio-framework-os-renderer-wgpu --lib --message-format=short
Reached dependencies after the earlier directory trait drift was repaired, then stopped before Shell
in the active stdio Phase 1.5 repair wave (2,848 errors in the latest run, including MP4 corruption).
```

Release and Bun task-surface results are appended after their current shared-build lock completes.

## Thread and blocking census

The raw repository command output is `🧪️p1i-thread-census.txt`; the targeted output is
`🧪️p1i-owned-census.txt`, both in this ticket.

Targeted production result:

```text
rg -n 'std::thread::spawn|thread::Builder|block_on|recv\(' Shell/🧊️component.rs directory/🔌️client/🦀️component.rs
```

- Identity/directory production sites: **0**.
- Shell matches at the end of the file are test-only `pollster::block_on` and two explicit Send-boundary tests.
- Directory matches are documentation or nonblocking `try_recv`.

Repository-wide production spawn classification from the fresh census:

- `⏳️async/🦀️component.rs`: the intended `WorkerPool` worker constructor; all other matches are tests.
- renderer `📦️glue.rs:384`: dedicated `semio-kernel` thread, owned by the active renderer/actor-job packets, not P1i.
- plugin process transport: two blocking child-pipe reader threads, explicitly registered I/O boundaries.
- repo CLI `📦️glue.rs:759`: per-client blocking Unix-domain-socket reader boundary; outside this Phase 1 product-runtime packet and not yet registered in the shared census.
- procedural WFC parallel engine `🦀️component.rs:47`: scoped CPU threads; outside P1i and not yet re-hosted on `WorkerPool`.
- remaining matches in services, DB actor, async, pack, renderer Shell, plugin executor, and fixtures are test/fixture-only or documentation.

Therefore the two Shell residuals named by P1h are closed, but the strongest literal repository-wide "UI thread plus WorkerPool workers only" wording remains blocked by the renderer-kernel and procedural-WFC CPU sites unless their concurrent owners remove them; the two process/CLI blocking-I/O boundaries require explicit registration/acceptance.

## Files

- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`

## Scratch policy

All diagnostics are `.txt` inside the Phase 1 ticket. No script was added. `find PHASE-1-ONE-POOL-WORKER-RUNTIME -type f -name '*.log'` returns no results.
