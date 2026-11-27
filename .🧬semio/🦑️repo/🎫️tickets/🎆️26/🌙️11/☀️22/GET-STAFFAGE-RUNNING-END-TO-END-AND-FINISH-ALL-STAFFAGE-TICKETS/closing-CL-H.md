FINAL (2026-11-26 16:10: all five assets rewritten at the cause and proven; 0 failing matrix cells each, the three red `🧊️model-skeleton` cases and the check CLI of `person-queuing` are green; no test, pin or fixture touched)

# 🧍️ Closing Lane CL-H (Humans): Five Assets Against the Detail Pins

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-H` (tools, kept), `GEN` = `U/🗑️generated/matrix/CL-H` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time, peaks 64 to 888 MB.

## 1. Cause

None of the five defects came from the multiview repairs: `W/probe.py` over the git states of 11-23, 11-24 (twice) and 11-25 reads the same values as the catalog of today. The four `🧊️model-skeleton` defects sat in `volumes` (3D) since before the case ran against these assets; the fifth is a 2D shape of the 1:500 band. So the cure is in the asset, the pins are right.

## 2. Result per asset

| Asset | Before | Change (ops in `W/ops/<id>.json`, applied by `W/fix.ts`) | After | Digest before -> after |
| --- | --- | --- | --- | --- |
| `person-on-crutches` | `tibia-l`, `tibia-r` at 1:200 and 1:500: narrowest section 12.5 x 12.5 mm against 12 x 13 mm of the 1:50 shaft, 0.707 mm (limit 0.1 mm) | the shaft station at 0.29 m of both tibiae at 1:200 and 1:500 gets `rx` 0.012, `ry` 0.013 (8 values) | 0.000 mm at the four scales, all ten paired shafts | `afe14bf5dcd2126c` -> `d9d3738dd793438c` |
| `tour-guide` | `humerus-l`, `humerus-r` at 1:200: a cylinder of 15.5 mm radius at all four stations against the 1:50 shaft of 10.7 mm, 6.79 mm | the four stations of both humeri at 1:200 get the 1:50 radii 12.4, 11.1, 10.7, 12.8 mm and the solids declare `slender: true` as at 1:500 (the thick cylinder predates the ruling of 11-22 that a long member keeps its true section) | 0.000 mm at the four scales | `25dbdb3d993acedf` -> `11af475089707b56` |
| `parent-carrying-toddler` | `th-toddler-nose-breath` 2.236 mm (1:50, 1:100) and 11.18 mm (1:200, 1:500) beside the anchor `toddler-nostrils`; `th-toddler-mouth-breath` 2.0 mm beside `toddler-mouth` at four scales (limit 1 mm) | both outlets sit on their anchors at every scale ((-0.004, -0.092, 0) and (-0.019, -0.08, 0) in `toddler-head`). At 1:200 and 1:500 the coarse head has no nose, the anchor lies about 11.5 mm in front of the head skin, more than the declared aperture of 5.5 mm, so the thin outlet cannot stay a thin face there: the nose outlet becomes a sphere of one smallest feature (radius 0.0156 m and 0.039 m), the convention `parent-with-stroller` already uses for its child | 0.000 mm for both outlets at the four scales | `4533d58d75c5a7b3` -> `a9d6445426694a01` |
| `parent-with-stroller` | 1:200 has no member solid for `child-foot-l`, `child-foot-r` although the 1:50 capsule (75 x 12 mm) is admitted there as a slender member | `child-foot-bones-l`, `child-foot-bones-r` of 1:100 copied into the structure of 1:200 behind `child-tibia-l`, `child-tibia-r` | both feet carry their member at 1:50, 1:100, 1:200 (1:500 is not admitted and stays without) | `9916430ac2179aa9` -> `6de73db4b4bb0c59` |
| `person-queuing` | check CLI `✗`: `suitability-feature 1:500 sil-seat: 0.184 mm < 0.2 mm`. Measured (`W/boxes.ts`): in the front view only a strip of 0.119 mm of the seat shows below the hem of `sil-body`; 0.184 mm in `looking-ahead`, 0.198 mm in two more poses | the crotch edge of `sil-seat` at 1:500 lies 16 mm lower (`x` -0.108485 -> -0.124485 in `pelvis`), in the front view and in the elevation alike so that the two views stay in step (4 values) | 0.216 mm (`looking-ahead`), 0.230 to 0.232 mm in the four other poses; check CLI `✓`, 1:500 suitable | `80a50a826166d1c9` -> `983f320f2f848b51` |

## 3. Proofs

Light proof on the trial copy BEFORE the write (`W/light.sh`: gate over every subject, gate over the poses, silhouette guard with `validateAsset` and the three-views sweep, check CLI `--file`): PASS for the five copies (14:08 to 14:15), each with 0 hard findings, `silhouette: OK`, 0 issues, 0 sweep findings, `✓`.

After the write (`W/prove.sh`, 14:50 to 15:13; the catalog file equals the proven copy byte for byte, checked again at 16:10):

| Asset | Gate | Scan | Silhouette guard, `validateAsset`, sweep | Check CLI | Matrix (252 cells) |
| --- | --- | --- | --- | --- | --- |
| `person-queuing` | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 63 ok, 126 group-drift, 63 range-end (136 s) |
| `person-on-crutches` | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 231 group-drift, 21 range-end (66 s) |
| `tour-guide` | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 70 ok, 182 group-drift (130 s) |
| `parent-with-stroller` | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 7 ok, 203 group-drift, 42 range-end (312 s) |
| `parent-carrying-toddler` | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 77 ok, 175 group-drift (348 s) |

Real test files (`W/suites.sh`, 16:05 to 16:07, on the written files):

- `🧊️model-skeleton` with the filter `(two forearm bones|passenger)`: 3 passed of 3 selected (the shaft case over its eight subjects, the two passenger cases), 24 skipped.
- `🧊️catalog-silhouette` with each asset as filter: 1 passed each, five runs.

Not run by this lane: the rest of `🧊️model-skeleton` (other lanes' assets) and the full suites; lane MZ re-runs these five (lines `CL-H rewrote …` under the two closing sections of `requests.md`).

## 4. Stated plainly

- Print suitability at the band ends (`../MC/prints.ts`) still refuses shapes this lane did not touch, the same before and after: `person-queuing` `sil-shoe-l` 0.198 mm at 1:350 and `backpack` 0.194 mm at 1:1000 (the `sil-seat` refusal at 1:500 is gone, 4 -> 3); `person-on-crutches` crutch tips 0.184 mm at 1:1000; `parent-with-stroller` `canopy` 0.196 mm and `rear-axle` 0.198 mm at 1:350. The matrix reports them as `range-end`, not as failures, and the check CLI (key scales) is green. They were not in this lane's order.
- The coarse nose outlet of the toddler is a feature-sized sphere at 1:200 and 1:500, not a thin face: a modelling decision forced by the pin (outlet on the anchor) together with `volumes-feature` (a thin outlet must lie within one aperture of the tissue).
- `person-queuing` at 1:500: in `weight-on-left` and `weight-on-right` the seat was wholly hidden in the front view and now shows 0.232 mm of its lower edge.

## 5. Tools (`W`)

`fix.ts` (ops to a trial copy, same indentation), `ops/`, `make.sh`, `light.sh`, `lights.sh`, `prove.sh`, `proves.sh`, `suites.sh`, `chain.sh`, `probe.py` (the pinned details of a folder of documents), `boxes.ts`/`boxes.sh` and `feature.sh` (visible feature of a shape), `check-asset.sh`, `prints-before.sh`, `queuing.sh`, `note.py`, `q.sh`, `env.sh`.
