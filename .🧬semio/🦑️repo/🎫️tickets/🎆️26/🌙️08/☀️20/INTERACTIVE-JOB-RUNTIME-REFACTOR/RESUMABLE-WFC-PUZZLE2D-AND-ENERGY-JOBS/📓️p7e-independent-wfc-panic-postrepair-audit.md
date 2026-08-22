# P7e Independent WFC Panic Postrepair Audit

Date: 2026-08-22  
Scope: Current Phase 7 production Assembly/WFC ActionBus route, `semio.infer` bridge, WFC materializers, and guest cold relay.  
Method: Fresh read-only source/static audit. Read `AGENTS.md`, `p7a-wfc-job.md`, `p7c-energy-job.md`, and all of `p7d-independent-wfc-relay-final-audit.md`, including its appended repair disposition. No Cargo, Bun, build, test, runtime, Wasm, cache/target, ticket-status, or production-source mutation was performed.

## Verdict: REJECT

The prior P0 slot-loss and commit-side-reservation findings are repaired in the current source. However, the relay still discards every ordinary `cancel-job` error and reports cancellation as successful. The lease consequently restores the instance to `Available`, and the relay clears its cleanup obligation, despite the guest cancellation having trapped, exhausted, or otherwise failed. A later mounted route can reuse an instance whose previous guest job may still be live. This violates the required no-unsafe-reuse and successful-cancellation-only cleanup contracts.

## P0 — Failed `cancel-job` Is Silently Treated As Success And Reuses The Guest

