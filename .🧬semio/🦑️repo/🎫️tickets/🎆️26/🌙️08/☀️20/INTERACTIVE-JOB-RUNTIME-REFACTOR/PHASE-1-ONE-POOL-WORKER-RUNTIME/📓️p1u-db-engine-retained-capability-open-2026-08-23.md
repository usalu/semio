# P1u DB-engine Retained Capability-open — 2026-08-23

Date: 2026-08-23

## Verdict

**SOURCE-AUDIT-READY, NOT ACCEPTED.** This packet removes exactly the production storage-capability
`db_actor::block_on` group from DB open and leaves the five named DB-engine groups untouched. The
retained authority is source-verified and fail-closed, but no Cargo, Nx, Wasm, browser, runtime,
network, or native test was authorized. The backend/platform capability query remains an honest
Phase 9 indivisible-latency residual. Phase 1 remains RED with five DB-engine wait groups and the
remaining runtime work.

## Pre-edit census

The required caller and reachability census was written before source edits:

- `📓️p1u-db-engine-capability-open-caller-census-2026-08-23.md`.

It records the sole selected `Database::open_with` bridge, the four public constructors, authored
Hub/CLI/testkit callers, six original DB-engine groups, and the exact five-group target.

## Source changes

### Independent-rejection remediation

The four findings in
`📓️sol-independent-p1u-db-engine-retained-capability-open-audit-2026-08-23.md` are remediated
source-only:

- every Pending/Ready/panic poll branch republishes the exact work owner, and Ready also publishes
  the staged result and phase, before the release transition clears `polling`; a queued wake cannot
  observe an empty poll owner;
- `DatabaseCapabilityOpenRejected` now exposes exact storage take, retry, one-owner close,
  terminal emptiness, and error-after-close. The live `open_with` path explicitly closes that one
  exact owner and takes the error; the former `.into_parts().0` discard is absent;
- `DatabaseCapabilityOpenTerminalResult` is a shallow generation-owned checkout. The result stays
  in the public registry across checked-out Drop and supports exact take, resume, one-result close,
  and terminal emptiness. Main terminal close blocks while a ticket is checked out; and
- retry generation uses one checked `compare_exchange` opportunity. Contention retains the exact
  retry job/armed state and schedules one generation-keyed timer callback for the next opportunity;
  no retry loop remains.

Four direct fixtures cover wakes at Pending/Ready/panic publication boundaries, rejected-storage
take/retry/close pointer identity, post-ready terminal-result Drop handback/take/resume, and retry
CAS contention. Four faithful verifier mutations reverse owner-publication order, restore the live
`.0` discard, remove terminal-result checkout, and restore a retry spin.

### Retained operation authority

`db/engine/🦀️component.rs` now owns `DatabaseCapabilityOpenFuture` and a fixed process registry:

- 64 generation-keyed slots;
- 8 items and 16 KiB per operation;
- 512 aggregate items and 1 MiB aggregate bytes;
- exact checked item/byte/generation claim and release;
- one fixed waker slot;
- exact `Arc<DbBackend>` returned both on pre-admission rejection and successful completion; and
- exact saturated/rejected WorkerPool jobs retained through generation-keyed timer retry.

The operation is submitted only to the supplied process `WorkerPool` `Lane::Io`. Its persistent
phases are `Handoff`, `Poll`, `RetainWork`, `DrainWork`, `ReleaseWork`, `RetainResult`, `Publish`, and
`Terminal`. A callback therefore advances one handoff, one backend poll, one owner-retention or
release action, or one publication action; it does not run the retained future to completion.

Cancellation, stale admission generation, panic, quiet saturation, abandoned futures, and terminal
submission faults retain work/result/job roots in the fixed registry. The public terminal API
supports exact-generation take, oldest-abandoned take, exact retry-job resume, one-owner
`close_step`, and a terminal-empty witness that includes admission and all roots.

### Live DB open cutover

