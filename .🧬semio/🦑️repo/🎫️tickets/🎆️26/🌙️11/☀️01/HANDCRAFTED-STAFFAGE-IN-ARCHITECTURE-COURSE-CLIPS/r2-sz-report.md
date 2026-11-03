# R2 sz report: smaller cast frames, identical pictures

Agent `sz`. Scripts: `🧰️sz/` (`anatomy.py`, `compare.py`, `parsetime.py`, `parts.py`, `precision.py`, `repin.ts`, `run_py.sh`). Generated output (`🗑️generated/sz`) is deleted.

## Change

- `S/🔨️modules/📤️export/🖼️svg/🟦️.ts`: new `SvgOptions { compact }` (re-exported from `S/📦️packages/🟦️typescript/🟦️.ts`). `writeSvg(scene)` without options is unchanged for every other consumer. With `compact`:
  - `fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round"` are written once on each layer group (inherited, SVG 1.1); a path writes `stroke-linecap`/`stroke-linejoin` only when its stroke is not round.
  - Paths without a group (no value, bone or region) lose their `id`; nothing addresses them (the Python parser reads `<g id="shape-...">` only, tests read `staffage-cast`, `shape-figure`). Group ids stay.
  - Path data stays absolute (`M`, `L`, `C`, `Z`), a run of one command is written once, leading zeros and the space before a minus sign are dropped (`compactData`). Numbers are the same decimals as before.
- `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`: the cast exporter calls `writeSvg(scene, { compact: true })`.
- `V/🧍️staffage/🐍️.py` `_parts` (`staffage_parts`): `M`/`L` now read every coordinate pair of a run (before it read the first pair only, which an `L` run would have truncated). `_outline` / `_parse_frame` already handled runs, `C` runs and ignore the hoisted attributes, so they needed no change.
- Cache: `_source_digest` hashes the whole `🔨️modules` tree, so the new frame form gets new cast keys and never mixes with old casts. The coordinator's note on a `l` failure came from a relative-path experiment of mine (see below) that the live exporter wrote into the cache for a short time; the final writer emits only `M L C Z` (absolute, uppercase), which every Python reader supports. Nobody else needs a format constant; cv2 owns the key.

## Tried and rejected

- Relative `l`/`c` path data (about 25 % smaller on the oak): MuPDF accumulates in float32, 62 of 255 sampled frames differed (max 19 of 255). Not pixel-identical, dropped; the Python parser support for it was removed again.
- Coordinates at 3 decimals (1 micrometre): oak 10 % smaller but 77 349 of 18.8 million pixels differ (max 24). Rejected.
- A `<style>` with classes: not needed once the attributes are inherited from the layer group, and class selectors are less safe in the PyMuPDF/rsvg renderers.

## Measurements (98 casts in the manifest, 16 039 frames, quality l, same tree, exported back to back)

| | Before | After |
|---|---|---|
| Total bytes | 312 523 663 | 270 620 557 (-13.4 %) |
| Largest frame (oak `demand-primary-energy-tree/0000.svg`) | 294 405 | 279 654 |
| Python `_parse_frame`, 4 095 frames (every fourth + largest per cast), 4 728 080 points | 36.1 s, 36.7 s | 35.3 s, 33.2 s (about -5 %) |

Why the oak stays above 200 KiB: 85 % of its bytes (238 511 of 280 000) are the `d` coordinates (27 436 numbers at 4 decimals); ids, caps and joins were only about 12 %. Reaching 200 KiB needs fewer coordinates (fewer canopy points in the asset) or a coarser grid, both of which change pictures. The two volume casts are the same.
Remaining levers, all outside "identical pictures": a repeated `<metadata>` legend block of about 3 KB per frame (15 % of all bytes, not read by Python), and the `data-*`/`<title>` strings of every shape.

## Proof of identical pictures

`compare.py` rasterised (PyMuPDF, 300 dpi, alpha) the first, middle, last and largest frame of each of the 98 casts, both exports: **255 frames, 0 different pixels**, identical `🔣️.json` index files, identical file names. `parts.py`: `_parse_frame` points, spans and styles of 213 frames and 1 756 `staffage_parts` polygons are bit-equal between the two exports.

## Tests

- New Python test `Parser.test_the_compact_frame_form_parses_like_the_long_form` (`V/🧍️staffage/🧪️tests/🐍️.py`): long and compact text give equal `_parse_frame` result and equal `_parts`, covering `M`, `L` runs, `C` runs, `Z`, leading-zero and minus-separator numbers. Staffage python suite: 41 tests, 40 pass; the failure `Fit.test_the_unit_is_the_largest_that_fits...` (fitted width 0.4 vs 2.0) is in `fitted_staffage`, not touched here.
- New vitest `export-performance` "compact frame form" (same d numbers as the plain form, caps and joins only on layer groups, no bare path ids). cast-export, export-performance, paper-dimensions, plan-dimension: 4 files, 98 passed, 5 skipped, 0 failed.
- Re-pinned digests (intentional format change): `S/🧫️fixtures/🚀️export-performance/🔣️.json` `castDigests` of `walker-line`, `walker-thermal`, `radiator-transient`, `radiator-rated` (40 hashes, nothing else touched, recorded with `🧰️sz/repin.ts`). The radiator pins were recorded on the current tree, so they also carry any radiator catalog change that agent ap has made; ap must re-pin only if the radiator changes again.

## Open

- The oak and the two volume figures remain above 200 KiB (see above); decide whether the budget is per frame (OI-11) before changing assets.
- `Fit` test failure above belongs to the owner of `fitted_staffage`.
