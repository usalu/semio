# r2-bjv: verification of bj breath-jet change

Verdict: PASS. bj's change holds in the tests I ran and in frame 0 of all five transport casts. No source was edited.

## 1. Staffage vitest (library, alias config, file filter)
Filter: transport-field, transport-view, respiration, parcel-jet, transport-assets, cast-export, schema-twin, schema-conformance.
- Test Files 8 passed (8). Tests 155 passed, 1 skipped (156).
- Log: `🗑️generated/bjv/vitest.log` (deleted after this report).

## 2. Python suites (`m9_tests.sh`)
- staffage_only guard (module line 13): Ran 22 tests, OK.
- ventilation volume (line 3): Ran 2 tests, OK.
- ventilation architecture (line 4): Ran 5 tests, OK.

## 3. Frame 0 of the five transport casts
Exported with `ensure_casts` (cast names from the manifest; the brief's short names are not cast ids):
shared-person-pathlines (75 frames, 15 fps), ventilation-need-occupant-mirrored (75), ventilation-volume-office-worker (72), ventilation-volume-student (72), ventilation-volume-cook (60).

Visual check of frame 0 (PNG sheet read, then deleted):
- No straight arm-like bundles from the head on any of the five. Each head plume is wavy and dispersing; the standing casts fan sideways at the nose.
- Office-worker and student: long horizontal streaks run from the laptop/keyboard area across the desk. These start at the desk, not the head, and are the separate desk jet that bj left unchanged. Not a failure of this change, but they are the streaks most likely to be read as arms.

Loop seam (mean absolute pixel difference, 260 px raster; seam = last frame to frame 0):

| cast | mean step | max step | seam step |
|---|---|---|---|
| shared-person-pathlines | 0.593 | 0.949 | 0.684 |
| ventilation-need-occupant-mirrored | 0.592 | 0.940 | 0.689 |
| ventilation-volume-office-worker | 0.555 | 0.795 | 0.435 |
| ventilation-volume-student | 0.488 | 0.684 | 0.454 |
| ventilation-volume-cook | 1.182 | 1.697 | 1.453 |

The seam step is an ordinary adjacent-frame step for every cast (within the observed max, close to the mean). No jump at the wrap. Note that frame 0 and the last frame are not identical (they are one frame apart in time), so "match" here means "no seam jump", not pixel equality. The cook seam sits above its mean, but well below its max.

## Scripts
- `🧰️bjv/tests.sh` (test runs), `🧰️bjv/render.py` (export and frame 0), `🧰️bjv/seam.py` (seam metric).

## Open
- Desk streaks on the seated casts (office-worker, student) are the existing desk jet, not breath. Not changed.

## 4. Coordinator follow-up: two `transport-field` jet failures seen by agent ap
Not reproducible at 19:50 (after my run and the coordinator's message):
- `transport-field` alone: 21 passed (21).
- Full library vitest (`q2-vitest-alias.config.ts`): Test Files 56 passed (56), Tests 1144 passed, 11 skipped (1155).
- Source and test files were last modified at 17:57 (`🔨️modules/💨️transport/🧮️parcels/🟦️.ts`) and 18:49 (`🧪️tests/💨️transport-field/🟦️.ts`). Nothing changed during the window.
- The transport code has no `Math.random`, `Date.now` or `performance.now`. The random walk and spread are seeded from (seed, emitter, step), so runs are deterministic.

Conclusion: no failing expectation to fix and no source change. The ap failures most likely came from a run made before bj's last edits (17:57 to 18:49) or from a concurrent edit. If ap can share the failing assertion text, it can be checked against this log. No test expectations were changed.
