# Tests agent report

Scope: energy teaching test suite (65 modules plus the new references module). Per-module runs with SEMIO_TEST_MANIM=1, profile final, language en, at most 3 in parallel (logs in `🗑️generated/tests-out/logs`).

## Registration (`🏗️authoring/📜️script.ts`)
Added 16 unregistered modules: ventilation `🧪️architecture` numbers and transitions, visuals `labels`, `🎚️profile`, `🎞️playback`, posters, hierarchy `pointer`, `⏱️purity`, running-example, roadmap, primary-energy chain and scale, recap `🧮️balance`, demand `🔗️chain`, physics fundamentals. The references module was registered by another agent. `tests/inventory.py` reports no missing module.

## Results (individually verified after fixes)
Green: physics fundamentals, narration guard (3 tests; `narration-guard.sh` for all five videos is clean), demand guard, ventilation architecture (numbers, transitions), cooling (3 modules, also combined in one process), visuals integration (9 tests, captions), scenes, media_staffage, staffage, geometry, browser, posters, `clip/tests`, audience, output, hierarchy chapters pointer, presentation native pointer test, and every module that was green in the first sweep.
Fixed but not re-run to completion after the last edit: `interaction` click-target test, hierarchy `chapters` (portable coverage fix; one run aborted because render.mjs was edited during it), `presentation/tests` (publish_export patch, concatenate kwargs), `staffage_only` (hang fix).

## Fixes (file, reasoning)
- physics `fundamentals/py` SHOWER_NOZZLE and icon window: the shower-head asset (commit 743) now has its head at 1.42-1.71 m, the old crop window (0.5-0.83 m) was empty, hence "need at least one array". Nozzle (0.15, 1.42), window (-0.02, 1.1, 0.26, 1.72). Visual check of the shower beat NOT done (drop lines still run to y=-0.95).
- physics `compare2` en shortened to 2 caption lines; cooling `system-sizing` clause `load` en rewritten like the German text.
- cooling language leak (f): removed `_VO_LANGUAGE` / `set_vo_language` from visuals; `narration_seconds` reads `SEMIO_TEACHING_LANGUAGE`; removed the import-time call and alias from 30 scene modules (greenfield, no compat layer). Physics test uses an env patch.
- `presentation/browser/render.mjs`: since commit 745 every frame opens a fresh page, so press then release never produced pointerup/click. The release frame now replays the previous pressed frame (render, move, down), keeps the DOM (no reset) and releases before the new hierarchy is rendered when the opened path changes, as before 745.
- Tests and fixtures updated to the new truth: `visuals/tests/integration.json` (all 76 pinned captions regenerated from current narration, each verified to fit 2 lines; `rho` case retargeted to `density`), `scenes.py` (person cast height from catalog), `media_staffage.py` and staffage fixture (bridge for `_fresh`/`_imports`, fake clock for the 2 s TTL, unreachable `new.ts` must not change the key), staffage test `logical.heightM`, `clip/tests` curriculum counts (138 segments, 130 sources, roadmap first), audience and presentation tests (patch `publish_export`, `concatenate(**kwargs)`), output test (settings need `profile`), narration guard test (`\unknownmacro`, scoped to scripts under energy; optimization/excel narration is not ours), demand guard fixture (`shared: roadmap`), ventilation transitions fixture (`need.icon` no longer uses `.become`), hierarchy chapters (ignore manifest `poster` keys), geometry `bodyPixel` (poster image now in the body), `staffage_only` forbidden-path scan (`os.walk` with pruning; `rglob` over the whole tree hung).
- demand roadmap `e_p` -> `e_E` (3 places, demand report open point 1; `G_t` unchanged).

## Remaining failures / open
1. references tests (owner: references/consistency agent): `test_every_shown_designation_is_a_registry_form` lists about 290 designations without edition or registry form (concepts md, quiz JSON, `integration.json` lines with `DIN 1946-6`, heading fixture `DIN V 18599`); `test_must_flag_cases...` fails for `DIN V 4108-6:2003-06, 6.4`.
2. Full official run `repo tools exec --light bun -- script.ts test` (PID 4588) started, still running after about 60 minutes. A few `E` early (BIM/Grasshopper/Excel/Optimization side) and 2 more after about 110 tests. Result unavailable at writing; see `generated/tests-out/official.log`.
3. Pending: shower beat visual check; re-runs of interaction, hierarchy chapters, presentation and staffage_only.
4. Speech for changed clauses not re-synthesised.
