# r2-fv1: final TS-side verification (2026-11-02, 07:30 to 11:30)

Host was heavily loaded (about 20 agents); full library runs went from 145 s to over an hour. Runs used `repo tools exec bun` (capacity grants were admitted within seconds; the `mcp__repo__capacity_status` queue was never blocking, 2 waiters, all admitted). Generated logs deleted.

## Fixed (a): ours

`S/🔨️modules/🖨️representation/🪞️copies/🟦️.ts` `scaledPath`: the command-splitting regex `([a-zA-Z])([^a-zA-Z]*)` treated the `e` of an exponent number (`-8.333e-05`) as a path command, so the scaled number became `-4.1665e-2.5` and `flattenPath` threw `invalid path data at offset 163`. This crashed the full catalog check at `running-child` (aborting the run at 16/149) and left 3 assets invalid in the React store. The regex now excludes `e`/`E` (`[a-df-zA-DF-Z]`). Same bug fixed in the test helper `scaleShape` and a regression case added in `S/🧪️tests/🪞️representation-copied/🟦️.ts` ("scales a path whose numbers use an exponent..."). `representation-copied` alone: 8 of 8 pass.
No other (a) failure was found: no failing assertion concerns an asset or code that this ticket created and that is not also in the other ticket's change set.

## 1. Library vitest (`q2-vitest-alias.config.ts`)
- Run 1 (07:36, before my fix, host quiet): 64 files, 62 passed, 2 failed; tests 1453 passed, 15 failed, 11 skipped (1479).
- Run 3 (verbose, 10:00, killed after hang at the end; the last file `asset-validation` never reported): 1391 passed, 22 failed, 11 skipped reported. `asset-validation` alone: 72 of 72 pass. The worker hang is host load (a 180 s `dxf-reopen` timeout preceded it).
- Failures by class:
  - (b) `three-views`: 14 (run 1) / 12 to 14 (run 3): `spread-elevation-x-plan-x`, `spread-front-x-plan-y`, `elevation-x-plan-x` 1:500, `front-height`, `elevation-width` for agave, herring-gull, horse, mallard-duck, person-with-white-cane, pigeon, runner-with-blade-prosthesis, sleeping-person, standing-adult, tour-guide, hosta-clump, date-palm, pregnant-person, running-child, seated-reader, skateboarder, parent-with-stroller, person-with-rollator, person-with-umbrella. Front and plan geometry is still being authored by the concurrent ticket (herring-gull and motor-scooter assets changed at 07:40 / 08:16).
  - (b) `export-performance`: 1 digest (`walker-line/🔣️.json`, run 1; the radiator digest from r2-ty is now fine) plus, since the feature-width rework (`measure/🟦️.ts` modified 09:46, `report-FW.md` "Feature Width Measure"), 10 `feature width` cases (for example square-6 gives 5.9756 against 6) in run 3. Alone: 11 failed, 25 passed across `dxf-reopen`, `dimension-leader`, `export-performance`; all 11 are in `export-performance`.
  - (c) `dxf-reopen` "ezdxf audit" timeout 180 s (run 3) and `dimension-leader` TRV "no suitable representation at 1:50" (run 3, mid-edit suitability): both pass when re-run alone (`dxf-reopen` + `dimension-leader`: 2 of 3 files green, 25 tests pass together with the export-performance rest).
  - cast-export SIGTERM case skipped (ffmpeg/environment skip, unchanged: 11 skipped).
- (a): 0 remaining.

## 2. React vitest (`q2-vitest-react-alias.config.ts`)
Final: 18 files, 17 passed, 1 failed; tests 116 passed, 1 failed, 1 skipped (118).
- (b) `catalog-loading` "one ready row per manifest entry with an empty manifest check": manifest-mismatch rows on `poses` (`office-swivel-chair`, earlier `cargo-bike`, `motor-scooter`): the pose lists of those assets are being edited by the concurrent ticket. Before my regex fix the same test also showed 3 invalid rows (the exponent crash), now gone (146 ready, 3 invalid became manifest rows only).

## 3. Typecheck (`🧰️ty/tsconfig.lib.json`, `🧰️ty/tsconfig.react.json`)
Library: 0 errors (rc 0). React: 0 errors (rc 0). Measured before my regex edit; the edit changes no types.

## 4. Full catalog check (default 9 samples, all viewpoints)
Via `repo tools exec bun -- 🩺️check/🟦️.ts` (the repo capacity layer is built in, admitted at once, no direct fallback needed). Takes about 35 min under load.
- Before the fix: aborted at 16/149 (exponent crash).
- After the fix: complete, rc 1: `✓` 125 entries, `✗` 25 entries (manifest plus 24 assets) of 149 assets.
- All ✗ reasons are (b): `suitability-feature` 76 and `suitability-gap` 3 lines (feature-width rework plus new front/plan shapes), `elevation-lateral` 19 (new validator rule for spread bones), `manifest-mismatch` 2 on `poses` (cargo-bike, motor-scooter being edited), `extent-width` 2 (skateboarder). Assets affected: parent-with-stroller, person-with-white-cane, pregnant-person, runner-with-blade-prosthesis, running-child, seated-guitarist, seated-office-worker, seated-reader, skateboarder, sleeping-person, standing-adult, standing-phone-user, tour-guide, dairy-cow, domestic-cat, hedgehog, labrador-dog, hosta-clump, bar-stool, mvhr-unit, thermostatic-radiator-valve, cargo-bike, delivery-van, sectional-garage-door. None is a crash and none an error of this ticket's code.
- Not re-run after the other ticket settles; re-run needed once it closes.

## 5. Full manifest cast export
`repo tools exec bun -- S/🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest V/🧍️staffage/🔣️.json --out <T>/🗑️generated/fv1/cast`: rc 0, 94 casts (94 index files), 15291 SVG frames. No `NaN`, `Infinity` or `undefined` in any file, no frame under 200 bytes, no failure in the log (only win32 capacity-probe warnings). Output deleted.

## Open
- (b) items above clear only when the `TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE` rollout finishes (front/plan geometry, pose lists, feature-width tests and digests).
- Library vitest has not completed one clean end-to-end run since 07:36 because of host load; re-run once the host is quiet.
- Scripts kept in `🧰️fv1/run.sh`.