`Database::open_with` now calls `Self::open_retained(pool.clone(), storage)` and awaits the retained
future. The result hands the same storage `Arc` and capability scalar into the existing open path.
There is no synchronous wrapper or compatibility route. The facade reexports the retained future,
result, rejection, progress, close step, terminal handle, and terminal-take functions.

### Verifier

The root interactivity verifier now requires the exact five-group census, retained open call,
fixed credit/registry symbols, every persistent phase, one panic-safe backend poll, I/O-lane
submission, exact rejected-job handback, generation-keyed retry, exact result ownership, and public
terminal take/resume/close. Nineteen mutations reject:

1. the old capability `block_on`;
2. a sixth engine wait;
3. an unbounded byte cap;
4. missing aggregate item credit;
5. handoff combined with backend poll;
6. a backend poll loop;
7. missing work-retention phase;
8. missing empty-work release phase;
9. dropped saturated jobs;
10. missing generation validation;
11. missing cancel revalidation immediately before publication;
12. missing stale-generation revalidation immediately before publication;
13. missing exact rejection handback;
14. missing public terminal take; and
15. missing cap/+1/ABA fixture;
16. clearing `polling` before republishing the exact work owner;
17. restoring the live rejected-storage `.into_parts().0` discard;
18. removing public terminal-result checkout/handback; and
19. restoring a retry-generation spin loop.

The accepted P1t history verifier was updated only from its former six-group residual assertion to
the post-P1u five-group assertion.

## Fixtures

Eight permanent Rust source fixtures cover:

- per-operation item +1, byte +1, 64/+1 process saturation, exact aggregate credits, slot reuse,
  and stale-generation ABA rejection;
- successful exact storage pointer and capability scalar return;
- cancellation and deliberately stale generation retaining the exact storage owner through public
  one-owner close;
- actual I/O-lane queue saturation, exact retry job retention, abandoned terminal take, public
  resume, exact owner completion, and shutdown;
- queued wakes at the Pending, Ready, and panic publication boundaries;
- rejected-storage exact pointer take, retry, and explicit one-owner close;
- post-ready terminal-result checked-out Drop handback, retry, and exact result resume; and
- retry-generation contention advancing by one compare-exchange opportunity per callback.

These fixtures were not executed because builds/runtime were prohibited. Their presence and the
production contracts are enforced by the root verifier mutations.

## Exact post-edit census

The production portion before `//#region 🧪️Tests` contains exactly five
`db_actor::block_on` groups:

1. catalog-root read;
2. empty-catalog initial CAS;
3. create-document catalog CAS;
4. compaction; and
5. sync hello.

`db_actor::block_on(storage.capabilities())` is absent. The one remaining
`storage.capabilities().await` is inside the retained backend future and is the declared Phase 9
platform-call residual.

## Commands and evidence

P1u-scoped passed:

- `rustfmt --edition 2021 --check <db-engine> <db-facade>`;
- exact production scans: five DB-engine waits, zero capability `block_on`, one retained backend
  capability call, live `open_retained`/await cutover, zero retry loop, zero `.into_parts().0`, and
  owner publication before every polling-release transition.

Both `bun ./📜️script.ts verify interactivity --self-test --format json` and the plain form execute
all nineteen P1u mutations and report no P1u finding. The current whole verifier remains RED on one
concurrent Phase 3 prepared-raster interpreter finding; it is not a P1u failure.

Scoped working/staged/HEAD and whole working diff checks pass. Whole staged/HEAD checks remain RED
only on two concurrent Phase 2 reports outside this packet, each with a blank line at EOF:

