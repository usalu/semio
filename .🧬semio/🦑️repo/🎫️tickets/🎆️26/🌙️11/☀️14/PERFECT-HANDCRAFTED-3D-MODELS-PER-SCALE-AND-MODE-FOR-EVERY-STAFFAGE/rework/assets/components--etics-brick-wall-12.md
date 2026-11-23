# 🧥 Brick Wall with 12 cm ETICS (`components/etics-brick-wall-12`)

## 2026-11-22 23:00 — coarse scales for their own smallest feature (umbrella rework, report `U/rework-walls.md`)

Findings before (`r3-validate.ts components/etics-brick-wall-12 --catalog-only --no-print`): 122 `volumes-feature` (1:100: 29, 1:200: 32, 1:500: 57 in the body, 4 anchors of 60 mm in the 1:500 structure): the brick bond of 1:50 at every scale and the two render coats (8 and 12 mm, one material with two gauges).

Cause: the 1:100, 1:200 and 1:500 lists carried the parts list of 1:50. Each coarse scale is now modelled for its own smallest feature, following what the 2D drawing of that scale retains, merges and omits.

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:100 (15 mm), 66 → 6 solids | `course-a`, `course-b` (83 mm bands over the 380 mm leaf with the plaster); the two lower boards and the upper board course 130 mm deep with the adhesive in them; `render` as one 20 mm coat; anchors left out | all six layers as own shells, boards as before (8) | unchanged: leaf, support, mass, four anchors (7); decision `Minimal:` because 1:50 holds the same seven |
| 1:200 (30 mm), 61 → 3 | `bond-band` (six bands of 167 mm, 380 mm deep); lower and upper board course 150 mm deep (adhesive, wool, render), as the two drawn fields | all layers as own shells, two board courses (7) | leaf, support, mass; anchors gone (7 → 3) |
| 1:500 (75 mm), 60 → 1 | `wall`: one homogeneous 530 mm body | all layers as own shells (6) | leaf, support, mass (7 → 3); decision `Minimal:` because 1:200 holds the same three |

Sheets: Opened at half size: 1:100 body, 1:200 thermal. Not opened: every other sheet of this asset and its 2D drawings (unchanged); the sister asset `etics-brick-wall-30` was opened at 1:200 body and 2D 1:100.

Proof (2026-11-22, pinned bun, one process at a time, each run to its end):

- `r3-validate.ts components/etics-brick-wall-12 --catalog-only`: `VALID components/etics-brick-wall-12 (catalog file as it is): schema 0, validateAsset 0, checkVolumes not run (--volumes)`
- `r3-validate.ts components/etics-brick-wall-12 --catalog-only --volumes`: `VALID components/etics-brick-wall-12 (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`
- check CLI `--asset etics-brick-wall-12`: `✓ etics-brick-wall-12 (1/1)`
- volumes source `volumes/components/etics-brick-wall-12.json` equals `asset.volumes` of the catalog file (compared after the splice with `volumes-insert.py`).

Script: `rework-etics-brick-wall-12-coarse.py` (states every hand-chosen number; it refuses a second run on the rebuilt source, a rerun needs the source of before this pass). The 2D drawings were not changed: their `decisions` (retained, merged, omitted) per scale were the specification the 3D lists were brought to. No evaluated manifest value changes.

What stays imperfect: Two scales carry a `Minimal:` decision for the structure list (1:100 equals 1:50, 1:500 equals 1:200 in count); the body and thermal lists do simplify at every step. The bond bands of 1:200 show no seams (copies of one solid). No reference table against the real subject was written and no pose or clip was rendered in this pass; no independent review.
