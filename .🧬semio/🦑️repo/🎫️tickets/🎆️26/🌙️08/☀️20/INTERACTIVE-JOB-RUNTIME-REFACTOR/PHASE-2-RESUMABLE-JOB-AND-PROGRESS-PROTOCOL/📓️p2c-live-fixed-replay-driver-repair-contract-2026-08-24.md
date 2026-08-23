# P2c Live Fixed Replay Driver Repair Contract

Date: 2026-08-24
Owner: `/root` coordinator
Verdict: **PREPARED — implement after P2a1 universal ownership acceptance.**

## Purpose

Finish the third Phase 2 packet by replacing the test-only dynamic replay record with a mounted,
fixed, retained replay authority that drives the same live shard/session and accepted P2d preview/
commit publication path at worker counts 1, 2, 4, and host default.

The result must prove one job step per actor turn, deterministic publication prefixes, lossless
checkpoint/commit/fault ownership, and exact terminal cleanup. It must not create a second replay-
only job executor or preview store.

## Preserved Foundation

Keep and compose:

- `InteractiveJob`, `StepContext`, operation/revision/generation/sequence validation;
- P1n fixed shard executor and one-retained-future polling;
- P2a1 fixed universal payload/session/terminal authority;
- the accepted P2d fixed live preview/progress overlay and exact surface invalidation;
- actor job wire records and owned pack codecs; and
- the real ActionBus/WorkerPool mounted job route.

The old `JobTurnBridge` remains only as a test oracle if it cannot be deleted. Production replay
must exercise the same mounted route as ordinary execution.

## Current Gaps

`JobReplayLog` is a cloneable `Vec<JobPublication>` with zero production callers. Existing tests
construct two scripted logs and compare bytes; they do not replay a recorded execution.

The live shard maintains separate job state and historically emitted `ShardOutcome::Job`; P2d now
mounts publication consumption, but no fixed replay owner captures and replays that exact stream.
Checkpoint failure must never become empty state through `unwrap_or_default` or another lossy
adapter.

The torture job exercises local protocol properties but not the complete ActionBus → shard →
WorkerPool → P2d overlay → commit/close path.

## Schema-First Replay Identity

Define a versioned owned replay schema containing:

- plugin/package/actor/window/document identity;
- controller/tool/inference route identity;
- job ID and operation ID;
- base revision and checked generation;
- initial request schema/version/digest;
- deterministic seed/RNG contract;
- ordered turn ordinal;
- granted fuel/deadline class and cancellation observation;
- publication kind and exact step/preview/checkpoint/commit/fault sequence;
- page/item/byte digests and typed outcome metadata;
- final commit/result digest; and
- terminal/close witness.

All identifiers and payloads use fixed/page owned backing with exact item/byte/control admission.
The schema declares a maximum turns, publications, checkpoint pages, commit pages, preview pages,
fault bytes, child records, and aggregate live-plus-replay working set.

Generations and ordinals use checked nonzero arithmetic. Exhaustion refuses the exact producer and
permanently exhausts the slot; no wrap, saturation, reset, or ABA alias is allowed.

## Fixed Replay Log Authority

Replace the live `Vec` record with a fixed/page append cursor. One publication is admitted before
the live publication owner transfers. Capturing advances through:

1. reserve one record/header/output slot;
2. transfer scalar identity fields;
3. transfer one payload page or typed metadata item;
4. seal the record;
5. validate prior prefix digest/sequence;
6. append one fixed record slot; and
7. ACK the live owner to continue ordinary publication.

One worker grant advances one scalar/page/control/record unit. Recording backpressure retains the
exact live publication and stops the next job turn; it never drops a checkpoint/commit/fault or
runs ahead with an incomplete log. Replaceable preview may coalesce in the P2d overlay, but the
replay contract explicitly records whether the preview was accepted, displaced, or rejected so the
same observable policy is reproduced.

Checkpoint, commit, final result, and fault records are lossless. A full log returns/preserves the
exact owner and transitions the operation into retained fault/close; it does not silently truncate.

## Mounted Replay Driver

The production-shaped replay driver consumes one sealed replay record per turn and dispatches the
same typed operation through ActionBus/shard/WorkerJobSession. It does not call `drive_step`
directly, construct a private WorkerPool, loop to terminal, or bypass P2d.

For each expected record:

1. validate route/job/operation/base/generation and prefix digest;
2. admit exactly one live actor/shard turn;
3. let the shared WorkerPool execute at most one job step;
4. consume the resulting publication through the accepted P2d cursor;
5. compare the exact typed publication identity, sequence, policy outcome, and payload digest;
6. publish/retain it through the normal overlay/checkpoint/commit/fault owner; and
7. advance the replay prefix only after an exact match.

Missing, duplicate, reordered, extra, stale, corrupt, wrong-worker, wrong-seed, wrong-generation,
or wrong-payload records produce an owned replay fault. The current live document/operation state
remains unchanged until an accepted terminal commit uses the normal revision/generation validation.

Replay cannot use recorded wall-clock timestamps as correctness input. Scheduling-independent
turn order and deterministic seeds define the result; timing is measured separately.

