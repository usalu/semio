# r2-bj: arm-like breath jets of standing people (volume Beat3)

Scripts in `T/🧰️bj/` (`export.py` exports casts, `frames.py` rasterises cast frames with PyMuPDF, `flow.ts` flow/periodicity numbers, `stack.py`).

## Root cause
- Not a missing warm start: frame 0 samples absolute time 3.5 s with the full 3.5 s history (checked).
- The solver put every exhaled parcel on one ray (nose direction 45 deg down, 1.8 m/s peak). About 70 parcels per exhale have collinear pathlines that all start at the nostril, so the bundle reads as a straight, thick arm from the head, in every frame of the loop (worst late in the exhale, which is where frame 0 sits).
- Turbulent random walk was keyed by absolute step, so the field was not periodic: the jet at loop end (t 8.5) differed from frame 0 (t 3.5) (reach 0.26 vs 0.42 m), a jump at each 5 s wrap.

## Fix (general, all five transport casts)
- `💨️transport/🧮️parcels`: exhaled parcels get a direction spread about the nozzle axis, sigma `BREATH_SPREAD_RAD = atan(2 alpha_jet)` (8.6 deg, clamped to 2 sigma), hashed from (seed, emitter, birth step). A new `periodS` option (`Resolved.periodSteps`) makes random walk and spread repeat with the clip length, so the field is periodic.
- `🎭️cast` exporter passes `periodS` = clip duration for looping clips.
- New cast field / depiction option `transportTrailS` (contracts, cast schema json + TS twin, `💨️transport-view`): only the last N seconds of each pathline are drawn, in 3 bands fading to the tail, so a jet is a dispersing puff, not a nozzle-anchored bundle. Manifest: `transportTrailS: 1` on the five transport casts.
- Assets unchanged (a trial of -115 deg / 20 mm nozzle made the jet horizontal, worse; reverted).

## Verified
- Frames 0/mid/last of `shared-person-pathlines` (and need-mirrored, office-worker, student, cook, 6 frames each): light fanning puffs, plume still vertical, frame 72/74 now match frame 0. Laptop vent streak of the seated casts is the separate desk jet (unchanged).
- Rendered volume Beat3 (ql, 15 fps, jobs 1): at 52 s, 57 s, 66 s the four persons show light dispersing breath puffs, no arms.
- Vitest (alias config): transport-field 21/21 (3 new: periodic one loop later, non-periodic without period, jet spread), transport-view, respiration, parcel-jet, transport-assets, cast-export, schema-twin, schema-conformance: all pass in the first run except my two new tests, fixed; transport-field re-run 21 passed. New fixture vectors: trail without transport, trail of zero. New view test: trail bands. staffage_only Python guard not re-run by me.

## Open
- Zigzag of strand noise is the existing turbulence random walk (sigma floor 0.03 m/s); not changed.
- The cast-export and python-side suites were not re-run after the final test edits other than transport-field.
