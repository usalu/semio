# Round 3 — objects/storage-crate-stack (2026-11-20)

Status: done, spliced, catalog VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts --product` PASS at 1:50, 1:100, 1:200, 1:500.

Scripts: `r3-storage-crate-stack-kit.py` (crate geometry stated by hand: outlines, notches, ledges, taper), `r3-storage-crate-stack-roots.py` (root fixes outside `volumes`), `r3-storage-crate-stack-build.py` (all volumes of all scales). The candidate document under `🗑️generated/r3-storage-crate-stack/` was removed after the catalog write. Sources: https://rotom.de/pdf/specification/13/Rotom_Produktspezifikation_69751.pdf, https://transoplast.de/produkt/stapel-nestbare-kiste-euronorm-600-x-400-x-h-300-mm-grau/pdf/.

## 1. Reference

The asset is a stack of five stack-and-nest crates of the 600 x 400 module, 300 mm high, closed walls, two hand holes. The real product class is the rotating stack-nest container (Drehstapelbehälter): turned by half a turn against the crate below it stands on stacking ledges; in the same orientation it nests into it.

| quantity | real value | source | asset before | asset after |
|---|---|---|---|---|
| outer size at the rim | 600 x 400 x 300 mm | ISO 3394 module; Rotom 69751 data sheet (outer 600 x 400 x 300, inner 490 x 332 x 295, PP, closed walls and floor, two open hand holes, 2.14 kg); Transoplast 9567.720.765 data sheet (outer 600 x 400 x 300, inner 457 x 335 x 271, HDPE, 2.16 kg, nesting ratio 72 %, 15 kg load, 200 kg static stack load) | 620 x 400 x 300 (rim 10 mm proud of 600 on the long axis only) | 600 x 400 x 300 |
| mass per crate | 2.14-2.16 kg (2.2-2.4 kg for bale-arm variants) | Rotom 69751, Transoplast 9567.720.765 | 1.9 kg (estimate) | 2.15 kg (literature) |
| nesting step | 300 mm x (1 - 0.72) = 84 mm | Transoplast nesting ratio 72 % | 80 mm | 80 mm kept (4 mm per step is below the 1:50 feature; the low and half heights 0.62 / 1.06 m stay) |
| wall | 2.5-3.5 mm injection-moulded HDPE/PP, straight draft of 2-5 degrees | moulding practice; inner/outer difference of the data sheets | tray of 4 mm with a 2 mm inner wall, taper 0.56 -> 0.61 m (uniform) | 3 mm, draft 4.4 degrees on the ends and 2.8 degrees on the sides (uniform taper of the kernel, factor 1.10 from foot to rim) |
| foot | moulded foot ring under the floor, set in from the wall | moulding practice | 20 mm ring 0.54 x 0.30 m, 8 mm wall | 14 mm ring 0.52 x 0.32 m, 4 mm wall, notched like the wall |
| rim | outward U-band: flange on top, skirt hanging outside the wall, open below | moulding practice | solid 20 mm plate ring with an opening of 0.284 x 0.524 m (the next crate could neither nest nor reach inside) | 38 mm skirt + 8 mm flange, opening 0.578 x 0.361 m |
| hand holes | one open hand hole in each end wall just under the rim, about 120 x 35 mm | Rotom 69751 ("zwei offene Handgriffe"), common practice | three dark decals 100 x 40 mm on each long wall at mid height ("vent slots") and one decal in each end at mid height | true openings 120 x 35 mm at 0.215-0.25 m in both end walls, the grip bar above them |
| stacking | turned by 180 degrees the foot of the upper crate lands on stacking ledges at the top of the lower crate; in the same orientation the ledges pass through recesses (stepped wall channels) of the upper crate and it nests | Drehstapelbehälter principle (Rotom, Transoplast: "um 180° drehbar") | foot ring said to sit "on the rim" while the drawn rim opening was smaller than the foot: stacking and nesting were geometrically impossible together, the rig had no turn | three ledges (two in the end walls, one in the far long wall) and three matching wall channels; every crate's own bone turns about the vertical (twist), a zero-length seat bone carries the nesting travel |
| goods load | 10 kg per crate (max 15 kg) | Transoplast data sheet | 10 kg | 10 kg |

## 2. Audit (round-2 state)

| item | found | real / required | verdict |
|---|---|---|---|
| rig | each crate bone along the bone-local +x of a horizontal frame (angle 0): the bone ran 0.30 m sideways from the base centre, tip in the air beside the crate; prismatic axis 90 degrees | the load path of a crate runs from the foot up to the rim; the bone up the crate | root defect, fixed (crate 1 at 90 degrees, all children along it) |
| rig, stacking | translation only; the crates could only slide into each other | a stack-nest crate stacks only when turned by half a turn against the crate below, nests in the same orientation | root defect: added a turn about the vertical per crate (revolute twist 0..180 degrees) and a zero-length seat bone that carries the nesting travel (-0.22 m) and a 20 mm lift |
| stacking geometry | foot ring 0.54 x 0.30 m said to stand on a rim whose solid plate reached in to 0.524 x 0.284 m: stacking possible, nesting impossible (the next crate could not pass the rim); wall tray 4 mm with a 2 mm inner "structure" wall | ledges at the top inside, matching channels in the wall, foot on the ledges when turned | rebuilt: three ledges (two ends, far long wall), channels 40/32 mm deep, nesting play 0.6-4 mm checked by hand for every height (kit numbers) |
| dimensions | widthM 0.62 (rim 10 mm proud of 600 on one axis only), massKg 9.5 | 0.60, 10.75 (5 x 2.15 kg, data sheets) | fixed |
| mass per bone | 1.9 kg at comY 0.15 (in the old horizontal frame) | 2.15 kg at 0.13 m (floor and foot low) | fixed |
| anchor footprint | 0.54 x 0.34 | foot ring 0.52 x 0.32 | fixed |
| 2D 1:50 | 3 vent-slot decals on the long walls (a closed-wall crate has none), a mid-height decal hand hole on the ends, rim 20 mm at 0.62 m, foot 0.54 m | hand holes under the rim in the ends, true openings; rim 38 mm at 0.60 m | redrawn (elevation, front with the hand hole as a hole of the end wall, plan with flange opening, floor, ribs, three ledges) |
| 2D 1:100-1:500 | rims 0.62 m, rectangles at 1:500 | 0.60 m rim, tapering tub with collar | redrawn at every scale; every shape of a turning crate is an extruded `solid` (the product's rule `elevation-lateral` for twisted bones) |
| body 1:50 | foot ring 8 mm wall, 12 mm floor plate, tray, plate rim; slots as texture decals | real moulding | rebuilt (13 solids per crate) |
| thermal | envelopes with the old taper (0.56 -> 0.61) | follow the new body | rebuilt on the new outline, same ruling |
| structure | `member-tray` 2 mm inside the 4 mm skin (a second wall), joint ring at the rim, one support pad | real load path | rebuilt: wall box section, floor, foot, rim beam, ledges, bearing pads under each upper crate, support, mass |
| 1:500 | five blocks 0.62 x 0.30 x 0.40 (solid, could not nest), thermal area total failing (0.85 > 0.3) | | rebuilt as tubs with collar, ledges and channels at 75 mm |

## 3. Root fixes (old -> new), all written by `r3-storage-crate-stack-roots.py`

- `provenance.references` + `rotom-69751`, `transoplast-9567720765`; `provenance.notes` and `summary` rewritten (construction, masses, nesting ratio, turning).
- `dimensions`: widthM 0.62 -> 0.60, massKg 9.5 -> 10.75 (heightM 1.5, depthM 0.4 kept).
- `rig`: crate-1 angleDeg 0 -> 90; crate-2..5 parent crate-(k-1) at (0, 0.3) prismatic -> parent seat-k at (0, 0), revolute 0/0 with twist 0..180; new seat-2..5 (parent crate-(k-1) at (0.3, 0), length 0, prismatic axis 0, -0.22..+0.02); masses 1.9 kg at (0, 0.15) -> 2.15 kg at (0.13, 0), source literature.
- `anchors.base-contact`: footprint 0.54 x 0.34 -> 0.52 x 0.32.
- `poses`: high: crate-2..5 twist 180 (each turned against the one below); half: crate-2, crate-3 twist 180, seat-4, seat-5 dx -0.22; low: seat-2..5 dx -0.22 (before: dy on crate bones); loads at (0.1, 0) of the vertical bone; descriptions rewritten.
- `clips`: unload-stack 4 -> 14 keys (lift 20 mm, turn, lower, crate by crate), load-stack the reverse; durations, phases, events kept; descriptions rewritten.
- `structure.assumptions` rewritten (load through the three ledges).
- `representations` 50/100/200/500: geometry redrawn as above, decisions rewritten. Thermal regions, areas, materials untouched.

## 4. Volumes (`r3-storage-crate-stack-build.py`; per crate, five crates)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 13: foot ring (notched, hollow), floor 8 mm, wall below the hand holes, two band halves leaving the hand holes open, wall above, rim skirt 38 mm, flange 8 mm, ledges a/b/c, floor ribs (texture). 65 solids | 3: wall envelope, rim edge, foot edge (sectionFaces -u/+u, ruling). 15 | wall box section, floor, foot, rim beam, 3 ledges, 3 bearing pads (crates 2-5), support ring (crate 1), mass. 53 |
| 1:100 | 10: floor 16 mm, wall in three heights with the hand holes, skirt 25 mm, flange 15 mm, 3 ledges; 4-segment corners. 50 | 2: wall envelope, rim edge. 10 | wall box section, floor, rim beam, ledges, 15 mm pads, floor support, mass. 48 |
| 1:200 | 9: floor 30 mm, wall in three heights with the hand holes, collar 30 mm, 3 ledges; 3-segment corners. 45 | 2. 10 | wall, floor, collar, ledges, 30 mm pads, support, mass. 48 |
| 1:500 | 6: floor 75 mm, one wall (2 mm, so that the 80 mm channels nest), collar 75 mm, 3 ledges of 75 mm; 2-segment corners. 30 | 2. 10 | wall, floor, collar, ledges, one 75 mm joint block per upper crate, support, mass. 40 |

Final check line: `volumes-check.ts objects/storage-crate-stack --product`: PASS (all 4 scales; silhouette 1:50 IoU 1.000 / 0.995 / 1.000, p95 0 / 4 / 0 mm; thermal area per crate 0.588 / 0.582 / 0.589 / 0.606 m2 against 0.6; monotonic strict at every step; cross-scale front 1:100 IoU 1.0). `r3-validate.ts objects/storage-crate-stack --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

