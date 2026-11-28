FINAL (2026-11-27 08:30: chord of a scale now grows with the denominator for every asset; triangles fall at every scale and mode in all 375 assets, 1:500 -10.6 to -14.5 %, 1:50 unchanged; all required proofs green, 24 digests of the sample figure re-pinned)

# CX-Q: tessellation quality monotone in the scale

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, tools `U/🧰️matrix/CX-Q/`.

## Cause and cure

`qualityOf` took `chord = min(largest / 3200, 2 % of the feature of the scale)`. For an asset under 4.8 m the first term wins and does not depend on the scale, while a marker (mass, joint, glyph) is sized by the scale (one feature in diameter: 7.5, 15, 30, 75 mm). Constant chord with a growing radius gives more segments: an ellipsoid marker of a 0.6 m asset had 120 / 210 / 360 / 960 triangles at 1:50 / 1:100 / 1:200 / 1:500.

New rule (`S/🔨️modules/🧊️volume/⏱️job/🟦️.ts`, `ROUNDNESS.referenceScale = 50`):

`chord(asset, scale) = min(largest * max(scale, 50) / (50 * 3200), 0.02 * feature(scale))`

- Both terms are proportional to the denominator from 1:50 on, so the chord never falls towards a coarser scale and a solid sized by the scale keeps its segments (the ratio chord / feature is constant).
- At 1:50 and finer the value is exactly the old one: the 1:50 quality of every asset is unchanged (measured: 0 of 375 assets differ at 1:50). Assets over 4.8 m (feature term) are unchanged at every scale: chords are bit-identical.
- Marker triangles for assets of 0.1 m and more: constant 600 / 210 / 120 / 120 ... from 1:50 to 1:500 by asset size, never growing (before, 0.1 m: 600 / 1190 / 1520 / 960). A 1 cm asset: 3968 / 2208 / 1520 / 960 (was growing in the other direction at the coarse end).
- The segment caps (96, 64, 48, 40, 32) and the rest of the rule are untouched. Docstrings of `ROUNDNESS` and `qualityOf` updated.

## Test (language-agnostic, with third-party check)

- Fixture `S/🧫️fixtures/🧊️model-roundness/🔣️.json`: `referenceScale`, the rule in the `$comment`, and `markers` (`markerFeatures` 1, assets of 0.01, 0.03, 0.1, 0.3, 0.6, 0.9, 1.7, 2.5, 4.8 and 10 m, scales 50 / 100 / 200 / 500).
- Feature `S/🧪️tests/🧊️model-roundness/🥒️.feature`: scenario "A marker sized by the scale never has more triangles at a coarser scale".
- Test `S/🧪️tests/🧊️model-roundness/🟦️.ts`: the chord formula case now follows the new rule (also asserts the chord sorted ascending); new `describe` with, per asset size, chords not falling and triangles of sphere, cylinder, capsule and round-loft markers not growing from 1:50 to 1:500; the triangles of a marker sphere equal three's `SphereGeometry(r, n, ceil(n / 2))` for `n` computed from the rule (third-party check; the kernel's ellipsoid equals three's sphere of n x ceil(n / 2) segments for n = 12 ... 32, verified in `U/🧰️matrix/CX-Q/rings.ts`); and markers of assets of 0.1 m and more keep the triangles of 1:50 at the coarser scales. trimesh (existing oracle) still counts the meridians of the fixed-radius cases.

## Effect over the catalog (375 assets, default pose, every scene; kernel tessellation old chord vs. new chord in one pass, `tri.ts`)

| scale | mode | triangles before | triangles after | change |
|---|---|---|---|---|
| 1:50 | body | 31,790,092 | 31,790,092 | +0.0 % |
| 1:50 | structure | 22,754,770 | 22,754,770 | +0.0 % |
| 1:50 | thermal | 41,002,988 | 41,002,988 | +0.0 % |
| 1:100 | body | 11,787,711 | 11,738,541 | -0.4 % |
| 1:100 | structure | 12,690,848 | 12,632,274 | -0.5 % |
| 1:100 | thermal | 16,685,999 | 16,628,949 | -0.3 % |
| 1:200 | body | 4,375,868 | 4,257,078 | -2.7 % |
| 1:200 | structure | 7,658,520 | 7,385,718 | -3.6 % |
| 1:200 | thermal | 7,047,720 | 6,904,674 | -2.0 % |
| 1:500 | body | 2,099,104 | 1,841,201 | -12.3 % |
| 1:500 | structure | 5,726,544 | 4,893,408 | -14.5 % |
| 1:500 | thermal | 3,430,889 | 3,067,244 | -10.6 % |

