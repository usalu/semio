# Phase 1 and Phase 2 Readiness Audit — 2026-08-23

## Scope and Method

This is a read-only source-and-artifact audit of the One-Pool Worker Runtime
(Phase 1) and Resumable Job and Progress Protocol (Phase 2). It reads the
governing plan, the master and phase reports, current Rust source, and retained
captures only. It deliberately did **not** run Cargo, Nx, Wasm, browser, or any
runtime workload while the implementation lanes are active. Consequently,
source structure is not presented as runtime proof and historical passing
reports are not treated as current evidence.

## Verdict

| Phase                                         | Readiness | Reason                                                                                                                                                                                                                                                                  |
| --------------------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Phase 1 — One-Pool Worker Runtime             | **RED**   | The pool and permit primitives exist, but three live production paths retain blocking or additional-runtime behavior that contradicts the sole-pool and bounded-closure gate. No current serialized runtime capture proves the worker/thread census or permit behavior. |
| Phase 2 — Resumable Job and Progress Protocol | **RED**   | The job protocol and source-level synthetic tests exist, but the required generation-tagged preview overlay and an executable replay harness are absent. The Phase 1 executor residual also prevents the bounded-step/cancel gate from qualifying.                      |

These are readiness verdicts for these two phases only; they are not a verdict
on the parent refactor or any dependency wave.

## Evidence Classification

### Current source facts

- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` contains `WorkerPool`,
  logical lanes, `PermitLedger`, admission checkout/release, per-process worker
  sizing, and the native interactive reserve. Its `block_on` is compiled only
  with `test` or the `entrypoint` feature. The services crate nevertheless
  enables that `entrypoint` feature for its complete dependency, so source does
  not mechanically restrict its use to a process bootstrap.
- `HostAsyncRuntime` no longer exposes `run_blocking`; the services
  `ComputePool` accepts `InteractiveJob` work and its
  `schedule_compute_job_step` calls `drive_step` once per scheduling call.
  These are useful structural advances, not executed runtime validation.
- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` defines `StepContext`,
  `StepOutcome`, `InteractiveJob`, `ProgressEvent`, channel policy, checkpoints,
  batch adapters, and `TortureJob`. Its test source includes watchdog,
  preview-frequency, `< 8_000 µs` cancellation percentile, worker-count, and
  deterministic-seed cases.
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` has `JobTurnBridge`, one
  source-level `drive_step` dispatch, turn-status mapping, and a serializable
  `JobReplayLog`. The existing tests exercise scripted step, checkpoint,
  cancel, stale-preview, and byte-equality cases.

### Historical or report-only evidence — stale/unverified

- Phase 1 reports `📓️p1g-runtime-gate.md`, `📓️p1l-process-wide-pool-enforcement.md`,
  and `📓️p1m-interactive-compute-closure.md` claim prior test/census successes,
  but the phase folder does not retain the raw logs named by the latter reports.
  They cannot establish the live checkout after concurrent changes.
- The retained `📓️p1j-interactivity-audit.txt` is explicitly historical and
  records a warning census of 102 findings, including 60 unallowlisted
  `block_on`/`run_blocking` findings. It conflicts with the later narrative;
  neither document substitutes for a fresh source review and serialized run.
- `📓️p2a-job-protocol.md` and `📓️p2b-actor-job-bridge.md` report tests as
  passing, but retain no raw test artifact. The latter also records an upstream
  stop before the native plugin-host target compiled. These claims are
  report-only for this audit.

## Phase 1 Gate Audit

| Governing gate                                                          | Status                  | Evidence / exact residual                                                                                                                                                                                                                                                                                                                                             |
| ----------------------------------------------------------------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| One logical process-wide worker pool with logical lanes                 | **AMBER**               | `process_worker_pool` and lane/permit structures exist in the async component. It is not runtime-proven, and a live MCP transport path still creates a separate Tokio runtime.                                                                                                                                                                                        |
| Interactive admission reserve and release accounting                    | **AMBER**               | `PermitLedger`, checkout/release, and the interactive reserve are in current source, with source tests. No current release-mode saturation or over-release capture was run.                                                                                                                                                                                           |
| `run_blocking` removed from host runtime / compute service              | **GREEN (source only)** | The host trait has no `run_blocking` and compute scheduling is job-step based. `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs` still exposes `run_blocking_op` for I/O work, including an inline `None => work()` path; that qualified I/O bridge needs explicit latency/ownership validation before a broader non-blocking claim.              |
| `block_on` only at test/process-entry boundary; bounded worker closures | **RED**                 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs:255-276` places repeated `semio_framework_async::block_on(...)` calls inside a WorkerPool job. The closure loops, pumps, awaits receives, decodes, and can continue for an updated epoch rather than yielding after one bounded turn. This is production source, not a test-only helper. |
| Actor turns do not nested-block inside the pool                         | **RED**                 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1531-1554` submits `run_job` to the WorkerPool and calls `self.runtime.block_on(actor.run_turn())`. The source gives no `StepContext` deadline or one-turn runtime proof for that nested block.                                                                                                  |
| Exactly UI thread plus pool workers; no subsystem runtime/pool          | **RED**                 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs:237-241` constructs `tokio::runtime::Runtime::new()` then blocks it in `HttpTransport::serve`. Its runtime reachability was not executed here, but its live source ownership contradicts an unconditional one-runtime census.                                                                 |

Phase 1 therefore cannot advance on the strength of the existing reports. The
three RED residuals are current source findings, independent of whether some
historical tests once passed.

