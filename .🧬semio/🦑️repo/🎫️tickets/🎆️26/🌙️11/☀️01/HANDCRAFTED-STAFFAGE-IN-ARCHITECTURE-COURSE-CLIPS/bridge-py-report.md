# bridge-py report: Python staffage loader

V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## Files
- `V/🧍️staffage/🐍️.py`: the loader (`staffage`, `staffage_motion`, `staffage_height`, plus `ensure_casts`, `cast_key`).
- `V/🧍️staffage/🔣️.json`: course cast manifest. Migration agents Edit it; it is re-read when its mtime changes. Seeds: `standing`, `walking`, `radiator`, `seated-reader` (with `theme`, which the schema now requires).
- `V/🧍️staffage/🧪️tests/🐍️.py` + `🔣️.json`: 17 tests, registered in `🏗️authoring/📜️script.ts` (`test` list).
- `V/🐍️.py` re-exports `staffage`, `staffage_motion`, `staffage_height` and `ensure_casts` (after the theme imports).

## API
```python
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
V.ensure_casts(["standing", "walking", "radiator"])           # one exporter call for all missing casts; call once at the top of the clip
g = V.staffage("standing", unit=0.38, anchor=floor_point, color=V.P_WHITE)   # VGroup, frame 0, feet (world origin) on anchor
w = V.staffage_motion("walking", unit=0.38, anchor=floor_point, rate=1.0, color=V.P_ORANGE)  # VGroup that plays during wait/play
h = V.staffage_height("standing")                              # metres (heightM of the index)
```
- `unit` = Manim units per world metre. `0.38` keeps the old 0.66-unit person.
- Placement: x = anchor + (x_mm - originX) * unit/mmPerMetre; y = anchor + (originY - y_mm) * unit/mmPerMetre. It undoes the flip Manim applies about the content centre. Verified against `svgelements`.
- Stroke widths follow the physical scale (`mm * unit/mmPerMetre / 0.01`), floored at 1.0 so lines stay visible.
- `color` recolours strokes of `*-line` views only. `dark-solid`, `light-solid`, `thermal`, `forces` and `moments` are never recoloured. `opacity` multiplies fill and stroke opacity.
- The group bounding box is the cast's `boundsM` rectangle, so `next_to`, `move_to` and `get_bottom` are stable across frames.

## Motion
- `staffage_motion` returns a VGroup whose updater accumulates `dt * rate` of cast time and shows frame `floor(t * fps)`, looped (`index.loop`) or held on the last frame.
- Extra handles: `.seek(seconds)`, `.advance(dt)`, `.frame`, `.frames`, `.rate`.
- The family is constant (fixed slot paths, plus 3 invisible probe points). `shift`, `scale`, `rotate`, `flip` and `set_opacity` or `FadeIn` or `FadeOut` of the group carry over to every later frame, and Manim's family-pairing animations stay valid.
- Manim suspends updaters of the animated mobject only for animations that suspend them (Transform and `.animate` do). For a walker that crosses the screen use a velocity updater: `w.add_updater(lambda m, dt: m.shift(RIGHT * v * dt))`, or put it in a parent group moved by an updater. `FadeIn` and `FadeOut` keep it playing. Checked headless: FadeIn, wait, `.animate.shift`, FadeOut.
- For a pose with a thermal `timeline`, the same mechanism plays the transient.

## Cache and export
- Key: first 32 hex chars of sha256(canonical cast entry, the asset's `🗂️catalog/*/<slug>/🔣️.json`, the environments catalog when the cast has `environment` or `timeline`, and the digest of all files under `S/🔨️modules` and `S/🧬️schema` sorted by path). The source digest is memoised per process. Folder: `.🧬semio/🎓️teaching/⚡️cache/staffage/<key>/<cast>/` (honours `SEMIO_TEACHING_CACHE_ROOT`). Longest file path is about 134 chars.
- A batch writes a private temp manifest holding only the missing casts, so one invalid entry elsewhere in the course manifest never blocks other clips. It runs `repo tools exec bun -- <exporter> --manifest <tmp> --ids a,b --out <tmp>/out --lang en`, then renames each cast folder atomically into its key folder. A concurrent winner is tolerated. Stderr is streamed with the prefix `[staffage]`. KeyboardInterrupt and errors kill the process tree.
- The repo binary comes from `SEMIO_REPO_BINARY`, then `PATH`, then `.🧬semio/🦑️repo/native-repo-path`.
- Stale key folders are never pruned (greenfield; delete the `staffage` cache folder to reset).

## Testing
- `repo`-driven: `📜️script.ts test` includes `…🎨️visuals.🧍️staffage.🧪️tests.🐍️`. Direct: `PYTHONPATH=<root> .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m unittest 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️`.
- Tests need no bun: key stability and sensitivity to each input, placement maths from a language-agnostic case (`🔣️.json`) plus an `svgelements` cross-check, stroke scale, opacity, colour rules per view, motion stepping, loop, rate, hold, carried transform and fade, updater, batching with a fake exporter, re-exports.
- End to end with the real exporter: the four seeds render in about 4 s, give heights 1.77, 1.76, 0.85 and 1.34 m, and a rendered still shows the figures grounded on one line.

## Notes and deviations
- The contract gained a required `theme` per cast (schema from bridge-ts). Python passes the entry through untouched.
- Import check: `bim`, `excel` and `grasshopper` clip visuals and V itself import fine from a foreign cwd with only the workspace root on `sys.path` (V and the staffage module find the root by `🧰️framework`).
- Pre-existing, foreign: `🧪️tests.🐍️silhouettes` fails on `_ACTIVITY` of internal-gains while the migration removes the activity SVGs.