## Worker-Count and Schedule Determinism

Record once through the real mounted route. Replay the same sealed log using real process pools
configured for 1, 2, 4, and host-default workers, with deliberate variations in unrelated job
availability and wake timing.

Every run must produce:

- identical accepted publication kind/order/sequence;
- identical checkpoint/commit/final digests;
- identical P2d overlay observable generations;
- identical final authoritative document/result;
- identical cancellation/fault terminal classification; and
- exact empty counters after close.

Concurrency may change when a turn becomes ready, but it cannot change the actor/job publication
sequence or deterministic reduction order.

## Torture Job Mount

Register the synthetic torture job as a test-only operation through the real production factory,
ActionBus, shard, session, P2d overlay, replay capture, replay driver, commit validation, and close
path. Do not expose it in production catalogs.

The mounted fixture must prove:

- long operation with continuous distinct previews;
- one job step per admitted actor turn;
- cancellation observed within the declared p99 bound;
- checkpoint capture/restore and replay continuation;
- stale preview and commit rejection;
- structured child completion cannot terminally succeed with live children in release;
- worker panic/stuck job/closed channel/fault ownership;
- bounded log/output saturation with exact producer handback; and
- terminal/result/overlay/log/process counters return to zero.

## Structured Children and Terminality

P2a1 supplies a fixed child registry and release-mode completion law. P2c records child spawn,
progress, completion/fault/cancel, and parent terminal order as typed records. Replaying a parent
terminal before a live child is a mismatch/fault and cannot publish a commit.

Window/document/app close freezes replay/capture admission, cancels the exact descendant scope, and
moves log, expected/current record, P2d overlays, checkpoints, candidates, children, terminal
results, and faults into one retained close graph.

Dropping the replay handle during partial close leaves a generation-addressable registry authority
that resumes exactly once. One close grant retires one record/page/string/control owner. Terminal-
empty is exhaustive.

## Error and Checkpoint Ownership

Remove any mounted `unwrap_or_default`, empty checkpoint substitution, discarded receiver, or
owner-erasing string adapter. Checkpoint construction/transport/restore failure retains the exact
typed fault plus every partial page and operation owner.

Completed-but-unclaimed terminal, full output registry, panic between result production and
capture, and cancellation after completion all have exact take/resume/close paths. A saturated
terminal/log/output slot cannot livelock close; close advances an existing matching terminal first.

## Hostile Fixtures

Add fixtures and matching verifier mutations for:

- zero/max/max+1 turns, records, items, pages, bytes, checkpoints, commits, previews, faults,
  children, terminal slots, and control owners;
- one-step proof with zero/insufficient fuel and expired deadline;
- missing/duplicate/reordered/extra/corrupt records and bad prefix digest;
- wrong route/job/operation/base revision/generation/seed/worker metadata;
- checked ordinal/generation exhaustion and stale/ABA tokens;
- capture backpressure before every transfer and exact live-owner preservation;
- replay mismatch before and after P2d overlay publication;
- cancel/fault/panic/drop at every capture/replay/commit/close phase;
- full terminal/output/log registries and exact FIFO handback without external checkout;
- dropped handle during partial close;
- structured child races and release-mode parent completion rejection;
- record once/replay at 1/2/4/default workers with unrelated wake perturbation;
- byte-identical final/checkpoint/commit/replay digests; and
- no actor turn, worker step, or publication step at or above 8 ms.

Every mutation must touch production-shaped behavior and make the focused self-test fail.

## Permanent Verifier Predicates

Extend the root `📜️script.ts` interactivity region. Deny:

- production `JobReplayLog: Vec<_>` or cloneable whole logs;
- zero production replay capture/driver callers;
- direct `drive_step`, run-to-terminal loop, private pool, or P2d bypass in replay;
- ignored `ShardOutcome::Job` or checkpoint/commit/fault owner;
- `unwrap_or_default`/empty checkpoint substitution/discarded terminal receiver;
- dynamic unbounded shard/log/outcome queues on the mounted replay path;
- more than one live job step per actor turn;
- wrapping/saturating/reset generation or ordinal;
- missing child/replay/overlay/terminal close ownership; and
- a torture fixture that does not traverse the live ActionBus/shard/P2d path.

## Owned Files and Collision Boundary

Expected ownership is limited to actor replay schema/authority, narrow shard publication capture,
P2d composition, test-only torture factory/fixtures, root verifier, and this report. Do not overlap
active P6 progress/reactor source; re-census after P6g settles. Do not edit universal P2a1 internals
while its packet is active; compose only through its accepted interface.

## Acceptance Gates

Source handoff requires exact production caller census, scoped rustfmt/diff, verifier self-test/live
focused success, deterministic replay ledgers, and independent Terra audit. Final acceptance
requires serialized debug/release/strict warnings, native/both Wasm, real 1/2/4/default WorkerPool,
browser-mounted preview, allocation/cancel/panic/stuck/close stress, deterministic replay, and the
8 ms/p99 timing matrix on the same final tree.

P2c and Phase 2 remain RED until all gates pass.
