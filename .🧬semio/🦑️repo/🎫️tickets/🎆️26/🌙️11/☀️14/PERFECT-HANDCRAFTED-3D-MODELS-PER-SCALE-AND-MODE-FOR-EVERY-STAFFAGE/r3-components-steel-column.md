# Round 3 — components/steel-column

State: DONE

Scripts: `r3-steel-column-volumes.py` (every solid stated by hand, writes `volumes/components/steel-column.json`), `r3-steel-column-roots.py` (2D root fixes, replaces only the `representations` member of the catalog file in its own text style), `r3-steel-column-detail.ts` (close-up renders with the shading of `r3-sheet.ts`; the whole-height sheets show a 3 m column at about 4 px per 10 mm, too small to judge welds and anchors). No earlier round-3 work existed; the catalog `volumes` were identical to the source at the start (the 03:40 bulk passes did not touch this asset).

## 1. Reference

| item | value | source |
|---|---|---|
| HEB 200 | h 200, b 200, tw 9, tf 15, root radius r 18 mm; A 78.1 cm2, Iy 5696 cm4, 61.3 kg/m, 1.15 m2/m | EN 10365 (asset reference `en-10365`), section tables |
| check of the modelled outline | 2 x 200 x 15 + 170 x 9 + 4 x 18^2 (1 - pi/4) = 78.08 cm2 | own calculation |
| shaft | 2.96 m between two 20 mm plates, overall 3.00 m | asset `dimensions`, 2D 1:50 |
| base plate | 300 x 260 x 20 mm, flame-cut square corners | asset `symbols` dimensional note, 2D plan |
| head plate | 240 x 220 x 20 mm, four 18 mm holes for the bolts of the beam above (gauge 120, pitch 100, in the chambers so the nuts can be reached) | asset 2D; holes: usual practice, EN 1090-2 clearance 2 mm for M16 |
| anchors | 4 x M16 at 250 x 200 mm, hole 18 mm, washer 30 x 3 (ISO 7089), nut 24 across flats, 27.7 across corners, 14.8 high (EN ISO 4032), rod end 37 mm above the nut | drawn nut 24 x 18 mm = M16 nut on washer; the old 10 mm rod with a 24 mm nut matched no thread |
| gussets | 16 mm plate in the web plane outside each flange, 50 mm wide, 90 mm high, nose 25 mm, snipe 15 mm clearing the flange weld | asset 2D 1:50 (size), workshop practice (nose, snipe) |
| welds | fillet weld a = 6 mm all round (legs 8.5 mm) shaft to both plates; gussets a = 4 mm (5.7 mm, under the 7.5 mm feature) | EN 1993-1-8 (a <= 0.7 t), practice |
| mass | 61.3 kg/m x 3.0 = 183.9 kg (asset value) | EN 10365 |

## 2. Audit (before)

- Rig: `column` 3.0 m from the footing, `eccentric-carrier` sliding across the head: right. Elevation looks onto the web, bending about the strong axis: consistent with Iy. No rig change.
- `dimensions` 3.0 x 0.3 x 0.26 m: right. `massKg` 183.93 is the shaft formula over the full 3.0 m; the real assembly (2.96 m shaft, plates, gussets) is about 203 kg. Not changed (physical datum with a cited source; see section 7).
- Baseline `volumes-check --product` FAILED at 1:50 and 1:100: `base-plate` area 0.213 / 0.232 against 0.12 (the underside on the footing was counted).
- Body 1:50: shaft as three solids (seams where a rolled section has none), plates with 15 mm clipped corners the plan does not draw and real plates do not have, 10 mm rods under 24 mm nuts, no washers, no welds, no head plate holes, plain trapezoid gussets.
- 2D: a "splice" line at 1.5 m at 1:50 and 1:100 (a 3 m, 184 kg column is one piece; a splice at mid height is where no engineer puts one), and the structure list carried invented 300 x 120 x 8 web cover plates for it. Gussets 90 mm high at 1:50 but 165 mm at 1:100. Fillet lines at 65.5 mm instead of the tangent at 67 mm and only above 0.14 m. 1:500 drew the plates 20 mm thick (0.04 mm on paper) while the model needs 75 mm.
- Structure: the load pad was bound to `column`, so it stayed on the axis in the default pose `eccentric-load`; mass point at 1.45 m instead of the centre of mass at 1.5 m; support hidden under the plate (qa-visual).
- Thermal 1:50 to 1:200: flange region and web region as two full copies of each flange in the same place.

## 3. Root fixes (catalog `representations` only)

