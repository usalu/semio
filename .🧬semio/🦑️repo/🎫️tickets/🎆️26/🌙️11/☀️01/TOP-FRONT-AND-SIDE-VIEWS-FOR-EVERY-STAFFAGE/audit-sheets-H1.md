# Audit H1: visual read of the human staffage at 1:50 and 1:100

Scope: seven humans (child-on-kick-scooter, courier-with-hand-truck, cyclist, elderly-with-cane, jogger, kneeling-gardener, leaning-person), every pose and every clip instant (driver fractions 0.1, 0.5, 0.9 of each clip duration), scales 1:50 and 1:100. Read only, no asset or source file changed.

Render: `T/🧰️QA4/all.ts` with `QA4_SCALES=50,100`, output `T/🗑️generated/R-H1/`, 150 three-view renders at 150 dpi (`rasterize.json` through `T/🧰️QA1/raster.py`). Sheets: `T/🧰️R-H1/sheets.py`, three renders per sheet with filename labels.

Check output (`checkAssetDetailed`, samples 3, plus sweep and consistency): all seven assets `ok` except jogger, which reports three feature-size findings at 1:500 (see row J-3). No sweep or consistency finding at 1:50 or 1:100.

## Sheets read

**54 sheets read** (every sheet at 1:50 and 1:100, one Read each):

| Asset | 1:50 sheets | 1:100 sheets | Renders on sheets |
|---|---|---|---|
| child-on-kick-scooter | 3 | 3 | 11 |
| courier-with-hand-truck | 4 | 4 | 10 |
| cyclist | 5 | 5 | 16 |
| elderly-with-cane | 4 | 4 | 12 |
| jogger | 4 | 4 | 16 |
| kneeling-gardener | 3 | 3 | 8 |
| leaning-person | 4 | 4 | 12 |

Renders total 150 over the 54 sheets. The index is in `T/🗑️generated/R-H1/sheets/index.txt` (deleted at the end; see cleanup below).

## Defects

Severity: high = wrong geometry in a view a reader would misread; medium = visible error or unclear form at print size; low = readability or annotation nit.

| ID | Asset | Scale | Pose / instant | View | Defect | Severity |
|---|---|---|---|---|---|---|
| K-1 | kneeling-gardener | 1:50 and 1:100 | dig-cycle @0.16, @0.8, @1.44; reach-to-trug @0.4, @2, @3.6; reaching-trug; kneeling-weeding | front | In the bent poses the head is not readable. The front view is a tall block: a box at head height with no face or head outline, then stacked segments. The upright and kneeling-upright poses read correctly (hat with brim), so the problem is the bent head and shoulder silhouette. Confirmed on a 2.2x zoom of dig-cycle @0.16 at 1:50. | medium |
| C-1 | cyclist | 1:50 and 1:100 | pedal-cycle @0.4 (also @2, @3.6), riding-upright, standing-on-pedals, dismount @1.2 | front | Front wheel contact: the front tyre and the feet look a little above the front-panel ground line (about 0.2 m at 1:50 on a 2.5x zoom of pedal-cycle @0.4), while the elevation wheels sit on the ground. A small dash sits under the ground line in several front renders. Not confirmed by pixel measurement (the scan timed out); verify before relying on it. | medium (to verify) |
| J-1 | jogger | 1:50 and 1:100 | stretch-hamstring, stretch @10.8 | front | The torso in the bent-over pose reads as a flat box above the head. The head sits inside the box, so the fold is hard to read. | low |
| J-2 | jogger | 1:50 | stretch-quad-r, stretch @6 | front | The raised, bent leg is drawn as a block; the shin and foot are unclear. Readable, but ambiguous at print size. | low |
| J-3 | jogger | 1:500 (check output, not on these sheets) | jogger check | feature | Feature-size violations at 1:500: `hand-r` 0.167 mm, `forearm-l` 0.182 mm, `hand-l` 0.174 mm, all below the 0.2 mm minimum. Owner: QA4 and FW's feature rule. | medium |
| E-1 | elderly-with-cane | 1:100 | cane-gait, cane-planted, rest-on-cane, resting-on-cane | plan | The plan silhouette is a small "C" blob; the cane shaft and hand are not separable at 1:100. Readable only at 1:50. | low |
| CS-1 | child-on-kick-scooter | 1:50 | kick-and-glide @0.22 (and kick-and-glide @1.1 at 1:50) | front | The arms and handlebar read as one wide block below the shoulders, like a basket, at hip level. | low |
| CS-2 | child-on-kick-scooter | 1:50 and 1:100 | kick-and-glide @1.1 and @1.98 | front | Height dimension extension lines span across the full front panel. This is the dimension annotation, not the asset; noted for the depiction owner. | low (annotation) |

No defects found on the 1:50 or 1:100 sheets for: courier-with-hand-truck (all poses, all clip instants); leaning-person (all poses; wall, shelf and plan consistent); the three views agree in left-right placement, moving parts show the right spread (arms out in front and plan where the pose says so); no floating or detached limbs were seen in the elevation views; no shape below ground in elevation.

## Cited PNGs

Kept in `T/🗑️generated/R-H1-cited/` for the defects above:
- `child-on-kick-scooter--kick-and-glide@0.22--views-dark-line-1-50.png` (CS-1)
- `cyclist--pedal-cycle@0.4--views-dark-line-1-50.png` (C-1)
- `kneeling-gardener--dig-cycle@0.16--views-dark-line-1-50.png` (K-1)
- `jogger--stretch-hamstring--views-dark-line-1-50.png` (J-1)

## Remaining doubts

- C-1 needs a pixel-measured check of the front-wheel and foot bottoms against the front-panel ground line (the zoom was not conclusive).
- K-1 and J-1 are readability issues of the front view in folded poses; a head or torso outline in the front view would fix both.
- J-3 is from the check output only; the 1:500 sheets were already read in QA4 and not repeated here.
