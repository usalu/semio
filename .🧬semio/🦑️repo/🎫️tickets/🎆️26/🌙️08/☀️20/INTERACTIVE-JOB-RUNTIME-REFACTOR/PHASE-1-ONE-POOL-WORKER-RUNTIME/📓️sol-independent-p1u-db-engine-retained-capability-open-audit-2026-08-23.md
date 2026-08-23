# Sol Independent P1u DB-engine Retained Capability-open Audit — 2026-08-23

## Audit admission

This is an independent source audit of the P1u capability-open packet. I did not edit production
source or the verifier. I read the P1u caller census and implementation report, the accepted P1r
and P1s reports, the current DB engine/facade and verifier, and the current working/staged/HEAD
diffs. Cargo, Nx, Wasm, browser, runtime, network, and native integration were prohibited and were
not run.

## Verdict

**REJECT — source packet.** The selected capability `block_on` is gone, the five-group residual
census is exact, the fixed admission constants and eight named phases exist, and the backend
capability query is truthfully one indivisible platform residual. The live authority nevertheless
has a readiness/owner publication race, the live `open_with` rejection path discards the exact
storage owner, the public terminal resume path cannot recover `terminal_result`, and the retry
generation path still contains a spin loop. The permanent predicate's fifteen mutations do not
exercise these paths.

This rejection does not alter the accepted P1r/P1s source packets and does not accept Phase 1.

## Accepted evidence retained

### Census and call graph

The production source before `//#region 🧪️Tests` has exactly five `db_actor::block_on` groups:

1. catalog-root read;
2. empty-catalog initialization CAS;
3. create-document catalog CAS;
4. backend compaction; and
5. sync hello.

There is no production `db_actor::block_on(storage.capabilities())`. The capability region contains
one `storage.capabilities().await`, inside `DatabaseCapabilityOpenWork::new`. `Database::open`,
`open_at`, `open_with_emit`, and `open_with_authz` all reach the retained `open_with` seam. No
capability-region `block_on`, `submit_blocking`, `ask_blocking`, subsystem `WorkerPool::new`, or
thread spawn remains.

### Fixed authority and separated phases

The source has 64 generation-keyed slots, 8 items and 16 KiB per operation, and checked 512-item / 1
MiB process aggregates. Admission precedes `DatabaseCapabilityOpenWork::new(storage)`. The live
state names and dispatches `Handoff → Poll → RetainWork → DrainWork → ReleaseWork → RetainResult →
Publish → Terminal`; `drive_one` matches one phase and the Handoff arm does not poll. The backend
poll is wrapped in `catch_unwind`, and publication textually checks cancellation and admission
freshness before completion. Saturated jobs use `error.into_job()`, a generation-tagged timer-wheel
callback, and an I/O-lane resubmission. Success and direct `open_retained` admission rejection both
carry the original `Arc<DbBackend>`.

These structural successes are insufficient because the following live paths violate the packet's
ownership and wake contract.

## Blocking findings

### 1. Readiness can run before the exact poll owner is republished

In `DatabaseCapabilityOpenState::poll_backend_once`, the exact work owner is removed from
`poll_work`, polled, and then `polling` is cleared **before** any `Poll::Pending`, `Ready`, or panic
arm puts the work owner back:

```rust
let polled = catch_unwind(... work.poll(...));
self.polling.store(false, Ordering::Release);
match polled {
    Poll::Pending => *self.poll_work.lock(...) = Some(work),
    ...
}
```

A backend wake in that interval sees `polling == false`, passes the generation/current checks, and
schedules a second Poll grant. On a multi-worker pool that grant can run before reinsertion, observe
`poll_work == None`, and terminalize with “poll owner missing”. This is a false fault caused by the
wake protocol itself; the original closure can subsequently put the owner back after terminal
phase selection. Coalesced scheduling does not close the interval because `drive_one` cleared
`scheduled` before entering the poll.

The exact owner/staged result and phase must be durably republished before the polling guard is
cleared. Pending wake consumption must then re-arm at most one successor from the stable state.

### 2. The live constructor discards the exact rejected storage owner

`Database::open_with` currently uses:

```rust
Self::open_retained(pool.clone(), storage)
    .map_err(|rejected| rejected.into_parts().0)?;
```

`into_parts()` correctly returns `(DbError, Arc<DbBackend>)`, but selecting `.0` ordinary-drops the
exact storage owner on the live Hub/CLI/framework constructor route. The exact handback exists only
for direct callers of `open_retained`; the selected production path hides and destroys it. The live
open result must expose an owner-bearing rejection or place that exact owner in a public retained
terminal handback before returning the error.

### 3. Cancel/stale-after-ready results cannot be taken or resumed

