# Round 3 — humans/seated-guitarist

Author of round 3, 2026-11-19. Most of the work was done by an earlier author of this asset who hit a usage limit before splicing and before writing this report. I resumed from their scripts: I checked the state, rebuilt everything, looked at every list and scale, fixed what was left, wired the texts they had written but not yet applied, wrote the catalog, spliced and checked.

Scripts (all in `T`):

| Script | Role |
|---|---|
| `r3-seated-guitarist-ref.ts` | Reference dimensions (guitar, stool) |
| `r3-seated-guitarist-roots.ts` | Rig, anchors, poses solved in 3D |
| `r3-seated-guitarist-kit.ts` | Section and loft helpers |
| `r3-seated-guitarist-s50.ts` | 1:50 body |
| `r3-seated-guitarist-coarse.ts` | 1:100, 1:200 and 1:500 bodies |
| `r3-seated-guitarist-lists.ts` | Thermal, structure, materials and root texts |
| `r3-seated-guitarist-texts.ts` | Bilingual decisions |
| `r3-seated-guitarist-draw2d.ts` | 2D redrawn per ruling E1 |
| `r3-seated-guitarist-build.ts` | Writes the candidate document and the source |
| `r3-seated-guitarist-write.py` | Writes the root members into the catalog file (text splice per member) |
| `r3-seated-guitarist-sheet.ts`, `-look.sh` | Sheets of a candidate file |
| `r3-seated-guitarist-suit.ts` | Print suitability |
| `r3-seated-guitarist-probe.ts` | Probe |

## Reference

| Quantity | Real value | Source | Model (rig / solids) |
|---|---|---|---|
| Stature H | 1.76 m (asset) | asset `BodyPhysiology` | vertex 1.775 above the sole in boots (about 25 mm of sole and heel) |
| Hip joint | 0.52–0.53 H = 0.915–0.93 m | Winter 2009 tab. 4.1 (Drillis–Contini) | 0.945 in boots, 0.92 barefoot (0.523 H) |
| Knee | 0.285 H = 0.502 | Winter | 0.515 in boots, 0.49 barefoot (0.278 H) |
| Ankle | 0.039 H = 0.069 | Winter | 0.080 above the sole of the boot |
| Shoulder joint | 0.818 H = 1.44 | Winter | 1.475 in boots, 1.45 barefoot (0.824 H) |
| Thigh / shank | 0.245 H = 0.431 / 0.246 H = 0.433; de Leva 0.427 / 0.439 | Winter; de Leva 1996 | 0.43 / 0.435 |
| Upper arm / forearm / hand | Drillis–Contini 0.186 / 0.146 / 0.108 H = 0.327 / 0.257 / 0.19; de Leva 0.285 / 0.272 | Winter; de Leva 1996 | 0.310 / 0.270 / 0.18 (upper arm between the two sources, forearm equal to de Leva) |
| Hip joint spacing | about 0.17–0.18 m (hip breadth 0.191 H = 0.336 outer) | Winter; ANSUR II | 0.18 |
| Shoulder joint spacing | biacromial 0.245–0.259 H = 0.43–0.455 outer, so the joint centres are about 0.38 | Winter; ANSUR II | 0.38 |
| Head | menton–vertex 232, length 197, breadth 152 mm | ANSUR II | head loft 0.235 tall, 0.151 broad, 0.19 long; beard and hair over it |
| Chest depth, bideltoid | 254 mm, 510 mm | ANSUR II | shirt loft depth 0.24–0.254, bideltoid 0.50 |
| Guitar (Martin D-18/D-28) | body 508, lower bout 397, waist 273, upper bout 292, depth 124 to 98 mm, scale 645, nut 43 mm, joined at the 14th fret, overall length about 1.03 m, mass 2.1 kg | published Martin specification (`guitar-spec`) | `BOUT` stations, `GUITAR` in `ref.ts`, exactly these values |
| Stool | 24 in = 610 mm seat height; round seat about 330 mm; four splayed legs; rungs at 250 and 330 mm; 3.4 kg | catalogue ranges of guitar stools | seat r 0.165, top 0.61, legs 19 to 15.5 mm splayed 0.085 to 0.19, rungs 11.5 mm |

## Audit (catalog state before round 3)

The rig:
- `stool` was a horizontal bone (angle 0) of 0.56 m: the seat was too low for a guitar stool.
- `hip-lift` was hung at y 0.65.
- Shoulder joints were at z ±0.218 (spacing 0.436 m, which is the outer biacromial width, not the joint centres).
- Hip joints were at ±0.105 (0.21 m).
- The shank was 0.42 m (knee 13 mm low).
- The torso was 0.460 m and the head bone 0.27 m.
- The guitar bone hung on the torso, pointed ahead in the sagittal plane at -58°, and was shortened to 0.78 m "for three-quarter projection". A real dreadnought is 1.03 m and is held across the body.

