# Exact Worker Closure Retirement Receipt

## Smallest Existing-Queue Extension

This is a source/API and test plan only. No shared async/job production code or test mount changes here. It extends the existing WorkerPool lane queues and the existing WorkerJobSubmission producer; it adds no scheduler, thread, polling subscriber, wire ABI, or semantic job-outcome authority.

Use one preadmitted completion slot for each submitted closure. Its storage belongs to the existing pool, not to an Arc captured by that closure. The caller holds an opaque non-cloneable lease with a private pool identity, slot and checked positive generation. The queued entry owns only the original Job and a private scheduler completion key. Native and cooperative workers use the same completion transition helper after consuming that exact Job.

The intended required-argument cutover is:

```rust
WorkerPool::reserve_completion(&self, target: &mut Option<WorkerClosureLease>, grant: WorkerAdmissionGrant)
    -> Result<WorkerAdmissionStep, WorkerAdmissionError>;
WorkerPool::try_submit(&self, lane: Lane, source: &mut Option<Job>, completion: &mut WorkerClosureLease)
    -> Result<WorkerSubmissionTicket, WorkerSubmitErrorKind>;
WorkerClosureLease::receipt(&self, ticket: WorkerSubmissionTicket)
    -> Result<Option<WorkerClosureReceipt>, WorkerReceiptError>;
WorkerClosureLease::close_step(&mut self, grant: WorkerRetirementGrant)
    -> Result<WorkerRetirementStep, WorkerReceiptError>;
```

Names are proposed, not callable. The two grant types must carry the existing allocation/retirement byte and item authority rather than minting it. They do not represent another quota. A completion slot is admitted before closure construction/placement. Refused queue admission leaves `source` and its original completion lease unchanged; it cannot consume a closure and silently destroy it on error. On success the existing lane entry is the sole Job owner. Queue capacity includes the actual larger entry and completion backing. Generation exhaustion refuses before moving the source. All existing submit/try_submit and WorkerJobSubmission callers must be migrated coherently when this API mounts; no optional completion argument or untracked compatibility branch is proposed.

The proposed `close_step` only retires completion bookkeeping and already typed/admitted fault owners. It cannot magically retire an uninvoked arbitrary `Box<dyn FnOnce()>`: that box may have an unbounded destructor. Pre-invocation cancellation must retain the original queued source until its concrete admission contract supplies bounded cancellation, or transfer it back to its original structural producer without dropping it. For the runtime-close domain, seal the actual capture shape and retain its app/pool payload roots outside the closure before admitting its fixed shell. A generic queue receipt cannot infer that property from byte size. This is an explicit queued-source acceptance obligation, not a promise hidden in the proposed method name.

`WorkerClosureReceipt` has private fields and no public constructor. Only scheduler-owned code can mint it. The scope is exact completed-closure retirement, not semantic job success or native instance emptiness. `WorkerSubmissionTicket` must name the pool completion slot/generation, not reuse `WorkerJobTicket`'s operation step sequence. The latter remains the outcome identity and can contain the distinct submission ticket internally where the same caller owns both.

## Actual Call Sites To Change

| Existing source | Required change |
| --- | --- |
| `⏳️async::admitted_job_queue` and both native/cooperative queue element types | Reserve actual queue-entry plus completion-slot capacity before any accepted closure; preserve the existing lane scheduler and fairness. |
| Native `WorkerPool::try_submit` around 1714 | Validate the exact pool lease, empty completion phase and queue capacity before moving `source`; publish only submitted identity, never completion. |
| Cooperative `WorkerPool::try_submit` around 1924 | Same admission helper/contract; no inferred completion from a pump return value. |
| Native `worker_loop` around 1642 | Keep the original completion slot structural, run `catch_unwind(AssertUnwindSafe(job))`, retain its result, and only afterward prepare the exact completion transition. |
| Cooperative `WorkerPool::pump` around 2007 | Use the same post-invocation transition, after Box invocation/unwind and capture destruction, not inside the callback. |
| `🧵️job::WorkerJobSession::try_submit_step` around 2890 | Reserve completion before moving WorkerJobAuthority into WorkerJobSubmission; retain its lease in the session. Preserve existing exact rejection handback. |
| `WorkerJobSubmission::run` around 2655 | Its outcome publication remains semantic only; it must never mint the closure receipt. |
| `drive_worker_job_authority` around 2650 | Its inner `match Err(_)` must hand the original panic payload into the preadmitted WorkerJobAuthority fault owner. Otherwise the pool's outer catch can never recover it. A replacement fault string is not the lost owner's retirement. |
| `PluginRuntime::try_schedule_runtime_close` | Retain the completion lease in the original quarantine/aggregate before submission; do not detach the worker shell until this exact completion receipt and all original child owners have returned. |

