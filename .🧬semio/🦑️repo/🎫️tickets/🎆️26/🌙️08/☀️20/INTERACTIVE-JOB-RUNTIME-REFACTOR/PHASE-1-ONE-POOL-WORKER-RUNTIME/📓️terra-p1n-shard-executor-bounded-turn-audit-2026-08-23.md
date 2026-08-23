# Terra Audit: P1n ShardExecutor Bounded-Turn Handoff — 2026-08-23

## Verdict

**REJECT** for this narrow source packet.

The preceding pool-closure block_on, blocking receive, and epoch drain loop were
removed from the live ShardExecutor::run path. Formatting, the root source
audit, its five adversarial fixtures, and all requested whitespace checks pass.
Those successes do not overcome three current source blockers:

1. A retained pending drive has a no-op waker and is immediately resubmitted,
   which permits a worker-pool spin instead of suspension retention.
2. A rejected try_submit closure is stored but has no retry/wakeup path when
   ingress becomes quiet, so a contended or saturated first admission can strand
   its work indefinitely.
3. The new deferred registration, event, job-step, and cancellation owners are
   uncapped, and retained registrations are drained wholesale in one worker
   closure.

This is source-only rejection. No conclusion is made about compilation, runtime
behavior, timings, worker census, or Phase 1 acceptance.

## Scope and Snapshot

Read AGENTS.md, the preceding Phase 1/2 readiness audit, the implementation
report 📓️p1n-shard-executor-bounded-turn-handoff-2026-08-23.md, and the live
production source/diff. No source, script, manifest, lock, coordinator,
checklist, or ticket metadata was edited. Cargo, Nx, Wasm, browser, network,
and root lint were not run.

At inspection time the packet sources were live **unstaged** working changes:
the index contains only the related root-script change (71 additions, 2
deletions); the working tree contains the executor (128/49), shard component
(105/42), actor transport (9/4), and additional root-script (145/3) changes.
This mixed concurrent state is recorded for reproducibility, not attributed as
a blocker; the findings below are against the live working source.

## Requested Static Gates

| Gate                                        | Result            | Evidence                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Scoped Rust formatting                      | **PASS**          | rustfmt --edition 2021 --check on actor transport, shard component, and executor exited 0.                                                                                                                                                                                                                                                    |
| Root interactivity audit                    | **PASS**          | bun ./📜️script.ts verify interactivity exited 0 in deny mode: one expected test-only allowlist record; no unlisted executor blocking bridge.                                                                                                                                                                                                  |
| Root adversarial-fixture execution          | **PASS, limited** | bun ./📜️script.ts verify interactivity --self-test exited 0. The audit invokes the shard self-tests unconditionally before scanning, so the five fixtures did execute.                                                                                                                                                                        |
| Production executor scan                    | **PASS, narrow**  | Excluding its #[cfg(test)] module, ShardExecutor::run contains no block_on(, no .recv().await, no while let Some(bytes), and no loop {. SharedThreadTransport::recv still forwards self.0.recv().await at line 50, but concrete ThreadTransport::recv delegates to non-parking try_recv_now; that forwarding call is not in the pool closure. |
| Working, staged, and HEAD whitespace checks | **PASS**          | git diff --check, git diff --cached --check, and git diff HEAD --check all exited 0; the packet-scoped working diff check also exited 0.                                                                                                                                                                                                      |

The fixture result is deliberately qualified. The five mutations—literal loop,
second drive expression, stale check after lock, dropped rejected closure, and
multi-close loop—do flow through interactivityShardExecutorFailures and are
rejected. They are useful regression tripwires, rather than mere fixture
counts. The verifier is nevertheless lexical (includes, occurrence counts, and
indexOf), so it does not prove control-flow, ownership, liveness, or allocation
properties. The live residuals below pass that lexical check.

## Gate-by-Gate Source Findings