The poses: arms and hands did not reach the neck and the strings in 3D, and both feet rested on the rung.

The 2D drawings:
- They were constructed three-quarter views, not projections of one body.
- The guitar had a length of 0.78 m and was drawn from the side.
- The stool in the plan had three legs.

Other:
- `dimensions`: width 1.107 / depth 0.53 did not match a body holding a guitar across it.
- Volumes before: body 44/41/35/25, thermal 68/44/38/28, structure 41/41/32/26 solids (1:50/100/200/500).
- Kernel traps: the stool seat revolve had an implied closure, and the hair loft had self-intersecting sections.
- `model-findings` row: the guitar's closed sound box counted its inner faces in the thermal area.

## Root fixes (old -> new, written into the catalog)

Rig:
- `stool`: angle 0 -> 90, length 0.56 -> 0.61, centre of mass (0, 0.3) -> (0.45, 0).
- `hip-lift`: (x 0, y 0.65, angle 0, prismatic axis 90) -> (x 0.70, y 0, angle -90, axis 0).
- `torso` 0.4601 -> 0.475; `head` x 0.4601 -> 0.475, length 0.27 -> 0.255, centre of mass 0.135 -> 0.12, twist ±70° added.
- `upper-arm-l/r` z ±0.218 -> ±0.19.
- `thigh-l/r` z ±0.105 -> ±0.09.
- `shank-l/r` 0.4199 -> 0.435 (centre of mass 0.1872 -> 0.194); `foot-l/r` x 0.4199 -> 0.435.
- `guitar`: parent torso -> pelvis, origin (0.12, -0.17, 0) -> (0.0303, -0.2433, -0.1135) (the waist resting on the right thigh), angle -58 -> 0 (set by the poses), length 0.78 -> 0.74 (from the pivot to the head: the instrument is 1.03 m), joint ±25 -> ±180, centre of mass 0.2 -> 0.18.

Poses: all five are solved again with the product's `solveChain` (Levenberg–Marquardt in `roots.ts`).
- The right boot is on the front rung (rms 0.00 mm) and the left foot is on the floor (0.00 mm).
- The guitar axes are solved exactly.
- The left hand reaches its grip: rms 0.16–2.4 mm.
- The right forearm over the lower bout reaches its goal at 10–13 mm rms. This includes the soft elbow goal; the wrist itself sits within a few mm.
- New anchors: `rung-support`, `fret-hand`, `strum-hand`, `guitar-rest`, `seat`, `mouth`, `nostrils`, `crown`.
- `contact-l` and `heel-l` were added to every clip's contacts.

Other members:
- `dimensions`: height 1.495 -> 1.519, width 1.107 -> 0.931, depth 0.53 -> 1.00 (the body bounding box at 1:50 in the default pose; depth = the guitar across the body).
- `materials`: added cotton-jersey, rubber, steel and bone.
- `thermal.regions.stool.areaM2` 0.645 -> 0.515 (factor 1.25, within C16). Hand calculation for the new 24 in stool: seat 2 × π × 0.165² + edge 2π × 0.165 × 0.038 = 0.171 + 0.039 = 0.21; four legs 2π × 0.017 × 0.59 × 4 = 0.25; four rungs 2π × 0.0115 × about 0.3 × 4 = 0.087. Total about 0.55, of which the faces at the leg ends and joints do not count. Model 0.515.
- `summary`, `provenance.notes`, `provenance.references` (`guitar-spec` cited exactly; `winter-2009` and `ansur-2-2012` added), `structure.assumptions`, and `representations[*].decisions`: bilingual, rewritten for the new model. These texts had been written by the earlier author but not yet applied. I wired them into `finish()`.

2D (ruling E1): every view at every scale is redrawn by `draw2d.ts` from that scale's body in the default pose, with the same bones and roles.
- Stool, pelvis and trunk are flat outlines solved into the bone frame.
- Limbs are revolved radius profiles.
- Guitar parts are their outlines swept through their thickness.
- Head, hands and boots are the true outline as a plate in the bone.

Because of this, the other poses project in 3D. Checked visually: the default pose, `resting` (2D, 1:100), `tuning` and the `chord-change` clip at t 0.6 s (3D, 1:50).

Shape counts, before -> after:

