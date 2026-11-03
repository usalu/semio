# Staffage cast contract (coordinator ruling, 2026-11-01)

Abbreviations: S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket folder.

## Goal

The course clips draw people, furniture, objects, devices, vegetation, vehicles, machines and building components only from the handcrafted staffage archive S. The following are removed:

- `V/🫟️silhouettes` (both the outlines and the README)
- `person_glyph`, `seated_person_glyph` and `object_silhouette` (for staffage-like kinds)
- the activity SVGs under `⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🏃️activity`
- ad-hoc radiator, tree, desk and chair drawings

Diagram icons that are not staffage stay as glyphs: sun, moon, flame, microbe, clouds, arrows, thermometers (`thermometer_glyph`), clocks (`clock_glyph`) and charts. Abstract energy gauges stay diagrams: `energy_tank`, including the practice "Quelle" source gauge, and the track-and-fill `meter` gauge. Meters and building stores are device casts, not glyphs: the gas meter (`gas-meter`), the electricity meter (`electricity-meter`) and the building hot-water store (`hot-water-cylinder`). Energy clips use the thermal view (`thermal`), animated through staffage clips and transient environments, wherever a figure carries heat-related meaning, for example internal gains, metabolism, radiation, convection or ventilation needs.

## Data flow

```
V/🧍️staffage/🔣️.json (cast manifest, one for the whole course; schema S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json)
   └─▶ TS: S/🔨️modules/📤️export/🎭️cast/🟦️.ts  (evaluate → depict → SVG frames + index), CLI command `cast`
         └─▶ cache dir <key>/ (per cast: 🔣️.json index + NNNN.svg frames)
               └─▶ Python: V/🧍️staffage/🐍️.py  staffage(...) / staffage_motion(...) → manim VGroup
```

### Cast manifest (`V/🧍️staffage/🔣️.json`)

```json
{ "version": 1,
  "casts": {
    "<cast-id kebab>": { "asset": "<catalog slug>", "pose": "<pose slug>" | "clip": "<clip slug>", "view": "dark-line|dark-solid|light-line|light-solid|thermal|forces|moments",
                          "viewpoint": "elevation|plan", "scale": 50, "fps": 15, "environment": "<environment preset slug, thermal only>", "mirror": false } } }
```

Rules:

- Give exactly one of `pose` or `clip`.
- `fps` applies to clips only. A pose renders one frame, except that the `thermal` view in a transient environment timeline renders frames.
- `mirror` flips about the world origin's vertical axis, so a figure can face left.
- Casts have no language: they print no text (no legend, title, label, dimension, scale bar or ground line; the exporter uses `textlessDepictionOptions` with all annotations off in `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`), so the output, the cache key and the exporter command carry no language.

### Rendered output (one folder per cast id)

`<out>/<cast-id>/🔣️.json`:

```json
{ "version": 1, "cast": "<id>", "asset": "<slug>", "view": "...", "frames": ["0000.svg", ...], "fps": 15, "durationS": 2.4, "loop": true,
  "mmPerMetre": 20, "originMm": [x, y], "boundsM": { "minX": .., "minY": .., "maxX": .., "maxY": .. }, "heightM": 1.75 }
```

- Every frame SVG uses paper millimetres in one fixed viewBox. All frames share the same world-origin alignment.
- `originMm` is the world origin (ground y = 0, x = 0) in SVG coordinates, with y pointing down as in SVG.
- `boundsM` is in world metres, y up.

### Cache and invocation

- Cache root: `.🧬semio/🎓️teaching/⚡️cache/staffage/<key>/<cast-id>/`.
- `key` = sha256 of: the cast entry (canonical JSON), the bytes of the asset's catalog category file, and the bytes of every source file under `S/🔨️modules` and `S/🧬️schema`, sorted by path.
- Python batches all missing casts into one invocation: `repo tools exec bun -- S/🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest <file> --ids a,b,c --out <dir>`. The CLI writes progress to stderr and is cancellable with SIGINT. Output goes to a temp dir first, then is renamed into place atomically.

### Python API (`V/🧍️staffage/🐍️.py`, re-exported from `V/🐍️.py`)

```python
staffage(cast: str, *, unit: float, anchor=ORIGIN, color: str | None = None, opacity: float = 1.0, window=None) -> VGroup
staffage_motion(cast: str, *, unit: float, anchor=ORIGIN, rate: float = 1.0, color: str | None = None, window=None) -> VGroup
fitted_staffage(cast: str, *, width=None, height=None, color=None, rate=None, loop=False, window=None)   # V/🐍️.py
staffage_height(cast: str) -> float   # metres
```

