# Round 3 — `humans/standing-adult`

State: root fixes and volumes are in the catalog asset. `r3-validate.ts humans/standing-adult --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. `volumes-check.ts humans/standing-adult --product`: `PASS` at every scale. All four scales resolve as `authored` under the default print profile.

Scripts (ticket folder): `r3-standing-adult-apply.sh` (build, validate, write catalog only when valid), `r3-standing-adult-build.py` (candidate document), `r3-standing-adult-roots.py` (rig, anchors, poses, clips, dimensions, provenance, 2D decisions, shapes), `r3-standing-adult-model.py` + `r3_standing_adult_body50.py`, `r3_standing_adult_s50.py`, `_s100.py`, `_s200.py`, `_s500.py` (every section stated by hand), `r3_standing_adult_kit.py`, `r3_standing_adult_fmt.py`, `r3_standing_adult_strokes.py` + `r3-standing-adult-2d.ts` (2D redraw), `r3-standing-adult-view.ts`, `-frames.ts`, `-extents.ts`, `-suit.ts` (probes). The first four files were started by the earlier author of this asset (rig draft, kit, 1:50 body draft); everything was continued from them.

## 1. Reference (slim adult, stature 1.69 m barefoot, 59 kg; shod on a 20 mm sneaker sole)

| quantity | reference | value in the asset | source |
|---|---|---|---|
| hip joint height | 0.52 to 0.53 H | 0.883 m barefoot, 0.903 shod (0.522 H) | brief; Winter 2009 table 4.1; Drillis and Contini |
| knee joint height | 0.285 H = 0.482 m | 0.470 barefoot, 0.490 shod (0.278 H) | Winter; segment lengths in the asset follow de Leva 1996 (female), kept, 12 mm low |
| ankle joint height | 0.039 H = 0.066 m | 0.066 barefoot, 0.086 shod | Winter |
| shoulder joint height | 0.818 H = 1.382 m | 1.3775 barefoot, 1.3975 shod | Winter |
| elbow / wrist height | 0.63 H / 0.485 H | 1.064 / 0.817 barefoot (0.630 / 0.483 H) | Winter |
| C7 height | 0.855 to 0.86 H | 1.445 barefoot, 1.465 shod | de Leva 1996 (cervicale) |
| head height chin to vertex | H / 7.5 = 0.225 m | 0.222 m | brief; ANSUR II menton to vertex |
| head breadth / length | 0.145 to 0.155 / 0.187 to 0.197 m | 0.147 / 0.190 m | ANSUR II (Gordon et al. 2014) |
| upper arm / forearm / hand length | 0.186 H / 0.146 H / 0.108 H | 0.3135 / 0.247 / 0.1805 m | Winter |
| thigh / shank length | 0.245 H / 0.246 H | 0.413 / 0.404 m | Winter; de Leva |
| hip joint spacing | 0.17 to 0.19 m | 0.17 m | Bell et al. 1990 (from inter-ASIS distance) |
| shoulder joint spacing, biacromial, bideltoid | biacromial 0.365 to 0.415 m; joints about 3 cm inside each acromion | 0.306 / 0.375 / 0.435 m nude, 0.46 m with sleeves | ANSUR II; brief gives 0.245 H as upper bound |
| chest / waist / hip breadth x depth (nude) | about 0.285 x 0.215 / 0.25 x 0.175 / 0.34 x 0.225 m | sweatshirt sections 0.32 to 0.37 x 0.23 to 0.24 m with ease | ANSUR II, DIN 33402-2, slim build |
| thigh girth top / mid / above knee | 0.56 / 0.50 / 0.38 m | sections 0.166 x 0.186, 0.154 x 0.172, 0.11 x 0.12 m | ANSUR II |
| calf / ankle girth | 0.355 / 0.22 m | 0.110 x 0.124 / 0.089 x 0.099 m in jeans | ANSUR II |
| upper arm / forearm girth, wrist | 0.265 / 0.245 m, 0.055 x 0.038 m | sleeve 0.12 x 0.13 at the deltoid, 0.097 x 0.103 at the elbow | ANSUR II |
| hand length / breadth / thickness | 0.181 / 0.079 / 0.027 m | 0.1805 / 0.080 / 0.027 to 0.036 m | ANSUR II |
| foot length; sneaker | 0.152 H = 0.257 m; low-top 0.275 to 0.285 m long, 0.10 m wide, 20 mm sole, collar below the malleoli | 0.281 x 0.100 m, sole 24 mm, collar top 0.09 m | Winter; manufacturer size tables (from memory, no web lookup) |
| skeleton landmarks | iliac crest 0.60 H, sternal notch about 0.82 H, acromion 3 to 4 cm above the shoulder joint, biiliac breadth 0.27 to 0.28 m, rib cage 0.24 m wide | crest 1.036, notch 1.405, acromion 1.428, biiliac 0.27 m | standard anatomy (Gray; own knowledge) |
| segment masses and centres | de Leva 1996 | unchanged; three centres re-expressed in the new bone frames | asset `provenance.references` |
| skin area | DuBois 1.675 m² | unchanged (C18) | asset |

## 2. Audit findings

Rig (before)
- The skeleton stood barefoot on the ground while the body wore sneakers: no sole height anywhere (ankle 0.066 m with a shoe drawn round it).
- `torso` ran straight up from the pelvis tip to 1.4204 m: C7 at 0.840 H (reference 0.855 H = 1.445 m), 25 mm low, and no offset of the spine behind the hip joints.
- `neck` 0.0951 m and `head` 0.1746 m: the head joint lay at 1.515 m, inside the jaw, 26 mm below the atlanto-occipital joint (0.912 H); the head bone ended 6 mm above the vertex.
- Foot: ball of the foot 0.1496 m ahead and the bone 0.1635 m long for a 0.066 m ankle; no room for a sole.
- Relaxed pose: thigh +6, shank -14, foot +8 degrees, a knee flexed 14 degrees in "relaxed standing"; arms without any spread, both in the sagittal plane through the shoulder joints, so the hands hung inside the hip outline.
- `arms-crossed`: both forearms pointed straight ahead (planar pose); nothing was crossed.
- Hip, shoulder, elbow, wrist heights, limb lengths, joint spacings: within 12 mm of the reference, kept.
- Anchors `mouth` and `nostrils` lay 4 to 5 cm off the lips and nose base; heel and ball anchors described a bare foot.

Dimensions (before): 1.69 x 0.34 x 0.40 m (height, forward, lateral). The lateral 0.40 m is narrower than the figure's own shoulders with sleeves (0.46 m).

2D (before, all 12 drawings): constructed, not views of one body. Elevation with `path` outlines of a figure leaning back with bent knees; front a patchwork (ellipse head, 4-point right arm against a 24-point left arm, left and right thigh with 42 and 36 points); plan of capsules and ellipses (`waist`, `trunk`, `shoulders`, `neckwear`), shoes drawn above the legs; different ids for the same part per view; 1:500 a silhouette without a far arm.

Solids (before; 1:50 85 / 64 / 37): box-like trunk loft, tube limbs, ball shoulders, slipper shoes, face of loose ellipsoids; thermal mostly references plus uniformly shrunk copies as "layers" on every limb (a T-shirt down to the wrists, briefs down to the ankles); structure = the body lofts again with ball joints, at every scale.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[pelvis].y` | 0.883 | 0.903 (20 mm sole) |
| `rig.bones[torso]` y / angleDeg / lengthM | 0 / 0 / 0.4422 | 0.035 / -1.84 / 0.46709 (lumbosacral joint 35 mm behind the hip joints, to C7 at 1.465) |
| `rig.bones[torso].mass` comX / comY | 0.2043 / 0 | 0.2053 / -0.0284 (same point in space) |
| `rig.bones[arm-l,arm-r]` x / y / angleDeg | 0.39941 / 0 / 180 | 0.40033 / -0.02216 / 181.84 (same shoulder joint, 1.3975 m) |
| `rig.bones[neck]` x / angleDeg / lengthM / comX | 0.4422 / -8 / 0.0951 / 0.0476 | 0.46709 / -17.228 / 0.10163 / 0.0508 |
| `rig.bones[head]` x / angleDeg / lengthM / comX | 0.0951 / 8 / 0.1746 / 0.0728 | 0.10163 / 19.068 / 0.149 / 0.0463 (joint at 1.561, vertex 1.710, same centre of mass) |
| `rig.bones[foot-*]` angleDeg / lengthM / comX | 66.193 / 0.1635 / 0.0703 | 60.133 / 0.17251 / 0.0742 |
| `rig.bones[toe-*]` x / angleDeg | 0.1635 / 23.807 | 0.17251 / 29.867 |
| `rig.bones[torso].title` | "Torso with coat" | "Torso with sweatshirt" |
| `anchors` heel / ball | (-0.0096, -0.0764), footprint 0.0616 x 0.0585 / (0.1635, 0), 0.088 x 0.09 | (0.015, -0.0904), 0.07 x 0.062 / (0.17251, 0), 0.09 x 0.092 |
| `anchors` mouth / nostrils / crown / ear | (-0.0049, -0.0777) / (0.0291, -0.1009) / (0.1697, 0) / (0.045, 0) | (-0.031, -0.101) / (-0.011, -0.107) / (0.149, 0) / (0.021, 0.01) |
| all poses, legs | thigh 6, shank -14, foot 8 | thigh -1, shank -5, foot 6 (ankle 5 cm behind the hip, foot flat) |
| hanging arms (relaxed, pointing left arm) | arm 3 / 5, forearm 8 / 10, hand 4 / 6, no spread | arm 0 / 1 with `abductionDeg` +6 / -6, forearm 7 / 9, hand 3 / 4 |
| pose `arms-crossed`, arms | arm 2 / 6, forearm 98 / 104, hand -60 | arm 30 / 37 with `abductionDeg` 4 / -4 and `twistDeg` 80 / -80, forearm 92 / 99, hand 32; description rewritten |
| clip `shiver`, keys | arm 0..8, forearm 95..107 | arm 30 +-2 / 37 +-2 (with spread and twist), forearm 92 +-3 / 99 +-3; thigh offsets -7 |
| clip `idle-breathing` | arm-r 6.5 / 4 / 6, arm-l 4 / 2, forearm-r 11.5 | 2.5 / 0 / 2, 1 / -1, 10.5 |
| clip `fanning` | — | `arm-r.abductionDeg` 0 in every key |
| `dimensions` heightM / widthM / depthM | 1.69 / 0.34 / 0.40 | 1.72 / 0.32 / 0.49 (stature 1.69 + 20 mm sole + 10 mm hair; toe tip to hood; hand to hand with the arms 6 degrees off the trunk) |
| manifest entry heightM / widthM | 1.69 / 0.34 | 1.72 / 0.32 |
| `provenance` | — | references `gordon-2014`, `din-33402-2-2005`; round-3 note (en, de) |
| `structure.assumptions[1]`, pose descriptions | "close together" / "Feet together" | "at hip-joint width" / "Feet at hip width" |
| `representations[*].geometry` | 318 shapes | 359 shapes, redrawn (below) |
| `representations[*].decisions.silhouette` (and `merged`, `omitted` at 1:50, 1:500) | described the constructed drawings | describe the views of the body |