| scale / view | shape | old | new |
|---|---|---|---|
| 50 elevation, front | `splice-line` | line at 1.5 m | removed |
| 50 elevation | `fillet-a/b` | y +-0.0655, x 0.14..2.98 | y +-0.067, x 0.02..2.98 |
| 50 elevation | `stiffener-a/b` | trapezoid (0.02,0.1)(0.02,0.15)(0.11,0.13)(0.11,0.1) | (0.02,0.115)(0.02,0.15)(0.045,0.15)(0.11,0.11)(0.11,0.1)(0.035,0.1): nose, slope, snipe |
| 50 front | `anchor-nut-a/b` | 24 mm wide | 27.7 mm (across corners), centred on +-0.1 |
| 50 plan | `anchor-nut-*` / `anchor-bolt-*` | r 12 / r 5 mm | r 15 (washer) / r 8 (M16) |
| 50 plan | `cap-hole-00..11` | none | four 18 mm circles at (+-0.05, +-0.06) |
| 100 elevation, front | `splice-joint` | line at 1.5 m | removed |
| 100 elevation | `stiffener-left/right` | up to 0.2 m, 30 mm top | (0.035,0.1)(0.035,0.15)(0.055,0.15)(0.11,0.115)(0.11,0.1) |
| 100 front | `stiffener-edge` | h 0.165 | h 0.075 (top at 0.11) |
| 100 elevation, front, plan | `anchor-a/b`, `anchor-00..11` | none | studs 24 x 40 mm (0.24 x 0.4 mm), r 12 mm dots under the head plate in plan |
| 500 elevation, front | `column` | plate steps 20 mm | plate steps 75 mm |
| 50, 100, 500 | `decisions` silhouette / retained / omitted / thinElements | | rewritten for the above, en and de |

Not changed: rig, anchors, poses, clips, dimensions, thermal data (`areaM2` untouched: all four regions pass at their declared values), structure data, symbols. Candidate validated with `r3-validate.ts --file` before writing.

## 4. Model per scale (body / thermal / structure solids)

- **1:50 (41 / 44 / 43).** Body: one rolled H extrusion with r 18 fillets (6 chords each); base plate with four 18 mm holes; two gussets with nose and snipe; per anchor an M16 rod with chamfered end and its 3 mm washer as a collar (one turned solid: the validator refuses a separate 3 mm solid) under a hexagon nut with bore; head plate with four holes; fillet welds at both ends as ten wedge runs (flange faces, tips, inner faces, two chords round each root fillet, web) plus four quarter cones round the flange corners, each with its half-turn twin by `repeat`. Thermal: flange plates 13 mm (flange region) and two chamber halves (half web with fillets plus a 2 mm skin on the inner flange face, web region) meeting on the axis; welds assigned to the shaft region they lie on; base plate with its underside as mounted face (`sectionFaces -u`, C25 b), gussets and anchors, head plate. Structure: shaft member, base plate as the fixed support (green, visible), welds, gussets, anchors and head plate as joints, the load pad on `eccentric-carrier` (follows pose and clip), mass at 1.5 m.
- **1:100 (9 / 12 / 11).** H with real 15 / 9 mm and one 45 degree chamfer per root; drawn 35 mm plates without holes; 20 mm gussets to the true 0.11 m; four 24 x 40 mm studs (qa-visual asked for all four); no welds. Thermal as at 1:50 with the skin 0.2 mm proud of the full 15 mm flange plate (a 13 mm plate is refused at this scale).
- **1:200 (3 / 5 / 5).** Plain H 30 / 30 mm, 50 mm plates with square corners. Thermal: the three plates of the H plus the two plates. Structure: member, support, head plate joint, load pad, mass.
- **1:500 (3 / 5 / 4).** H at the 75 mm feature (chambers 50 x 62.5 mm), 75 mm plates. Thermal: two flange plates and the web running 12.5 mm into the flanges (the validator refuses any thermal solid under 75 mm, so a 50 / 50 mm H that I tried first could not be split by region). A decision starting "Minimal:" declares body and thermal as few as at 1:200.

