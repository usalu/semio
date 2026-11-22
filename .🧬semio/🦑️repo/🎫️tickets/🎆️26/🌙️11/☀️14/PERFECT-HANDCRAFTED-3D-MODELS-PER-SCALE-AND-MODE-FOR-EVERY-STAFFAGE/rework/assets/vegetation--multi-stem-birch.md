# `vegetation/multi-stem-birch` — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. The asset was found invalid (two measured findings and one feature finding); it is one connected object at every scale now and the 1:500 plan drawing is redrawn. The 1:50 body, the rig, the structure and thermal lists were looked at and judged, not rebuilt. What is still not right is listed at the end.

### Starting point

- No report of this asset existed in `rework/assets/`. `rework/12.md` only has its reference row (batch 12 did not reach it; its script `rework-12-multi-stem-birch-model.py` was never applied: the catalog still has the 22 bones of round 3, not the `br-l-1 … br-r-8` rig that script would write).
- Source `volumes/vegetation/multi-stem-birch.json` and catalog `asset.volumes` were identical at all four scales (compared member by member). Nothing half-written.
- The state is the round-3 / round-5 model (`r3-vegetation-multi-stem-birch.md`, scripts `r3-multi-stem-birch-*.ts`, `r5-multi-stem-birch-*.ts`).
- First full run `r3-validate.ts vegetation/multi-stem-birch --volumes --no-print`: `INVALID`, `volumes-feature` 1:500 (`tip-c-b` 37.4 mm thin, limit 42.1 mm), `volumes-connected` 1:100 (one group, `leaf-l-b-v1` 7.7 mm from `leaf-l-b-s`), `volumes-connected` 1:500 (126 groups floating, up to 334 mm from the next solid).

### Reference (*Betula pendula*, three-stemmed clump, about 25 to 30 years), written before judging

| quantity | real subject | model (measured) | verdict |
|---|---|---|---|
| height | 12 to 15 m at this age | 13.02 m | right |
| crown spread | 6 to 9 m, wider along the row of the outer stems | 9.2 m by 6.9 m | right, upper end |
| stems | three from one stool, 14 to 20 cm at breast height, 20 to 27 cm at the base, about 6 cm at 10 m, gently sinuous, splayed 8 to 16 degrees | radius 104 / 120 / 112 mm at the stool, 82 / 94 / 88 mm at 1.3 m, 33 mm at the stem top; lean 14, 1 and -16 degrees in the elevation, tripod in plan | right |
| stool | 0.6 to 0.75 m across, shallow surface roots | 0.74 by 0.70 m, five roots at 1:50 | right |
| first-order branches | many, thin (2 to 5 cm), 1.5 to 3 m, leaving at 40 to 60 degrees and arching over | 6 on bones (radius 36 to 19 mm) and 40 without bones (19 to 6 mm), 0.65 to 2.85 m | right in number and size |
| second and third order | thin pendulous whips of 1 to 2 m, hanging plumb, in loose curtains; the crown is a veil with the stems visible through it | leafy tresses as closed spindles 0.25 to 0.45 m thick and 0.6 to 1.4 m long, four to six per bough | **too heavy**: reads as thick paddles, see open list |
| leaf | rhombic, 3 to 7 cm, on long stalks | not modelled singly (below every feature size) | acceptable |
| bark | white with dark lenticel bands, black fissured base to 0.5 to 1 m, black scar under each branch | dark thicker base, bands, fissures, scars at 1:50 | right |
| rig | stems bend little (2 to 4 degrees per segment in a gale), branches 8 degrees, foliage ends 14 degrees; pivots at the stool, at mid stem, at each branch base and where the leafy end hangs | 22 bones: stool, 2 per stem, 6 branches, 6 branch ends, 3 stem tops; revolute limits 2.5 / 3.5 / 8 / 14 degrees; lateral `z` places the tripod | right topology for a tree; only 6 of 46 branches are boned (the other 40 ride on the stem bones), accepted |

### Defects found (by rendering all four scales, and by the measured rules)