## 5. What I saw (final renders)

- 1:50 body sheet (`🗑️generated/r3-sheet/objects/storage-crate-stack.s50.body.png`): front (from +x): five tapering end walls with the open hand holes under the rim, the channels of the turned crates alternating left and right; right side: tapering long walls, the 32 mm channel of every other crate in the middle (the turned ones show it on the far side); back: as front, mirrored; left side: channels on the alternate crates; top: rim band, flange opening, floor with the two ribs, the three ledges; three-quarter (also `--turn 30 --tilt 35` at full size): reads at once as a stack of turned stack-nest crates, each foot standing inside the rim of the one below on its ledges, inner pillars of the channels visible inside the top crate, hand holes see-through.
- 1:50 low pose, cut: the walls nest with even play, floors 80 mm apart, rim bands stacked, ledges passing through the channels; no part runs through another.
- 1:100 clip unload-stack t = 2 s: crate 4 lifted 20 mm and turned part way with crate 5 on it, clear of the ledges below.
- Thermal 1:50 (cut): five coloured envelopes following the wall with its channels, rim and foot edges, hatched section faces top and bottom.
- Structure 1:50: blue wall boxes, rims, ledges; orange pads under each upper crate on the ledges; magenta masses at 0.13 m; green support under the lowest foot.
- 1:200 and 1:500 sheets: overlays green everywhere (IoU 0.996-1.000), tubs with collars, ledges and channels still legible, hand holes open at 1:200.
- 2D (`r3-2d.ts`, 1:50 high and low, 1:100 half): stacked and nested outlines correct, heights 1.50 / 1.06 / 0.62 m, hand holes open in the front view.

