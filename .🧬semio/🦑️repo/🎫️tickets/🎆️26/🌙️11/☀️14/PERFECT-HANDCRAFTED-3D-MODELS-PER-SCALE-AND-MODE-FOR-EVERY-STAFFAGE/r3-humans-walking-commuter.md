# Round 3 — `humans/walking-commuter`

State: spliced. `r3-validate.ts humans/walking-commuter --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/walking-commuter --product`: `FAIL: 1 measurement(s) over 1 rule line(s)` (`areaDeviation:shoes` 0.640 at 1:50, see "Still imperfect"); every other rule is green at every scale.

Taken over from an author who was cut off. Found: `r3-walking-commuter-root.py` (rig, anchor and pose fixes written as a candidate, not committed), `-io.ts`, `-look.ts`, `-check.ts`, `-probe.ts`; the catalog untouched and valid, no model, no report. Kept after checking against the reference: the root script (with three changes: shoulder spacing, grip anchor, re-runnable commit) and the tools. Everything else is from this run.

Scripts (ticket folder): `r3-walking-commuter-kit.ts` (constructors, candidate rig frames), `-s50.ts` (every section of the 1:50 body, thermal, skeleton), `-coarse.ts` (1:100, 1:200, 1:500, each stated anew), `-build.ts` (writes `volumes/humans/walking-commuter.json`), `-draw2d.ts` (2D redraw), `-root.py` (root fixes, candidate, `--commit`), `-texts.json` (decisions), `-dimensions.json`, `-run.sh` (build, draw, candidate, validate), `-commit.sh` (catalog write and splice), `-fk.ts`, `-look.ts`, `-check.ts` (candidate tools).

## 1. Reference (man of 1.78 m and 72 kg, 1.80 m in derby shoes)

| quantity | reference | asset now | source |
|---|---|---|---|
| stature shod | 1.78 m + heel 25 mm less foot settling = 1.80 m | 1.803 m | asset `provenance.notes` (1.78 m barefoot); shoe data |
| hip joint height | 0.52 to 0.53 H = 0.935 m, + 24 mm shod | 0.959 m | Winter 2009 table 4.1, Drillis and Contini 1966 (asset references) |
| knee joint height | 0.285 H = 0.507 m, + 24 mm | 0.524 m | Winter 2009 |
| ankle joint height | 0.039 H = 0.069 m, + heel lift 24 to 30 mm | 0.099 m | Winter 2009; heel 25 mm |
| thigh / shank | 0.245 H / 0.246 H = 0.436 / 0.438 m | 0.435 / 0.425 m | Winter 2009 |
| shoulder joint height | 0.818 H = 1.456 m, + 24 mm | 1.479 m | brief; Winter 2009 |
| C7 above the shoulder joint, head joint below the vertex | about 65 mm; atlanto-occipital joint about 0.15 m below the vertex | neck base 1.554 m, head joint 1.653 m, vertex 1.803 m | anatomy; ANSUR II tragion to vertex 0.13 m |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.331 / 0.260 / 0.192 m | 0.33 / 0.26 / 0.19 m | Winter 2009 |
| shoulder joint spacing | biacromial 0.245 H = 0.436 m less 2 x 28 mm (the acromion overhangs the humeral head) = 0.38 m | 0.38 m | brief; ANSUR II |
| hip joint spacing | 0.17 to 0.19 m | 0.179 m | Bell et al. 1990 |
| carrying angle of the elbow | 10 to 15 degrees: the wrist hangs 3 to 6 cm outside the elbow | sleeve, radius and ulna run 31 mm outwards to the wrist | anatomy |
| head height / breadth / length | H / 7.5 = 0.237 m; 0.152 m; 0.197 m | 0.233 / 0.148 / 0.199 m (skull), hair + 7 mm | brief; ANSUR II |
| chest breadth x depth, clothed | 0.32 x 0.24 m body + shirt, jacket, overcoat (about 15 to 20 mm per side) | 0.344 x 0.261 m at 1.28 m | ANSUR II |
| bideltoid breadth, clothed | 0.49 m + coat | 0.52 m over the sleeves | ANSUR II |
| waist / hip breadth, clothed | 0.29 / 0.34 m body | 0.348 / 0.388 m | ANSUR II |
| thigh / knee / calf / hem (trousers) | girths 0.58 / 0.38 / 0.375 m body; hem about 0.42 m round | 0.18 x 0.20, 0.122 x 0.136, 0.118 x 0.136, 0.106 x 0.125 m | ANSUR II; straight wool trousers |
| overcoat | knee length (hem just above the knee), single-breasted, three buttons of 22 mm, flap pockets at the hip, stand-fall collar, notch lapels | hem 0.556 m, buttons at 1.325 / 1.19 / 1.055 m, pockets at 0.985 m | product knowledge (chesterfield type coat) |
| hand breadth / thickness | 0.086 / 0.028 m | 0.088 / 0.027 m | ANSUR II |
| derby shoe EU 43 | 0.295 m long, 0.105 m wide, heel 25 mm, sole 10 to 12 mm, top line below the malleoli | 0.297 x 0.108 m, heel 25 mm, sole 11 mm, tongue top 97 mm | manufacturer size tables |
| shoulder bag | leather satchel for a 13 inch laptop about 0.31 x 0.23 x 0.085 m, strap 36 mm, hangs at the hip of the carrying side | 0.31 x 0.225 x 0.085 m, top at 1.06 m, strap 36 x 6 mm, 1.2 m | product dimensions; asset summary ("weight carried by the near shoulder") |
| segment masses and centres, body surface | de Leva 1996; DuBois 1.894 m² | unchanged | asset |

