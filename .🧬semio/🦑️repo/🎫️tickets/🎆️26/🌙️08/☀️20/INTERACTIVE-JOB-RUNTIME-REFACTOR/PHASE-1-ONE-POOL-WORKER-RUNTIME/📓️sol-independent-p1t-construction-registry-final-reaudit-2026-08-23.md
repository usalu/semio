# Sol Independent P1t Construction-registry Final Re-audit — 2026-08-23

## Audit admission

This is an independent Sol High source re-audit of the latest P1t construction-registry repair. I
did not author the implementation and made no production-source edits. I read the live source and
diff together with:

- `p1t-db-engine-retained-history-replay-2026-08-23.md`; and
- `sol-independent-p1t-public-terminal-construction-fault-final-audit-2026-08-23.md`.

The audit was limited to permitted Rust-2021 parsing/formatting, the root interactivity verifier and
its mutation self-test, exact source predicates/censuses, and diff checks. Cargo, Nx, Wasm, browser,
runtime, network, and root lint were not run.

## Verdict

**REJECT — source-only P1t construction-registry repair.**

The prior public raw-error escape is repaired. Reservation construction claims a fixed 64-slot
generation authority before its first reservation allocation; raw construction fault ownership is
crate-private; there is no construction `into_parts` or public raw cursor; the fallible scratch
allocation remains inside the private builder; builder unwind and unchecked checked-out fault Drop
return their exact partial owners and error to the registry; and `HistoryFuture::submit` installs the
fault owner in `terminal_construction` before taking its error for observable completion.

One generation/owner defect remains in the registry return path. The private construction token is
`Clone + Copy`, and `handback_history_replay_reservation_construction` neither checks slot bounds by
fallible access nor validates `occupied`, `checked_out`, and the exact generation. It unconditionally
rewrites the slot generation and assigns `error` and `cursor`. A stale or duplicated token can
therefore overwrite a newly reused slot and ordinary-drop that slot's exact retained error/cursor
owners inside the handback callback. This defeats the required ABA rejection and the claimed
infallible exact-owner handback.

The advertised ABA fixture does not exercise this case. It proves only that a replacement receives
a different generation. The permanent verifier similarly requires only the fixture name and checks
generation on `release`, not on `handback`; it has no stale/duplicate-handback mutation. An
independent focused predicate therefore rejects the live source even though the repository's
existing self-test passes.

Phase 1 remains **RED**. The six production DB-engine waits, bounded backend-syscall latency,
compilation/runtime evidence, timing/fairness evidence, and the native/Wasm/browser/platform matrix
also remain open independently of this focused rejection.

## Directly verified repairs

### Fixed pre-allocation construction authority

- `HISTORY_REPLAY_CONSTRUCTION_SLOTS` is exactly 64, backed by a fixed array and a monotonic
  generation scalar.
- `HistoryReplayReservationConstructionBuilder::new()` claims a free registry slot before the
  first source-slot, result-slot, page, range, entry, or scratch `try_reserve_exact`.
- Registry saturation returns a tokenless error before any partial reservation owner exists. The
  inspected 64/+1 fixture checks token absence, cursor absence, later slot reuse, and a different
  replacement generation.
- Source/result slots, each result page, operation ranges, entries, and scratch all remain fallible
  builder-owned allocations. Construction does not use `?` to escape a partial owner graph.

### Private raw fault and retained public delivery

- `HistoryReplayReservation::try_new` and
  `HistoryReplayReservationConstructionFault` are `pub(crate)`, not public API.
- There is no `HistoryReplayReservationConstructionFaultCursor` public type and no construction
  `into_parts` method. The public engine boundary exposes only
  `ArtifactHistoryTerminalConstructionFault` with retained resume/close/terminal operations.
- Builder Drop moves all partial roots plus its unwind error to the preclaimed slot. Fault Drop moves
  a checked-out error/cursor back to the registry; the inspected unchecked-error fixture retrieves
  the same three retained pages, drops the checked-out fault, retrieves it again, and cursor-closes
  it.
- The engine constructs `ArtifactHistoryState` with the fault already stored in
  `terminal_construction`; only afterwards does it call `take_error` and publish completion. Thus the
  partial roots and construction token stay retained while the error is observable.

### Close ordering and preserved replay invariants

- `HistoryReplayReservationCloseCursor::close_step` advances at most one source page, operation
  range, entry, result page, scratch root, or vector shell per call and returns immediately.
- `HistoryReplayReservationConstructionFault::close_step` first drives that cursor, then retires its
  cursor shell, error, and registry token on distinct successful calls. The engine retains its
  admission until nested terminal ownership has cleared.