## 6. Still imperfect (plainly)

- The kernel's taper scales a profile uniformly about its centroid, so the draft is 4.4 degrees on the ends and 2.8 degrees on the sides instead of one draft angle, and the two band halves around the hand holes carry a residual offset of up to 0.8 mm against the wall (seam visible as a grey line at the band heights).
- The structure is the crate's monocoque (wall, floor, foot, rim, ledges): for a moulded crate the shell is the load path, so the members look like the body; the pads, masses and support are the idealisation.
- Thermal stays the outer-wall envelope of the area ruling (0.6 m2); the honest outer area of the open moulding (about 1.6 m2 per crate) is listed in `area-review.md`, not applied.
- Nesting step kept at 0.08 m (data sheet ratio gives 0.084 m); the 1:500 wall is 2 mm instead of 3 mm so that the 75 mm ledges and 80 mm channels still nest.
- Underside floor ribs and moulded labels are not modelled (not visible in any drawn view).
- Plan corners of the extruded `solid` shapes would be square; the plan rims are therefore drawn as plain rounded rectangles.

## Round 5 handcraft (2026-11-21)

Renders under `🗑️generated/r5-storage-crate-stack/before/` (every scale body, thermal cut, structure, 2D; poses high, half, low; clip unload-stack 2 s), all opened. No change was needed.

