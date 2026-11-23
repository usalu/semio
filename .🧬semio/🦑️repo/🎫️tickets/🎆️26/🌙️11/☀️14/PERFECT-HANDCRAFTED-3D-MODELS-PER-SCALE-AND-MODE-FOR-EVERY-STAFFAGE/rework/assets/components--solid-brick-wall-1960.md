# 🧱 Solid Brick Wall 1960 (`components/solid-brick-wall-1960`)

## 2026-11-22 23:00 — coarse scales for their own smallest feature (umbrella rework, report `U/rework-walls.md`)

Findings before (`r3-validate.ts components/solid-brick-wall-1960 --catalog-only --no-print`): 112 `volumes-feature`, all in the body lists (1:100: 27, 1:200: 30, 1:500: 55): the 57 bricks and joints of the 1:50 bond at every scale (joints 10 and 12.3 mm, cut bricks 25 and 27.5 mm, every brick 71 mm at 1:500).

Cause: the 1:100, 1:200 and 1:500 lists carried the parts list of 1:50. Each coarse scale is now modelled for its own smallest feature, following what the 2D drawing of that scale retains, merges and omits.

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:100 (15 mm), 57 → 3 solids (14 with repeats) | `course-a` and `course-b`: header and stretcher courses as two alternating 83 mm bands (six each) over the 380 mm leaf with the plaster in it, as drawn; `render` 20 mm stays | `th-plaster` as own shell, four brick depth cells, render (6) | the two course bands as members, support, mass (57 → 4) |
| 1:200 (30 mm), 57 → 1 (6 with repeats) | `bond-band`: six bands of 167 mm (one header and one stretcher course) over the full 400 mm | three layer shells as own solids, two depth cells (4) | `member-bond-band`: the bearing 365 mm leaf in the same six bands, support, mass (57 → 3) |
| 1:500 (75 mm), 57 → 1 | `wall`: one homogeneous 400 mm body | three layer shells, one depth cell (3) | `member-leaf` 365 mm, support, mass (57 → 3) |

Sheets: Opened at half size: 1:100 body and thermal; 1:200 body and structure. Not opened: 1:100 structure, 1:200 thermal, all three lists of 1:500, the 2D drawings (unchanged).

Proof (2026-11-22, pinned bun, one process at a time, each run to its end):

- `r3-validate.ts components/solid-brick-wall-1960 --catalog-only`: `VALID components/solid-brick-wall-1960 (catalog file as it is): schema 0, validateAsset 0, checkVolumes not run (--volumes)`
- `r3-validate.ts components/solid-brick-wall-1960 --catalog-only --volumes`: `VALID components/solid-brick-wall-1960 (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`
- check CLI `--asset solid-brick-wall-1960`: `✓ solid-brick-wall-1960 (1/1)`
- volumes source `volumes/components/solid-brick-wall-1960.json` equals `asset.volumes` of the catalog file (compared after the splice with `volumes-insert.py`).

Script: `rework-solid-brick-wall-1960-coarse.py` (states every hand-chosen number; it refuses a second run on the rebuilt source, a rerun needs the source of before this pass). The 2D drawings were not changed: their `decisions` (retained, merged, omitted) per scale were the specification the 3D lists were brought to. No evaluated manifest value changes.

What stays imperfect: The six bond bands of 1:200 are copies of one solid, so the sheet shows no seam between them: the body reads as one mass, which is what the drawing shows. Header and stretcher courses of 1:100 differ only by their id and position, the longitudinal joint that told them apart is below the feature. No reference table against the real subject was written and no pose or clip was rendered in this pass; no independent review.
