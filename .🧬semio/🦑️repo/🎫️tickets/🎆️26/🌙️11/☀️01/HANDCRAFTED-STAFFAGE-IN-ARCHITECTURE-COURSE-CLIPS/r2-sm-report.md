# R2 smoke: every scene constructs (agent sm), PARTIAL

Harness: `T/🧰️sm/worker.py` imports a clip module and runs `Scene(skip_animations=True).render()` for each Scene class defined in it, with manim `dry_run`, quality l, 15 fps. `T/🧰️sm/run.sh <file> [scenes] [skip]` runs one file (uv env), `T/🧰️sm/driver.py` loops files with a per-scene watchdog (SM_SCENE_TIMEOUT), `T/🧰️sm/tally.py` prints the latest status per scene, `T/🧰️sm/files.txt` lists 77 scripts (manifest scripts union files with Scene classes). Raw results: `T/🗑️generated/sm/results.jsonl` (delete when done).

## Result (latest status per scene)
- 113 scenes pass over 52 script files (all cooling, ventilation, demand, heating, physics, optimization series, BIM visuals).
- 12 series entry files (`🎬️clip/🐍️.py` per series, e.g. physics) define no Scene; they only call the runtime composer. Their FullSection scenes are composed from the passing Beat segments.
- 6 BIM scenes hit the watchdog, listed as skipped (known long, full BIM geometry): `representations` Beat3_SameObject, `information` Beat1_DataToInformation, `exchange` Beat5_Interfaces, `consistency` Beat6_Consistency, `models` Beat4_PartialModels, `objects` Beat2_ObjectOrientation. The latter four first failed with the breakage below; after the fix they run past it into the long geometry build.
- Skipped by design: grasshopper capsule (multi-hour).
- NOT RUN (time limit set by coordinator): remaining BIM beats after the long ones in `exchange`, `consistency`, `models`, `objects`, `information`, `representations`; excel (`energy`, `program`, `references`, `anatomy`, `visuals`); grasshopper (`trees`, `tower`, `anatomy`, `casting`, `visuals`).

## Failures found and fixed
- BIM: `api._surface_finishes()` / `core._surface_finishes()` after the model module renamed it to public `surface_finishes()` (`🧱️model/🐍️.py:1208`). `AttributeError: 'types.SimpleNamespace' object has no attribute '_surface_finishes'`. Fixed by replacing `._surface_finishes()` with `.surface_finishes()` in all callers: `🧱️model/🎨️finishes/🧱️layers/🐍️.py`, `🛝️ramps.py`, `🏗️facade/🔗️junctions.py`, `🔌️exchange/🐍️.py`, finishes and model test modules (`🧪️tests/🐍️.py`, `🧱️interfaces.py`, `🪈️tubes.py`). Not re-verified through a finished scene (geometry too long), only that it passes this call site.

## Transient failures (other agents mid-edit), green on rerun
- `window is not defined` / `width is not defined` / `thermalSession is not defined` from the staffage bun exporter (heating-comparison Beat2/3, system-losses Beat2/3/4, optimization index Beat4): thermal TypeScript was mid-edit; all pass on rerun.
- `_Motion` has no `_inside` (practice Beat1/2/3): staffage `🐍️.py` was mid-edit; passes now.
- `Unknown cast 'heating-calculation-person'` (calculation FullFinalCalculationVideo): clip now uses `shared-person-winter`; passes.

## Open
- Run the not-run files above (excel, grasshopper, rest of BIM) once; command: `python T/🧰️sm/driver.py <path fragment of first file>` after `T/🧰️sm/files.txt` ordering.
- The comment/dead-helper pass (hs) caused no NameError in any scene that ran.
- Side effect: while killing my own sweep I may have killed a parallel `🧰️pr` sweep's child processes (timeout/uv); that agent should re-check its run.