- `window` = `(min x, min y, max x, max y)` in cast metres (feet on the ground at 0) keeps only the drawing paths fully inside it and crops the cast, for example a blind without its wall stubs. `fitted_staffage` also fits the window to `width` and `height`, so a tall cast can show only its head as an icon.
- `unit` is manim units per metre. The clips already work at "human scale": the old `person_glyph` was 0.66 units tall for about a 1.75 m person, so `unit ≈ 0.38` keeps layouts. Each call site picks the unit from its own drawing scale, for example house sections or room sections.
- `anchor` is where the world origin (the feet on the ground) lands.
- `color` recolours line views to a clip palette colour. It is never applied to `thermal`, `forces` or `moments`.
- `staffage_motion` swaps frames from scene time through an updater (`dt` accumulation × `rate`, looping when `loop`), so animated walking, breathing and thermal transients play during `self.wait` and `self.play`.

## Ownership

| Area | Owner |
|---|---|
| `S/🔨️modules/📤️export/🎭️cast/**` (TS + schema + tests + fixture) | bridge-ts agent |
| `V/🧍️staffage/**` (Python loader + manifest + tests) | bridge-py agent |
| new catalog assets under `S/🗂️catalog/**` (+ manifest rows) | asset agents (one category file each, see brief) |
| clip call-site migration per series | migration agents |

Every new asset follows the handcrafted authoring rules of ticket `26/10/29/ARCHITECTURAL-STAFFAGE-ARCHIVE` (`authoring-v2.md`, `content-worker-brief.md`) and passes the catalog check.

## Additions by bridge-ts (2026-11-01, backward-compatible with the contract above)

- Manifest entry gains optional `timeline`: `[{ "t": seconds, "environment": "<preset slug>", "ease": "step|linear" }]` (thermal only, excludes `environment`). A pose with a timeline renders `round(lastT·fps)+1` frames (`fps` default 15), `loop` false, `durationS` = last `t`.
- Static pose index: 1 frame, `fps` 1, `durationS` 0, `loop` false. Clip `fps` default 15 when the manifest names none.
- Frames have a transparent paper (no background rect), no ground line, margin 1 mm; all content sits in `<g id="staffage-cast" transform=...>`. `mirror` is that group transform; `originMm` is the same for every frame.
- `--ids` is optional (default: all casts). Stdout prints one cast folder per line.
- Schema: `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json`; twin + validator `castManifestIssues`/`parseCastManifest` in the neighbouring `🟦️.ts`.

## Changes

- 2026-11-01 (bridge-ts, for bridge-py and migration agents): every cast entry now REQUIRES `"theme": "dark"|"light"` (no default; schema, twin and validator reject its absence). It sets the ink and palette of the analytic views (`thermal`, `forces`, `moments`); clips use `"dark"`. All entries of `V/🧍️staffage/🔣️.json` and the bridge-py cache-key fixture were updated; new entries must carry it. The exporter itself needs no Python change (the entry is passed through and hashed as before).
- Optional `"transport": ["parcels"|"pathlines"|"streamlines"|"condensation", ...]` draws air-transport layers (e.g. breathing plumes) on the figure. Figure-only: the depiction gates emitter markers and transport labels behind `annotations.labels`, which the cast exporter turns off. The cast `environment`/`timeline` also feeds the transport solver.
- 2026-11-01 (m9): `fitted_staffage(cast, width=, height=, color=, rate=, loop=)` and `pendant_lamp(...)` live in `V/🐍️.py` (not in `🧍️staffage/🐍️.py`); clips import them from `V`. `rate` plays a clip, `loop` repeats clips whose index has `loop: false` (fan spin, tealight flicker).

## Performance (p1, 2026-11-01)

Animated thermal staffage is now cheap: use `staffage_motion` for every device that has a clip or a transient. Static first frames are no longer needed for cost reasons.

- Benchmark (one thermal occupant + 3 animated thermal devices, 10 s, quality l, 15 fps): about 13 s wall and 214 MB peak; a re-render hits the manim cache (6 s, 122 MB). Before: over 20 minutes of loading and several GB per process.
- `--disable_caching` is NOT needed for staffage any more: a `staffage_motion` hashes to a small deterministic value (it carries a content key, its frame names and the current slot state; the frame data lives outside the hashed `__dict__`). The segment runtime still passes the flag for every scene; that is a runtime decision, not a staffage requirement.
- Long transients are thinned by the exporter to at most 600 evenly spaced frames, so the cast `fps` of a clip longer than 40 s (toaster 300 s, laptop 600 s, vacuum 1500 s) is fractional (about 2, 1 and 0.4). Play such a clip faster than real time with `rate` (for example `rate = clip seconds / scene seconds`); at `rate=1` you see the first seconds in coarse steps. Read `index["fps"]` if you need the step.
- Frames are parsed lazily by a dependency-free parser into a bounded cache (256 frames per process), so memory no longer grows with the frame count. The exporter renders one cast at a time (peak about 370 MB for the five worst casts, 14 s) and runs as `repo tools exec --light`, so it no longer waits in the machine capacity queue.
- Do not worry about the number of casts per clip. Do keep one `staffage_motion` per visible device and reuse the same cast id for repeated devices.
- Changing exporter sources changes every cast key (a full re-export once; see the measured figure below, not the "about a minute" of round 1).