2D redraw (E1). Every filled shape is the orthographic outline of the body solids of its part at that scale, Douglas-Peucker at 0.8 / 1.5 / 3 / 6 mm. Shapes and points, elevation / front / plan: 1:50 63, 743 / 62, 804 / 35, 545; 1:100 32, 380 / 32, 441 / 21, 282; 1:200 21, 221 / 21, 263 / 19, 177; 1:500 18, 133 / 18, 146 / 17, 116. Old ids, roles, tones, regions, hatches kept where the part exists (`sleeve-*`, `forearm-*`, `cuff-rib-*`, `hand-*`, `thumb-*`, `thigh-*`, `shank-*`, `shoe-*`, `sole-*`, `shoe-toe-*`, `toe-sole-*`, `neck`, `hoodie`, `hood`, `hem-rib`, `head`, `hair`, `ear`, `eye`, the stroke ids, `sil-*` at 1:500); one id per part in all three views. New ids: `jeans-seat`, `sock-*`, `toe-cap-*`, `hood-roll`, `hair-fringe`, `sole-line-*` (front), `sil-seat`, `sil-neck`. Gone: `waist`, `trunk`, `shoulders`, `neckwear`, `face`, `nose` (plan), `upper-arm-*`, `shoe-heel-*`, front finger lines (the hand is seen edge-on), the second drawstring in the side view. Elevation: bone-local outlines, arm parts as prisms swept across. Front: lateral outline of each part as a prism through its depth, feet flat. Plan: arm and leg segments as solids of revolution, hands as prisms, trunk, head and feet flat, layered by height (shoes under legs, the old order had them on top). Inner lines hand-stated at the model's landmarks.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 80 / 85 / 176.** Body: head loft with jaw and chin, nose, lips, brows, eyes, ears, hair cap; neck; sweatshirt as one garment (hips, waist, chest, shoulder slope to the acromion, open neckline), hem rib, collar rib, kangaroo pocket, drawstrings, hood on the upper back with a rolled edge over the shoulders; sleeves with deltoid mass, forearms, rib cuffs; palm, thenar, two-segment thumb, four two-segment fingers; jeans seat with gluteal masses, thighs, kneecaps, shanks with calf; socks; sneaker upper, sole, toe box, toe cap, toe sole, laces. Thermal: outer surfaces by reference; T-shirt shell under the trunk and as short sleeves only; briefs under the pelvis only; socks inside the uppers and at the ankle; face as its own solid ahead of the hair line; outlet patches under the nose, on the lips, on the crown. Structure: cranium, facial skeleton, mandible with rami, zygomatic arches; cervical spine; thoracolumbar spine with lordosis and kyphosis, nine spinous processes; 12 ribs each side from their vertebra round to the sternum, lower ones ending free, costal arch; sternum, clavicles, scapulae with spine, acromion, glenoid; sacrum, hip bones with wings turned 34 degrees, pubic and ischial rami; femur with head, neck, trochanter, bowed shaft, condyles; patella, tibia with malleolus, fibula; talus, calcaneus, tarsus, five metatarsals, five toes; humerus with head, tubercle, condyle; ulna with olecranon, radius; carpus, metacarpals, phalanges; 17 joints (discs, balls, hinge pins); 4 masses. No support solids (`structure.supports` is empty).
- **1:100 — 25 / 28 / 81.** New sections: head with nose and ears, hair, neck, sweatshirt with hem and collar merged, hood, seat, sleeve, forearm with cuff merged, mitten hand and thumb, thigh, shank, shoe, toe box. Skeleton with merged bones, six rib hoops per side.
- **1:200 — 20 / 23 / 43.** One mass per part. Skeleton of 31 mm bones, three rib hoops per side, one bar per shoulder girdle, ball joints at hip, knee, shoulder, elbow.
- **1:500 — 19 / 22 / 23.** Fewest masses, hood merged into the back. Structure: stick figure (see 6).
- Decisions: three bilingual sentences per scale (body, thermal, structure).

