# Sol Independent P1t Retained History Terminal Remediation Second Re-audit — 2026-08-23

## Audit admission

This is an independent Sol High source audit of the current P1t retained history replay remediation.
I did not author the P1t implementation and made no production-source edits. I read the current
implementation and diff together with:

- `p1t-db-engine-retained-history-replay-2026-08-23.md`;
- `sol-independent-p1t-db-engine-retained-history-replay-audit-2026-08-23.md`; and
- `sol-independent-p1t-terminal-fixed-owner-remediation-reaudit-2026-08-23.md`.

The audit was limited to the permitted source, formatting, verifier, census, and diff evidence. No
Cargo, Nx, Wasm, browser, network, root lint, or runtime command was run.

## Verdict

**REJECT — source-only P1t retained-history terminal remediation.**

The replay-internal remediation is materially improved and directly verifies: the twelve retained
phase variants remain owned across the injected pre-transition panic, the runner converts the panic
to `FaultRetire` and reschedules, cleanup advances one retained phase/page/range/entry/result
page/scratch/backing opportunity at a time, cached accounting replaces the rejected cumulative
fixed-capacity scans, and `Complete` is not installed before live phase work.

The packet is nevertheless not source-acceptable because the public terminal-handle close path can
publish a false terminal-empty witness while retaining the operation admission forever. In addition,
the partial fixed-reservation construction-fault path identified by the prior independent audit still
bulk-drops every already allocated page during `try_new` error propagation. The permanent verifier
and new pre-handoff fixture do not exercise either live failure.

Phase 1 remains **RED**. The six other production DB-engine waits, backend syscall duration,
compilation, runtime behavior, fairness/timing, and the native/Wasm/browser/platform matrix remain
open independently of this rejection.

## Directly verified remediation

### Pre-handoff ownership foundation

- A successfully constructed `ArtifactHistoryAdmission` cannot ordinarily drop a live reservation:
  its `Drop` asserts `reservation.is_none()` before releasing the generation slot and aggregate byte
  credit.
- Cancel, stale-generation detection, final WorkerPool rejection, and abandoned-future handling all
  terminalize an unhanded `Request` through `begin_unhanded_reservation_close`; the exact completed
  reservation moves into `HistoryReplayReservationCloseCursor` before the admission can be released.
- `HistoryReplayReservationCloseCursor::close_step` retires one occupied source page, operation
  range, entry, result-page slot, scratch owner, or empty vector backing per call. Its Drop asserts an
  exact terminal witness, and a cursor can resume only before retirement begins.
- The fixed eight-slot generation authority remains occupied while the reservation cursor is live.
  The direct `artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release`
  fixture demonstrates this for a manually extracted admission/cursor and proves generation changes
  on subsequent slot reuse.

### Replay phase and fault retirement

- `HistoryReplayFuture` stores `phase: Option<HistoryReplayPhase>` separately from
  `HistoryReplayTransition::{InProgress, FaultRetire, Complete}`. Live work borrows the current phase
  instead of replacing it with a premature completion marker.
- `Complete` is installed only after a successful view transfer or after `FaultRetire` has removed the
  phase and driven `close_step` to no progress. There is no `HistoryReplayPhase::Complete` or
  pre-work `mem::replace(...Complete)` path.
- The adversarial phase fixture explicitly constructs all twelve current variants: `Probe`,
  `SegmentLen`, `PageStart`, `PageRead`, `Frame`, `Envelope`, `CopyMutation`, `Frontier`,
  `ClearPending`, `Publish`, `Retire`, and `FinalizeSuccess`. It injects the panic before transition
  commit, verifies the active phase remains present, requests fault retirement, polls the retained
  cursor, and asserts terminal-empty for every variant.
- The actor runner catches a replay panic while the turn remains stored, calls `request_close`, closes
  ingress, and calls `schedule`; it does not take or destroy the active history turn.