## 2. Audit findings

Rig
- Ankle joint at 0.075 m: the bare-foot value plus 6 mm, although the figure wears shoes with a heel; hip, knee and shoulder were bare-foot heights too. Root defect (fixed: + 24 mm).
- Shoulder joints 0.301 m apart (0.169 H): 8 cm too narrow; the arms hung inside the rib cage. Root defect.
- Neck base at 1.50 m and head joint at 1.60 m with a head bone of 0.18 m: the head joint lay at the level of the mouth. Root defect.
- Foot bone and toe bone carried the old ankle height; heel anchors likewise.
- Bag bone without lateral offset: the bag hung in the mid-plane of the body; the old model put it on the left side, the summary and the 2D side view put it on the near (right) side.
- `strap-grip` anchor on a hand that hung 0.5 m below the strap; the right arm of the standing and walking poses did not hold anything.
- Hip, knee, limb lengths, hip spacing, masses: right, kept.

Dimensions: `heightM` 1.78 is the bare-foot stature of a shod figure; `depthM` (lateral extent) 0.5 was the old narrow figure with a 0.14 m thick box.

2D drawings (all 12): side view of an older study with a waisted short coat, front view a symmetric mannequin with square shoulders 0.40 m wide, a rectangular bag floating beside the left hip without contact to shoulder or hand, plan made of capsules and ellipses; elevation and front disagreed on the height (1.77 against 1.78 m).

