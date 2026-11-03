# r2-cv: correctness review of the staffage cast bridge

Scope: `S/🔨️modules/📤️export/🎭️cast/🟦️.ts` and its schema, the thermal session and phase-change code it calls, and `V/🧍️staffage/🐍️.py` plus `fitted_staffage` and `pendant_lamp` in `V/🐍️.py`. Repro scripts are in `T/🧰️cv/` (run with `T/🧰️cv/run.sh <script>`).

## Confirmed and fixed

1. **Cache key missed the bundled print profiles** (`V/🧍️staffage/🐍️.py`, `_source_digest`).
   - The exporter's import closure reaches `S/🗂️catalog/🖨️print-profiles/🔣️.json` through `🖨️representation/🧾️profiles`. `depict` resolves the drawn representation under `DEFAULT_PRINT_PROFILE`, so editing the profiles changes cast output.
   - `_source_digest` hashed only `🔨️modules` and `🧬️schema`, so an edit to the profiles reused stale casts.
   - Found with `T/🧰️cv/closure.py`, which resolves the import closure and lists files outside the hashed folders. Only the profile JSON and the zip writer (output-neutral) came out.
   - Fix: the new `_BUNDLED` tuple is hashed along with the sources.
   - Regression: `Cache.test_bundled_inputs_outside_the_sources_change_the_key`, driven by the JSON case `key.bundled`. `T/🧰️cv/negative.py` shows it fails without the fix.

2. **`set_opacity` gave unfilled (open-stroke) paths a fill** (`V/🧍️staffage/🐍️.py`, `_apply`, `_place`, `_Motion.slots`).
   - `_apply` set `fill=None` paths to fill opacity 0, but Manim's `VMobject.set_opacity(o)` sets the fill opacity of every family member to `o`.
   - So `pendant_lamp(...)["on"].set_opacity(1)` and `.animate.set_opacity(1)`, used in the demand `🧪️practice` clip, painted the open filament and strokes of `shared-lamp-winter` with a white fill. The same applies to a faded `_Motion` group.
   - Repro: `T/🧰️cv/repro_opacity.py <cast folder>` printed 3 of 3 unfilled paths gaining a fill. After the fix it prints 0 of 3.
   - Fix: new `_Path(VMobject)` with an `unfilled` flag. `_apply` sets the flag, and `set_fill` holds the opacity at 0 for unfilled paths. `_place` and the motion slots use `_Path`.
   - Regression: `Motion.test_set_opacity_never_gives_an_unfilled_path_a_fill`, driven by the JSON case `fade`. It fails without the fix (`negative.py`).

## Plausible but unconfirmed or not fixed

- **Transport solve ignores the cancel signal** (`🎭️cast/🟦️.ts:148`). `solveTransport(..., { maxAgeS })` is called without `signal`, and so is the thermal call of `transientThermal` in `💨️transport`. A cancel therefore waits for the current frame's transport solve. The Python tree-kill after 3 s bounds the leak, so no repro and no change.
- **Transient thermal under a timeline environment.** In `💨️transport/🟦️.ts`, `transientThermal` passes the constant `Environment` sampled at each call time into a shared `thermalSession`. Cached history instants then carry the environment of the first call, so results differ from the session-free solve when the environment varies. No course cast combines a timeline with a region-following vent (only the kettle, with constant `office-neutral`), so it is latent.
- **Windows job assignment race** (`_Tree._assign`). The process is added to the job after `Popen`, so a grandchild spawned in that window would escape the job. The window is milliseconds and `repo` spawns `bun` later. Not reproducible, so no change.
- **Python `rename` has no retry for transient Windows locks** (`ensure_casts`), unlike the TS `patiently`. The race test is clean, so only the scanner-lock case is open.
- **Cache directories are never pruned.** Every source edit makes a new key folder; `.🧬semio/🎓️teaching/⚡️cache/staffage` already holds 1139. This is disk growth, not memory.
- **Pose existence is not checked before writing** in `exportCast`, although the doc comment says it is. A bad pose throws in `evaluate` during the render, so earlier casts of that call are discarded by Python. No wrong output.
- **Looping clips whose `durationS × fps` is not an integer** have a loop period of `count / fps` instead of `durationS` (for example 2.333 s against 2.35 s). This is by design in `frameSchedule` and the same on both sides.

## Checked and found clean

- **Frame indexing with `rate`, `loop` and the 600-frame cap.**
  - `thinned` uses `600 / durationS` for loops and `599 / durationS` for non-loops, so the last frame lands on `durationS`.
  - `seek` computes `floor(t·fps + 1e-9) % count` for loops and `min(…, count − 1)` otherwise. The seam and clamp checks match.
  - `fitted_staffage`: `rate=0.0` is not `None`, so it yields a motion held on frame 0. `loop` ORs into `loops`.
- **Content-keyed cache.**
  - Entry (including `thermalSpanK`, transport options, `mirror` and `theme`), the asset file, environment presets when `environment` or `timeline` is set, and the source digest are all keyed.
  - `T/🧰️cv/catalog_ids.py` shows the 147 asset ids equal their folder names, with no duplicates, so the Python lookup agrees with the TS `asset.id` lookup.
  - Environment presets are not hashed for casts without an environment, which is correct because they use the asset's own environment.
- **Races.**
  - `ensure_casts` does atomic directory renames from a private temp folder, and a lost race falls back to the existing index. `T/🧰️cv/race.py` ran 8 threads on 3 casts: no errors and no leftovers.
  - TS temp folders are per pid, and a cast is published only after its index is written.
  - Readers never see partial folders.
- **Cancellation.**
  - The TS loop checks the signal each frame, and the temp folder is removed in `finally` with retries.
  - In-flight writes into a removed folder fail silently inside `writer`.
  - `_run` tree-stops on any `BaseException`, joins the reader with a timeout, and closes the job handle (`KILL_ON_JOB_CLOSE`).
  - A `progress` callback that raises is carried across the pump thread.
- **Thermal session.**
  - Chain nodes are keyed `startS|stepS` and are a pure function of the grid. The cached prefix is shared; the final partial step is computed per query.
  - Body-placed region temperatures are ignored by `assemble`, so their stale values in a shared chain are harmless.
  - Eviction limits are FIFO and consistent.
- **Phase change.**
  - Unit bookkeeping (W per outer area × `areaM2`, latent per enclosed area) is consistent between `stepRegion`, `assemble` and the vent flow.
  - The overshoot energy `C·(next − point)` is converted to kg.
- **Parser and emoji paths.**
  - SVG numbers are rounded to 4 decimals, so `[eE]` never appears in path data and `_PATH_COMMAND` stays safe.
  - Emoji paths pass through `Popen` (UTF-16) and the UTF-8 manifest correctly.
  - Cache path length stays under 256 characters.
- **`pendant_lamp`.** The on and off casts have identical bounds in the course (`heightM` 0.8, the same `boundsM`), so the top alignment is exact.
- **Memory.** Per-cast memory is bounded by 600 buffers; the parsed-frame LRU holds 256; the transport track cache holds 6 tracks per asset.

## Test output

- Baseline before the edits: 36 tests in the staffage suite, OK.
- After the fixes: the full run (staffage suite plus `🐍️icons`) is in `T/🗑️generated/cv/full.txt`, summarised below.

Result: 46 tests (the staffage suite plus `🐍️icons`), OK, 151 s. The generated output was deleted.
