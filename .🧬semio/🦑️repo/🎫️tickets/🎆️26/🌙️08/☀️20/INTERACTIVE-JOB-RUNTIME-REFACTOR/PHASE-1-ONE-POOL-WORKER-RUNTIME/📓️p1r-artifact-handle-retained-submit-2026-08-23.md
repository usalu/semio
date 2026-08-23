# Phase 1r ArtifactHandle Retained Submit

Date: 2026-08-23

## Source Status

This source packet is ready for independent Terra audit. It does not claim Phase 1 acceptance or ticket closure. Cargo, Nx, Wasm, browser, network, root lint, compilation, and runtime timing were intentionally not run under the packet constraints.

## Definition, Caller Census, and Reachability

The removed boundary was `db_engine::ArtifactHandle::submit` in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`. It synchronously called `db_actor::block_on(now_ms())`, submitted an I/O-lane closure, and called `db_actor::block_on(authority.submit_blocking(...))` inside that closure. The authority then reached `ArtifactRunner`, whose one pool closure called `block_on` for every submit/query/frontier/run-query/snapshot/outbox message.

A direct repository census found 17 `ArtifactHandle`-shaped `.submit(...)` callers:

- two in authored Hub source: one live `submit_commands` product/network route and one test;
- two in DB CLI source: one live process-entry profile route and one test;
- seven DB-engine tests, one DB-facade test, and two DB-testkit law calls;
- three stale generated Compose Hub callers, reported but not edited under the packet scope.

No direct plugin caller of `db::ArtifactHandle::submit` exists. Plugin matches named `ArtifactHandle` are the distinct framework-kernel handle. Product/UI reachability is nevertheless proven by the authored Hub request/frame path into `submit_commands`; the DB CLI reaches the same handle at its process entry.

## Retained End-to-End Authority

`SubmitFuture` is now an owned operation rather than a reply channel fed by a blocking closure. Its authority contains:

- 64 fixed generation-keyed operation slots;
- 16 KiB logical pages, 64 pages / 1 MiB maximum per operation, and 1024 pages / 16 MiB process aggregate;
- 256 envelope and 4096 envelope-plus-dependency item ceilings;
- preflight of envelope/dependency Vec backing owners, every identifier/schema String capacity, and both forward/inverse payload capacities before request ownership moves;
- the original Request owner, retained actor ask future, completion, cancellation, progress, rejected closure, retry closure, terminal work/result/job, caller waker, authority generation, and admission credit.

One I/O-lane closure advances exactly one Request-to-actor-future handoff or polls the actor future once. A real weak, generation-tagged `Wake` schedules at most one successor. Pending does not resubmit. Contended/saturated WorkerPool admission retains `error.into_job()` and arms one generation-coalesced callback on the existing pool timer wheel, with eight finite attempts. Shutdown, poison, or retry exhaustion exposes the exact closure; work, result, and the underlying actor-runner terminal job have explicit take/resume/close APIs. `close_step` releases at most one terminal authority.

The operation timestamp now comes from the owned process WorkerPool monotonic clock. The decorative caller-thread `block_on(now_ms())` is absent.

The end-to-end actor route is retained as well. `ArtifactAuthority::submit_retained` constructs the bounded mailbox `AskFuture`. `ArtifactRunner` owns either one build future or one actor-turn future and polls one future once per UserVisible-lane grant. Consuming a mailbox message only constructs the retained turn; its first poll is a successor opportunity. The six former `block_on(engine.*)` arms are absent. Authority query/frontier/run-query/snapshot/outbox methods now await the bounded mailbox future rather than invoking `ask_blocking` inside an async method.

The final terminal-job ownership review also orders runner close before rejected-job destruction. Both authority `close_step` and `ArtifactRunnerTerminalJob::close` upgrade the weak runner while the rejected closure still retains its strong owner, call the one-cursor `finish`, and only then release the closure. This prevents final-closure destruction from bypassing retained engine/build/turn release.

Live database authority construction uses `ArtifactEngine::{create_retained,open_retained}` through the same one-poll runner. The synchronous engine wrappers remain crate-private for the internal DB testkit/process fixtures; no Hub, CLI, database-authority, product, or plugin construction call targets them.

## Source Fixtures and Verifier

Direct Rust fixtures cover:

- late Pending readiness and one-shot wake scheduling;
- quiet pool saturation with exact closure retention and timer-wheel retry;
- cancellation before, during, and after completion ownership;
- stale operation/authority generations and admission-slot ABA;
- missing/closed authority terminalization before mailbox mutation;
- terminal job/work/result/actor-job take, resume, and one-owner close;
- item cap plus one and nested byte cap plus one;
- one retained runner future poll and zero live runner `block_on`.

The existing root `📜️script.ts` verifier now reads both DB engine and artifact production source. Its adversarial corpus rejects an outer submit `block_on`, an inner runner `block_on`, missing nested byte credit, mutation before freshness, a future poll loop, wake-storm duplicate scheduling, saturation without timer retry, missing terminal work retrieval, blocking mailbox admission, a synchronous live authority constructor, and dropping the rejected runner closure before the retained close cursor runs.

## Permitted Gate Results

- Scoped `rustfmt --edition 2021 --check --config skip_children=true` over DB engine and artifact: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS in deny mode. One existing allowlisted renderer process-entry finding remains; this packet contributes zero findings.
- `bun 📜️script.ts verify interactivity`: PASS with the same exact baseline.
- Production region scans: zero `block_on`, `ask_blocking`, `submit_blocking`, private runtime/thread/pool construction, mandatory `WorkerPool::submit`, or poll/drain loop in the live `ArtifactHandle` and `ArtifactRunner` retained regions.
- The old `block_on(now_ms())`, `block_on(authority.submit_blocking(...))`, and mandatory I/O-lane submit patterns have zero matches.
- Scoped working, staged, and HEAD whitespace checks: PASS.
- Whole-repository working, staged, and HEAD whitespace checks: PASS.
- The untracked report was checked independently against `/dev/null`: PASS.

## Explicit Residuals

This packet does not claim that one poll completes within the interactive time ceiling. A compiler-generated engine future may advance through multiple immediately-ready internal states in one poll. More importantly, Fs and SQLite waits ultimately reach P1q's retained `DbIoOperation`, whose one backend `std::fs` or Rusqlite syscall remains an indivisible latency residual. Admission and ownership are bounded; backend syscall duration is not. SQLite remains a Phase 9 owned-event-log residual.

The crate-private synchronous `ArtifactEngine::{create,open}` wrappers still contain the DB actor entry-point bridge for internal testkit/process laws. A fresh caller census found no authored Hub, CLI, database-authority, product, or plugin call to those wrappers. Other pre-existing `db_engine` blocking bridges outside the `ArtifactHandle` submit call graph remain separate Phase 1/runtime-matrix work, including database/catalog setup, VCS integration, compaction, and hello paths.

Generated Compose Hub source remains unreconciled and was explicitly out of scope. Runtime compile evidence, thread census, saturation timing, fairness, cancellation latency under real backend stalls, and interruption ordering remain open. Phase 1 therefore remains open.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `📜️script.ts`
- this report
