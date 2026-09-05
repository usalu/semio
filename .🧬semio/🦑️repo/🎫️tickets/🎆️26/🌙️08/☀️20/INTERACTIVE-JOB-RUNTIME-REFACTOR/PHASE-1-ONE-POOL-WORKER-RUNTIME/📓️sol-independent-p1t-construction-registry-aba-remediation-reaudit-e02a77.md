# Sol Independent P1t Construction-registry ABA Remediation Re-audit — 2026-08-23

## Audit admission

This is an independent Sol High source re-audit of the P1t construction-registry ABA repair. I did
not author the implementation and made no production-source edits. I read the live source and diff,
the P1t implementation report, and
`sol-independent-p1t-construction-registry-final-reaudit-2026-08-23.md`.

The audit was limited to Rust-2021 parsing/formatting, repository interactivity self/plain DENY,
read-only source predicates and mutations, exact scans, and working/staged/`HEAD` diff checks.
Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run.

## Verdict

**ACCEPT — source-only P1t construction-registry ABA remediation.**

The rejected overwrite path is removed. The exact partial construction graph is anchored in its
fixed registry slot at claim, the checkout token is non-copyable, and handback compares bounds,
occupancy, checkout state, generation, and the resident cursor before changing only the
`checked_out` scalar. It never republishes or replaces an error, cursor, page, or generation owner.
A stale, duplicate, out-of-bounds, or mismatched return therefore cannot Drop either the current
graph or a rejected graph. The rejection remains usable as an exact slot/generation result while
the borrowed linear token remains with the caller and the current registry graph remains
retrievable.

The live semantic ABA fixture is discriminating: it releases and reuses the same slot, proves the
generation changes, attempts stale and out-of-bounds handbacks while a replacement is checked out,
checks the exact replacement result-page and error-string pointers after each rejection, performs a
normal Drop handback, rejects a duplicate handback, retakes the current generation, rechecks both
pointers, and cursor-closes it to terminal-empty. The fixed 64/+1 admission and fresh-generation
reuse fixture remains intact.

This acceptance does not accept P1t runtime behavior or Phase 1. Six named production DB-engine
wait groups, bounded backend-syscall latency, compilation/runtime timing and fairness evidence, and
the native/Wasm/browser/platform matrix remain RED.

## Direct ownership findings

### Registry residence from construction claim

- `HISTORY_REPLAY_CONSTRUCTION_SLOTS` is exactly 64 and the registry owns a fixed array of slots.
- Claim first finds a free slot and validates `next_generation.checked_add(1)`. Only then does it
  publish the occupied, checked-out generation with resident source-page, result-page,
  operation-range, entry, and scratch roots.
- The private builder carries only `Option<HistoryReplayReservationConstructionToken>`; all
  fallible root reservations and publications edit the exact resident slot after bounds,
  occupancy, checkout, generation, and error checks.
- Registry saturation returns an unregistered scalar error before a partial graph is constructed.
  The 64/+1 fixture retains all 64 registered faults, rejects the extra claim, closes one exact
  slot, and proves fresh-generation reuse.

### Linear token and non-overwriting handback

- `HistoryReplayReservationConstructionToken` derives only `Debug`, `PartialEq`, and `Eq`; it does
  not implement `Clone` or `Copy` and is private to the DB artifact module.
- `handback_history_replay_reservation_construction` uses `get_mut`, so an out-of-bounds token is an
  ordinary exact rejection rather than an indexed panic.
- Before mutation it requires `occupied`, `checked_out`, exact `generation`, and `cursor.is_some()`.
  Its sole live-state mutation is `slot.checked_out = false`.
- Handback does not assign `slot.generation`, `slot.error`, or `slot.cursor`. Rejected tokens contain
  no graph owner: every page/error/cursor remains resident in its existing generation slot.
- Builder unwind installs an error only after the same exact slot/generation/cursor comparison, then
  performs scalar handback. Checked-out fault Drop likewise performs only checked scalar handback.

### Checkout, close, and final release