The new completion backing must be private pool-owned storage with a stable slot until exact release. Publishing the slot's terminal state is the worker's final access to that slot. A new incarnation cannot reuse it until the caller consumes/releases the exact ticket. This avoids the recursive flaw of placing a completion flag in another callback-owned Arc whose final release would itself need proof. Pool lifetime storage is a separate shared owner; global pool emptiness is not substituted for the per-submission receipt.

### Timed and Shutdown Entry Census

`submit_at` wraps the original Job and a pool clone in a TimerCallback. On timer fire it calls `pool.submit` only if not shut down; otherwise the wrapper implicitly drops the original Job. `callback_at` places a callback directly in the same TimerWheel, bypassing lane admission. `TimerWheel::fire_due_batch` currently removes callbacks into a local Vec and invokes each directly, with no per-callback successful retirement ticket. `fire_due` requests an unbounded maximum batch. Native and cooperative `shutdown` call `fire_due(u64::MAX)`; native shutdown then joins threads, while queued lane jobs have no exact cancellation handback in that method.

These are separate source-confirmed entry/retirement paths. A try_submit-only packet cannot claim them. Timed admission must retain both the timer wrapper and original job under exact linked completion ownership, with separate wrapper and lane-job completion when both execute. Shutdown needs retained cancellation of queued/timed owners, not executing or dropping all callbacks under a Boolean shutdown flag. The same scheduler-owned completion helper may be reused at actual invocation sites, but timer registration/admission and final queued cancellation still need their own real tests. No timer, shutdown, or shared production source was modified.

## Invocation, Unwind and Final Clock

The existing call expression consumes the Box. Once its `catch_unwind` returns, the original callable and ordinary captured fields have been destroyed. The scheduler can therefore record their exact completion without a callback-authored Boolean, Weak-upgrade inference, sampled Arc count, or pool-wide occupancy test. Any pre-admitted scheduler bookkeeping and bounded telemetry work occur before the final real-clock sample. The prepared receipt is committed only after that sample. A clock fault preserves the original completion state and exact fact that the callable has already retired; a later close-only turn must not execute it again.

There must be one callback clock authority spanning invocation, unwind, captured-field/Box retirement and the bounded completion preparation. Extend the existing callback authority's measured boundary; do not add a second Watchdog that independently grants success, and do not move retirement outside that interval. Inner job outcome validation remains a semantic/job-grant check and cannot substitute for this enclosing actual callback boundary.

There is an important additional owner: `catch_unwind` returns a panic payload. The current `let _ = ...` drops that payload implicitly. An arbitrary `panic_any` payload can itself contain an escaped Arc or other user-owned object. The extension must retain that payload in the preadmitted completion slot and must not publish full closure-descendant retirement while it is live. Typed static-string/String panic retirement can be bounded; an unrecognized payload must remain an explicit owned fault, not be generically dropped or called empty. The exact callback domain must supply its panic-retirement contract before a successful unwind-cleanup claim. A scheduler-after-call marker by itself is not sufficient for this case.

The existing job driver has an earlier catch: `drive_worker_job_authority` maps `Err(_)` to its preadmitted JobFault and discards the actual payload. Its original WorkerJobAuthority must gain the preadmitted panic handoff alongside the existing callback verdict/quarantined outcome, outside the inner catch closure. Retain both the diagnostic fault and original payload; only the typed panic owner may retire the latter under close grants. This path is required even when the outer pool callback returns normally. The outer completion receipt must therefore join this inner fault-owner terminal handoff, rather than assuming a normal outer return erased all descendants.

