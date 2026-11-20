# Round 3 — `humans/parent-carrying-toddler`

State (2026-11-20): root fixes, redrawn 2D of all four scales and `volumes` are in the catalog asset. `r3-validate.ts humans/parent-carrying-toddler` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/parent-carrying-toddler --product`: every rule `ok` at every scale (availability, silhouette, views-disagree, bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied) except `volumes-thermal-coverage` at 1:50 and 1:100: `FAIL: 12 measurement(s) over 2 rule line(s)`. All twelve are body-share areas (section 5), filed in `author-requests.md`.

No earlier round-3 author had started this asset (no `r3-<slug>-*` files, no report); started from the catalog as found (VALID, old volumes).

Scripts (ticket folder, prefix `r3-parent-carrying-toddler-`): `roots.py` (rig, anchors, poses, clip texts and advance, summary, provenance, structure assumption, dimensions, 2D shapes and decisions into a candidate; `--commit`), `anchors.json`, `references.json`, `dimensions.json`, `texts.json` (every bilingual decision), `kit.ts` (station helpers), `io.ts`, `s50.ts` (1:50 body, every section stated), `lists50.ts` (1:50 thermal and both skeletons; region map and emitters shared with the coarse scales), `coarse.ts` (1:100, 1:200, 1:500, each stated anew), `build.ts`, `draw2d.ts` (ruling E1), `run.sh`, `commit.sh`, `check.ts` (product rules on the candidate), `fk.ts`, `look.ts` (multi-direction renders of the candidate).

## 1. Reference

| quantity | reference | asset now | source |
|---|---|---|---|
| parent stature, mass | 1.70 m barefoot, 66 kg; boots lift 22 mm, beanie 10 mm | crown of the beanie 1.730 m | asset `thermal.body`, `dimensions` |
| hip / knee / ankle joint | 0.52 H / 0.285 H / 0.039 H = 0.884 / 0.485 / 0.066 m, + 22 mm shod | 0.905 / 0.505 / 0.088 m | brief; Winter 2009 tab. 4.1 |
| thigh / shank | 0.235 H / 0.246 H | 0.400 / 0.418 m | Winter 2009 |
| shoulder joint | 0.818 H + 22 mm = 1.413 m; joints 2 to 3 cm inside the acromia | 1.414 m, ±0.175 m | brief; ANSUR II (Gordon 2014) |
| C7 | 0.86 H + 22 mm = 1.484 m, 4 cm behind the hip line | 1.483 m | de Leva 1996 |
| L5/S1 | about 0.1 m above and 4 cm behind the hip joints | (−0.04, 1.010) | anatomy |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.316 / 0.248 / 0.184 m | same | Winter 2009 |
| head height / length / breadth | H/7.5 = 0.227 / 0.19–0.20 / 0.15 m | 0.225 / 0.199 / 0.144 m (skin) | brief, ANSUR II |
| chest, waist, seat (clothed: sweater + coat + ease) | body 0.30 x 0.23, 0.27 x 0.20, 0.35 x 0.23 m; +20 to 35 mm per side | coat 0.36 x 0.28, 0.34 x 0.26, 0.39 x 0.29 m | DIN 33402-2, EN 13402-3 |
| coat | wool, double-breasted, hem 8 cm above the knee | hem 0.578 m, two rows of three buttons, flap pockets at 0.86 m, stand collar | product knowledge |
| jeans | thigh 0.57 m, knee 0.37 m, calf 0.35 m round + ease, straight leg over the boot | thigh 0.18 x 0.19 m, knee 0.112 x 0.12 m, hem 0.104 x 0.124 m | DIN 33402-2 |
| ankle boot EU 41 | 0.28 x 0.10 m, shaft 0.15 m, heel 25 mm, sole 12 mm | 0.284 x 0.106 m, shaft top 0.152 m, heel 25 mm, sole 12 mm | product knowledge |
| toddler, 24 months | 0.865 m, 12.2 kg; head circumference 0.48 m, head height 0.17–0.18 m (about H/5); sitting height 0.52 m; biacromial 0.20 m; leg 0.39 m to the hip joint; foot 0.13 m | head 0.184 m (with chin to vertex), length 0.165, breadth 0.135 m; seat to crown 0.53 m; shoulder joints ±0.09; thigh 0.18, shank 0.17, ankle 0.045 m; boot 0.15 m | WHO 2006, Snyder 1977 |
| toddler limbs (padded overall, 12 mm fill + 8 mm air) | thigh 0.27 m, calf 0.20 m, upper arm 0.165 m round + 20 mm per side | suit thigh 0.123 x 0.128 m, upper arm 0.087 m across | Snyder 1977; asset layers |
| one-arm seated carry | the toddler sits on the forearm, elbow at the waist, its back against the carrier's chest; carrier leans back about 3 degrees | elbow 1.098 m, forearm 3 degrees up, toddler's seat on the sleeve at 1.152 m | own knowledge |

No web lookup was made; values are from the asset's references, the added references (`gordon-2014`, `din-33402-2`, `snyder-1977`, `en-13402-3`) and my knowledge.

## 2. Audit (state found)

- Rig: torso straight up from the hip joint, C7 at 1.433 m (5 cm low, 29 mm above the shoulder joint); head bone 0.261 m; upper arm 0.2995 / forearm 0.2608 (forearm 5 % long, upper arm 5 % short against Winter); knee 0.481 m and ankle 0.077 m shod (2.5 and 1.1 cm low); the toddler was a child of the parent's pelvis, placed in front of the chest at z 0.06 with thighs of 0.13 m and shanks of 0.14 m (the legs of a 9-month baby) and single-bone arms; the near/far naming of the toddler's limbs and the texts ("near hip", "near arm cradles") contradicted the geometry (left arm carrying, far side).
- Poses: the carrying arm hung forward 15 degrees with the hand at 0.80 m, nowhere near the toddler's seat at 1.0 m (the toddler floated in front of the parent, qa-visual-3: "toddler floating in front"); hoisting moved the pelvis, not the arm.
- Anchors: head emitters 3 to 6 cm off the face, `toddler-seat` at the toddler's hip joint in the air.
- Dimensions: `depthM` 0.45 against a real lateral extent of 0.607 m of the carry.
- 2D: old study shapes and `fit-*` patches (64 extra polygons), capsule plans, toddler ellipses; not views of one body.
- Solids (1:50 32 / 67 / 40): box torso, long tube coat, tube limbs, a toddler of capsules; qa-visual-3 grades B- to C+, structure at 1:500 one block.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `pelvis.lengthM` | 0.0966 | 0.105 (hip joints to L5/S1) |
| `torso` y / lengthM / comX | 0 / 0.4444 / 0.2029 | 0.04 (L5/S1 behind the hips) / 0.4735 / 0.216 |
| `head` angleDeg / lengthM / comX | 0 / 0.2608 / 0.1304 | −6 / 0.2388 / 0.119 (C7 to vertex) |
| `upper-arm-*` x, y, z / lengthM | 0.4155, 0, ±0.1737 / 0.2995 | 0.402, −0.04, ±0.175 / 0.316 |
| `forearm-*` lengthM / z | 0.2608 / — | 0.248 / +0.04 left (hanging arm clears the coat), −0.03 right (carrying elbow at the side) |
| `hand-*` lengthM / z / comX | 0.1739 / — / 0.087 | 0.184 / +0.03 left, +0.05 right (the carrying hand runs inwards under the toddler) / 0.069 |
| `thigh-*` / `shank-*` lengthM | 0.4153 / 0.4056 | 0.40 / 0.418 |
| `foot-*` lengthM, mass com | 0.2125, (0.1062, −0.042) | 0.21, (0.06, −0.05) |
| `toe-*` x, y, lengthM | 0.1255, −0.0772, 0.0889 | 0.13, −0.088, 0.08 |
| toddler | `toddler-torso` on `pelvis`; `toddler-arm-n/f`, `toddler-thigh-n/f`, `toddler-shank-n/f` | `toddler-torso` on `forearm-r` (z +0.035, hip joints at (0.18, 1.20, −0.17), 0.28 m to C7), `toddler-head` 0.205, `toddler-arm-l/r` 0.135 + `toddler-forearm-l/r` 0.11 (new), `toddler-thigh-l/r` 0.18, `toddler-shank-l/r` 0.17, `toddler-foot-l/r` 0.115 (new); masses split (arm 0.32 + 0.29, shank 0.66 + foot 0.194, sums kept) |
| default pose `standing-carry` | arm hanging, toddler on the pelvis | torso +3 (lean back), right upper arm −5, forearm 95, hand 0; feet flat, stance re-stated |
| walk poses (8) and `hoisting` | old lengths | legs re-solved by two-link IK with the old sole points and foot angles; carrying arm still; far (left) arm swings −14 cos(2πt); toddler bob and leg swing kept on the new bones; hoisting lifts the forearm 17 degrees with the toddler compensated |
| clip `walk-carrying` advanceM | 0.9496 | 0.9466 (measured planted travel) |
| anchors | heads off the faces, seat in the air | `mouth`, `nostrils`, `crown` and the toddler's three on the modelled lips, nostrils and cap crowns; `toddler-seat` on the sleeve under its seat; `grip-r` on the palm; heels and balls on the new sole |
| `dimensions.depthM` | 0.45 | 0.607 (default-pose body; height 1.73 and width 0.607 confirmed) |
| `summary`, `provenance.notes`, `structure.assumptions`, pose and clip descriptions | hip carry on the left/far side | one-arm seated carry on the near forearm, four authored scales |
| `materials` | 8 | + `bone` |
| `representations[*].geometry` | 127 / 104 / 86 / 79 shapes | 174 / 137 / 112 / 105, all redrawn |
| `representations[*].decisions` | old | all eight keys rewritten per scale |

Carry chosen: the rig is planar (every bone rotates about the lateral axis), so a straddle hip carry (thighs around the waist) cannot be rigged without diagonal limbs whose bone lines miss the joints; the seated forward-facing carry on the near forearm keeps every toddler joint exact, reads in the side elevation and matches qa-visual-3 ("forearm under the thighs").

2D (E1): `draw2d.ts` states per view which solids one shape unites and the layer order; each outline is the projection of the body in the default pose, traced and simplified (0.9 / 1.8 / 3.5 / 7 mm) and solved into its bone frame by the product's placement (residual 0.000 mm). Strokes by hand: coat front edge (1:50, 1:100), mouth lines (1:50). At 1:200 and 1:500 ears and other small forms are `body` role (no detail pen there).

## 4. Model (body / thermal / structure solids)

- **1:50 — 113 / 155 / 187.** Parent: head of 13 slices with nose, brow, lips, ears, eyes; hair at the nape; knit beanie with slack crown and rolled cuff; neck; scarf ring and two ends; stand collar; coat of 15 leaned slices (seat, waist, bust, shoulder blades, sloping shoulders); two skirt halves bound to the thighs (open with the stride); six buttons, two flap pockets flush on the skirt; set-in sleeves with deltoid and lateral drift to the elbow joint; forearm sleeves; glove cuffs; left glove hanging (palm, thenar, four three-phalanx fingers, thumb); right glove palm up under the toddler's thighs (fingers along the thighs, thumb along the outer thigh); jeans seat, thighs, shanks with calf and break; ankle boots (shaft and quarters, toe box on the toe bone, heel block, sole, toe sole). Toddler: padded suit trunk with pot belly, collar, zip (texture), thighs, shanks over the boot shafts, rubber boots with soles, sleeves and forearms, mittens with thumbs on its thighs, head of 11 slices, nose, lips, ears, eyes, hat with cuff and pompom.
- 1:50 thermal: outer garment or skin per region with nested shells (coat > sweater > torso-air on the coat body, sleeve > sleeve-air, denim > leg-air, boot > boot-air > sock, beanie > beanie-air, overall > suit-air, welly > boot-air > sock); face bands open below the caps, scalps as the caps' air layers; neck under scarf left out; four breath outlets on lips and nostrils, two plume patches on the cap crowns.
- 1:50 structure: parent skeleton (spine with lordosis and kyphosis, 9 rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, cranium, facial bones, mandible, cervical spine, femur with neck, trochanter and condyles, patella, tibia, malleolus, fibula, calcaneus, midfoot, toes, humerus, radius, ulna, carpals, every finger ray), toddler skeleton (spine, 4 rib hoops, ilia, pubis, clavicles, cranium, face bones, mandible, cervical spine, femora, tibiae, fibulae, tarsus, metatarsals, humeri, forearm and hand bones), joints at every rig joint, masses at the segment centres. No supports (none declared).
- **1:100 — 53 / 57 / 55**, **1:200 — 42 / 48 / 52**, **1:500 — 35 / 41 / 37**: each stated anew with its own tables (coat 9 / 6 / 5 slices, skirt halves 4 / 3 / 3, legs 4 / 3 / 2 stations, mitten-like gloves, heads 8 / 6 / 4 slices, caps by hand, toddler simplified to suit, limbs, boots, mittens, head, hat); structure merged per scale (four / two / no rib hoops, girdle bars, one bone per segment, ball joints at 1:100 and 1:200, 75 mm stick skeleton at 1:500).
- Decisions: four sentences at 1:50, two at each coarser scale, en and de (`texts.json`).

Final check line (`volumes-check.ts --product`, catalog): `FAIL: 12 measurement(s) over 2 rule line(s)`, only `volumes-thermal-coverage` 1:50 and 1:100. Silhouette IoU elevation / front / plan: 1:50 0.996 / 0.998 / 0.996, 1:100 0.994 / 0.996 / 0.995, 1:200 0.993 / 0.996 / 0.993, 1:500 0.988 / 0.992 / 0.993; cross-scale IoU 0.9996 / 1 / 1; bounds within 6 mm at 1:50.

## 5. Areas (not changed, C18)

Red at 1:50 / 1:100 (outer area against `areaM2`): `scarf` 0.146 / 0.128 vs 0.0794, `coat-torso` 1.247 / 1.229 vs 0.757, `jeans` 0.854 / 0.860 vs 0.613, `boots` 0.286 / 0.202 vs 0.106, `toddler-face` 0.053 / 0.050 vs 0.0317, `toddler-boots` 0.123 / 0.114 vs 0.037. Passing: face 0.088, sleeves 0.405 vs 0.321, gloves 0.114 vs 0.097, toddler suit 0.424 vs 0.582 (1:50; 1:100 within after the longer suit legs), mittens 0.029 vs 0.034. The declared values are skin shares; the honest garments are larger (parent +54 % in total). The toddler's total is −8 %, so a C27 re-division would fix all four toddler regions; proposal and measurements in `author-requests.md` ("2026-11-20 r3 humans/parent-carrying-toddler"). Model partition changes I made (legitimate under C18): face and toddler face are the open skin bands below the caps, the neck under the scarf is not skin surface, the toddler's suit legs reach over the boot shafts.

## 6. What I saw (opened with the Read tool)

- Catalog before: `r3-sheet` 1:50 body and `r3-2d` 1:50: tube coat, toddler floating in front of the chest on short legs, sloppy 2D (as in the audit).
- Candidate during work (own `look.ts`): 1:50 body six directions with rig (first version: bib-like scarf end, protruding pocket slabs, cap brims; fixed: two narrow scarf ends, pockets flush on the skirt, rolled cuffs, cheek blobs removed); heads close-up front, right, three-quarter (beanie and hat read as knit caps, faces plain but proportioned, the toddler's large cranium and small face read as a two-year-old); pockets close-up; 1:50 structure (both skeletons inside the bodies) and thermal cut; 1:100 and 1:500 bodies (same carry, simpler; 1:500 heads made rounder after the first look); hoisting pose and walk t 0.1.
- Catalog after the splice: `r3-sheet` 1:50 body (front: toddler with mittens on its thighs covering the parent's near chest, double-breasted coat, hanging far arm and glove; right side: forearm forward under the seat, glove under the thighs, toddler's boots hanging ahead of the coat; back: coat with the vent between the skirt halves, toddler's shoulder showing; left side: the toddler's legs over the forearm ahead of the parent; top: carry in plan; three-quarter: reads at a glance), 1:50 thermal cut (shells nested, face bands red under the green caps, legend complete), 1:500 structure (stick skeletons of both inside the ghost), 1:200 body (same figure, plain), 1:50 walk-carrying t 0.6 (stride, the far arm swinging, the toddler steady on the arm, the forward skirt half swinging with the thigh).
- `r3-2d`: candidate 1:50 (line and tone views read as the same carry in all three views), candidate 1:500 (tiny, legible, 1.73 m), catalog 1:50 `walk-3` tone view (elevation strides correctly; plan shows the swinging skirt half far forward, see 7).
- Rendered but not opened: `r3-sheet` 1:100 / 1:200 thermal and structure, 1:500 body and thermal, 1:100 and 1:200 `r3-2d` candidates; the `volumes-render` overlay sheets (the silhouette numbers in section 4 stand in for them).

## 7. Still imperfect

- Six body-share areas red at 1:50 and 1:100 (section 5), waiting for the area reviewer.
- The coat skirt halves are rigid on the thighs: at 40 to 50 degrees of hip flexion (swing leg, hoisting) the half swings out with the thigh like a stiff panel; a real coat drapes. Solid `weights` could halve the swing, but the round-3 look and 2D tools ignore weights, so it was not used.
- Front and plan 2D outlines are flat polygons (no `solid`): exact in the default pose, approximate in the gait poses (the plan of `walk-3` shows the skirt half and toddler limbs stretched).
- Faces are plain (no eyelids, nostrils, philtrum); garments have no drape folds beyond the trouser break and the caps' slack; the toddler's suit seams show as ragged intersection lines where limbs enter the trunk.
- Not run: the staffage test suites and model exports; fixtures embedding this asset may need regenerating after the rig change.
- Grades as I see them: 1:50 body B+ (A- for the carry and proportions), thermal A-, structure A-; 1:100 B+; 1:200 B+; 1:500 B.