| View | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| Elevation | 40 -> 58 | 23 -> 40 | 19 -> 29 | 16 -> 24 |
| Front | 44 -> 66 | 33 -> 41 | 31 -> 29 | 23 -> 24 |
| Plan | 45 -> 55 | 24 -> 40 | 23 -> 29 | 21 -> 24 |

## Modelled per scale and list (solid counts body / thermal / structure)

- **1:50 (95 / 91 / 126)**
  - Body:
    - The flannel shirt is a 12-section loft from the hem to the collar base, with collar band, T-shirt at the open neck, placket, buttons and two chest pockets with flaps.
    - Sleeves have a deltoid cap, elbow folds and buttoned cuffs.
    - Jeans: seat, thighs and shanks with round knees, a crease at the groin, and a stacked hem over the boots.
    - Boots: heel counter, instep, toe cap, heel block, arched sole and shaft.
    - Head: skull and jaw loft, nose, ears, eyes, brows, mouth, hair and beard.
    - Hands: palm, thenar, four fingers and thumb, shaped for the neck and for the strings.
    - Dreadnought guitar: body loft on 14 real bout stations with depth 124 to 98 mm, spruce top, sound hole, pickguard, bridge, neck loft, fingerboard, heel, head with a 14° break angle, six machine heads, and six slender strings.
    - Stool: revolved seat, four lofted splayed legs and four rungs.
  - Thermal: nested layer shells (skin/air/garment) for shirt, sleeves, jeans and boots; face, hair and hands; guitar and stool; emitters as outlets at the mouth, nostrils and crown.
  - Structure: skull, cervical to lumbar spine with its curvature, ribcage with ribs, scapulae, pelvis, humerus, radius and ulna, femur, tibia and fibula, hand and foot skeletons simplified; the guitar's body and neck as its own load path; stool seat, legs and rungs; joints at the rig joints; supports at the floor anchors, the seat and the rung; masses at the de Leva centres.
- **1:100 (46 / 43 / 95)**: mitten hands with thumbs, one-piece boots with toe caps, the guitar's fingerboard merged into the neck, the machine heads as one bar per side, a simplified ribcage, and the layers merged into one shell per garment.
- **1:200 (29 / 32 / 60)**: one form per segment, fists, a featureless head with hair, and the guitar as sound box, neck and head.
- **1:500 (24 / 27 / 42)**: trunk, limbs with the hands merged into the forearms, the guitar as sound box and one neck bar, and the stool with seat, four legs and the front rung.

Every coarser scale is authored anew (`volumes-copied` 0), and `decisions` are bilingual at every scale.

My own changes to the geometry this session: the 1:50 machine heads floated 6.5 mm off the head edge. I lengthened them along their shaft so they grow out of the head (centre at head half + 10.5 mm, half length 13.5 mm). I moved the 1:100 machine-head bars from b ±0.0495/0.0515 to ±0.0425/0.0445 so they touch the head instead of floating 5 mm away. The 2D was redrawn afterwards.

## Check

- `volumes-check.ts humans/seated-guitarist --product`: **PASS**, 42 rules ok.
- Silhouette IoU (elevation / front / plan):
  - 1:50: 0.972 / 0.974 / 0.977
  - 1:100: 0.978 / 0.983 / 0.974
  - 1:200: 0.978 / 0.983 / 0.979
  - 1:500: 0.979 / 0.985 / 0.973
