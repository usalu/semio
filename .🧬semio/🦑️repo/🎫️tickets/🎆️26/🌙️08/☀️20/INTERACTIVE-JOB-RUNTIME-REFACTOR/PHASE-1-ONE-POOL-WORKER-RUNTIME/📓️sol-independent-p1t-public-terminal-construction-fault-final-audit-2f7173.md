# Sol Independent P1t Public-terminal and Construction-fault Final Audit — 2026-08-23

## Audit admission

This is an independent Sol High source audit of the repaired P1t public-terminal and reservation-
construction-fault packet. I did not author the implementation and made no production-source edits.
I read the live source and diff together with:

- `p1t-db-engine-retained-history-replay-2026-08-23.md`; and
- `sol-independent-p1t-retained-history-terminal-remediation-second-reaudit-2026-08-23.md`.

The audit was limited to the permitted source, Rust-2021 parser/formatting, interactivity verifier,
exact census, and diff evidence. Cargo, Nx, Wasm, browser, runtime, network, and root lint were not
run.

## Verdict

**REJECT — source-only P1t public-terminal/construction-fault packet.**

The two previously reported outer-state defects are substantially repaired. Public terminal close
now gives the state roots and actor authority at most one opportunity each through early-return
grants, and admission/registry release occurs only on a later final grant. The public terminal
witness includes `finished`, empty admission, every state root/check-out, scheduling/retry state, and
the actor authority. Reservation construction failures also move already allocated roots into the
registered `ArtifactHistoryState`, and the checked-out
`ArtifactHistoryTerminalConstructionFault` has exact take/resume/Drop handback.

One public construction-error escape nevertheless remains. The lower-level public
`HistoryReplayReservation::try_new() -> Result<Self,
HistoryReplayReservationConstructionFault>` exposes an unregistered raw error owner.
`HistoryReplayReservationConstructionFault` has no Drop handback, and its public
`HistoryReplayReservationConstructionFaultCursor` has only an assert-on-nonterminal Drop. A caller
that drops the unchecked `Err`, or calls `into_parts` and drops the returned nonempty cursor, does
not transfer the partial result-page graph to the fixed terminal registry. Its Drop path instead
panics while the nested nonempty cursor remains in ordinary drop glue. That is not the required
retained construction-error/Drop handback contract and leaves an ordinary unwind/abort route at the
very boundary this remediation was intended to close.

Phase 1 remains **RED**. The six remaining production DB-engine waits, bounded-backend syscall
latency, compilation/runtime/timing evidence, and the native/Wasm/browser/platform matrix remain
open independently of this focused rejection.

## Directly verified repairs

### Public terminal ordering and witness

- `ArtifactHistoryState::terminal_roots_are_empty` requires empty retry job, terminal job, terminal
  work, terminal result, reservation cursor, construction cursor, active work, and completion, plus
  both checkout bits clear.
- `ArtifactHistoryState::terminal_is_empty` additionally requires `finished`, `admission.is_none()`,
  no scheduled closure, and no armed retry.
- Both `HistoryFuture::close_step` and `ArtifactHistoryTerminalHandle::close_step` return immediately
  when `state.close_one()` advances. They next give `ArtifactAuthority::close_step` one opportunity
  and return immediately if it advances. Only when neither path advances do they invoke
  `finish_if_terminal_empty`.
- `finish_if_terminal_empty` rechecks every state root and scheduler bit, takes an admission only
  after its reservation is absent, releases the exact admission slot, unregisters the operation
  generation, and publishes `finished`. The semantic fixture confirms the admission remains live
  while roots remain, the release occupies the subsequent grant, the generation is unregistered,
  and the slot is reusable with a fresh generation.
- The public `terminal_is_empty` combines the exact state witness with
  `ArtifactAuthority::terminal_is_empty`; actor authority cannot be omitted from terminality.

### Registered construction-fault path

- `HistoryReplayReservation::try_new_retained` no longer uses `?` for result-page, operation-range,
  or entry backing reservation. Each explicit error moves all accumulated vectors into
  `HistoryReplayReservationConstructionFault`.
- `ArtifactHistoryAdmission::try_claim` preserves the already claimed generation/item/byte slot and
  moves the construction cursor into `ArtifactHistoryAdmissionError::Construction`.
  `HistoryFuture::submit` then installs both admission and cursor in the fixed
  `ArtifactHistoryState` before publishing the exact error as completion.
- `ArtifactHistoryState::close_one` advances the registered construction cursor one opportunity at
  a time. Result pages are popped singly. Range, entry, scratch, and vector backing owners are
  separate cursor grants, and admission release is deferred until the cursor and its registered
  shell have both retired.
- `ArtifactHistoryTerminalConstructionFault` uses an atomic checkout bit, supports retained
  `resume`, advances one close opportunity, and returns an unchecked checked-out owner to
  `terminal_construction` from Drop. Mutex poison is recovered without discarding the owner.
- The injected construction fixture covers actual failure at zero, one, midpoint, cap-minus-one,
  and cap result-page boundaries. A second sweep exercises every partial page count from zero
  through 960 and checks one page decrement per close call. A cap-plus-one injection cannot create a
  961st page owner.