- The outer replay still has the retained panic-to-`FaultRetire` path, public terminal ownership,
  cached accounting, bounded page/token processing, and no selected full-capacity scan in the
  inspected history region.
- The production DB-engine census is exactly six `db_actor::block_on` calls at current source lines
  1273, 1288, 1293, 1387, 1473, and 1482. The retained history region contains none.

## Blocking ABA defect

The live return function at the DB artifact construction registry currently performs this logical
sequence:

1. index `registry.slots[token.slot]`;
2. overwrite the slot generation with `token.generation`;
3. force `occupied = true` and `checked_out = false`; and
4. assign the returned error and cursor.

It does not first require that the slot is already occupied and checked out by that same generation,
nor that its error/cursor positions are empty. Because the token itself is copyable, the type system
does not provide uniqueness instead. Assignment to an occupied slot invokes ordinary Drop for the
displaced retained owners. Generation checks in `release_history_replay_reservation_construction`
and generation lookup in `take_history_replay_reservation_construction_fault` do not protect this
earlier overwrite.

The focused read-only predicate reported:

```text
rule: construction handback rejects stale/duplicate ABA before owner publication
validates: false
unconditionalOwnerOverwrite: true
copyableToken: true
verdict: REJECT
```

The corresponding verifier audit reported no handback-generation/state predicate and no
stale/duplicate-handback mutation, while confirming only the fresh-generation fixture is present.

## Required focused repair packet

1. Make checkout ownership definitionally unique: remove `Clone`/`Copy` from the construction token
   and prevent reconstruction or duplication outside the one checked-out registry lease.
2. Make handback generation/state safe before owner publication. It must accept only an occupied,
   checked-out, exact-generation, owner-empty slot. A stale/duplicate/malformed return must not
   overwrite or ordinary-drop either the current slot owners or the rejected returned owners.
   Prefer keeping error/cursor roots resident in the registry while a public checkout holds only a
   generation lease, so checkout Drop changes only scalar state.
3. Add a semantic ABA fixture that retains an old checkout/token, releases and reuses the same slot
   at a new generation, attempts stale and duplicate handback, and proves the new generation's exact
   error/page pointers remain retrievable and unchanged. Close both retained authorities one
   owner/scalar per distinct grant and prove terminal slot reuse.
4. Add permanent verifier mutations for removal of the handback occupied/checked-out/generation
   checks, reintroduction of a copyable token, unconditional `slot.error`/`slot.cursor` assignment,
   and removal of the stale/duplicate-handback fixture. Each mutation must fail the intended ABA and
   exact-owner predicate, not merely a fixture-name census.
5. Preserve the accepted public privacy, pre-allocation claim, builder/fault handback, scratch
   fallibility, engine retain-before-error order, outer terminal ordering, panic/`FaultRetire`, and
   six-wait census.

## Gates run

| Gate | Independent result |
| --- | --- |
| Rust-2021 `rustfmt --check` on replication codec, DB artifact, DB engine, DB CLI, and DB facade | **PASS**, exit 0, no diagnostics |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | **PASS**, exit 0; DENY clean; one approved test-only blocking finding |
| `bun ./📜️script.ts verify interactivity --format json` | **PASS**, exit 0; same clean baseline |
| Fixed registry/private fault/pre-allocation/scratch/engine publication predicate | **PASS** |
| Independent stale/duplicate handback predicate | **REJECT**; no state/generation validation, unconditional owner overwrite, copyable token |
| Permanent mutation audit | **REJECT**; no handback-state/ABA mutation; fresh-generation fixture-name check only |
| Production DB-engine wait census | **PASS**, exactly six; retained history region zero |
| Selected replay blocking/loop/full-capacity/raw-public scans | **PASS**, zero matches |
| Scoped working/staged/`HEAD` `git diff --check` | **PASS**, no whitespace errors |
| Whole working/staged/`HEAD` `git diff --check` | **Concurrent RED** only for `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:459` trailing whitespace and `PHASE-3-UI-THREAD-ISOLATION/📓️sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md:102` blank line at EOF; outside P1t scope |
| Cargo, Nx, Wasm, browser, runtime, network, root lint | **Not run; prohibited** |

## Audit conclusion

The public raw construction-fault escape, pre-allocation registration, checked-out owner recovery,
and engine retain-before-error ordering are source-repaired. The fixed registry is not yet ABA-safe
at its most important ownership publication function, and the current permanent evidence cannot
detect that defect. P1t is not ready for source acceptance until stale/duplicate handback is
definitionally impossible or rejected without losing either owner set, with a discriminating live
fixture and verifier mutation.