1. 1:500 body: the round-3 list kept the tresses of the side shoots but left out the side-shoot sleeves and every branch, so 126 tress spindles and then all 27 boughs hung in the air (visible in the top view as loose blobs round the crown).
2. 1:500 plan drawing: drew the floating blobs.
3. 1:500 `tip-c-b`: a branch end thinner than its drawing.
4. 1:100: side-shoot sleeve `leaf-l-b-v1` did not reach its bough sleeve.
5. All coarse scales: the lists are the 1:50 parts list thinned (600 / 542 / 405 solids), not masses modelled for 15 / 30 / 75 mm. Not solved in this pass.
6. 1:50 crown: tresses too thick and too closed for a birch (reference row above). Not solved in this pass.

### What was rebuilt in this pass

Script `rework-multi-stem-birch-coarse.py` (hand-chosen numbers at the top; every step skips what is done):

- **1:500 body, side-shoot sleeves**: 60 sleeves `leaf-<bough>-v<k>` restated from the 1:200 sections for this scale: five sections instead of six (the two middle ones merged), the three inner sections grown about their centres to at least 0.10 m across (feature 75 mm). Each carries its two tresses and meets its bough sleeve.
- **1:500 body, branches**: the six boned branches `br-*-a/b` and the forty unboned `branch-*` are in the body at their true section with `slender: true` (20 to 36 mm radius over 0.65 to 2.85 m), so every bough is carried by wood that reaches a stem.
- **1:500 `tip-c-b`**: thickened about its own centre line to 56 mm across its widest section (factor 1.09, then 1.12).
- **1:500 thermal**: `leaf-crown` references for the 60 sleeves; `branch-bark` references of the added branches as at 1:200.
- **1:100 body**: the starting section of `leaf-l-b-v1` carried 50 mm into `leaf-l-b-s`.

Script `rework-multi-stem-birch-plan500.py`:

- **1:500 plan drawing** (`representations.500.geometry.plan`): the 21 crown shapes redrawn as the true plan view of the reworked body (outlines measured with `r3-project2d.ts`, 37 shapes; stems and branch-bark shapes kept). Written into the catalog as a text splice of that one array, every other byte of the file unchanged (asserted). Elevation and front still agree with the body and were kept (`r3-project2d --check`: 0.978 / 0.979).
- **1:500 body decision text** (en, de) rewritten: the branches are in the body now.

Measured after the edit (fast loop): 1:500 `connected, feature, slender, solids, cross-scale, copied, thermal-coverage, bounds, monotonic, silhouette, views-disagree` no finding; 1:100 `connected, feature` no finding.

VALIDATION_LINE

LOOKED_LINE

### Open list for the next pass (precise)

OPEN_LIST

### Fundamental issues (outside this asset)

FUNDAMENTAL

## Pass 1 (second round), continued 15:20–17:10 without a report; proven by the umbrella finisher 22:35–22:45

After the report above the author went on (scripts `rework-multi-stem-birch-veil.py` 16:19, `…-veil-join.py` 16:17, `…-2d.py` 16:35, `…-2d-gaps.py` 17:01; catalog written 17:05, last own validation `validate-12.txt` 17:09) and was cut off before reporting:

- **Crown as a veil**: every leafy sleeve and hanging tress restated about its own hand-placed centre line with the section of a Betula pendula spray (sleeve about 0.3 m round its bough, pendulous strand 0.1 to 0.17 m, fullest in its lower third); strands thicken with the scale (0.16 m at 1:50 and 1:100, 0.2 m at 1:200, 0.23 m at 1:500).
- **Joins**: side-shoot sleeves start on the centre line of the sleeve that bears them; four tightly curved top sleeves restated with sections square to a smoothed centre line; foliage mass markers on the centre line of their sleeve.
- **2D**: crown shapes of every view redrawn over the veil crown (outlines measured with `r3-project2d.ts`), old paddles and their leaf strokes removed; pieces closer than the printable gap drawn touching; the three stems of 1:500 a little stouter.

Proof by the finisher: `r3-validate.ts vegetation/multi-stem-birch --catalog-only --volumes` → `VALID vegetation/multi-stem-birch (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ multi-stem-birch (1/1)`, four scales suitable. Source = catalog.

Sheet read at full size: 1:50 body six views (715 solids). Verdict: three leaning white stems from one stool, rising boughs, a veil of pendulous shoots; reads as a weeping birch from every side, nothing floats. Rendered, not read: 2D 1:100. Not rendered by the finisher: 1:100 to 1:500, thermal, structure, poses, clips.

Remains imperfect: the tresses are thick strands (0.16 m at 1:50), a real birch veil is finer; the asset file is 17 MB, the catalog validation alone took about ten minutes on the loaded host.