## Phase 2 Gate Audit

| Governing gate                                                                              | Status    | Evidence / exact residual                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------------------------------------------------------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Resumable `InteractiveJob` step protocol, checkpoints, child/batch adapters                 | **AMBER** | The job component has the required protocol types and adapters. Its unit and torture tests are present, but this audit did not execute them.                                                                                                                                                                                                                                                                   |
| Progress taxonomy and delivery policy                                                       | **AMBER** | Current source defines the expected progress variants and policy matrix. It is not a runtime proof that consumers receive coalesced/ordered events under load.                                                                                                                                                                                                                                                 |
| One logical step per actor turn                                                             | **AMBER** | `JobTurnBridge` invokes one `drive_step` in source. The plugin shard executor’s repeated blocking pump/receive loop means the production bridge can still drain more than one buffered turn in one pool closure.                                                                                                                                                                                               |
| Generation-tagged preview overlay separate from committed scene state                       | **RED**   | `ProgressEvent::PreviewPatch` exists, but an exact owned-source search finds no `PreviewOverlay` implementation. `SceneStore` in the actor component keeps `current`, pending actor patches, and pending node delta—not an operation/base-revision/generation-keyed preview layer. The planned preview state, stale-generation rejection, cleanup, and non-persistence boundary are therefore not implemented. |
| Replay log plus executable deterministic replay harness                                     | **RED**   | `JobReplayLog` only stores entries and supports pack encode/decode. The existing byte-equality test executes scripted outcomes twice; it does not replay a recorded log through a driver across 1..N worker counts. The `TortureJob` has a useful 1/2/4 source test, but it is not connected to a persisted replay-log executor.                                                                               |
| Synthetic responsiveness, continuous preview, cancellation `<8 ms p99`, and replay identity | **RED**   | The named test source exists but there is no current captured runtime result. In addition, the Phase 1 shard executor residual invalidates a credible bounded-turn/cancellation measurement until it yields fairly.                                                                                                                                                                                            |

## Smallest Disjoint Next Implementation Packet

The smallest immediately actionable packet is **ShardExecutor bounded-turn
handoff**. It is not a Phase 1 closure packet: MCP transport and store-sync
remain separately owned blockers. It should be kept narrow and coordinated with
the active job lane:

1. In the plugin shard executor and the minimum shard API surface it needs,
   replace the WorkerPool closure’s `block_on`/receive drain loop with a
   non-blocking, single-work-item handoff. Re-submit only when work remains;
   never wait on an async receive in a pool closure.
2. Preserve the existing queue/epoch semantics, but make the scheduling unit
   one actor turn or one `InteractiveJob::drive_step` opportunity. Add a source
   test for a full queue proving the closure yields and another proving one
   admitted job produces no more than one step.
3. Add an ownership-specific interactivity assertion that rejects production
   `block_on` in this executor. Do not change MCP, store-sync, pool sizing,
   preview storage, or job schema in this packet.

Before dispatch, recheck the current diff because concurrent lanes may touch
the plugin shard surface. Once this packet is settled, the actor preview/replay
substrate is the smallest independent Phase 2 packet: add an operation,
base-revision, generation, and sequence keyed overlay beside `SceneStore`; make
its cleanup/commit rules explicit; then add a recorded-log replay executor that
can prove byte-identical output for worker counts 1, 2, and 4. It must not
share or persist preview patches with committed scene/undo state.

## Required Post-Source Serialized Verification

Run only after the owners have finished their source changes, from repository
root, in this order. `--skip-nx-cache` is intentional; any UI/browser command
is outside this Phase 1/2 audit.

```sh
bun ./📜️script.ts verify interactivity
bun x nx run @semio-tech/framework-async-rs:test-quick --skip-nx-cache
SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun x nx run @semio-tech/framework-async-rs:test-long --skip-nx-cache -- --release
bun x nx run @semio-tech/os-services-rs:test-quick --skip-nx-cache
SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun x nx run @semio-tech/os-services-rs:test-long --skip-nx-cache -- --release
bun x nx run @semio-tech/framework-job-rs:test-quick --skip-nx-cache
SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun x nx run @semio-tech/framework-job-rs:test-long --skip-nx-cache -- --release
bun x nx run @semio-tech/framework-actor-rs:test-quick --skip-nx-cache
SEMIO_TEST_BUDGET_MS=120000 SEMIO_BUILD_BUDGET_MS=120000 bun x nx run @semio-tech/framework-actor-rs:test-long --skip-nx-cache -- --release
bun x nx run @semio-tech/framework-actor-rs:wasm --skip-nx-cache
bun ./📜️script.ts verify rust-warnings --target native
bun ./📜️script.ts verify rust-warnings --target wasm32-unknown-unknown
bun ./📜️script.ts verify rust-warnings --target wasm32-wasip2
```

The current project surface has no dedicated Nx target for a native plugin-host
runtime torture test. Before either phase is marked green, add or identify a
supported, isolated owner command that actually runs its synthetic host path;
then serialize it after the async/services tests and before the multi-target
warning checks. That command must capture: worker/thread census, interactive
reserve saturation and release error, UI responsiveness under background load,
continuous preview delivery, cancellation p99 below 8 ms, and replay identity
for 1/2/4 workers. A warning-only Rust check is not a substitute for that
runtime evidence.

## Audit Boundary

No production source, manifest, lockfile, coordinator list, checklist, ticket
metadata, or lifecycle state was changed. This report is the sole audit output.
