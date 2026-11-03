# R2 fm report: print-suitability feature measure

Agent `fm`. Scripts: `🧰️fm/` (`run.sh`, `ab.sh`, `digests.py`; uses `🧰️pf/bench.ts`). Generated output (`🗑️generated/fm`) is deleted.

## Profile (oak cast `demand-primary-energy-tree`, before)

33.5 s CPU. 31 of 32.8 s inside `featureWidthMm`: `squaredDistanceTransform` 12.3 s self, `exceeds` 9.4 s, `sparseVertical` 6.8 s, setup 1.8 s. 120 distinct filled shapes (all memo misses): 30 returned `top` after one transform, **90 ran the full bisection** (13 transforms each, about 0.4 s per shape, canopy blobs of 500 to 575 points, widths 67 to 93 mm). Nothing else (`whiteSliver`, `gapsBetween`) is visible in the profile.

## Finding

The caller (`🖨️representation/🟦️.ts:87-91`) uses the width only through `minFeature = min(minFeature, max(raw, min(boxMin, profile)) + weight)` and the violation test `width < profile.minFeatureMm - 1e-9`. A shape whose width is already proven to be at least the running minimum (and at least the profile limit) changes neither, so its exact width is never needed. The oak has thin leaves (1.9 mm) next to 90 mm canopies.

## Change (exact)

- `🖨️representation/📐️measure/🟦️.ts`: `featureWidthMm(rings, minFeatureMm, atLeastMm = Infinity)`. The result is unchanged without `atLeastMm`. With it, the bisection returns a proven lower bound (at least `atLeastMm`, at most the exact width, never memoised). The bisection path is exactly the old one: `low` only rises to a tested `mid` and `high >= low` always, so `low >= atLeastMm` proves `result >= atLeastMm`. One extra first probe at `top / 2^j` (the smallest such node still `>= atLeastMm`; all its bisection ancestors are larger): if `exceeds(probe)` is false the result is `>= probe` whatever `exceeds(top)` says, so a wide shape costs one transform instead of 14. Tested widths are cached inside the call (`tested` map), so a failed probe is not repeated. `measureFeatureWidthMm` returns `{ widthMm, exact }`; only exact results enter the memo.
- `🖨️representation/🟦️.ts:85-91`: shapes are measured in ascending order of their smaller box side (an upper bound of the width, so thin shapes come first and set the minimum early), each with `atLeastMm = max(minFeature, profile.minFeatureMm) - weight + 1e-9`, then the old loop reads the stored values. Order of `featureWorst` insertion (violation order) is unchanged because the second loop is the old one.
- Not done: anything that changes widths (raster cap, pitch, tolerance, bounding-box cropping would change the pitch-derived raster only if the cap were involved; nested threshold sets are not provably nested on the discrete grid, so no incremental bisection).

## Measurements (bun, quality l, one process, shared host)

| | Before | After |
|---|---|---|
| `demand-primary-energy-tree` | 33.5 s CPU (33.1 s via the A copy) | 9.4 s CPU |
| Full manifest, 145 casts, 15 536 files (A/B, same tree and catalog, back to back) | 88.1 s CPU, 121 s wall, 1008 MB | 65.5 s CPU, 100 s wall, 858 MB |

Equality: `ab.sh` exports the whole manifest with an exporter copy holding the old measure and caller (A) and with the live tree (B); the SHA-256 lists of all 15 536 files are **identical**. Oak alone: identical to the earlier baseline as well. (An earlier before/after comparison across hours differed in laptop, radiator and watt casts only because other agents edited `🗂️catalog/…/laptop` and `panel-radiator` meanwhile; the A/B on one tree removes that.)

Remaining oak cost 9.4 s: 120 shapes of setup (rasterise, dense outside transform) plus one small-radius transform each.

## Tests

- New test in `🧪️tests/🚀️export-performance/🟦️.ts` (lower bound property on all feature vectors for five limits, bit-exact below, memo not polluted) and a scenario in `🥒️.feature`.
- Vitest (ticket-local alias config, file filter `export-performance print-suitability cast-export`): 3 files, 89 passed, 1 skipped, 1 failed: `exported frames > equal the pinned digests` on `radiator-transient/0000.svg`. Cause is not the measure: `catalog/🧱️components/panel-radiator/🔣️.json` is modified in the working tree by another agent (A/B of the full manifest is identical with the live catalog). The pin needs re-recording by whoever owns the radiator change.

## Open

- The scipy oracle test (`🐍️.py`) was not run separately (Python resolution is broken, see pf report); the vitest file asserts the same vectors.