- Backend-owned rejected pages are parked in `terminal_page`; source pages use cached `page_count`;
  the reservation cursor uses cached `source_page_count`. Each is removed on a distinct cleanup
  opportunity.

### Fixed accounting and live cursor bounds

- `retained_operation_bytes` and `retained_result_bytes` are computed once from admitted capacities;
  segment admission reads the cached operation credit and result publication uses the cached result
  credit.
- `source_page_count` is maintained on source-page publication and decremented directly on retirement.
- The production `HistoryReplay` region contains none of `.rposition(`,
  `result_pages.iter()`, `source_pages.iter().all`, `pages.iter().all`, or
  `pages.iter().filter`. Result item accounting uses O(1) vector lengths.
- Each storage read requests at most one 16-KiB page. CRC, frame tokenization, envelope/frontier
  decoding, raw command ranges, and result copying retain their previously audited bounded cursors.
- The source verifier has 22 retained-history mutations, including pre-handoff reservation drop,
  premature complete, cleanup stranding, fixed-owner `rposition`, cached-source-count removal,
  capacity/credit loss, stale-after-handoff, and bulk page retirement. All were exercised by both
  permitted interactivity invocations.

### Production wait census

The production portion of DB engine contains exactly six executable `db_actor::block_on` calls:

1. storage capabilities during open;
2. catalog-root read during open;
3. initial catalog-root CAS during open;
4. create-document catalog CAS;
5. compact-document; and
6. sync hello.

The removed history helper and direct `db_wal::replay_document(&storage.wal...)` history bridge remain
absent.

## Blocking findings

### 1. Public terminal close reports empty before releasing the admission

`ArtifactHistoryState::finish_if_terminal_empty` is the sole path that takes the admission, triggers
its exact slot/byte-credit release, and unregisters the operation. The public
`ArtifactHistoryTerminalHandle::close_step`, however, only evaluates
`self.state.close_one() || self.state.authority.close_step()` and never calls
`finish_if_terminal_empty`.

This becomes terminally incorrect on the last public close grant:

1. `close_one` removes the final terminal result and returns `true`;
2. `ArtifactHistoryState::terminal_is_empty` now returns `true`, because it checks jobs, work,
   completion, reservation, and checkout ownership but does **not** check `admission` or `finished`;
3. `ArtifactHistoryTerminalHandle::terminal_is_empty` forwards that false witness; and
4. the admission remains `Some`, its slot remains occupied, and the registry's `Arc` keeps the state
   alive. No later close call can fix it: another public `close_step` still does not invoke
   `finish_if_terminal_empty`.

This is the actual recovery API for a dropped/abandoned `HistoryFuture`, so the direct raw-admission
fixture does not prove the live contract. Cancel/stale/reject can cursor-retire every reservation
owner and still permanently retain the aggregate operation admission after reporting terminal-empty.

The same omission is visible by comparison with `HistoryFuture::close_step`, which performs the
identical owner step and then explicitly calls `finish_if_terminal_empty`.

### 2. Partial reservation-construction rejection still bulk-drops accumulated pages

`HistoryReplayReservation::try_new` allocates 960 independent result pages in a `for` loop. A
`try_reserve_exact` failure on page N returns `DbError` through `?`; Rust then ordinarily destroys
the `result_pages` local and all N already-populated 16-KiB page owners in the same stack return.
Failures while reserving the later operation-range or entry backing have the same problem after all
960 pages exist.

The outer `ArtifactHistoryAdmission::try_claim` only receives the final `Err`; it has no rejected
reservation-builder owner to publish into the terminal registry. Its shallow claim releases the
aggregate admission after `try_new` has already bulk-dropped the partial graph. This is the
construction-fault part of the prior independent repair packet and remains unchanged.

The cap/+1 tests exercise logical byte/item admission against a successfully built reservation. They
do not inject an allocation/build failure after many populated page owners or prove one-owner cleanup
for the rejected builder.