| Required property                                                        | Result                          | Independent source finding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------------------------------------------ | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No block_on, receive wait, or epoch drain loop in the pool closure       | **PASS**                        | executor.rs:310-355 has one admission and returns; the old closure-level block_on and receive-draining loop are absent.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| At most one transport frame and one actor turn or job step per drive_one | **PASS, source structure only** | shard component.rs:450-455 receives one frame; :477-482 selects one actor; :512-514 selects one job only when no actor was driven and requeues the rest. No runtime assertion was run.                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Stale epoch checked before shard-state mutation/consumption              | **PASS, source structure only** | executor.rs:311-315 returns before state.lock() at :317. A subsequent concurrent epoch bump is still handled only by the next admission; that race has not been runtime-tested.                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Exact rejected-closure retention without spin                            | **RED**                         | :288-303 does retain the exact rejected.into_job() and returns without an immediate loop. But if the first try_submit fails as Contended or Saturated, no running closure exists to call schedule, and only a later send_frame can take handoff. The async pool explicitly defines these finite outcomes at ⏳️async/🦀️component.rs:1663-1679; the executor provides neither timer/backoff nor pool-capacity wakeup. Shutdown is also retained rather than terminally owned. Work can therefore remain stranded after quiet ingress.                                                                             |
| Suspension retention without spin                                        | **RED**                         | ShardDriveWake::wake is empty (executor.rs:182-205). When retained drive polls Pending, :334 produces ShardDrive::Blocked; :346, :350-353 immediately resubmit it. Production Wasmtime paths use store.run_concurrent(...).await for turn and job work (plugin host component.rs:1995-2000, :2091-2093), expressly supporting guest suspension on a host-async import. A pending future therefore neither wakes a scheduler nor waits for external readiness: it can be repeatedly admitted and polled, consuming pool capacity.                                                                                |
| FIFO registration                                                        | **AMBER**                       | register uses push_back (executor.rs:241-246) and the later drain uses pop_front (:320), so order is FIFO. It is not bounded: one closure runs while let Some((actor, instance)), draining every retained registration before its drive opportunity.                                                                                                                                                                                                                                                                                                                                                            |
| No new unbounded allocation                                              | **RED**                         | The packet adds unbounded VecDeque registration retention and uncapped pending_events: HashMap<u64, Vec<Event>>, pending_job_steps: BTreeMap<u64, JobTurn>, and pending_cancels: BTreeMap<u64, VecDeque<u64>> (shard component.rs:310-336). No capacity, rejection, coalescing bound, or credit is present. Unlike the fixed 1,024-slot WorkerPool lane queues, these owners can grow while each admission makes one unit of progress.                                                                                                                                                                          |
| Failure and close ownership                                              | **RED**                         | One cancel authority is selected by VecDeque::pop_front (shard component.rs:796-817), but outcome decode errors are silently discarded by if let Some(Ok(outcome)) (executor.rs:340-344). A ShardDrive::Fault is stored in a last-value Option (:347-348), and an exhaustive source search finds no production caller of public take_failure beyond its definition. Failure is neither queued nor delivered to an owner. On cancel failure, the remaining per-actor cancellation queue is removed and the actor is unregistered (:807-810); its terminal contract needs a real behavior test before acceptance. |

## Exact Repair Requirements

A re-audit requires all of the following, without broadening into the MCP or
store-sync Phase 1 blockers:

1. Replace the no-op retained-drive waker plus immediate Blocked resubmit with
   one owner-driven wake path. A pending drive must schedule exactly one later
   retry when its waker fires, never submit/poll in a tight loop.
2. Give every rejected try_submit closure a finite retry trigger independent of
   later ingress (for example, a bounded timer callback or explicit
   capacity-release notification). Retain exact closure identity on Contended
   and Saturated; terminally resolve Shutdown and Poisoned rather than retaining
   forever.
3. Replace all new deferred dynamic containers with fixed-capacity or
   credit-governed owners. State the admission/rejection outcome for every full
   queue and prove no authority is lost.
4. Preserve FIFO registration with a fixed queue and admit **at most one
   registration per worker closure** before the drive opportunity; do not run an
   unbounded while-drain under the pool job.
5. Deliver malformed outcome decoding and drive failure through a defined,
   observable failure owner. The close path must retain or terminally report
   every authority, including cancellation failure.
6. Add source and runtime tests for pending-drive wake/no-spin, quiet-ingress
   saturation recovery, capacity boundaries, one-registration closure fairness,
   malformed outcome delivery, and cancellation failure ownership.

## Remaining Phase 1 Blockers

Independent of this rejected packet, the preceding readiness audit remains
current: store-sync still nests an actor block_on inside a worker-pool job, and
MCP HTTP transport constructs a separate Tokio runtime. Fresh serialized native
evidence for the worker/thread census, permit reserve/release behavior, and the
supported plugin-host synthetic runtime path is also outstanding.

## Commands Run

```sh
rustfmt --edition 2021 --check '🧰️framework/🔨️modules/🎭️actor/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs'
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity --self-test
git diff --check
git diff --cached --check
git diff HEAD --check
```

No commands outside the read-only/static boundary were run.
