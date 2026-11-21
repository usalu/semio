# Round 3 — humans/skateboarder

State: spliced. `r3-validate.ts humans/skateboarder` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/skateboarder --product`: 1:200 `PASS`, 1:500 `PASS`, 1:100 `FAIL: 3 measurement(s) over 1 rule line(s)`, 1:50 `FAIL: 4 measurement(s) over 1 rule line(s)`; every failure is an `areaDeviation` of `volumes-thermal-coverage` against a physiological or decided `areaM2` that an author may not change (see "Still imperfect"). `r3-skateboarder-suit.ts`: all four scales print-suitable.

Two authors worked on this asset. The first (cut off by a usage limit) made the reference, the rig and pose solver, the bodies of all scales, the thermal and structure lists and a first 2D; this author found the catalog valid but with 9 + 8 + 1 + 1 failing measurements (plan silhouette 0.77 IoU, structure outside the body, cross-scale plan, thermal triangles at 1:500, areas), repaired the 2D at the root (two forward bones, true plan outlines), the thermal face and sleeves, the 1:500 board and the stale decision texts, and wrote this report.

Rebuild: `./r3-skateboarder-ship.sh` (root -> build -> draw2d -> validate candidate -> apply -> `volumes-insert.py` -> validate catalog). Scripts: `r3-skateboarder-ref.py` (reference numbers), `-fk.py`, `-root.py` (rig, anchors, poses by inverse kinematics), `-kit.py`, `-body50.py`, `-coarse.py`, `-volumes.py` (thermal, structure), `-draw2d.py` + `-project2d.ts` (2D, ruling E1), `-texts.py`, `-build.py`, `-apply.ts`, `-suit.ts`, `-extents.ts`, `-bonecheck.ts`.

## 1. Reference

| Quantity | Value | Source |
|---|---|---|
| Rider | man, 19 years, stature H 1.78 m, 70.25 kg with board | asset, de Leva 1996 |
| Hip joint / knee / ankle above the sole | 0.52 H = 0.926 / 0.285 H = 0.507 / 0.039 H = 0.069 m | brief shares, Drillis and Contini 1964 |
| Thigh / shank | 0.4183 / 0.4379 m | from the shares |
| Shoulder joint / elbow / wrist | 0.818 H / 0.63 H / 0.485 H; upper arm 0.3346, forearm 0.2581, hand 0.19 m | brief shares |
| C7 | 0.861 H; head and neck above C7 0.1395 H = 0.248 m (head about H / 7.5) | de Leva 1996 |
| Biacromial / shoulder joint spacing | 0.405 m / 2 x 0.178 m | ANSUR II male (Gordon et al. 2014) |
| Hip joint spacing | 2 x 0.087 m | ANSUR II, Seidel et al. 1995 |
| Foot length | 0.152 H = 0.271 m, ball at 0.73 of it | Drillis and Contini |
| Shoe | low vulcanised skate shoe, sole 14 mm, 108 mm wide | catalogue ranges |
| Deck | 8.0 x 31.9 in = 0.2032 x 0.81 m, seven-ply maple 11.5 mm, concave 8 mm, kicks about 21 degrees, nose 54 mm and tail 50 mm rise | `board-spec`, catalogue ranges |
| Trucks, wheels | wheelbase 14.5 in class: axles at +-0.205 m, axle 8.0 in (half 0.1015 m), truck height 53 mm, wheels 54 x 32 mm, deck middle 0.085 m above ground | `board-spec` |
| Stance | regular: across the deck, left foot over the front bolts, facing the toe edge, head turned to the nose; pushing with the hips and front foot turned to the nose | knowledge of the sport |

## 2. Audit (before round 3)

- Rig: rider modelled facing along the board like a walker (no real stance); hip joints at z = 0 (no spacing), shoulders 2 x 0.11 m (real 2 x 0.178), upper arm 0.3136 (0.3346), forearm 0.2731 (0.2581), thigh 0.4349 (0.4183), shank 0.4247 (0.4379), foot bone 0.2225 m straight forward (ankle to ball is 0.1543 m, dropping 19.5 degrees), torso 0.4653 from a pelvis without sacral offset (0.513, L5/S1 50 mm behind the hips), head bone 0.2731 untilted (0.2488, 6 degrees forward); axles at +-0.1875 (wheelbase too short, +-0.205).
- `dimensions`: 1.774 x 1.1 x 1.04 m (height without the board under the feet; width and depth of no pose).
- 2D: capsules, ellipses and mixed flat shapes of a side-on walker; plan of capsules; not views of one body.
- Solids (qa-visual-3: B / B- body): box trunk, straight arms, mitten blades, board 15 % short, wheels small, thermal one grey region from 1:100, structure a copy of the body at 1:500.

## 3. Root fixes (asset file)

- `rig.bones`: wheel-f/-r x +-0.1875 -> +-0.205; new `hip-sway` (board -> hip-lift, lets the hips move to the toe or heel edge); hip-lift re-parented; pelvis 0.1011 -> 0.094; torso x 0.1011 -> 0.094, y 0 -> 0.05, length 0.4653 -> 0.513; head x 0.513, angle 0 -> -6, length 0.2731 -> 0.2488; upper arms x 0.435 -> 0.4364, y 0 -> -0.05, z +-0.11 -> +-0.178, length 0.3136 -> 0.3346; forearms 0.2731 -> 0.2581; hands 0.1821 -> 0.19; thighs z none -> +-0.087, 0.4349 -> 0.4183; shanks 0.4247 -> 0.4379; feet angle 90 -> 70.543, 0.2225 -> 0.1543; toes re-seated (angle 19.457, 0.075). Segment centres of mass moved with the lengths (de Leva shares).
- Two new fixed bones (this author): `shoulder-girdle` (torso, x 0.42, pointing forward, 0.12 m; carries hood and collar in the body, clavicles and scapulae in the structure) and `gaze` (head, x 0.13, pointing forward, 0.01 m; carries the nose). Reason in section 5.
- `anchors` (14): re-seated on the new sole, tail, hands and face. `poses` (10) re-solved by inverse kinematics in the board frame (worst residual 0.37 mm): shoes flat on the deck or ground, knee flexion 27-32 degrees cruising, 101-103 crouch; `clips` (3) keep their keys and now read as a real push cycle, pump and ollie.
- `dimensions`: 1.774 / 1.1 / 1.04 -> 1.846 / 0.813 / 0.345 m (extents of the cruising pose; height includes the 0.091 m deck top; width is the board).
- `representations`: every shape of every scale and view redrawn (section 5); all eight `decisions` of every scale rewritten in en and de (they still described the old drawing); `summary` and `provenance.notes` rewritten.
- `thermal.regions[*].areaM2`: unchanged (C16, C18).

## 4. Volumes

| Scale | body / thermal / structure | What is modelled |
|---|---|---|
| 1:50 | 79 / 91 / 74 | Hoodie as one garment loft of 17 sections (hem, waist, chest, shoulder slope, neck) with hood, collar ring, pocket, hem band and drawcords; sleeves with deltoid cap, forearms, rib cuffs, wrists, palms with four fingers and thumb; seat with gluteal mass, thighs, shanks breaking over the shoes, trouser hems; shoes with upper, sole, toe box across the ball joint, stripe; neck, skull, jaw, nose, lips, ears, eyes, brows, cuffed beanie; concave deck in two halves with grip tape, trucks with baseplate, hanger, bushings, kingpin, pivot, axle, four profiled wheels. Thermal: every region, nested layer shells at the real thicknesses (hoodie 3, sleeves 2, trousers 2, shoes 3, beanie 2), face skin, breath and plume outlets. Structure: spine with lordosis and kyphosis, ribcage, clavicles and scapulae on the girdle, pelvis, cervical spine, skull, mandible, femur with head and neck, patella, tibia, fibula, tarsus, phalanges, humerus, ulna, radius, metacarpus; joints, masses; deck, truck parts and wheels as the board's load path. |
| 1:100 | 35 / 37 / 65 | Same figure re-stated with 5-9 sections per mass: cuff into forearm, mitten with thumb, sole into shoe, one deck plate of 15 mm, truck of three parts; thermal per region part (hoodie, sleeves, trousers, shoes, hands, face, beanie stay distinct); skeleton with every shaft at least 15 mm. |
| 1:200 | 25 / 28 / 47 | Hood merged into the trunk, one head form, mittens, deck plate 30 mm, truck profiles; one bone per segment, the girdle one bar. |
| 1:500 | 17 / 24 / 37 | Two to five sections per mass at 75 mm; this author replaced the board (a prism across the axle width that read as a boat) by one slab with the deck's round-ended outline and kicks; stick skeleton, the girdle one bar. |

Changes of this author in the volumes: hood, collar and nose re-bound to the two forward bones (same form in space); clavicles and scapulae onto the girdle; thermal face closed in the collar with its rim face a section cut (the skin continues as scalp under the beanie, C25 a) instead of an open shell counted on both sides; thermal sleeve starts at the armhole seam (the dome inside the trunk only keeps the joint closed); 1:500 board.

Final check lines: 1:50 `FAIL: 4 measurement(s) over 1 rule line(s)` (areaDeviation face 0.93, trousers 0.43, shoes 0.65, trucks 0.54); 1:100 `FAIL: 3 ...` (face 0.83, trousers 0.43, trucks 0.50); 1:200 `PASS`; 1:500 `PASS`. Silhouette IoU elevation / front / plan: 1:50 0.946 / 0.943 / 0.958 (p95 11 / 9 / 17 mm), 1:100 0.944 / 0.939 / 0.946, 1:200 0.952 / 0.939 / 0.970, 1:500 0.955 / 0.933 / 0.968; cross-scale IoU 1.000 at every step. Other areas at 1:50: hoodie 0.10, sleeves 0.28, hands 0.04, deck 0.10, wheels 0.13.

## 5. 2D (ruling E1) and why two bones were added

The rider stands across the board, so every body bone leaves the rig plane by up to 88 degrees; `validateAsset` (`lateral-ambiguous`) then requires every shape to be a solid (extruded or revolved). An extruded profile is a box seen from the third side, so the plan of trunk and head could only be rectangles (found: IoU 0.77, p95 77 mm) and slicing them drew bands across the face. A plan outline can only be swept through the height on a bone whose axis lies across the trunk; none existed. The shoulder girdle and the line of sight are such bones (rigid with trunk and head, anatomically meaningful, with their own solids). Result:

- Elevation and front: trunk, seat, head, beanie, hands, shoes are the outline that faces the viewer in the default pose, swept through the depth of the part (`sweep` w or v with a taper); limbs are revolved about their bones; board, trucks, wheels plain outlines; the beanie in the face-on view is its front half so the face stays visible.
- Plan 1:50 and 1:100: hoodie (6 slabs by height), hood, head (5) and beanie (4) are their true outlines from above swept along the trunk or head axis, painted bottom-up; seat as round columns; limbs revolved; shoes are their sole outlines; deck with round ends.
- Plan 1:200 and 1:500: no hood or nose solid exists there, so trunk, seat, head and beanie are round columns about their bones (one to six circles each).
- Paint order from the depth of the cruising pose per view; `knit-rib` hatch on the beanie at 1:50.

## 6. What I saw (opened with the Read tool)

- `r3-sheet` 1:50 body, cruising: front (rider's left side): slight knee bend, hood on the back, hands hanging in front of the thighs, shoes flat on the deck; right side (his front): shoulders 0.41 m, hoodie with pocket and cords, feet over the bolts, head turned to the nose, beanie cuff; back and left side: hood, seat, heels; top: shoulders along the board, beanie, toes over the toe edge; three-quarter: reads at once as a skater on a real popsicle deck. A few jagged seam fragments where the hoodie hem meets the seat (intersection lines, not gaps).
- 1:50 `pushing` and `crouch`, 1:100 `ollie` at 0.9 s (3D): joints stay closed, pushing foot on the ground beside the board, arms spread in the flight, board tilted under the feet.
- 1:50 structure: skull and mandible, S-curved spine, ribcage (one closed cage, no single ribs), scapulae on the back, clavicles to the shoulder joints, pelvis, long bones inside the limbs, deck plate on two trucks. 1:50 thermal cut: nested shells inside hoodie, sleeves, trousers, shoes, beanie; red face skin; coloured trucks and wheels.
- 1:100 body: the same figure, simpler; 1:500 body: a clear standing figure, mittens blunt, board a round-ended slab with kicks.
- `volumes-render` 1:50 overlay: green nearly everywhere; thin red fringes along limbs and the back edge of the plan.
- `r3-2d` 1:50, 1:100, 1:200, 1:500 default; 1:50 `crouch` and `pushing`; 1:100 `push-cycle` at 0.5 s: default views agree with the model in all three views; the plan follows the lean and the turn of the trunk in the other poses.

## 7. Still imperfect

1. Areas (not changed, C16 / C18 / C27; rows added to `area-review.md`): face 0.0739 declared against 0.143 modelled (head band below the beanie, neck, ears, nose); trousers 0.6589 against 0.943 (loose twill trousers are larger than the leg skin); shoes 0.1294 against 0.213 (a pair of skate shoes against the foot skin); trucks 0.105 against 0.0485 (the decided value is two 75 x 50 x 180 mm boxes, the real trucks are smaller; factor 2.2).
2. Other poses in the two elevations: trunk, seat and head are prisms of the default-pose outline. When the rider turns to the nose (push cycle) the front elevation shows a square-shouldered trunk and the side elevation a slab with the hood as a block; the plan and all limbs stay right. The drawing engine has no solid with two profiles; wish written to `tool-requests.md`.
3. `crouch` front elevation: a sliver of the head form shows above the half beanie.
4. Plan at 1:200 and 1:500 is a cluster of circles (about 3 mm on paper), not the true outline.
5. Elevation and front IoU 0.94 (other figures reach 0.97-0.99): revolved limbs are envelopes of garment sections that are not round, and swept hands and shoes are slightly larger than the model from the side.
6. The ribcage is one closed form at every scale; the trousers have no modelled folds beyond the break over the shoe.
7. `sectionFaces` on the thermal face rim is my reading of C25 a (cut through skin that continues under the beanie); if the reviewer disagrees the rim cap counts as well and the face area reads higher than 0.143 (not measured).

## Round 5 handcraft (2026-11-20)

Scripts: `r5-skateboarder-measure.ts` (rig and per-bone body extents in a pose; also used for the child), `r5-skateboarder-body.py` (reads the round 4 source kept as `🗑️generated/r5-skateboarder/volumes.before.json`, writes `volumes/humans/skateboarder.json`). Spliced; `r3-validate.ts humans/skateboarder --catalog-only` VALID (schema 0, validateAsset 0, print-suitable).

### Reference (modelled to) and model against it, 1:50, pose cruising

| quantity | reference | model | deviation |
|---|---|---|---|
| stature H | 1.78 m (man, 19 y; DIN 33402 / ANSUR II P50) | 1.78 m (rig sum: 0.926 hip + 0.094 + 0.513 trunk + 0.249 head, minus the sole) | 0 % |
| thigh (hip to knee joint) | 0.235 H = 0.418 m | 0.418 | 0 % |
| shank (knee to ankle) | 0.246 H = 0.438 m | 0.438 | 0 % |
| upper arm | 0.188 H = 0.335 m | 0.335 | 0 % |
| forearm | 0.145 H = 0.258 m | 0.258 | 0 % |
| hand | 0.108 H = 0.192 m | 0.190 bone, 0.174 body (fingers flexed 8/24/38 degrees) | -1 % |
| foot | 0.152 H = 0.271 m | 0.271 bone path, shoe 0.29 m over all | 0 % / shoe +7 % (sole and toe box) |
| head (chin to vertex) | H / 7.5 = 0.237 m | 0.231 (head loft 0.016 to 0.2474 above C7) | -2.5 % |
| biacromial / shoulder joints | 0.405 m / 2 x 0.178 m | 2 x 0.178 m joints, hoodie 0.41 m, over the sleeves 0.48 m | 0 % |
| hip breadth (clothed) | 0.355 m + 2 x 6 mm twill | 0.368 m | +0.3 % |
| thigh breadth at the crotch (clothed) | 0.17-0.19 m (loose twill) | 0.19 m | in range |
| shank breadth (clothed) | 0.12-0.13 m | 0.124 m | in range |
| knee breadth (clothed) | 0.11-0.13 m | 0.122-0.128 m | in range |
| deck | 0.81 x 0.203 m, top 0.091 m | 0.812 x 0.210 m (concave rails), top 0.091 | +0.2 % |
| wheelbase / wheel | 0.41 m / 54 x 32 mm | 0.41 / 54 x 32 | 0 % |

The rig was already right (round 3); every length lies within 1 % of the reference, so no root change of the rig, poses, clips or 2D was needed.

### Grades (before -> after; body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | B+ / A- / B+ | A- / A- / A- |
| 1:100 | B+ / A- / A- | A- / A- / A- |
| 1:200 | B+ / B+ / A- | B+ / B+ / A- (unchanged) |
| 1:500 | B / B+ / B | B / B+ / B (unchanged) |

Defects seen before (sheets opened: 1:50 body six views and the right view alone, 1:50 structure, 1:50 thermal cut, 1:100, 1:200, 1:500 body, 1:500 structure, 1:50 crouch, 2D 1:500): fingers hooked 70 degrees into claws; a ball at every knee (thigh end wider than the shank top and 70 mm past the joint, both surfaces coincident above the joint); a brief-shaped seat panel bulging in front of the thighs; the mandible of the structure hanging below the chin (structure 7.7 mm outside the body in elevation, a round 4 regression).

### Changes

- 1:50 hands: four fingers grouped side by side (radii 8.8-10 mm, 19 mm apart, so they touch), flexed 8 / 24 / 38 degrees (relaxed hand), phalanges 47 / 29 / 23 mm; thumb along the index, 13.5 to 7 mm.
- 1:50 and 1:100 knee: thigh 63 mm at u 0.40, 61 mm at the joint line, tapering inside the shank to 16 mm 64 mm past it; shank 20 mm at 50 mm above the joint, 56 mm at 14 mm above, 62-63 mm at the joint: one clean seam ring, no ball, closed in the crouch (101 degrees).
- 1:50 trouser seat: crotch narrowed (18 / 52 / 98 mm half width at 116 / 104 / 88 mm below the hip joint), front reach pulled back 6-16 mm so the thighs carry the front line; the back (buttocks) unchanged.
- Structure, every scale: skull re-seated (cranial base 4 mm higher and 5 mm further back, axis 10 degrees from the head bone instead of 16.4, facial skeleton 0.88): mandible inside the chin, `outsideBodyM:elevation` 0.0077 -> 0.0038 m.
- Thermal: the nested leg shells (leg-air, twill) of thigh, shank and seat redrawn from the new body at their own depth (`r4-g11-kit.edit`); hands are `{ of }` references and follow.

### Checks after

`volumes-check.ts --product`: every rule ok at every scale except `volumes-thermal-coverage` areaDeviation trousers 0.44 (1:50, 1:100) and shoes 0.64 (1:50), unchanged C18 physiological values that an author may not change (already in `area-review.md`); silhouette IoU elevation / front / plan 1:50 0.942 / 0.939 / 0.956, 1:100 0.946 / 0.934 / 0.948. Opened after: 1:50 right view (crotch, knees, hands), 1:50 crouch six views, 1:50 thermal cut, 1:50 structure head crop.

### Remaining (with reason)

- 1:200 and 1:500 not remodelled in this round: 1:500 board is a 75 mm slab from the ground to the deck top (the floor of 75 mm makes a 11.5 mm deck impossible; it reads as a board with kicks, not as wheels); torso and limbs at 1:200 are faceted lofts of 5-6 sections. Grade B+ / B.
- The crotch seam of the seat still shows as a narrow U between the thighs (two solids meeting); the hoodie has no cloth folds.
- Trouser and shoe areas (C18, area reviewer).