### Export performance, round 2 (pf, 2026-11-01)

Measured with one `bun` process per run on the whole manifest (145 casts, 19 015 files) and on single casts; CPU seconds are used because the host is shared (wall time swung 40 to 400 s for the same work). "Before" is the round 1 exporter, "after" the current one; every frame is byte-identical except that `data-value` is written with 6 significant digits.

| | Before | After |
|---|---|---|
| Full manifest, one process (CPU / wall) | 198 s / 258 s | 97 s / 158 s |
| Peak memory, full manifest | 1309 MB | 733 MB |
| `demand-primary-energy-tree` (1 frame, 294 mm tall oak) | 96 s CPU, 796 MB | 33 s CPU, 510 MB |
| `heating-internal-gains-server-rack` (600 frames) | 1285 MB | about 220 to 510 MB |
| `ventilation-need-occupant-mirrored` / `ventilation-volume-student` | 3.4 s / 4.5 s CPU | 2.5 s / 3.6 s CPU |

- The earlier "1848 s, about 17 s per cast" came from one `repo tools exec` process per cast (about 10 s start-up each). `ensure_casts` already exports all missing casts in ONE process, so the real cost of a full re-export is the single-process figure (97 s of CPU, 2 to 3 minutes on a shared host), not 30 minutes. Do not export casts one process at a time.
- Where the time went: 55 % of the CPU was the print-suitability feature measure (`featureWidthMm`, 14 distance transforms of up to 2.5 million cells per filled shape; the oak alone needs 120 shape measurements), 20 % the thermal history, the rest evaluation, depiction and SVG text.
- Fixes: a linear exact distance transform (Meijster) that visits only the cells that can matter (window, cap, sparse sweep) and a boolean "does the thin area exceed the tolerance" with early exits instead of measuring the largest component; a memo of identical shapes (1024); one `thermalSession` per cast, so the frames share the input history and the integrations that start at the same time (bit-identical to solving every frame alone); frames kept as UTF-8 buffers, placed, SHA-256 hashed, written with 16 writes in flight and released one at a time (no more three string copies of a 600 frame cast); transient `EPERM`/`EBUSY` of the Windows scanner on the temporary folder are retried; `data-value` is written with 6 significant digits (was 17, `22.000000000000014`).
- Not changed on purpose: coordinate precision (already 4 decimals; the bytes are in the repeated `stroke-linecap`/`stroke-linejoin`/`id`/`data-bone` attributes, not in digits) and the raster cap of 2.5 million cells (it would change the measured feature widths). Three single frames are still above 200 KiB (oak 288 KiB, the two seated ventilation figures 231 KiB) because of their path count; hoisting the repeated stroke attributes into the layer group would shrink every frame by about 20 % but changes the SVG writer output and is left to the owner of `writeSvg`.
- Guards: `🧪️tests/🚀️export-performance` (scipy and brute-force oracle for the transform, widths of the pre-optimisation module, shared session against solving alone, pinned SHA-256 of fixture cast frames, CPU budgets). Python staffage loader keys include the exporter digest, so the first export after this change re-exports everything once.

## Round 2 additions (2026-11-01, verified against the code by dc)

### Schema and solver