Final check: `volumes-check.ts components/steel-column --product` PASS, 41 rule lines ok. Silhouette IoU 0.9993 to 1.0 in all views and scales; area deviation 1:50 flange 1.5 %, web 1.6 %, base plate 9.9 %, head plate 2.0 % (limit 15 %); 1:100 2.3 / 5.4 / 21.5 / 11.1 % (limit 30 %); totals 1:200 0.3 %, 1:500 3.9 %. `r3-validate.ts components/steel-column --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0. Spliced with `volumes-insert.py`.

## 5. What I saw (all rendered after the last change and opened)

- `r3-sheet` 1:50 body: slender H column, flange face in front and back, web with fillet shading from both sides, base plate with gussets left and right in the side views, plan with head plate holes and washers at the corners; rig on the axis, carrier joint over the flange.
- Close-up base 1:50 (`detail.s50.body.base.png`): weld bead all round, bending round the root fillet and wrapped round the flange corners, gusset with nose standing over the continuous flange weld, washers, hexagon nuts, chamfered rod ends. First version had pyramid lumps at the flange corners (two wedges crossing): replaced by quarter cones. Close-up head from below: weld under the plate, holes in the chamber, clear of web and flange.
- 1:50 thermal: flanges red, chambers green including the inner flange faces, base group violet, head plate ochre; the top cut shows flange plate, skin, web halves and the weld runs in their regions, no shell outside the body. 1:50 structure: blue shaft, orange welds, gussets and anchors on the green plate, pad at the flange in `eccentric-load` and on the axis in `axial-self-weight`, mass on the web at mid height.
- 1:100 body / thermal / structure: chamfered roots, studs, squat gussets; inner flange faces green without flicker; pad inside the head plate edge after I reduced it from r 20 to r 18 mm.
- 1:200 and 1:500: clean H between two plates, three thermal plates, structure with green base. At 1:500 the mass point is mostly inside the web (r 80 mm in a 75 mm web) and shows as a small mark in the chamber.
- `volumes-render` overlays 1:50 to 1:500: all green, no red or blue. `r3-2d` 1:50, 1:100, 1:500 and the pose `axial-self-weight`: drawings read as before, with the new details.

## 6. Requests

None filed. Observation for the validator maintainer: `validateAsset` judges the thinness of own thermal solids, so a region surface thinner than the feature can only be given through a larger solid (collar, skin on a half web) or a reference.

## 7. Still imperfect

- `dimensions.massKg` and the bone mass (183.93 kg) do not include plates, gussets and anchors (about 203 kg real). Left as is: the value is cited physical data and changes the reactions of every pose.
- The anchors sit 25 mm from the flange face (the drawn layout); a socket does not fit there, a ring spanner does. Layout kept.
- The weld is sharp wedges; the 3 mm washer step and the nut chamfer are not in the 2D (0.06 mm on paper).
- 1:500 mass point is nearly hidden in the web.

## Round 5 handcraft

Agent C03, 2026-11-21. Scripts: `r5-steel-column-roots.py` (assembly mass, `--apply` writes only the two mass values and the centre of mass), `r5-steel-column-volumes.py` (round-3 solids with the corrected mass point), `r5_c03_lib.py`, `r5-c03-sheets.sh`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| HEB 200 h / b / t_w / t_f / r | 200 / 200 / 9 / 15 / 18 mm | EN 10365 |
| A, mass per metre | 78.1 cm², 61.3 kg/m | EN 10365 |
| base plate, head plate | 300 x 260 x 20, 240 x 220 x 20 mm | asset drawing, practice |
| anchors | M16, nut AF 24 x 14.8, washer 30 x 3 | ISO 4032, ISO 7089 |
| gussets | 16 mm, 50 x 90 mm | asset drawing |
| assembly mass | 181.43 shaft + 12.09 base + 8.13 head + 0.78 gussets + 0.53 anchor heads + 0.65 welds = 203.60 kg, centre at 1.462 m | own calculation (`r5-steel-column-roots.py`) |

### Model against reference (1:50)

| quantity | model | reference | deviation |
|---|---|---|---|
| section h / b / t_f / r | 200 / 200 / 15 / 18 mm | same | 0 % |
| web | 9 mm (two 4.5 mm chamber halves) | 9 mm | 0 % |
| shaft length, overall height | 2.96 m, 3.00 m | 2.96 m, 3.00 m | 0 % |
| plates | 300 x 260 x 20, 240 x 220 x 20 | same | 0 % |
| rig / `dimensions.massKg` | before 183.93 kg (bare shaft over 3.0 m), now 203.60 kg | 203.60 kg | before -9.7 %, now 0 % |
| centre of mass | before 1.5 m, now 1.462 m | 1.462 m | before +2.6 %, now 0 % |
| 1:100 plate thickness | 35 mm | 20 mm | +75 %: print floor of the drawing (0.2 mm minimum feature across the 1:75 to 1:150 band), 3D follows the drawing |
| 1:200 / 1:500 flanges, plates | 30 / 50 mm, 75 / 75 mm | 15 / 20 mm | feature floor of the scale |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- (weld corner returns read as small pyramids; rig mass 9.7 % light) | A- / A- / A (mass true) |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | B+ / B+ / B+ | B+ / B+ / B+ |
| 1:500 | B+ / B+ / B (mass point hidden in the 75 mm web) | B+ / B+ / B+ (mass sphere 95 mm, shows in both chambers) |

Poses `axial-self-weight`, `eccentric-load`, `lateral-wind` and clip `eccentric-load-sweep` t 2 at 1:50 opened: rigid column, the load pad follows the carrier; nothing torn.

Opened: `r3-sheet` body, structure and cut thermal at every scale, the three poses and the clip frame, `r3-2d` at 1:50 to 1:500 (`🗑️generated/r5-steel-column/before`), close-ups `detail.s50.body.r5base`, `detail.s50.body.r5head`, `detail.s50.structure.r5base`, `detail.s50.thermal.r5base`, `detail.s100/200/500.body.r5base`.

Changes: rig `column.mass.kg` 183.93 -> 203.60, `comX` 1.5 -> 1.4618, `dimensions.massKg` 183.93 -> 203.60; mass solid at 1.4618 m at every scale, 1:500 mass radius 0.08 -> 0.095 m; structure decisions name the assembly mass.

Check: `r3-validate.ts components/steel-column --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts --product` PASS; spliced.

Still imperfect: the welds are sharp wedges and the flange-tip weld returns read as small pyramids at close range; the 1:100 to 1:500 plates are thickened to the print and feature floors (stated in the decisions); the 1:500 shaft is an H at the 75 mm floor (chambers 50 mm), the smallest abstraction that still reads as a rolled section.