- Cross-scale 1:100 IoU: 0.995 / 0.997 / 1.000.
- Thermal areas at 1:50 are all within limits: face 8.6 %, shirt-torso 10.2 %, sleeves 17 %, hands 4 %, jeans 18.5 %, boots 16 % (limit 30 %); guitar 8.1 % and stool 0.02 % (limit 15 %).
- `r3-validate.ts humans/seated-guitarist --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- The catalog file equals the candidate document byte for byte in content (`catalog == candidate True`).

## What I saw in the final renders

- **1:50 body (`r3-sheet` plus the earlier author's six-panel sheets, strumming pose)**
  - Front: proportions read right: head about 1/7.5 of the figure, shoulder line and narrower pelvis. The guitar lies across the body with the neck rising to his left; the right forearm is over the lower bout and the hand over the sound hole; the left hand holds the neck at the third fret. The face shows a beard, eyes and nose, and the shirt pockets and placket are visible.
  - Right side: the stool is vertical with its splay; the right boot is on the front rung and the left boot flat on the floor; the trunk leans slightly forward.
  - Back: the shoulder blades are visible under the shirt; the guitar head is to the left.
  - Left side: the hand on the neck, and the guitar's depth tapering to the heel.
  - Top: the neck reaches to the left and forward; the stool has four legs.
  - Three-quarter (turn 50 and turn 70): the machine heads now attach to the head; fingers wrap the neck; the cuffs and the knee folds read.
- **1:100, 1:200, 1:500 body**: each is recognisable at a glance and generalised as described. At 1:500 the neck and the rung are 75 mm bars, as the feature size requires.
- **Other poses**: `tuning` (the left hand at the pegs, trunk and head inclined) and `chord-change` at t 0.6 s are closed at every joint, with no visible interpenetration.
- **Structure at 1:50 and 1:100**: the skeleton reads anatomically (curved spine, ribcage, scapulae, pelvis, long bones). The guitar and the stool are their own members. Joints, supports and masses are in place.
- **Thermal at 1:50 (cut) and 1:100 (cut)**: the layer shells are nested inside the garments, face and hair are separate, and nothing pokes out of the body. At 1:100 the merged garment regions draw grey in the `r3-sheet` legend. That is only the sheet's colouring of region-without-layer entries, not a model defect.
- **2D**:
  - 1:50 (candidate before the final write): elevation, front and plan are true projections of the body.
  - 1:100 `resting` and 1:500 (catalog): the 1:500 figure is legible as a seated guitarist.

## Still imperfect (stated plainly)

- **Upper arm length.** The upper arm is 0.310 m, between de Leva (0.285) and Drillis–Contini (0.327), so the elbow stands about 25 mm higher than the 0.63 H table value. I kept it: it lies within the spread of the two cited sources, and it is threaded through four scales, the structure list and the solved poses.
- **Right-arm residual.** The solver leaves 10–13 mm rms on the right arm, mostly on the soft elbow goal. In the renders the forearm lies on the lower bout without a visible gap or penetration.
- **Print suitability at 1:1000.** At the far end of the 1:500 representation's range, the hair shape's smallest feature measures 0.193 mm against 0.2 mm. The catalog before round 3 also failed there (a stool leg gap of 0.046 mm). `validateAsset` does not report it. Not fixed.
- **Guitar outline.** The body is lofted on 14 stations with 9 points of rounding, so at 1:50 close-ups the bouts show faint facets. This is not visible at print size.

## Round 5 handcraft

Round 5 author, 2026-11-20/21. Scripts: the round 3 pipeline (`r3-seated-guitarist-*`) extended in place (roots, s50, coarse, lists, ref), plus `r5-seated-guitarist-measure.ts` (model-vs-reference table). Output under `🗑️generated/r5-seated-guitarist/` and `🗑️generated/r3-seated-guitarist/`.

### Reference and model vs reference (1:50, H = 1.76 m man; `bun r5-seated-guitarist-measure.ts`)

| quantity | reference mm | model mm | deviation | source |
|---|---|---|---|---|
| thigh (hip-knee joint) | 431 | 430 | -0.3 % | Drillis-Contini 0.245 H |
| shank (knee-ankle joint) | 433 | 435 | 0.5 % | Drillis-Contini 0.246 H |
| upper arm (shoulder-elbow joint) | 327 | 327 | -0.1 % | Drillis-Contini 0.186 H |
| forearm (elbow-wrist joint) | 257 | 257 | 0.0 % | Drillis-Contini 0.146 H |
| hand bone | 190 | 190 | -0.0 % | Drillis-Contini 0.108 H |
| hip joint to shoulder joint | 524 | 530 | 1.1 % | 0.818 H - 0.52 H |
| shoulder joint spacing | 380 | 380 | 0.1 % | biacromial 0.23 H less 2 x 12.5 mm to the joint centres |
| hip joint spacing | 180 | 180 | 0.0 % | Bell et al. 1990, about 0.17-0.18 m |
| head chin-vertex (head + hair solids, u) | 235 | 238 | 1.2 % | H / 7.5 |
| head breadth (w) | 152 | 152 | 0.0 % | ANSUR II bitragion/head breadth 152 mm |
| head length (v) | 197 | 194 | -1.5 % | ANSUR II glabella-opisthocranion 197 mm |
| hand length (palm + middle finger, u) | 190 | 150 | -21.3 % | 0.108 H (strumming hand flexed, so shorter) |
| boot length (heel to toe cap) | 298 | 305 | 2.5 % | foot 0.152 H + 30 mm of boot |
| guitar overall length | 1030 | 1030 | 0.0 % | Martin D-28: 1029 mm |
| guitar lower bout | 397 | 397 | 0.0 % | Martin D-28: 397 mm |
| guitar body depth at the tail | 124 | 124 | 0.0 % | Martin D-28: 124 mm |
| stool seat height | 610 | 610 | 0.0 % | 24 inch guitar stool |

The hand length line measures the strumming hand as posed (fingers flexed round the pick); stretched it is palm 100 mm + middle finger 43 + 26 + 21 mm = 190 mm (0.108 H). Head length 194 mm is the skull loft without the nose and the hair. Everything else is within 3 %.

### Grades (before -> after)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B (upper arm 5 % short, forearm 5 % long, hand 5 % short, boot toe 7 % long, guitar outline faceted on 14 stations, ear low on the jaw) -> A | A -> A | A- (bone lengths followed the short arm) -> A |
| 1:100 | B (same arm lengths, long boots) -> A | A -> A | A- -> A |
| 1:200 | B (same) -> A | A -> A | A- -> A |
| 1:500 | B- (same; hair a thin crescent below the printable feature at 1:1000; flat cut discs at elbow and knee ends) -> A- | A- -> A- | B (toe bones 44 mm, below the feature) -> A |
| poses | strumming, tuning (1:50), chord-change t 0.6 s: A -> A (re-solved on the new arm, closed) | | |

### Changes (root first)

- Rig (catalog): `upper-arm-l/r` 0.3101 -> 0.327 m (0.186 H), centre of mass 0.179 -> 0.1887 (de Leva 57.7 %); `forearm-l/r` x 0.3101 -> 0.327, length 0.27 -> 0.257 (0.146 H), centre of mass 0.1235 -> 0.1176; `hand-l/r` x 0.27 -> 0.257, length 0.18 -> 0.19 (0.108 H), centre of mass 0.09 -> 0.095. All five poses re-solved with the product's `solveChain` (left hand rms 0.8–2.2 mm, right arm 11.6–14.8 mm with its soft elbow goal; legs and guitar exact). Clips unchanged (they key the same bones).
- 1:50 body: sleeve restated on the new humerus with a deltoid cap (0.057 lateral at the shoulder, 0.056/0.051 down to the deltoid insertion, biceps belly forward), elbow fold at 0.266–0.304, forearm, cuff and wrist restated on the 0.257 m forearm, the palm lengthened to 0.108 with the knuckles at 0.1 so the hand is 0.19 m; ear moved up and back to the level between brow and nose base; dreadnought outline restated from the D-28 template at 29 stations (lower bout 397, waist 273, upper bout 292 mm, tail and shoulders rounded into the end blocks) instead of 14 faceted stations; boot toe cap 14 mm shorter (boot 305 mm over a 268 mm foot).
- 1:100 / 1:200 / 1:500 bodies: sleeves, forearms and hands restated on the new segment lengths; boots shortened the same; 1:500 hair restated as a full cap (domed, 5 sections) so it is a printable mass at 1:1000; 1:500 sleeve, forearm, thigh and shank ends rounded (no flat discs at elbow and knee).
- Structure: humerus 0.311 + condyles at 0.32, radius/ulna to 0.253/0.251, metacarpals to 0.098, masses at the de Leva centres of the new segments; 1:500 toe bones 78 mm (were 44 mm thin).
- 2D: every view of every scale redrawn by `r3-seated-guitarist-draw2d.ts` from the new bodies (ruling E1).

### Checks

- `r3-validate.ts --file <candidate> --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0); catalog after write and splice: VALID.
- `volumes-check.ts humans/seated-guitarist --product`: PASS (47.9 s). Silhouette IoU elevation 0.973 / 0.978 / 0.978 / 0.982 (1:50 … 1:500).

### Seen (opened)

1:50 body sheet and three-quarter close-up, 1:200 body, 1:500 body (sheet and close-up after the hair and joint ends), 1:50 structure, 1:50 thermal cut, pose `tuning` at 1:50, 2D 1:50 (catalog) and 1:500 (candidate). Rendered without error and not opened in this round: 1:100 body, structure 1:100–1:500, thermal cut 1:100–1:500, clip `chord-change` t 0.6 s.

### Still not perfect

- Shirt sleeves read as loose tubes from the elbow down (true to a flannel shirt, but the forearm form is hidden).
- 1:500: the guitar neck is a 76 mm bar (the smallest feature of the scale forces it), so it reads heavier than a real neck.
- Right forearm on the lower bout: the solver leaves 12–15 mm rms (soft elbow goal); no visible gap or penetration in the opened views.
- The ribcage at 1:50 structure is a stack of tilted hoops; it reads as ribs but the hoops are uniform in section.