- `take_history_replay_reservation_construction_fault` selects an occupied, not-checked-out exact
  generation, flips its checkout scalar, and returns a fresh linear token without moving the graph.
- Construction-fault close advances the resident close cursor first. The cursor releases at most
  one source page, operation range, entry, result page, scratch root, or vector shell per call.
  Error and registry-token retirement remain distinct subsequent opportunities.
- Final release requires bounds, occupied + checked-out, exact generation, and both resident error
  and cursor empty before clearing generation/occupancy/checkout scalars.
- Engine terminal ownership remains retained. `ArtifactHistoryTerminalConstructionFault` returns an
  interrupted owner to `terminal_construction`; the public terminal witness still requires all
  roots and checked-out flags empty, admission absent, finished, and no scheduled/retry authority.
  Admission release remains the distinct final close opportunity.
- Panic still transitions the replay into retained `FaultRetire`; cached accounting remains O(1).
  The retained history region has no production blocking bridge.

## Independent mutation reconstruction

I evaluated a focused predicate against the actual DB artifact source, not a copied good-source
fixture. The baseline passed. Each of the following actual-source mutations independently failed:

1. replace fallible slot `get_mut` with direct indexing;
2. remove the occupied comparison;
3. remove the checked-out comparison;
4. remove the exact generation comparison;
5. remove the resident cursor witness;
6. restore `Clone + Copy` on the checkout token;
7. insert unconditional error/cursor assignment before handback;
8. remove registry-resident cursor construction from claim; and
9. remove the real stale/duplicate/out-of-bounds pointer-preservation fixture.

The permanent root verifier contains the corresponding handback checks and mutation classes. Its
self-test completed successfully, so the repaired source is not accepted by fixture-name presence
alone.

## Exact scans and gates

| Gate | Independent result |
| --- | --- |
| Rust-2021 `rustfmt --check` on replication codec, DB artifact, DB engine, DB CLI, and DB facade | **PASS**, exit 0, no diagnostics |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | **PASS**, exit 0; DENY clean, one approved test-only blocking finding |
| `bun ./📜️script.ts verify interactivity --format json` | **PASS**, exit 0; same clean baseline |
| Actual-source construction-registry predicate | **PASS**: fixed resident graph, linear token, exact checked non-overwriting handback, guarded release |
| Independent focused mutations | **PASS**: baseline true and all 9 mutations rejected |
| Pointer/ABA fixture inspection | **PASS**: stale, out-of-bounds, duplicate, exact page/error pointers, current-generation retake and terminal close |
| Capacity/generation fixture inspection | **PASS**: 64/+1 and fresh generation reuse |
| Public close/admission preservation | **PASS**: one nested owner per grant and distinct final admission release remain |
| Production DB-engine wait census | **PASS**: exactly six non-test `db_actor::block_on` sites; retained history region zero |
| Raw escape / bulk scan | **PASS**: no public construction token/fault, construction `into_parts`, `mem::forget`, selected fixed-capacity scan, or retained-history blocking bridge |
| Scoped working/staged/`HEAD` `git diff --check` | **PASS** |
| Whole working/staged/`HEAD` `git diff --check` | **Concurrent RED outside P1t**: trailing whitespace in `🐙️ueli.md:459` and blank EOF lines in `p3j-prepared-raster-producer-census-2026-08-23.md:53` and `sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md:102` |
| Cargo, Nx, Wasm, browser, runtime, network, root lint | **Not run; prohibited** |

## Residuals

This focused source acceptance preserves the broader red status:

- six named production DB-engine blocking wait groups remain;
- bounded backend-syscall latency is not proved;
- compile, native/Wasm/browser runtime, timing, cancellation, and fairness evidence is unrun; and
- P1t and Phase 1 remain RED until those independent gates close.

## Audit conclusion

The construction registry now rejects stale, duplicate, out-of-bounds, and mismatched handback
without overwriting or dropping either owner set. Exact current-generation page/error pointers
remain retrievable, final release is terminal-guarded, and the permanent and independently
reconstructed mutations detect every formerly missing check. The P1t construction-registry ABA
remediation is accepted at source level only.