### Preserved accepted invariants

- The replay enum still has exactly twelve retained phases: probe, segment length, page start, page
  read, frame, envelope, mutation copy, frontier, clear pending, publish, retire, and final success.
- Panic injection retains the current `Option<HistoryReplayPhase>` and the runner requests
  `FaultRetire`; completion is not installed before the retained phase/root graph is empty.
- Source/result item and byte accounting uses cached counters. The history replay region contains no
  `rposition`, whole-capacity `iter().all`, result-page iteration, loop, `block_on`, whole-segment
  read, or full-frame CRC path.
- The production DB-engine census remains exactly six `db_actor::block_on` calls, all outside the
  retained history region.

## Remaining rejection: unchecked public construction fault

The safe outer wrapper does not make the raw public boundary safe:

1. `db_artifact::HistoryReplayReservation::try_new` is public and returns the public raw fault.
2. `HistoryReplayReservationConstructionFault::into_parts` is public and returns the public raw
   cursor.
3. `HistoryReplayReservationConstructionFault` has no Drop implementation or registry reference.
4. `Drop for HistoryReplayReservationConstructionFaultCursor` only asserts
   `terminal_is_empty`; it cannot hand the exact cursor back to `ArtifactHistoryState` or another
   fixed close owner.
5. All live implementation call sites eagerly call `into_parts`, and all fixtures do the same.
   Therefore neither fixtures nor verifier mutations exercise `drop(Err(fault))` or dropping the
   cursor returned by `into_parts` before terminal retirement.
6. The verifier checks the presence of the registered outer handle and absence of the former `?`
   spelling, but has no rule rejecting a public raw construction fault/cursor with assert-only Drop.

This is a source-level authority defect even though it requires a real allocation failure to reach
through the production `try_new` call. Allocation failure is precisely the construction-fault path;
it cannot be exempted from the ownership contract.

## Required focused repair packet

1. Remove the public raw `Result`/cursor escape. Construction must begin under a fixed registered
   construction authority before its first fallible allocation. Expose only a generation-keyed
   retained handle whose Drop returns the exact partial builder to that registry.
2. Keep the raw reservation builder and close cursor private to the owning construction authority.
   If cross-crate ownership requires a public type, make its Drop transfer to a fixed quarantine by
   construction; an assert-only public Drop is not sufficient.
3. Preserve the successful outer ordering: partial pages/backings first, cursor shell next,
   admission/registry on the distinct final grant. A checked-out raw fault or cursor must keep the
   public terminal witness false.
4. Add live engine-level fault injection for every result-page boundary through
   `HistoryFuture::submit`, not only direct low-level `try_new_with_result_page_failure` calls. For
   each boundary, prove the exact state registry owns all partial roots before the error is
   observable, unchecked handle Drop hands ownership back, close decrements one real page/root per
   grant, and the admission slot is reused only after terminal completion.
5. Add separate operation-range, entry, source-slot, scratch/backing construction fault injections.
   Each error must retain the exact previously allocated owner set and follow the same public
   handback path.
6. Add verifier mutations for a reintroduced public raw `try_new` error, removal of raw-fault Drop
   handback, assert-only cursor Drop, direct `into_parts` escape, and a fixture that discards the
   checked-out error/cursor. Each mutation must fail the intended ownership predicate.

## Gates run

| Gate | Independent result |
| --- | --- |
| Rust-2021 `rustfmt --check` on replication codec, DB artifact, DB engine, DB CLI, and DB facade | **PASS**, exit 0, no diagnostics |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | **PASS**, exit 0; DENY clean; one approved test-only blocking finding |
| `bun ./📜️script.ts verify interactivity --format json` | **PASS**, exit 0; same baseline |
| Independent public-terminal predicate | **PASS**; distinct state-root, authority, and admission grants; exact state plus authority witness |
| Independent construction-fault ownership predicate | **REJECT**; public raw fault/cursor has no registry Drop handback |
| Production DB-engine wait census | **PASS**, exactly six calls; retained history region has zero |
| Retained phase/cached-accounting scans | **PASS**, twelve phases; zero selected blocking/loop/capacity-scan patterns |
| Scoped working/staged/`HEAD` `git diff --check` | **PASS**, no whitespace errors |
| Whole working `git diff --check` | **PASS** |
| Whole staged and `HEAD` `git diff --check` | **Concurrent RED**, solely `PHASE-3-UI-THREAD-ISOLATION/📓️sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md:102` (`new blank line at EOF`); outside P1t scope |
| Cargo, Nx, Wasm, browser, runtime, network, root lint | **Not run; prohibited** |

## Audit conclusion

The repaired public terminal ordering and registered outer construction-fault owner are source-
sound within the inspected path, and the accepted retained replay/panic/accounting invariants remain
intact. P1t is not ready for source acceptance until allocation failure cannot escape through the
public raw fault/cursor Drop path and the registry handback is exercised through the live engine
boundary.