`GuestRuntime::cancel_job` is explicitly fallible (`Result<(), TurnFault>`), and the owned and Wasmtime implementations can produce ordinary `TurnFault` values.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:660-675` declares the fallible `GuestRuntime::cancel_job` contract; `TurnFault` includes `Trapped`, `DeadlineExceeded`, and `FuelExhausted` at `:607-623`.
- `:1229-1238` propagates failure from the owned implementation; `:2041-2049` maps both Wasmtime cancellation failure points to `TurnFault::Trapped`.
- Despite that contract, `cancel_guest_job_once` deliberately discards the result at `:2765-2768` (`let _ = runtime.cancel_job(...).await`). It returns no success/failure value to its caller.

Every caller then publishes `GuestRelayCompletion::Cancelled` regardless of that discarded error:

- cancellation racing `start-job`: `:2801-2806`;
- cancellation racing `step-job`: `:2808-2813`;
- foreground cleanup request: `:2815-2818`;
- background cleanup task: `:2841-2847`.

On each such path the lease is dropped and returns the instance to `Available` (`:2820-2822`, `GuestInstanceLease::drop` at `:2736-2747`). The outer relay maps `Cancelled` to an ordinary terminal cancellation and explicitly clears `cleanup_required` (`:2981-2984`). It neither faults nor quarantines the failed cancellation. A later mounted request therefore acquires that `Available` instance normally (`:2793-2799`) and may run on a guest with unconfirmed previous-job termination.

The authored cancellation tests cover a panic (`:3429-3464`) and success-only mock cancellation (`MockGuestRuntime::cancel_job` always returns `Ok(())` at `:934-937`), but do not cover an error-returning foreground or background `cancel-job`. Thus static test review does not close this path.

Required repair: preserve the `Result` from the one allowed cancel admission. On `Err`, publish one typed fault for a foreground request and transition the restored instance to `Quarantined`; for background cleanup, quarantine before any subsequent route can acquire it. Do not clear `cleanup_required` or emit `Cancelled` unless the one cancel call returned `Ok(())`. Add source/runtime regressions for trapped/erroring foreground and background cancellation that assert one admission, a nonmissing quarantined slot, released permit, one terminal fault then `Yield`, and prompt rejection of the next mounted route.

## Static Checks That Pass

| Requirement | Current source evidence | Static result |
| --- | --- | --- |
| Explicit `Available` / `Leased` / `Quarantined` ownership | `GuestInstanceSlot` defines exactly those states at `host component:2689-2693`; acquisition uses poison-tolerant locking and preserves quarantine detail at `:2712-2728`. | PASS |
| Unwind-safe lease and permit order | `GuestInstanceLease::drop` restores through `PoisonError::into_inner` at `:2736-2747`. The request manually drops lease before permit at `:2820-2821`; retained-panic handling drops its retained future before invoking recovery at `:2636-2643`. | PASS (static) |
| Retained waker no lost/double normal wake | Scheduling has independent `scheduled` and `wake_requested` gates (`:2605-2614`); pending clears admission then reschedules if wake occurred during polling (`:2625-2629`); both `Wake` methods share that path (`:2649-2656`). | PASS (static; race runtime unrun) |
| Typed one-shot normal/panic completion | `GuestRelayCompletionSender` owns one sender in a poison-tolerant `Option` and takes it once (`:2672-2687`); panic recovery sends `Fault` after cleanup scheduling/quarantine decision (`:2852-2872`). | PASS (static) |
| Start/step panic recovery without slot loss | The panic handler for non-cancel requests schedules one cleanup and sends a typed fault (`:2863-2871`), after retained-future drop runs the lease. Authored mounted start/step tests inspect availability, worker progress, and next-route completion at `:3383-3426`. | PASS (static; tests unrun) |
| Foreground/background cancel panic quarantines | Cancel request panic chooses quarantine (`:2892-2909`, `:2863-2867`); background cleanup uses a quarantine panic handler (`:2837-2849`). Authored tests cover both at `:3429-3480`. | PASS for panic only; P0 covers non-panic cancel errors |
| Exactly-once schedule and admission gates | `cancel_scheduled` is claimed by compare-exchange at `:2833-2836` and `:2921-2925`; `cancel_admitted` is separately claimed at `:2765-2768`. | PASS (static) |
| Nonterminal drop, terminal dedupe, one worker progress | `Drop` schedules whenever cleanup remains required (`:3000-3005`); terminal re-entry yields (`:2940-2943`); the one-worker pending/competing test is authored at `:3307-3341`. | PASS (static; test unrun) |
| No production relay `block_on`, private pool/thread, nested scheduler, or self-requeue | Relay uses `plugin_host_worker_pool()` process singleton (`:3054-3057`), one `WorkerJobSession::step` per caller turn (`:3073-3084`), and one finite retained future poll per pool closure (`:2616-2646`). `WorkerJobSession` itself does not self-requeue (`🧰️framework/🔨️modules/🧵️job/🦀️component.rs:752-803`). The only `run_to_completion` hit in Assembly is the explicitly headless adapter (`Assembly inference:499-515`), not the public factory. | PASS (static) |
| WFC checked admission and pre-append bounds | Checkpoint capacity arithmetic is checked and exactly reserved at `WFC job:320-339`; every header/domain/trail/decision/observation write calls `ensure_materialization_space` before append at `:964-1047`. Commit checks item admission, byte arithmetic, and reserves serialized and assignment buffers at `:352-366`; `commit_one` checks item/byte capacity before each push/append at `:1050-1081`. | PASS (static) |
| Exact maximum / maximum-plus-one materializer coverage | Source test checks no assignment-capacity growth at exact maximum and rejects commit/checkpoint maximum plus one at `WFC job:1909-1940`. | PASS (static; test unrun) |
| Fixed lossless bridge with byte checks | Checkpoint/commit uses `[Option<LosslessInferenceItem>; 2]` (`infer bridge:69-83`), checks per-item/aggregate byte caps with checked addition (`:104-117`), and removes FIFO entries without allocation (`:119-127`). Exact byte and item saturation source coverage is at `:446-467`. | PASS (static; test unrun) |
| Production ActionBus route, dispatch, bounds, and freshness remain wired | Procedural `plugin()` registers the factory and declares routed metadata (`procedural root:139-147`). Factory fixes `semio.infer` / `s.assembly.solve` / `s.assembly.inference.request.v1` and uses `register_once` (`Assembly inference:16-18`, `:466-497`). The bridge validates resources, resolves the exact key/schema, dispatches wire input, and bounds worker slices (`infer bridge:184-223`). Host route ownership removes live authority and calls `validate_commit` immediately before exposing result (`host component:4465-4488`). | PASS (static) |

## Explicitly Unrun Gates

No compile, test, runtime, or timing pass is claimed. In particular, the following remain unrun for this current source audit:

- new failed-`cancel-job` error regressions (they are also currently absent), retained-waker lost/double wake, panic/poison, semaphore-release, terminal-race, and one-worker progress runtime tests;
- focused relay, Assembly/WFC, and inference-bridge tests in debug and release; allocation-pressure and p99/max watchdog evidence;
- native procedural development, strict `-D warnings`, and release gates; exact public-factory replay at 1/2/4/default worker counts; mounted `semio.infer` freshness integration;
- `wasm32-unknown-unknown` and `wasm32-wasip2` compilation/runtime checks.

The historical executed claims in P7a/P7c are not treated as verification of this postrepair source audit.

## 2026-08-22 repair disposition

Status: **the cited ordinary-cancel P0 and the two adjacent ignored-result sites are repaired;
ready for independent source/static re-audit. The historical REJECT above remains the prior verdict
until that re-audit.**

### Cold-relay disposition

`cancel_guest_job_once` now returns `Result<bool, TurnFault>`: `Ok(true)` is the only outcome that
means the atomic admission was won and guest cancellation succeeded. Foreground cancellation maps
that outcome alone to `GuestRelayCompletion::Cancelled`. `Err(TurnFault)` is converted to one typed
fault and retained as the instance's quarantine detail; an already-consumed admission is likewise
not represented as success and is never retried.

`GuestInstanceLease` owns an optional quarantine disposition. On ordinary cancellation failure it
restores its still-owned instance directly from `Leased` to `Quarantined` while holding the slot
mutex; the semaphore permit is released only afterward. The quarantine now owns `Vec<u8>` fault
detail rather than a static label. A subsequent mounted route clones and returns those exact stored
bytes without entering the guest. Foreground failure leaves `cleanup_required` set and produces one
fault followed by `Yield`; background failure has no false completion to publish and preserves the
same quarantine for the next route. The separate scheduling and admission gates remain unchanged,
so Drop, foreground resolution, and fault cleanup cannot retry `cancel-job`.

The cold relay now owns and observes the caller cancellation token; its generic `WorkerJobSession`
uses a separate live driver token. This prevents `drive_step`'s generic pre-cancel shortcut from
publishing `Cancelled` before the relay has received the fallible guest cancellation result.

Authored regressions inject an ordinary trapped cancel failure during context cancellation on a
one-worker pool, terminal Drop cleanup, start-failure cleanup, and step-failure cleanup. They check
one admission, direct typed fault exactly once where a receiver exists, quarantine before permit
release, worker progress, unchanged start/step counts after the next mounted rejection, and no
unsafe reuse or deadlock. Existing successful cancellation, start/step panic recovery, cancel-panic
quarantine, mutex-poison recovery, and lossless materialization paths are unchanged.

### Adjacent ignored-result audit

The scoped plugin-host scan found two additional discarded `GuestRuntime::cancel_job` results in
`ShardLoop`, and both are repaired. Effect-level cancellation retains `running_jobs`, `job_turns`,
and `job_placement` until the guest returns success. On failure it retires/drops the actor instance,
clears all actor-owned scheduling state through `unregister`, emits the exact `TurnFault` in a
`ShardOutcome::Fault`, and cannot later step or reuse the actor. Actor-level `Payload::Cancel`
attempts jobs in stable `BTreeSet` order, retains the first failure, always retires the instance,
and emits `Fault` rather than falsely emitting `Cancelled`. Source regressions cover each path,
including one admission, deterministic detail, retirement, no later step, no retry, and no false
success. The scoped scan now has zero ignored fallible `cancel_job` results.

### Allowed static evidence and open executable gates

- `rustfmt --edition 2021 --check` on the cold relay and shard host leaves: exit 0.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json`: exit 0; 775/775 bounded,
  zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero
  failures.
- Scoped `git diff --check` on both source leaves and both records: exit 0. Scoped scans found zero
  ignored guest cancellation results, production relay `block_on`/private pool/thread/batch driver,
  `mem::forget`/`ManuallyDrop`, or temporary debug-output hits. The shard leaf likewise has zero
  temporary debug-output hits.
- Source scans reconfirmed the WFC materializers' checked exact reservations and per-append bounds,
  plus the inference bridge's fixed two-item lossless array and checked aggregate byte addition.
- Cargo, build, test, runtime, and Wasm commands were not run by instruction. No compile, test,
  race, watchdog, native, strict-warning, release, mounted integration, worker-count replay,
  `wasm32-unknown-unknown`, or `wasm32-wasip2` pass is claimed.

Explicitly unrun on this current tree: all six new fallible-cancellation regressions; the retained
waker lost/double-wake, panic/poison, semaphore-release, terminal-race, and worker-survival runtime
tests; focused relay/shard/Assembly/WFC/bridge debug and release suites; allocation-pressure and
p99/max watchdog evidence; procedural native dev/strict/release; exact public-factory replay at
1/2/4/default workers; mounted freshness integration; and both Wasm targets.
