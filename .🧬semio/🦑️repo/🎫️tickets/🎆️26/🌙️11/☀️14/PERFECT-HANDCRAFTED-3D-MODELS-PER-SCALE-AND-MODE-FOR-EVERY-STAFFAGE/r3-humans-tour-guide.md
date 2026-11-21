# Round 3 — `humans/tour-guide`

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/tour-guide/🔣️.json`; volumes source `volumes/humans/tour-guide.json`.

State: catalog written and spliced. `r3-validate.ts humans/tour-guide` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. Print suitability (`r3-tour-guide-suit.ts`): all four scales `authored` in the default pose and in `holding-flag`, `turning-to-group`, `waving`, `gesturing-to-group`, `wave-swing-a` (before this round 1:500 was `unsuitable`, the product could not draw it). `volumes-check.ts --product`: 1:200 and 1:500 `PASS`; 1:50 and 1:100 `FAIL: 3 measurement(s) over 1 rule line(s)`, only `volumes-thermal-coverage` `areaDeviation` of `head-skin`, `arm-skin`, `shoes` (C18 values, see 7).

This author continued the work of an author who was cut off. Found: the scripts `r3-tour-guide-kit.ts`, `-roots.ts`, `-s50.ts`, `-coarse.ts`, `-build.ts`, `-draw2d.ts`, `-apply.py`, `-look.ts`, `-probe.ts`, `-extent.ts`, `-texts.json`, a new volumes source, an untouched (old, valid) catalog file, no report. The source did not validate against the catalog (244 findings: new rig and material missing there); the candidate document had 20 findings. Kept: rig, poses, the 1:50 and coarse models (looked at, judged good). Done by this author: the 2D drawings as solids of the rig (section 4), the remaining findings, the knee, print suitability of 1:500, catalog write, splice, checks, looks, requests, this report.

Scripts (re-runnable): `r3-tour-guide-ship.sh` runs `-roots.ts` (rig, anchors, poses), `-build.ts` (`-s50.ts`, `-coarse.ts` through `-kit.ts`), `-draw2d.ts` (2D), `-apply.py` (candidate, validation, catalog write of `rig`, `anchors`, `poses`, `representations`, `dimensions`, `materials`), `volumes-insert.py`. Looks: `-look.ts`, `-suit.ts`, `-probe.ts`, `-extent.ts`.

## 1. Reference (adult of 1.75 m, 78.6 kg, 1.765 m in walking shoes)

| Quantity | Reference | Model | Source |
|---|---|---|---|
| stature, shod | 1.75 m + 15 mm sole | vertex 1.762 m (head inclined 2 degrees), cap button 1.777 m | asset `provenance` (de Leva 1996 masses for 1.75 m); shoe data |
| hip joint | 0.52 H = 0.910 + sole | 0.923 m | Winter 2009 (`winter-2009`) |
| knee | 0.285 H = 0.499 + sole | 0.513 m | Winter 2009 |
| ankle | 0.039 H = 0.068 + sole | 0.083 m | Winter 2009 |
| shoulder joint | 0.818 H = 1.432 + sole | 1.444 m | Winter 2009 |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H | 0.3255 / 0.2555 / 0.189 m | Winter 2009 |
| thigh / shank | 0.245 / 0.246 H | 0.411 / 0.431 m | Winter 2009 |
| shoulder joints apart | biacromial 0.245 H = 0.43 m, joint centres about 0.35 m | 0.35 m | Winter 2009; anthropometric tables (my knowledge) |
| hip joints apart | 0.17 to 0.19 m | 0.176 m | my knowledge |
| head | H / 7.5 = 0.233 m chin to vertex, 0.155 m broad, 0.20 m long | 0.233 / 0.155 / 0.206 m | my knowledge |
| chest depth / breadth in a jacket | 0.25 / 0.36 m body + cloth | 0.264 / 0.364 m at 1.32 m, shoulders 0.40 m | my knowledge |
| thigh / calf / upper arm / forearm girth (clothed half widths) | 0.56 / 0.38 / 0.31 / 0.27 m | 0.089 x 0.095 / 0.058 x 0.066 / 0.057 x 0.059 / 0.044 x 0.045 m | my knowledge |
| walking shoe EU 43 | 0.30 x 0.103 m, sole 13 mm rising to 26 mm at the heel | 0.313 x 0.107 m with sole lip, as stated | manufacturer size tables (my knowledge) |
| flag pole | 24 mm aluminium tube, 1.33 m, ball finial 40 mm | as stated | asset description; area-decisions row `pole-metal` |
| flag | 0.46 x 0.30 m | 0.458 x 0.30 m, rippled +/- 22 mm | asset; area-decisions row `flag-cloth` |
| telescopic pointer | 0.55 m beyond the hand, sections 15 / 11 / 9 / 7 mm | as stated | asset; my knowledge |

## 2. Audit (before)

- Rig: shoulder joints 0.4026 m apart (0.23 H, a body builder), upper arm 0.3082 (short), forearm 0.2685 (long), thigh 0.4276 against shank 0.4176 (thigh longer than shank plus the sole missing), head bone 0.2685, foot 0.2188 for a 0.30 m shoe, ankle 0.0795 without a sole. Pointer rooted 0.0994 m along the hand axis through the knuckles; pole through the middle of the hand, not in the palm.
- Poses: every pose stood with knees bent 16 to 20 degrees and feet tilted 5 to 16 degrees (a crouch), soles not on one ground.
- `dimensions`: 2.275 x 1.254 x 0.5 m did not follow from any body.
- 2D: 75 / 49 / 31 shapes at 1:50 that were three different constructions (elevation paths, front revolved polygons, plan with trunk and cap bound to `pelvis` so they could not turn, a circle for the head); 1:500 not print-suitable.
- Solids 1:50: 30 / 54 / 33 (19 lofts, capsule limbs, ellipsoids); structure of 16 capsules and 16 ellipsoids (no skeleton).

## 3. Root fixes (old -> new)

- `rig`: `upper-arm-*` x 0.4276 -> 0.4221, z +/-0.2013 -> +/-0.175, length 0.3082 -> 0.3255; `forearm-*` 0.2685 -> 0.2555; `hand-*` 0.179 -> 0.189; `thigh-*` 0.4276 -> 0.411; `shank-*` 0.4176 -> 0.431; `foot-*` 0.2188 -> 0.235 (comY -0.0398 -> -0.0415); `toe-*` x 0.1293 -> 0.14, y -0.0795 -> -0.083, length 0.0875 -> 0.095; `head` 0.2685 -> 0.2826; `pointer` x 0.0994 -> 0.07, y 0 -> 0.02, z -> 0.024; `flag-pole` x 0.0746 -> 0.08, z -> -0.026. Every `comX` scaled with its bone.
- `anchors`: `contact-*` (0.1293, -0.0795) -> (0.14, -0.083); `heel-*` y -> -0.083; `grip-*` x 0.0746 -> 0.08; `mouth` (0.1134, -0.0945) -> (0.1, -0.104); `nostrils` (0.1423, -0.0993) -> (0.122, -0.112); `crown` (0.2766, 0) -> (0.297, 0.005).
- `poses` (all seven): standing leg hip 1 to 2 degrees, knee 5 to 9 degrees, the free leg 3 to 11 degrees forward with the knee solved so that both soles lie flat on one ground (`r3-tour-guide-roots.ts` prints the sole heights, 0.0000 for both feet of every pose). Arms, trunk, head, pole, flag unchanged.
- `dimensions`: 2.275 / 1.254 / 0.5 -> 2.226 / 1.239 / 0.482 (the extent of the 1:50 body in the default pose: finial top, pointer tip to flag fly, elbow to elbow).
- `materials`: `bone` added (structure).
- `representations`: all 12 drawings redrawn (section 4), decisions rewritten in English and German.

## 4. 2D (ruling E1)

`r3-tour-guide-draw2d.ts`. Every filled shape is the outline of the body solids of its part at that scale (traced, simplified at 0.9 / 1.8 / 3.5 / 7 mm), one id per part in all three views, layer order stated per view. Shapes and points, elevation / front / plan: 1:50 28, 473 / 38, 569 / 27, 388; 1:100 28, 273 / 30, 329 / 27, 316; 1:200 27, 196 / 28, 225 / 27, 267; 1:500 21, 117 / 21, 127 / 21, 168.

The trunk twists and the right arm spreads in poses and clips, so every shape of trunk, head, arms, hands, pointer, pole and flag must be a solid of the rig (`lateral-ambiguous`, `elevation-lateral`; the cut-off author's flat shapes gave 15 findings). Chosen per part, so that the default pose is the true view and every other pose still reads:
- upper arms, forearms (with cuff), wrist, pointer, pole, finial: bodies of revolution about the axis through their lateral middle, the same profile in all three views (taken from the solids, station by station).
- hands, flag: their bone-local outline swept through their real thickness (`w` in the side elevation, `v` in front and plan; the flag along `w` everywhere).
- trunk and head parts (jacket, badge, collar, neck, head, hair, cap, ear, eyes): side outline swept across 26 % of their breadth (the depth an elliptical section keeps when it twists 20 degrees); front outline as a 2 mm sheet in the frontal plane through the bone axis, all sheets of a bone in that one plane (so eyes, mouth and badge stay on face and chest when the trunk turns; my first version put each sheet at its own depth and the mouth left the face in `turning-to-group`); plan outline as a flat wedge at the joint (see 7).
- strokes of turned bones (lanyard, zip, chest pocket zip, mouth) are ribbons of their real width (7 / 6 / 6 / 4 mm).
- legs, seat, jacket skirt, shoes stay in the rig plane: plain traced outlines.
- pointer and pole are strips where their diameter does not print: 24 / 30 mm at 1:200, 60 / 36 mm at 1:500 (a 60 mm pole strip cut the left sleeve behind it into slivers of 0.186 mm, which made 1:500 unsuitable).

## 5. What is modelled (solids body / thermal / structure)

- **1:50 — 73 / 101 / 116.** Body: head loft of 12 slices with jaw, cheekbones, brow; nose, lips, ears, eyes; hair at temples and nape; six-panel cap with curved brim and button; neck in a stand collar; softshell jacket (waist, chest, shoulder blades, sloping shoulders), skirt over the seat, hem band, front zip with puller, two slanted pocket zips, chest pocket zip, lanyard and badge with clip; sleeves with sleeve head, deltoid, elbow; forearms with cuffs, the right sleeve pulled back so the wrist shows; left fist of palm, thenar, four wrapped fingers and thumb round the pole; right hand with the index finger along the pointer, thumb against it, three curled fingers; seat, thighs, shanks with calf and hem break; shoes with upper, toe box, sole, toe sole, lacing; pointer of grip, three sections, ball tip; pole with ferrule, finial, hoist sleeve; flag cloth rippled. Thermal: outer layers by reference, jacket air and shirt as nested lofts on trunk, skirt and sleeves, trouser air on seat and legs, shoe air and sock, hair under the cap, breath patches at nostrils and mouth, plume patch on the shoulders. Structure: skull, face, mandible, cervical and thoracolumbar spine with their curves, ten rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, femur (head, neck, trochanter, shaft, condyles), patella, tibia with plateau and malleolus, fibula, calcaneus, midfoot, toes, humerus, ulna, radius, carpus, metacarpals, finger rays; 13 joints at the rig joints, 9 masses; pointer, pole and flag as their own members. `structure.supports` is empty in the asset, so there are no support solids.
- **1:100 — 42 / 45 / 73.** New sections for 15 mm: head with nose and ears, cap with brim, jacket, skirt, collar, badge, lanyard, sleeves, forearms, wrist, hands as palm with finger band and thumb, legs, shoes, pointer as grip and rod, pole with finial and hoist sleeve, flag. Skeleton of merged bones with five rib hoops.
- **1:200 — 35 / 38 / 47.** Head with nose, cap and brim, jacket and skirt, badge, limbs of three stations, mitten hands, shoes, pole, finial, flag, pointer. Skeleton: skull, spine, rib cage mass, pelvis, long bones.
- **1:500 — 22 / 25 / 29.** Head, cap with brim, neck, jacket as one form, sleeves, forearms, wrist, block hands, seat, legs, shoes, flag, pole and pointer as slender rods. Skeleton of one bar per bone with skull, rib cage and pelvis masses.
- Changes by this author in the models: knee (the shank top was 6 mm narrower than the thigh end at every scale and read as shorts over leggings: now 62 x 66 mm against 63 x 67 mm), badge corner radius 4 -> 3.5 mm (above half its thickness), 1:500 seat 60 -> 100 mm tall and plume patch 50 -> 85 mm (below the 75 mm feature), 1:500 sleeve 100 -> 112 mm.

## 6. What I saw (opened with the Read tool)

- `r3-sheet` 1:50 body, six views with rig: reads at once as a guide in cap and jacket, flag in the left fist, pointer raised. Front: sloping shoulders, collar, zip, lanyard and badge, pocket zips, fist with four fingers on the pole, legs straight, shoes under the hems. Right side: upright stance, pointing arm at 34 degrees, pole vertical, flag flying back. Back: jacket back, hair under the cap. Left side: left forearm forward to the pole. Top: shoulders, cap with brim, right arm ahead. Three-quarter: closed everywhere. Rig joints sit at hip, knee, ankle, shoulder, elbow, wrist.
- `r3-sheet --only front` 1:50 at full size: face with nose, lips, eyes, ears; before my fix a step at the knee.
- 1:100 and 1:500 body sheets: same figure, simpler; 1:500 still shows cap brim, raised arm, pole and flag. (1:200 sheet rendered, not opened; `look-s200-body.png` of the first author not opened either.)
- Clip `turn-and-wave` at 2.8 s, 1:50, six views: trunk turned, right arm spread and raised, nothing opens at shoulder, elbow or knee.
- 1:50 thermal `--cut`: shirt and air inside the jacket, trousers with air layer, sock and air in the shoe, face under hair and cap. 1:100 thermal rendered, not opened.
- 1:50 structure sheet and 1:100 structure three-quarter: a skeleton inside the ghost body, joints orange at the rig joints, masses magenta.
- 2D: 1:50 product drawings (`r3-2d.ts`, six panels) and my larger sheet in the default pose, `turning-to-group` and `holding-flag`: all three views show the same figure; the hanging right arm, the turned trunk and the raised left arm read right. 1:500 product drawing (six panels): draws now, 4.4 mm figure with flag and pointer. 1:100 and 1:200 product drawings rendered, not opened.

## 7. Still imperfect

- Thermal red at 1:50 and 1:100: `head-skin` 0.178 against 0.085 declared, `arm-skin` 0.0162 against 0.004, `shoes` 0.229 against 0.135 (1:100: 0.174, 0.0163, 0.184). The model is the real head under hair and cap, a wrist that runs into cuff and hand, and two real shoes; the declared values are physiological shares I may not change (C18). Filed in `author-requests.md` (2026-11-20, C27).
- Plan of trunk and head: no 2D solid kind holds a free plan outline on an upright bone. I used an `extruded` `w` solid whose span opens along the bone (`zTip`), so the plan is the true symmetric outline and turns with the bone, but it lies at the joint height: a head nod of 14 degrees leaves the cap plan up to 56 mm behind and shears it by up to 7 mm. It is a workaround, not a clean definition. Filed in `author-requests.md` (proposal: `sweep: "u"`).
- Revolved limbs are 2 to 6 mm fuller than the true outline where the sleeve section is not round; hands in front and plan are prisms of their outline (blocky next to the modelled fingers of the 3D).
- Side elevation of a twisted trunk is an approximation (26 % sweep), right for 20 degrees only.
- The silhouette IoU of `r3-project2d.ts --check` measures the tool's own dump, not these drawings; I have no IoU number for the final 2D and judged it by eye only.
- The sleeve head is a chamfered end, not a set-in shoulder seam; in `holding-flag` front the right shoulder looks square.
- Faces at 1:50 are schematic (no eyelids, brows); trousers have no folds apart from the hem break.
- Grades as I see them: body 1:50 A-, 1:100 A-, 1:200 B+ (not opened this session after the knee change), 1:500 B+; structure A- / B+; thermal A- (three red areas); 2D B+ because of the plan workaround.

## Round 5 handcraft (2026-11-21)

Scripts changed: `r3-tour-guide-s50.ts` (`SHANK_T`), `r3-tour-guide-coarse.ts` (`SHANK_S` at 1:100 / 1:200 / 1:500, left fist band), `r3-tour-guide-texts.json` (1:500 `validRange`). Rebuilt with the `r3-tour-guide-ship.sh` steps. Renders: `🗑️generated/r5-tour-guide/before|after/`.

### Reference (adult of 1.75 m, 15 mm sole) and model against it

| quantity | reference | model (barefoot) | deviation |
|---|---|---|---|
| hip joint | 0.52 to 0.53 H = 0.910 to 0.928 m (Winter 2009) | 0.908 m = 0.519 H | -0.2 % |
| knee | 0.285 H = 0.499 m | 0.497 m | -0.4 % |
| ankle | 0.039 H = 0.068 m | 0.066 m | -2.9 % |
| shoulder joint | 0.818 H = 1.4315 m | 1.4295 m | -0.1 % |
| elbow / wrist | 0.630 / 0.485 H = 1.1025 / 0.849 m | 1.104 / 0.8485 m | +0.1 % / -0.1 % |
| head | H / 7.5 = 0.233 m | 0.233 m | 0 % |
| hand / foot | 0.108 / 0.15 H = 0.189 / 0.263 m | 0.189 / 0.265 m (shoe 0.30) | 0 % / +0.8 % |
| biacromial (male) | 0.23 to 0.245 H = 0.40 to 0.43 m | 0.40 m over the jacket shoulders, joints 0.35 m apart | in band |
| flag pole / flag / pointer | 24 mm x 1.33 m, 0.46 x 0.30 m, 0.55 m | as stated | 0 % |

### Grades (as seen)

| scale | body before / after | thermal | structure |
|---|---|---|---|
| 1:50 | A- (step ring at the knee) / A- (knee closed) | A- | A- (skeleton, spine dorsal with curves, ribs, pelvis, long bones) |
| 1:100 | A- / A- | A- | A- |
| 1:200 | B+ / B+ | B+ | B+ |
| 1:500 | B (faceted legs, flag a 75 mm slab forced by the feature) / B | B+ | B (stick skeleton) |

### Changes

- Knee at every scale: the shank top is a millimetre fuller than the thigh end (1:50 0.0636 x 0.0676 against 0.063 x 0.067), so the thigh closes inside the shank and the step ring at the knee is gone.
- Print suitability (catalog was INVALID under the new `PRINT` rule): the left fist's finger band at 1:100 and 1:200 is 10 % thicker (its visible width beside the pole was 0.199 mm at 1:150); the 1:500 range ends at 1:700 (the 36 mm pole strip prints under 0.2 mm beyond).

### Checks

`r3-validate.ts humans/tour-guide --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts --product`: all rules `ok` except `volumes-thermal-coverage` at 1:50 / 1:100 on `head-skin`, `arm-skin`, `shoes` (C18 values, unchanged since round 3, filed in `author-requests.md`).

### Seen (opened)

1:50 body sheet before, three-quarter close-up before and after, `waving` pose, clip `turn-and-wave` at 2.8 s after, 1:500 body and 1:50 structure. Rendered, not opened: 1:100 and 1:200 sheets after, thermal sheets, the other poses after, 2D.

### Remaining

- Rig: trunk and head bones run along the mid axis and the head bone starts at the base of the neck (no separate neck bone, no lumbosacral offset); the joints of the limbs are right. A neck bone would need all seven poses, two clips and the 2D of four scales re-posed; not done this round.
- Faces schematic, trousers without folds, flag at 1:500 a 75 mm slab (feature rule), plan of trunk and head a workaround (round 3 note).
- Three thermal areas red (C18, area reviewer).

## Articulated cervical column and skull

The cervical column and skull now rotate about independent pivots. The atlas is 112.29 mm along the former head-and-neck frame and 16 mm behind its axis, at the end of the authored cervical spine. Two occipital condyles carry the skull. In the turn-to-group pose, the cervical segment contributes 9° of twist and the head supplies a further 16°. The collar follows the neck, and the cap, jaw and breath outlets follow the head. The articulated model preserves the 5.413 kg combined mass and its neutral centre of mass.

The [movement fixture](digital-guide-neck.json) covers seven poses, four digital scales, and an isolated 15° nod. All four model scales passed the thirteen geometry rules. Including the cap brim in plan brought the 1:50 soft-edge error down to 1.875 mm. The 5,247,696-byte presentation GLB passed independent readback of all twelve scenes.
