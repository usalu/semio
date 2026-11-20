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
