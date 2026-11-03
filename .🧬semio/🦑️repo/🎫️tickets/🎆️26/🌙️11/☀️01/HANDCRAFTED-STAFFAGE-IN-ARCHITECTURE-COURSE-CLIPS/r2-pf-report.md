# R2 pf report: cast export performance

Agent `pf`. Scripts: `🧰️pf/` (`bench.ts`, `ab2.sh`, `all.ts`, `prof_summary.py`, `gen_fixture.ts`, `edtbench.ts`, `count.ts`, `same_modulo_digits.py`). Generated output (`🗑️generated/pf`) is deleted.

## Findings (profile of the full manifest, one process, before)

| Share of CPU | Where |
|---|---|
| 55 % | `featureWidthMm` (print suitability, `measure/🟦️.ts`): 14 float Felzenszwalb distance transforms per filled shape on rasters of up to 2.5 million cells; the oak has 120 shape measurements, each memo-less. |
| 20 % | `solveThermal`: history integrated from scratch for every frame (`solveThermalAt` re-evaluated the asset at every integration time and re-integrated from the window start). |
| 25 % | `evaluate`, `depict`, `writeSvg`, `placed` (three regexes over 150 KB strings). |
| memory | Every frame text was held three times (`drawn`, `placed` text as `Map` key, `files`); a 600 frame cast reached 1.2 GB. |

The "1848 s, 17 s per cast" of hp is the cost of one `repo tools exec` process per cast (about 10 s start-up each). `ensure_casts` (`V/🧍️staffage/🐍️.py:263`) already exports all missing casts in one process; the real full re-export cost is the single process figure below.

## Fixes (all exact; output unchanged except `data-value` digits)

- `S/🔨️modules/🖨️representation/📐️measure/🟦️.ts`: `squaredDistanceTransform` is now Meijster linear (integer vertical sweeps, stack envelope), values equal any exact transform; optional `DistanceWindow` (read spans, cap, margin, sparse) so only cells that can matter are visited; `featureWidthMm` loops only over mask cells, asks a boolean `exceeds(width, allowed)` with an early exit (total thin area, then first component above the tolerance) instead of measuring the largest component, and memoises identical shapes (bounded, 1024).
- `S/🔨️modules/🌡️thermal/🟦️.ts` (+ `⏳️transient`, `🕰️timeline`): `ThermalSession` (`thermalSession()`); the instants of the input history are memoised per absolute time and the integrations are kept as chains keyed by (start, grid step), so every frame that starts at the same time and grid continues the same chain and only does its final partial step. Results are bit-identical to solving alone (test). `HistoryPlan` carries `stepS`. Another agent has since extended `Trajectory` with `phaseKg` on top of this.
- `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`: one session per cast; frames kept as UTF-8 buffers until the shared viewBox is known, then placed, SHA-256 hashed (instead of string-keyed `Map`), written with 16 writes in flight and released; `placed` slices instead of a greedy regex; `patiently` retries `EPERM`/`EBUSY`/`ENOTEMPTY` of the Windows scanner (one full run died once in `rename`).
- `S/🔨️modules/📤️export/🖼️svg/🟦️.ts`: `data-value` with 6 significant digits (was `22.000000000000014`).
- Not done on purpose: coordinate precision is already 4 decimals; the SVG bytes are repeated `stroke-linecap`/`stroke-linejoin`/`id`/`data-bone` attributes (about 20 % of a frame), which belongs to the `writeSvg` owner; the 2.5 million cell raster cap stays (changing it would change feature widths). Streaming spill to disk was tried and rejected: three extra synchronous file operations per frame made the wall time worse on the scanned disk.

## Measurements (bun, quality l, shared host: CPU seconds are the stable figure)

| | Before | After |
|---|---|---|
| Full manifest, one process: 145 casts, 19 015 files | 198 s CPU, 258 s wall, 911 MB peak (1309 MB measured before the buffer change) | 97 s CPU, 158 s wall, 733 MB peak |
| `demand-primary-energy-tree` | 96 s CPU, 114 s wall, 687 to 796 MB | 33 s CPU, 40 s wall, 510 MB |
| `heating-internal-gains-server-rack` (600 frames) | 5.7 s CPU, 1285 MB (before the buffer change) | 5.4 s CPU, 220 to 510 MB |
| `ventilation-need-occupant-mirrored` | 3.4 s CPU | 2.5 s CPU |
| `ventilation-volume-student` | 4.5 s CPU | 3.6 s CPU |
| `physics-fundamentals-vacuum` (600 frames) | 4.0 s CPU | 3.1 s CPU |
| `demand-recap-desk` | 0.4 s | 0.4 s |

"Before" for CPU is the old measure module with a fresh thermal session per frame (`🗑️generated/pf/orig`, built from the same tree), so the A/B isolates this work. Equality: all 19 015 files of the 145 casts are identical between before and after, with `data-value` compared at 6 significant digits (`same_modulo_digits.py`); before this last digit change they were byte-identical.

Remaining cost: 35 s of the 97 s is still the oak's feature measure (2.5 million cell rasters, 14 transforms each). A further cut needs a different algorithm for the opening (local thickness) or a lower raster cap, both of which change measured widths.

## Tests

- New `🧪️tests/🚀️export-performance/` (`🟦️.ts`, `🥒️.feature`, scipy oracle `🐍️.py`, fixture `🧫️fixtures/🚀️export-performance/🔣️.json`): 28 tests. Distance transform against scipy (10 vectors) and brute force (60 seeded rounds, windowed sparse and dense sweeps), feature width against the pre-optimisation module (8 shapes, bit-identical) and analytic widths, thermal session against solving alone (walker clip in scrambled order beyond the window, radiator under a timeline), evaluation count, pinned SHA-256 of four fixture casts (measured values normalised to 4 digits because V8 and JavaScriptCore differ in the last bit of `exp`/`pow`), `data-value` digits, CPU guards (5 s for a 2.5 million cell transform, 30 s for a cap-size shape; observed 0.05 s and 1 s).
- Staffage vitest, ticket-local alias config, whole library: 54 files, 1101 passed, 11 skipped, 3 failed in `🎭️cast-export` while another agent was removing `language` from the cast command (re-run of cast-export, thermal-transient, print-suitability, catalog-physics, thermal-scenarios and export-performance afterwards: 6 files, 555 passed, 1 skipped, 0 failed). `tsc --noEmit` shows no error in the changed files.
- Visuals Python tests (`py_tests.sh` = `m9_tests.sh --no-sync`, because `uv` cannot resolve `pyet`/`pandas` at the moment): `🐍️staffage_only` + `🐍️`: 27 tests, 1 failure `test_thermal_span_every_clip_draws_its_thermal_casts_on_one_span` (a clip mixing two thermal spans, in the `thermalSpanK` work of another agent, not touched here).

## Open

- `repo dependencies ensure` / `uv` dependency resolution are broken independently of this work (pyet vs pandas 3.1).
- The first export after this change re-exports every cast (cast keys include the exporter source digest).