## Fixture and verifier discrimination

- The twelve-phase panic fixture is now meaningful for the intended panic-transition rule and was
  accepted as evidence.
- The cached-accounting fixture and no-scan verifier mutations discriminate the previously rejected
  `rposition`/fixed-capacity scan behavior.
- The full pre-handoff cancellation fixture manually owns `ArtifactHistoryAdmission`, calls
  `begin_reservation_close`, drives the raw cursor, and manually drops the admission. It never creates
  `ArtifactHistoryState`, abandons a `HistoryFuture`, retrieves
  `ArtifactHandle::history_terminal`, or drives `ArtifactHistoryTerminalHandle::close_step`.
- The permanent verifier requires the names of terminal APIs and reservation transfer, but has no
  predicate requiring public terminal close to invoke the admission-release witness or requiring the
  public terminal predicate to include released admission/finished state. Both verifier invocations
  therefore pass against the live false-terminal path.
- No allocator/build-fault injection fixture or mutation exists for partial reservation construction.

## Gates run

| Gate | Independent result |
| --- | --- |
| Rust-2021 scoped `rustfmt --check` on replication codec, DB artifact, DB engine, DB CLI, and DB facade | **PASS**, exit 0, no diagnostic |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | **PASS**, exit 0; DENY clean; one allowlisted blocking-bridge finding |
| `bun ./📜️script.ts verify interactivity --format json` | **PASS**, exit 0; same baseline |
| Independent retained-history predicate | **REJECT**, while confirming six waits, twelve panic phases, cached accounting, and no prohibited capacity scan; detected missing public admission witness |
| Production DB-engine wait census | **PASS**, exactly six production calls; selected history bridge absent |
| Scoped and whole working/cached/`HEAD` `git diff --check` | **PASS**, no whitespace error |
| Scoped source diff inspection | Completed; six scoped sources are +3,628/-271 relative to `HEAD`; unrelated concurrent edits were preserved |
| Cargo, Nx, Wasm, browser, network, root lint, semantic/runtime/timing gates | **Not run; prohibited** |

The failed independent predicate printed:

```text
P1t independent predicate: REJECT; waits=6; phases=12; cached accounting/no-scan=PASS; public terminal close admission witness=MISSING
```

## Required focused repair packet

1. Split the internal “nested owners are empty” predicate from the public exact terminal witness.
   `finish_if_terminal_empty` should consume the admission and unregister only after nested owners,
   scheduling, retry, and checkouts are empty. Public `terminal_is_empty` must additionally require
   `admission.is_none()` and the finished/unregistered witness.
2. Make every public terminal close route, especially
   `ArtifactHistoryTerminalHandle::close_step`, invoke that finish transition after its single owner
   opportunity without adding a second owner disposal in the same grant.
3. Add a live fixture that fills all eight admissions, abandons/cancels a pre-handoff future, retrieves
   the public terminal handle, pumps `close_step`, verifies terminal remains false until the admission
   is actually released, then proves exact slot reuse with a new generation. Cover stale authority and
   final WorkerPool rejection through the same public route.
4. Add verifier mutations removing the public finish call, removing admission/finished from the public
   witness, and substituting the current false terminal predicate. Each must fail the intended exact
   rule.
5. Replace fallible monolithic `HistoryReplayReservation::try_new` error unwinding with a retained
   builder/rejected-owner authority. On page/range/entry/scratch allocation failure, return the partial
   owner to a cursor that retires one page/backing/scalar per grant before admission release. Add an
   injectable failure after many populated pages plus exact pointer/slot-release ordering fixtures and
   a mutation restoring `?`-driven bulk unwind.

## Residual status

P1t remains source-**REJECTED**. Independently, the six named DB-engine waits, page-bounded backend
syscall duration, build/type validation, runtime semantics, cancellation/fairness/timing evidence, and
the native/Wasm/browser/platform matrix keep Phase 1 RED.