Final check line: `PASS (10.9 s)`. Silhouette IoU elevation / front / plan (before the last hand and pose edits; rerun passed): 1:50 0.996 / 0.987 / 0.966, 1:100 0.995 / 0.990 / 0.972, 1:200 0.994 / 0.983 / 0.966, 1:500 0.992 / 0.975 / 0.955. Outer areas at 1:50 against declared: face 0.1176 / 0.1173, hands 0.0769 / 0.0838, hoodie 0.889 / 0.8207, jeans 0.784 / 0.6109 (+28 %), sneakers 0.184 / 0.1424 (+29 %).

## 5. What I saw in the final renders

- `r3-sheet` 1:50 body, relaxed, six views with rig: joints sit at hip, knee, ankle, shoulder, elbow, wrist of the figure; front shows sloping shoulders, arms hanging slightly off the trunk, pocket, drawstrings, slim jeans, shoes at hip width; sides show the ankle behind the hip, calf, shoe with sole and toe cap, hood on the back; back shows the hood; top shows shoulders, head, feet ahead; three-quarter reads as a person in a hoodie at once.
- Upper-body close-up (front, 45 degrees, side, back; after the shoulder and neckline change): continuous line from neck over the shoulder into the sleeve, neck visible above the collar, rolled hood edge; face with eyes, nose, lips. Elbow seam still slightly ragged.
- Head and hand close-ups: seen only in the earlier author's state (before my edits to neck and shoulders; head and hands unchanged since).
- 1:50 body `arms-crossed` (close-up from three directions): forearms folded across the belly one above the other, hands on the opposite arm; one hand stands a little proud of the forearm.
- 1:50 structure, six views over the body ghost: reads as a skeleton at first sight, inside the body everywhere; rib cage with arch, pelvis with wings, femur neck. Earlier close-up: lower rib angles lie 3 cm behind the lumbar-thoracic spine line (anatomical, but they read busy in the side view).
- 1:50 thermal cut, six views: T-shirt inside the sweatshirt on trunk and upper arms, briefs inside the jeans at the pelvis, sock inside the shoe, red face under green hair.
- 1:100 body sheet: clean simplified figure, same proportions. 1:500 body and structure sheets: mannequin of 19 masses; thick blue stick figure inside it.
- `r3-2d` 1:50 relaxed (candidate, before the pose and hand edits) and 1:50 `arms-crossed` (final): line and tone drawings read as the same person in all three views; crossed arms read in the front view, in the side view the turned sleeve prism looks like a wide wedge.
- Rendered but not opened: 1:200 body, structure and thermal sheets; 1:100 and 1:500 thermal; 1:100 and 1:200 structure; `pointing`; the `shiver` frame at 0.15 s; `r3-2d` at 1:100, 1:200, 1:500; `volumes-render` overlay sheets.