- `phaseChange` on `Material` and on `ThermalRegion`: `{ "transition": "boiling"|"melting", "pointC", "latentHeatJKg" }` (`S/🧬️schema/🔣️.json` `PhaseChange`, twin `S/🧬️schema/🟦️.ts`). A region's own declaration overrides its material's. Regions of model `body`, `fixed` or `leaf` may not carry it (catalog check `phase-change-model`). The inventory is ample: no depletion, no freezing or condensation direction.
- `RegionField` gains `phaseChangeW`, `phaseChangeKgS`, `phaseChangedKg` and `evaporationKgS` (`S/🔨️modules/🔗️contracts/🟦️.ts`). `latentW`, `latentWM2` and `moistureGM2H` include the conversion, so the balance still closes.
- A vent, flue, fan or exhaust with a `region` follows that region's evaporation and needs `vapourKgPerKg` (catalog check `evaporation-vapour`).
- Assets: `electric-kettle` (water boiling at 100 °C, 2.257 MJ/kg; `🔌️appliances`) and `cooking-pot` (`🪑️objects`) both declare it.
- `thermalSpanK` (cast entry, thermal view only, positive): a symmetric ±K range around the environment air temperature. Absent means the shared absolute range of −20 to 60 °C. Schema, twin and exporter all read it (`S/🔨️modules/📤️export/🎭️cast/`). The course manifest sets 25 K on 77 of its 86 thermal casts and a wider span (70 K or 78 K) on the other 9.
- One span per scene: all thermal casts drawn in one scene share one `thermalSpanK`. The guard test `thermal_span` (`V/🧪️tests/🐍️staffage_only.py`, `scene_spans`) enforces this. Choosing the smallest span that covers the hottest cast of the scene is authoring guidance; the code does not check the size.

### Cast exporter CLI (`S/🔨️modules/📤️export/🎭️cast/🟦️.ts`)

- `--parent <pid>` is a positive integer (otherwise the error names `--parent`). A 500 ms unref'd watchdog cancels when that pid is gone, then exits 130 after an unref'd 5 s fallback. Python passes its own pid.
- Progress on stderr: `[pid n] ppid m` first, `[cast i/n] id` per cast, `[done/total] label` every 50 frames and at the end, `[peak] N MB` last. Cancellation prints `[cancelled]` and exits 130; any other failure exits 1.
- Cancellation: SIGINT, SIGTERM and SIGHUP on POSIX, SIGINT, SIGTERM and SIGBREAK on Windows. Every frame yields to the event loop, so a signal is seen between frames. `publish` re-checks the signal before it renames, so an interrupted cast is never placed. The `.<id>.tmp-<pid>` folder is removed in `finally`.

### Python API (`V/🧍️staffage/🐍️.py`)

- `ensure_casts(ids=None, progress=None) -> dict[str, Path]`: `progress` receives every exporter stderr line and may raise to cancel. It runs `repo tools exec --light bun -- …exporter… --parent <pid>`.
- `_Tree` and `_run`: stderr is read on a daemon thread, while the caller polls `wait(0.1)`, so `KeyboardInterrupt` is prompt on Windows. Any `BaseException` stops the tree: on Windows CTRL_BREAK then a job object that kills on close, on POSIX `killpg` SIGTERM, a 3 s grace, then SIGKILL. Only complete casts reach the cache.
- `_SHAPE` (line 42) accepts an optional `<title>`. Thermal SVG shapes carry one, so blind slats are found in thermal casts too.

### Guard (`V/🧪️tests/🐍️staffage_only.py`, rules in `🔣️staffage_only.json`)

- Rule A: an AST pass over clip functions named like a physical object that build it from primitives without a staffage call. Each violation must be in `rule_a.allowed`, which has 14 entries, each with a reason. Stale entries fail.
- Rule B: every cast an `⚡️energy` clip resolves (literals, module constants, `PREFIX + x`, f-strings) must be thermal or in `rule_b.allowed`. That list has 8 non-thermal furniture, storage and landscape ids; all 8 exist in the manifest and are `dark-line`. `forbidden_casts` adds `shared-watt-bulb`, `shared-watt-monitor` and `shared-watt-vacuum`.

### Archive

- `plate-heat-exchanger` and `sorption-wheel` moved from `🔌️appliances` to `S/🗂️catalog/🏗️machinery` (14 assets). Both asset files and the root `🗂️catalog/🔣️.json` entries say `machinery`. No alias exists.
- Device casts added for the course: `gas-meter`, `electricity-meter` and `hot-water-cylinder` (`🔌️appliances`), and `solar-thermal-collector` (`🏗️machinery`). Cast ids: `shared-gas-meter-run`, `shared-electricity-meter-run`, `shared-tank-charge-up` and `cooling-summer-strategy-collector`.

### Cast ids (the r2-g5 report uses stale ids)

- The lamps and blinds are now `shared-lamp-*` (`pendant-lamp`, thermal), `shared-blind-lowered` (`external-venetian-blind`, `summer-hot-sun`) and `cooling-summer-strategy-blind`. The ids `optimization-room-lamp-*`, `physics-fundamentals-blind` and `heating-solar-gains-blind` no longer exist in the manifest.

### Open items seen in code

- `S/🔨️modules/📤️export/🎭️cast/🟦️.ts:130` has a `[DEBUG] pf` hook, `process.env.PF_NO_SESSION`. Remove it or make it a documented option.
- `stopWatching()` runs only on the error path in `main`. The unref'd timer makes this harmless.