### Model against reference (default pose `high`, `r5-o02-measure.ts`)

| quantity | reference (Rotom 69751, Transoplast 9567.720.765) | model 1:50 / 1:100 / 1:200 / 1:500 | deviation |
|---|---|---|---|
| crate outer size at the rim | 600 x 400 x 300 mm | 600 x 400, 5 crates to 1.500 m at every scale | 0 % |
| stack height (5 turned crates) | 5 x 300 = 1.50 m | 1.500 m | 0 % |
| nesting step | 84 mm (72 % ratio) | 80 mm (low pose 0.62 m) | -4.8 % (kept, round 3: 4 mm is below the 1:50 feature; within the data-sheet spread of the class) |
| hand holes | 120 x 35 mm under the rim in the end walls | 120 x 35 mm, open | 0 % |
| wall | 2.5 to 3.5 mm with draft | 3 mm, draft 4.4 / 2.8 degrees | inside the range |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | unchanged |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | A- / A- / A- | unchanged |
| 1:500 | A- / A- / A- (tubs with collar, ledges and channels, no hand holes) | unchanged |

Poses: high (turned crates on the ledges), half and low (nested, even play) and the unload clip (crate 4 lifted 20 mm and turning with crate 5) are closed and natural. Catalog VALID with volumes (schema 0, validateAsset 0, checkVolumes 0).

### Remaining

As section 6 (uniform taper of the kernel gives two draft angles, nesting step 80 instead of 84 mm, structure is the moulded shell by necessity, thermal envelope area under review in `area-review.md`). The cut thermal sheets look jagged in the three-quarter panel because the cut plane runs through the channel pillars of turned crates; the solids are clean.

### Moulded sections and load transfer, 21 November

All five crates retain an 8 mm floor 12 mm above the foot, a 14 mm foot ring, 3 mm tapered side walls, a 38 mm rim skirt, an 8 mm flange and three 68 mm bearing ledges. The twelve upper-crate joints carry the load at the feet. The earlier coarse model had put the load on four blocks in the middle of the empty crates. The longitudinal section now cuts through the walls and floor in both the stacked and nested positions.

A Three.js fixture measured all twenty floor, foot and rim sections and sixty ledges. The structural model references the moulded foot on every crate at all scales. All twelve GLB scenes reloaded at 1,377,440 bytes total. Each of the four scales passed its measured volume checks after the two coarser elevation drawings recovered their foot contours. The exposed thermal region still represents the documented outer wall exchange. An audit of exchange with air inside open crates is recorded in area-review.md.

## Intermediate paper scales, 21 November

The fine drawing's four upper feet left visible features of 0.183732 mm at 1:66 where the office-laser minimum is 0.20 mm. The elevation now gives all five foot rings a 0.20 mm contour in the 1:50 drawing. Their modelled 14 mm height, 520 × 320 mm bearing footprint, attachment and positions remain as measured. A fresh check at 1:66 passed. Thirteen evenly spaced denominators in each of four authored ranges also passed, for 52 complete measurements over the high, half and low poses in elevation, front and plan. The complete volume data carried the same SHA-256 before and after the drawing edit, `d32688ecbf52f2dcc578140c4f3adc34f924597660cdc0fa5d488803dc29ef61`. The separate Three.js reader measured the twenty floor, foot and rim sections and sixty stacking ledges. `crate-paper.py` records one revision-tagged result per drawing and denominator.