## 6. Still imperfect

- Grade as I see it: body 1:50 B+ to A-, not A. Lofts are superellipses: no pectoral, scapular or separate calf-head relief, no cloth folds beyond hem, cuff, hood roll and pocket; elbow and knee are overlaps of two rigid segments with a visible seam; the face is plain.
- Coarse-scale sheets and several poses were rendered but not looked at (list above), so 1:200 and the `pointing` / `shiver` / `fanning` / `hands-warming` motions are unverified by eye after the rig change.
- Structure at 1:500 is a stick figure 75 mm thick and the hand member is as thick as the hand, forced by `volumes-feature`; filed in `author-requests.md` together with the jeans and sneaker areas, which pass at +28 % and +29 % of a 30 % band.
- Knee joint 12 mm below 0.285 H (de Leva segment lengths kept).
- Sneaker and anatomy reference values are from memory and the asset's references; no web lookup was made.
- Structure solids use material `skin` (no bone material in the asset). The body-plume outlet is 0.11 x 0.09 m on the crown, smaller than the emitter's 0.16 m, because the head is narrower.
- Front cuff lines at 1:100 and the second side-view drawstring were dropped; in the side elevation a twisted arm is drawn as a prism.
- Not run: the staffage test suites. Tests and fixtures that embed this asset (`structure-projection`, `multiview-coverage`, `multiview-sheet`, `dimension-leader`, `cast-export`, `transport-field`) may need regenerating after the rig, dimension and 2D change.
- `r3-standing-adult-roots.py` applies the rig and relative clip changes only once (guarded by the old hip height); a later rig change needs absolute values there.