If cancellation or staleness is observed after a backend result has been staged,
`RetainResult` moves the exact `DatabaseCapabilityOpenResult` into `terminal_result` and publication
places the error in `terminal_completion`. `DatabaseCapabilityOpenTerminalHandle::resume` checks
terminal/retry jobs, `terminal_work`, and `terminal_completion`, but never checks
`terminal_result`. It can therefore resume the error while leaving the exact storage/result owner
parked, or return `Err(self)` when only `terminal_result` remains. The only public action that
consumes this root is `close_step`, which drops it.

There is no public `take_terminal_result`/result-bearing resume path. The packet's exact terminal
take-resume-close contract is therefore incomplete for the post-backend cancel/stale branch.

### 4. Retry generation still spins

`arm_retry` increments `retry_generation` with an unbounded `loop` around
`compare_exchange`. It is the only forbidden loop in the capability region and can run from a
rejected successor-submission path. A retained one-opportunity authority must use a single checked
scalar transition (or park and re-arm on contention), not spin inside the scheduling call.

## Fixture and mutation assessment

The four authored Rust fixtures cover fixed admission cap/+1 and slot ABA, success pointer
identity, cancel plus a manually stale admission, and queue saturation/resume. They were inspected
but not executed because builds/runtime were prohibited. They do not discriminate:

- a wake between clearing `polling` and restoring `poll_work`;
- rejection through the live `open_with` wrapper with exact pointer handback;
- cancel/stale after `Poll::Ready`, followed by terminal-result take/resume;
- panic with exact terminal work/result recovery; or
- retry-generation contention without a loop.

The root verifier executes all fifteen P1u synthetic mutations on every interactivity run. I
reconstructed the set from `interactivityDatabaseCapabilityOpenSelfTests`: old capability block,
sixth engine wait, unbounded byte cap, missing aggregate item credit, combined Handoff/Poll, poll
loop, missing RetainWork, missing ReleaseWork, dropped saturated job, missing drive generation
check, missing publication cancel check, missing publication stale check, missing rejection
`into_parts`, missing terminal handle take, and missing cap/+1 fixture. Both permitted verifier runs
advanced past those mutation assertions, so all fifteen were denied and the unmodified P1u textual
predicate was accepted.

That corpus is not sufficient: it does not mutate poll-owner publication ordering, the live `.0`
owner discard, terminal-result retrieval/resume, or the retry CAS loop. Several fixtures are only
presence-checked by name.

## Required repair packet

1. Republish the exact work owner and staged output/phase before clearing `polling`; after the state
   is stable, consume the coalesced wake and schedule at most one successor. Add a synchronized
   late-wake fixture that deterministically exercises the former interval, plus wake-storm and ABA
   variants.
2. Replace `.map_err(|rejected| rejected.into_parts().0)` with an owner-bearing live open rejection
   or a public retained handback. Add direct and `open_with` saturation/cap rejection fixtures that
   compare the exact storage pointer.
3. Give `terminal_result` an exact public take and a result-aware resume path. Add cancel/stale
   immediately after backend readiness, future/terminal-handle Drop handback, retry, close, and
   terminal-empty fixtures.
4. Replace the retry-generation CAS loop with one checked scalar opportunity or retained
   contention transition. Add retry contention/overflow and quiet saturation fixtures.
5. Add faithful verifier mutations for all four classes. Mutations must preserve the rest of the
   accepted source so each fails for the intended P1u predicate.

## Gates

| Gate | Result |
| --- | --- |
| scoped edition-2021 `rustfmt --check` on DB engine/facade | **PASS** |
| exact capability block census | **PASS**: selected block zero; exactly five named production groups remain |
| capability-region forbidden scan | **REJECT**: zero nested executor/thread/pool/blocking calls, but one retry CAS `loop` remains |
| fixed admission and phase scan | **PASS**: 64 / 8 / 16 KiB / 512 / 1 MiB and all eight phases present |
| root interactivity self/plain DENY | **RED outside P1u**: both runs reached all P1u mutation checks and report no P1u finding, then fail on four concurrent P3 prepared-raster findings |
| scoped working/staged/HEAD diff checks | **PASS** |
| whole working diff check | **PASS** |
| whole staged/HEAD diff checks | **RED outside P1u**: a Phase 3 raster audit has a blank line at EOF and the shared user prompt has trailing whitespace |
| Cargo/Nx/Wasm/browser/runtime/network/native | **not run by instruction** |

## Residual status

P1u is source-rejected on the four findings above. Even after repair, the one backend
`storage.capabilities()` poll remains an honest indivisible Phase 9 platform-latency residual; the
five named DB-engine wait groups remain; and Phase 1 remains RED.