Similarly, a destructor panic must leave the completion slot and panic root structurally recoverable. No staging state is moved out into an unwind closure. The native instance aggregate remains in its original quarantine until the private receipt and any retained panic owner are terminal. Final runtime shell release then occurs inside the aggregate's measured turn, with the existing staged Retired ACK retained on a late verdict. The pool receipt does not replace that second final-shell/ACK boundary.

## Language-Neutral Test Vectors To Author Before Mounting

The following expected states are test specifications, not executed results. `receipt` means a scheduler-issued exact retirement receipt, not an outcome or callback marker.

| Event sequence | Expected receipt | Expected ownership |
| --- | --- | --- |
| Reserve → queue contention | None | Original Job remains in source; same lease remains reserved. |
| Reserve → submit → callback body publishes an outcome | None | Executing closure/captures still owned by queue execution. |
| Callback returns → captured Drop guard is held | None | Same completion slot remains executing. |
| Drop guard released → scheduler consumes original Box → valid clock | Exact ticket | No original Job/capture root remains. |
| Same sequence → final clock 8000us | None | Retired-callable fact retained; never reexecute Job; strict fault retained. |
| Panic with captured Drop guard held | None | Unwind is still executing; exact slot retained. |
| Unwind returns a retained String payload | None | Exact panic bytes remain owned until bounded close. |
| Unwind returns unknown `panic_any` owner | None | Original opaque fault root retained; no generic destruction credit. |
| Old ticket after slot reuse | Rejected | New slot incarnation unchanged. |
| Other pool, same slot and generation | Rejected | Original pool's lease and current source unchanged. |
| Submission generation exhausted | None | Zero source movement, no queue admission. |
| Original completion receipt acknowledged twice | One release | No successor generation consumed. |

Use a neutral fixture with supplied event sequences and exact expected phase/root counts, strict schema validation, and an independent reducer oracle. It must distinguish callable retirement from panic-payload retirement and final native shell retirement. No arithmetic tautology or event-name-only comparison can count as queue/admission proof.

## Actual Native and Cooperative Test Plan

1. Run the real native WorkerPool with a captured Drop guard that signals a channel and waits on a bounded release channel. Publish an ordinary semantic outcome before returning. Read the exact lease while Drop is held and require no receipt; release it and require the same ticket's receipt. Preserve and drain the pool afterward. This demonstrates actual capture destruction, not a dummy flag.
2. Repeat through an actual injected callback panic. Verify that the guard's original Arc and any typed panic bytes remain owned at each frontier. Unknown `panic_any` must be retained and explicitly reported nonterminal, not silently credited complete.
3. Inject a real-clock seam that crosses 7999/8000/8001 only after the held Drop tail is released. Check that no receipt commits at equality or late time and that retry never executes the closure twice. Real-clock unexcluded timing remains a separate runtime gate; fake-clock tests prove ordering only.
4. Hold the actual lane/completion admission lock and saturate the actual queue. Verify zero move of the original Job pointer and exact retained reservation on rejection, including allocator overshoot and generation exhaustion.
5. Run the shared test vectors through the actual cooperative `WorkerPool::pump` implementation on a supported wasm target and a real host. A native model of a cooperative counter is not that gate. Preserve native and cooperative output separately.
6. Finally mount the production runtime-close submission and require that quarantine removal cannot precede its exact pool receipt, and that final shell destruction precedes the lifecycle's strict final-clock/ACK commit. That is the actual native aggregate acceptance test; earlier primitive pool tests cannot substitute for it.

Add separate actual tests for an inner job-driver panic whose payload owns an Arc, timed wrapper cancellation before its lane submission, direct callback_at tail retirement, and shutdown with both queued and timed owners. The inner panic test must fail on the existing `Err(_)` disposal, not merely inject an outer pool panic. Each test retains/retires its original source and distinguishes wrapper completion from the child job's completion.

The Plugin construction snapshot and shared async/job source remain unchanged while the metadata baseline is pending. This plan needs the coordinator's agreed API boundary before the new shared schema/tests or production cutover are mounted.
