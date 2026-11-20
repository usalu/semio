# 📏️ A4 — semio-ui-contract-conformance size-derived fixtures

## Result
- Before: 38 passed / 5 failed (`a4-before`). After: 43/43 passed, both `--test-threads=1` and the default parallel run.
- An earlier parallel run (`a4-after`) timed out (exit 124). One test (`records::document_lease_publishes_complete_generation_and_closes_incrementally`) hung and 11 arena-bound tests failed. That run printed no panic messages. A serial rerun and then a parallel rerun on identical sources both passed 43/43, so this looks transient, perhaps host load or the concurrent edits to the `UiArenaRegistry` lock. Reproduction: none.
- TS twin `🧪️tests/🔬️fixed-list-storage/🟦️.ts` reads the same JSON fixtures, so it stays in lockstep automatically. Ran via `a4-ts-twin.ts` (`node --experimental-strip-types`): `fixed-list-page-oracle checks=75` on the new numbers.

## Measured (via temporary `[DEBUG]` eprintln, removed)
size_of UiNodeRecord=7168, UiPatchOp=7168, Component=3784, InputProps=3784, Option<UiText>=516, Vec<UiPatchOp>=24; full 128-node document limits {items:130, bytes:933762}; contract_backing_bytes=90056; required_open_bytes=1922; (aggregate-backing)/full = 62.

## Numbers old -> new
| fixture (under `🔨️modules/🖱️ui/🧬️contract/`) | field | old | new |
|---|---|---|---|
| 🎟️resident/🔄️refresh/🧪️tests/🧫️fixtures/🔣️.json | aggregateBytes | 58392576 | 58720256 |
| | documentBytes | 912384 | 917504 |
| | nodeRecordBytes | 7128 | 7168 |
| | openBytes | 1906 | 1922 |
| | fullDocumentBytes | 928546 | 933762 |
| | fixedBackingBytes | 2077760 | 90056 |
| | fullDocumentAdmittedRoots / RefusedRoot | 60 / 61 | 62 / 63 |
| | peakResidentBytes | 3589692 | 3609868 |
| | doubleBufferedPeakBytes | 7179384 | 7219736 |
| | laws[4] | sixty / sixty-first / four free | sixty-two / sixty-third / two free |
| 🎟️resident/🧪️tests/🧫️fixtures/🔣️.json | documentBytes, aggregateBytes | 912384, 58392576 | 917504, 58720256 |
| 🎟️resident/🗃️fixed/🧪️tests/🧫️fixtures/🔣️.json | aggregateBytes | 58392576 | 58720256 |
| 🎟️resident/🌳️root/🧪️tests/🧫️fixtures/🔣️.json | pressureAggregateBytes | 58392576 | 58720256 |
| ♻️retirement/🩹️patch/🧪️tests/🧫️fixtures/🔣️.json | native64.operationBytes / firstPayloadBytes | 7088 | 7168 |
| | native64.componentBytes / inputBytes | 3768 | 3784 |
| | native64.firstBackingBytes | 34760 | 34840 (= 1153×24 + 7168) |
| | placementGrants | [0,4096,7087,7088,32768] | [0,4096,7167,7168,32768] |
| | placedBytes | [0,0,0,7088,7088] | [0,0,0,7168,7168] |

Unchanged: ceilingSizedRoots 6, pressureRoots 7, directoryBytes 27672, the synthetic `arithmetic` block of the fixed fixture, and the historical `measured` block of the refresh fixture.

## Test rename
`populated_full_documents_admit_sixty_and_refuse_the_sixty_first_on_bytes` is now `populated_full_documents_exhaust_the_byte_ledger_before_the_slot_ledger`, so the name no longer encodes fixture-derived counts. No external references.

## Is the growth intended?
- Record +40 B / op +80 B / Component +16 B come from `PagedList` (`🔨️modules/🌱️value/📋️list/🦀️.rs`). It gained `reservation_epoch: usize` in commit fbf4da6a5aa6 (2026-11-06). The field is read by the reservation-binding identity check in `📋️list/🎟️reservation/🔎️selection/🦀️.rs`, which guards against stale reservations (ABA), and both reservation paths bump it. So it is a deliberate +8 B per `UiFixedList`, not bloat. The patch fixture's 7088 had been stale even before that: it was below the old record size of 7128, which is impossible because `UiPatchOp::Upsert` holds a whole record.
- fixedBackingBytes fell by 1.99 MB because commit 5797b702af0a (2026-11-06) moved `UiValueArena` out of static backing into the Source-owned `UiArenaRegistration`. `resident_static_backing_bytes` now counts only the registry plus handbacks. This is intended. It is also why 62 full documents now fit instead of 60.

## Files
- the 5 fixture JSONs above
- ♻️retirement/🩹️patch/🧪️tests/🩹️patch-pages/🦀️.rs (debug print added and removed; net no change)
- 🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs (test rename)
- ticket: a4-ts-twin.ts, a4-conformance-fixtures.md