- `📓️p2-current-status-gap-audit-2026-08-23.md`; and
- `📓️p2d-live-preview-progress-overlay-2026-08-23.md`.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs`
- `📜️script.ts`
- `📓️p1u-db-engine-capability-open-caller-census-2026-08-23.md`
- `📓️p1u-db-engine-retained-capability-open-2026-08-23.md`

## Residuals

- Rust typechecking, native tests, runtime timing, and backend integration are unproven by policy.
- The one backend capability poll may exceed 8 ms and remains Phase 9 evidence work.
- Five DB-engine synchronous groups remain deliberately untouched.
- Phase 1 runtime acceptance remains RED.

## Panic-wake remediation checkpoint

The remaining rejection in
`📓️sol-independent-p1u-four-blocker-remediation-reaudit-2026-08-23.md` is repaired
source-only. This checkpoint is audit-ready, not independently accepted.

- `publish_poll_terminal` now publishes cancellation, progress, the retained terminal error, and
  `RetainWork` while `polling` remains true.
- Pending first republishes its exact work owner, then performs cancel/stale terminal publication
  before its release store. Ready first republishes work and the exact result, selects
  `RetainWork`, and revalidates public cancellation/admission freshness before release. Panic
  republishes work and publishes deterministic fault/phase state before
  `release_terminal_poll`.
- `release_terminal_poll` is the terminal release boundary: only after all owners/scalars are
  visible does it clear `polling`, consume the coalesced wake bit, and admit one cleanup turn.
  A late real backend waker observes cancelled/terminal state and cannot schedule another Poll.
- Controlled test futures now execute real `Future::poll` Pending, Ready, and panic branches,
  retain the actual waker, wake during polling and after release, and assert one poll per governed
  opportunity. A test-only publication hook drives Ready→public `cancel` and Ready→stale before
  release, then exercises public result checkout, checked-out Drop handback, resume, exact storage
  pointer, close, and terminal witness.
- The permanent verifier now has 22 discriminating mutations. Mutation 16 moves every work
  publication behind wake release; distinct mutations move Ready phase after release, move panic
  fault after release, and remove the post-Ready cancel/stale fixture.

### Permitted gates

- edition-2021 `rustfmt --check` on DB engine/facade: **PASS**, exit 0.
- interactivity self-test DENY: **PASS**, exit 0; all 22 P1u mutations reject.
- interactivity plain DENY: **PASS**, exit 0.
- exact production census: **PASS**, five named waits and one retained capability await.
- capability-region scan: **PASS**, no loop/spin/private pool/thread/blocking bridge or production
  `.into_parts().0`.
- scoped working/staged/HEAD diff checks: **PASS**, exit 0.
- Cargo, Nx, Wasm, browser, runtime, network, and native tests: **not run by instruction**.

The backend capability call remains the declared Phase 9 latency residual. The five wait groups and
Phase 1 remain RED pending independent source audit and later serialized build/runtime evidence.

## Controlled successor evidence repair

The single evidence rejection in
`📓️sol-independent-p1u-panic-wake-final-audit-2026-08-23.md` is repaired without changing the
production state machine:

- a test-only submit hook intercepts only the existing `schedule → submit_drive_job → drive_one`
  seam and moves each exact submitted job into an eight-slot fixed test queue;
- the controlled fixture calls `state.schedule()`, takes and runs the initial callback, and therefore
  enters `drive_one` with the same scheduled transition as production. It no longer pre-sets
  `scheduled` or directly invokes `poll_backend_once`;
- Pending, Ready, and panic futures fire their real waker during the first poll. The fixture observes
  exactly one queued successor, fires the retained real waker again after release, proves no second
  successor appears, and runs the exact queued job;
- Pending is repolled only by that next governed callback. Ready and panic keep their poll count at
  one while the successor advances retained cleanup and preserves the exact terminal work and Ready
  result owners; and
- the permanent verifier now inspects the live fixture body and fixed queue. Two additional faithful
  mutations restore the `scheduled = true` mask or drop the queued successor instead of executing
  it, bringing the P1u mutation count to 24.

Edition-2021 scoped Rustfmt, the 24-mutation interactivity self-test, plain DENY, exact five-wait/
one-retained-await census, and scoped plus whole working/staged/HEAD diff checks pass. Cargo, Nx,
Wasm, browser, runtime, network, and native tests remain unrun by instruction. This repair is
source-audit-ready, not accepted; the Phase 9 capability call, five wait groups, and Phase 1 RED
status are unchanged.