- 305 of 375 assets change (58 at 1:100, 214 at 1:200, 305 at 1:500); no scene of any asset has more triangles than before; 0 assets with a mode whose triangles grow (before and after, on the current catalog; the growth findings of CL-T2 had already been cured in the assets).
- Largest 1:500 structure savings: seated-office-worker 42,492 -> 27,600, parent-carrying-toddler 42,964 -> 28,580, walking-toddler 28,850 -> 15,816, running-child 28,588 -> 16,054, schoolchild-with-backpack 27,810 -> 15,978.
- Because the rule now keeps markers constant, the ellipsoid masses and joints no longer depend on shrinking members for the budget; the structure views of the coarse scales are rounder only where they were rounder than 1:50 before, never coarser than 1:50.

## GLB size of the 10 largest pose files: not measured

The comparison needs the GLB writer of lane CX-GLB in one state; it stayed queued behind other lanes' grants for more than two hours, a first pass ran without the old chord by a shell bug of mine (the preload flag was passed as one word), and the corrected pass was dropped at the coordinator's request. What is known: the 10 largest poses are trees (norway-maple, black-alder, red-oak, rowan, crab-apple, saucer-magnolia, european-larch, multi-stem-birch), tower-crane and parent-carrying-toddler. All trees and the crane are over 4.8 m: their chord is bit-identical, so their GLBs do not change. Only parent-carrying-toddler (1:500 structure 42,964 -> 28,580 triangles in the default pose) can shrink, by about 1 to 3 % of its 7.6 MB (an estimate from the triangle counts, not a measurement).

## Proofs on the final rule (all through `repo capacity run`)

| Proof | Result |
|---|---|
| `🧊️model-roundness` (with the new monotone test and the old cases), `🧊️model-bounds`, `🧊️model-scales`, `🧊️model-frame`, `🧊️loft-shape`, `🧊️volumes-rules` | 6 files, 344 tests passed |
| single-asset `🧊️model-catalog` of `used-chess-table` (furniture, blocks of CL-T2), `walking-toddler` (human), `city-car` (vehicle), `tealight` (object), `shrub-rose` (vegetation), `kitchen-extractor-hood` (appliance) | 8 passed |
| `🧊️cast-lod`, `🧊️model-scale-files`, `🧊️model-glyph-edges`, `🧊️model-glyph-size`, `🧊️model-silhouette`, `🧊️model-skeleton` | 6 files, 88 passed |
| `🧊️model-exports`, `🧊️model-digests` after the re-pin | 2 files, 61 passed |

Not re-run: `🧊️catalog-silhouette`, `🧊️three-views`, `🧊️volumes-thermal`, `🧊️volumes-flood`, `🧊️volumes-connected`, `🧊️volume-schema`, `🧊️model-looks`, `🧊️model-batch` (a first chain of all of them timed out at 3400 s without output and was split; the second chain covered only the suites above). The coordinator's full model-catalog pass runs on the same rule.

## Digest re-pin (`S/🧫️fixtures/🧊️model-digests/🔣️.json`, 24 entries)

Digests of the sample figure were generated twice with the current writer (CX-GLB's compact writer included): with the old chord (preload `old-quality.ts`) and with the new one (`digests.ts`). The fixture on disk equals the old set in all 42 entries (nothing of other lanes pending), so every difference is this change:
- GLB, 6 poses (`stand`, `stride-left-forward`, `passing-left-stance`, `stride-right-forward`, `passing-right-stance`, `carry-bag`): `triangles`, `segments`, `scenes` (the s200 and s500 scenes), `accessors`, `byteLength`, `sha256` re-pinned. Example `stand`: s200 body 7000 -> 6660 triangles.
- FBX, 18 entries (`walk`, `idle-breathing`, `bag-swing` at s200 and s500 in `light-solid`, `thermal`, `forces`): `byteLength` and `sha256` only; the 18 entries at s50 and s100 are unchanged.

## Stated plainly

- No catalog file was rewritten; no ticket opened or closed.
- I used `pkill -f "CX-Q/glb"` once to remove my own stuck GLB requests (should have been by pid); it matched only my processes. My first waiters (`pgrep -f "glb.ts"`) matched their own command line; the coordinator stopped one; later waits are by pid.
- The structure views at 1:200 and 1:500 show markers with the same segments as at 1:50 (no longer finer); nothing was rendered or looked at.
- Measurement covers the default pose of each asset (not every pose).
- The temporary outputs under `U/🗑️generated/matrix/CX-Q/` are deleted.