Solids (catalog before this round, 99 / 120 / 38 at 1:50): tube limbs with ball caps, "skin" lofts under every garment, a box bag floating at the left with no strap, flat-topped head, structure = one loft per bone (body copies), thermal emitters as discs; kernel-traps listed four self-intersecting lofts.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[pelvis].y` | 0.935 | 0.959 |
| `rig.bones[torso].lengthM`, `neck.x` | 0.465 | 0.495 |
| `rig.bones[head].lengthM`, `mass.comX`, head anchors x | 0.18, 0.075 | 0.15, 0.045 (anchors moved by the same 0.03) |
| `rig.bones[arm-l|r].z` | +-0.1505 | +-0.19 |
| `rig.bones[foot-*]` angle / length, `toe-*` x / angle | 66.2 / 0.1858, 0.1858 / 23.8 | 59.79 / 0.1967, 0.1967 / 30.21 (ankle 0.099 m, ball 0.17 m ahead, sole level); mass centres re-expressed |
| `rig.bones[bag].z` | absent | -0.14 (the strap on the right shoulder) |
| `anchors[heel-*]`, `ball-*` | old foot frame | same points on the ground in the new frame |
| `anchors[strap-grip]` x, y | 0.085, 0.01 | 0.021, 0.0143 (in the closed hand) |
| `poses[*]` legs | old ankle | two-link IK re-solved per pose so every sole keeps its place |
| `poses[standing, contact-*, loading-*, passing-*]` right arm | e.g. -2 / 22 / -6 | -7 / 145 / 28: hand on the strap in front of the chest |
| `clips[*].keys` legs, `walk-cycle` right arm | same | same re-solve per key |
| `dimensions.heightM` / `depthM` | 1.78 / 0.5 | 1.8 / 0.56 (shod stature; extent over sleeves and bag) |
| manifest row `walking-commuter` `heightM` | 1.78 | 1.8 |
| `materials` | 11 | + `bone` (as the other round-3 skeletons: emissivity 0.9, absorptivity 0.5, 1900 kg/m³, authored estimate) |
| `thermal.regions[bag-leather].areaM2` | 0.263 | 0.32 (C16, factor 1.22: satchel with flap and strap, measured 0.322) |
| `representations[*].geometry` | 317 shapes | 329 shapes, all redrawn (E1) |
| `representations[*].decisions` | old drawing | `silhouette`, `retained`, `merged`, `omitted`, `thinElements` rewritten at every scale, `hatching` at 1:50, `overlays` at 1:200 and 1:500 |

2D redraw: `r3-walking-commuter-draw2d.ts` states per view which solids one drawn shape unites and the layer order (far to near), projects them in the default pose, traces and simplifies the outline (0.9 / 1.8 / 3.5 / 7 mm) and solves it into the bone frame with the product's own placement (`inverse` of `r3-project2d.ts`). Shapes elevation / front / plan: 1:50 40 / 49 / 30, 1:100 27 / 32 / 26, 1:200 22 / 23 / 22, 1:500 19 / 20 / 19. Drawn by hand: mouth, brows, nose line, coat front edge, back vent, side seam, shoulder seams (1:50), coat front edge (1:100), the fringe of the hair over the forehead in the front view, the strap as a stroke along its centre line (0.13 / 0.1 / 0.1 / 0.13 mm in the side view; 0.7 / 0.35 / 0.18 / 0.13 mm = its 36 mm in front and plan). Wool hatch kept on coat, collar and sleeves at 1:50 and 1:100.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 86 / 112 / 139.** Body: overcoat as one loft of 17 sections (hem, seat, waist, chest, shoulder blades, sloping shoulders), lapels of eight rows each, collar, three buttons, two flap pockets; sleeves with set-in heads and the carrying angle, shirt cuffs, wrists; seat and trouser legs with calf and break, socks; shoes with quarters, vamp, toe box, heel block, sole and toe sole cut to the inside; neck, scarf ring and scarf front; head of 13 sections with nose, brow, lips, chin, ears, eyes; hair of 10 sections; left hand with thenar, thumb and four three-part fingers; right hand closed on the strap; bag body, flap top, flap, clasp, two lugs, strap as a 36 x 6 mm band over the shoulder (slender). Thermal: shells for coat (jacket, shirt, tee), trousers (briefs), shoes (socks); open trouser tubes; scarf ring above the collar; face up to the hairline; outlets under the nose, on the lips and on the crown. Structure: spine with both curves, 9 rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, cranium, facial bones, mandible, cervical spine, femur (neck, trochanter, bowed shaft, condyles), patella, tibia, malleolus, fibula, calcaneus, midfoot, phalanges, humerus, radius, ulna, carpals, metacarpals, finger bones, 18 joints, 13 masses, strap and lugs by reference, bag load block.
- **1:100 — 35 / 37 / 74.** Coat 11 sections, one-plate lapels, scarf ring and front, collar, legs of four stations, shoes with merged sole, mitten hand with thumb, palm with finger mass and thumb, head 8 sections with nose and ears, hair 7, bag body and flap, strap. Merged skeleton (5 rib hoops, ischiopubic bars, jaw, hand plates).
- **1:200 — 22 / 23 / 42.** Coat 8 sections, legs of three stations, shoe rear and toe, mitten and fist, head 6, hair 5, scarf ring, bag block, strap. Skeleton: skull, cervical bar, spine, 3 hoops, girdle bar, pelvic ring, one bone per segment, 13 joints, 5 masses.
- **1:500 — 19 / 21 / 28.** Coat 7 sections, thigh, shank, one shoe per foot, sleeve, forearm, hands as single forms, scarf, head, hair, bag block, strap. Stick skeleton at 75 mm, 8 joints, 2 masses.

Final check line: `FAIL: 1 measurement(s) over 1 rule line(s)` (`areaDeviation:shoes` 0.640 at 1:50). Silhouette IoU elevation / front / plan: 1:50 0.997 / 0.996 / 0.992, 1:100 0.995 / 0.995 / 0.993, 1:200 0.995 / 0.994 / 0.989, 1:500 0.978 / 0.988 / 0.984. Thermal deviations at 1:50: face 0.27, hands 0.11, coat 0.12, scarf 0.24, trousers 0.08, bag 0.01.

## 5. What I saw in the renders

- `r3-sheet` 1:50 body, six views, default pose (final geometry): reads at first sight as a man in a knee-length overcoat with a scarf and a satchel. Front: sloping shoulders, lapels with the scarf between them, three buttons, pocket flaps, right hand on the strap, bag at his right hip, left hand hanging with fingers. Right side: upright trunk, bent arm with the fist in front of the chest, strap over the shoulder to both ends of the bag, flap and clasp, trousers breaking on the shoe. Back: plain coat back, strap down to the bag, hair to the nape. Left side: hanging arm with the hand ahead of the thigh. Top: head, shoulders, strap, shoes ahead. Three-quarter: convincing.
- Head close-up (own tool, three directions, before the last lapel change): rounded crown, hairline, ears, wedge nose, brow, lips; the face is still plain and somewhat square in front view.
- 1:50 `walk-cycle` t = 0 (own tool, four directions) and t = 0.3 (`r3-sheet` with rig): stride closed at hip, knee and ankle, heel strike and toe-off right, joints inside the limbs, bag and hand stay together; the forward thigh comes out through the rigid coat front as through an open skirt.
- 1:50 structure (own tool, four directions): recognisable upright skeleton, femora running inwards, rib cage, scapulae on the back, strap and bag load. 1:50 thermal cut (own tool, three directions, and the overlay sheet): shells nested, nothing outside the body.
- 1:100 body and 1:500 body (own tool, four directions, before the lapel and 1:500 smoothing changes), 1:200 body (`r3-sheet`, final): the same man, progressively plainer; at 1:200 the strap has a visible kink at the fist.
- 1:100 structure (`r3-sheet`, final): full merged skeleton inside the ghost.
- 2D: 1:50 candidate (product drawing and my large raster) before the strap became a stroke, 1:50 `contact-left` final (`r3-2d`): clean line and tone drawings; the walking pose strides correctly in the side view; the 0.7 mm strap stroke is heavy in front and plan.
- `volumes-render` 1:50 overlay sheet: all three overlays green.
- `r3-sheet` 1:500 body (final, six views): the same man as a plain figure, rounded head, hands as single forms, bag block; the strap shows a kink at the fist as at 1:200. `r3-2d` 1:500 (final): tiny but legible figure with the bag, 1.80 m and 0.56 m dimensioned.
- Rendered but NOT opened: `r3-sheet` 1:100 body (final), 1:50 thermal cut sheet, 1:100 thermal sheet, 1:500 structure sheet, `r3-2d` 1:100 and 1:200. For these only the green checks and the earlier looks stand.

## 6. Still imperfect

- **`shoes` area red at 1:50** (0.217 against 0.1326 m², + 64 %): the declared value is the skin of two feet; C18 forbids changing it. Filed in `author-requests.md` ("r3 humans/walking-commuter"), same gap as three other shod figures.
- The coat is rigid on the torso: in the stride the thigh passes through the coat front instead of pushing the skirt. Skinning weights on a split skirt would be the proper solution; not done.
- No cloth folds beyond the trouser break; the back vent, front edge and seams exist only as 2D strokes. The face is plain (no eyelids, nostrils or philtrum), limb sections are symmetric superellipses.
- The face-skin thermal surface is the head cut horizontally at the hairline height (1.748 m), so it includes the hair-covered back of the head below that level and leaves out nothing of the forehead; an honest area (0.120 against 0.0947 m²) but not the exact visible skin.
- The scarf thermal surface counts only the ring above 1.582 m and the front piece; the lower ring inside the collar is left out by construction.
- The right hand is authored in the frame of a hand bone that points up along the forearm; its turn towards the chest is in the solids, not in the rig (the rig rotates in the sagittal plane only). Elbow flexion 145 degrees is at the anatomical limit.
- The ball of the foot lies 0.17 m ahead of the ankle (about 2 cm ahead of the real metatarsal joint); kept so the gait keys stay valid.
- 1:500: the left hand is 78 mm thick because the feature rule follows the drawing; structure members are 75 mm sticks.
- Front and plan 2D shapes are flat outlines without `solid`: exact in the default pose, approximate in the gait poses (as before).
- Grades as I see them: 1:50 body A-, thermal A-, structure A-; 1:100 A-; 1:200 B+; 1:500 B+.
- `provenance.notes` still says 7.5 heads and does not mention this round; pose descriptions were not rewritten (they already say "near hand on the bag strap").
