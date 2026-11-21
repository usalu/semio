# Round 3 — `humans/jogger`

Author scripts (all in the ticket folder): `r3-jogger-kit.ts` (constructors), `r3-jogger-figure.ts` (every section of body, thermal and skeleton, stated by hand per scale; writes `volumes/humans/jogger.json`), `r3-jogger-draw2d.ts` + `r3-jogger-draw2d.py` (2D redraw and its splice), `r3-jogger-roots.py` (rig, anchors, dimensions), `r3-jogger-splice.py` (member-wise writer), `r3-jogger-frames.ts`, `r3-jogger-bbox.ts`, `r3-jogger-probe2d.ts` (probes).

State: spliced into the catalog. `r3-validate.ts humans/jogger --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/jogger --product`: every rule green at every scale **except `volumes-thermal-coverage` at 1:50 and 1:100** (four declared areas that honest garments cannot meet, see "Still imperfect").

## 1. Reference (lean male runner, 1.82 m shod = 1.808 m barefoot + 12 mm sole, 72 kg)

| quantity | reference | value used | source |
|---|---|---|---|
| hip joint height | 0.53 H | 0.962 m | Winter, Biomechanics and Motor Control (segment lengths as shares of stature); brief |
| knee joint height | 0.285 H | 0.517 m | Winter |
| ankle joint height | 0.039 H + sole | 0.083 m | Winter |
| shoulder joint height | 0.818 H | 1.510 m | Winter (0.818 H = 1.489 m barefoot, 1.501 m shod) |
| C7 (neck base) height | 0.86 H | 1.575 m | de Leva 1996 (cervicale 242.9 mm below vertex at 1.741 m) |
| head height chin to vertex | H / 7.5 | 0.237 m | brief; ANSUR II menton to vertex |
| head breadth / length | 0.155 / 0.197 m | 0.155 / 0.196 m | ANSUR II male means |
| upper arm / forearm / hand length | 0.186 H / 0.146 H / 0.108 H (Winter); 0.162 H / 0.154 H (de Leva) | 0.321 / 0.279 / 0.186 m | Winter; de Leva 1996 |
| thigh / shank length | 0.245 H / 0.246 H | 0.445 / 0.434 m | Winter |
| hip joint centre spacing | 0.17 to 0.19 m | 0.18 m | Bell et al. 1990 (hip centre from inter-ASIS distance); other catalog humans 0.17 to 0.19 |
| shoulder joint spacing | biacromial 0.41 m minus 2 x 3 cm | 0.35 m | ANSUR II biacromial breadth |
| bideltoid breadth | 0.49 m mean, lean build less | 0.474 m | ANSUR II |
| chest breadth x depth | 0.32 x 0.24 m | 0.33 x 0.244 m (shirt) | ANSUR II, lean build |
| waist | 0.78 m girth | 0.304 x 0.208 m (shirt hangs loose) | lean runner |
| hip breadth x depth | 0.34 x 0.24 m (girth 0.94 to 0.96) | 0.348 x 0.236 m | ANSUR II |
| thigh girth top / mid / above knee | 0.54 / 0.50 / 0.38 m | sections 0.16 x 0.178, 0.142 x 0.156, 0.111 x 0.122 m | ANSUR II, lean |
| knee / calf / ankle girth | 0.37 / 0.37 / 0.22 m | 0.106 x 0.116, 0.108 x 0.124, 0.062 x 0.078 m | ANSUR II |
| upper arm / elbow / forearm / wrist girth | 0.30 / 0.26 / 0.27 / 0.17 m | 0.094 x 0.106, 0.084 x 0.082, 0.088 x 0.086, 0.041 x 0.058 m | ANSUR II |
| hand breadth / thickness | 0.086 / 0.028 m | 0.084 / 0.028 m | ANSUR II |
| foot / shoe length, shoe width, heel stack | 0.152 H = 0.275 m; EU 44 running shoe 0.30 m, 0.10 m, 30 mm | 0.30 m, 0.102 m, 32 mm | Winter; manufacturer size tables |
| segment masses and centres | de Leva 1996 | unchanged, two centres rescaled | asset `provenance.references` |
| body surface | DuBois: 1.924 m² | unchanged | asset |

## 2. Audit findings

Rig
- Hip joints: `thigh-l`, `thigh-r` had no lateral position (both hip joints on the midline; the old solids and drawings carried a 9 cm offset themselves; foot contact anchors of both feet coincided at z = 0). Root defect.
- Trunk too short, head-and-neck too long: `torso` 0.4757 m put C7 at 1.541 m (0.847 H; reference 1.575 m), only 31 mm above the shoulder joints (reference about 65 to 75 mm); `head` 0.2792 m against 0.245 m. Root defect.
- Hip, knee, ankle, shoulder, elbow, wrist heights, limb lengths, shoulder spacing (0.35 m), toe joint position: within 1 cm of the reference; kept.
- Head anchors `mouth`, `nostrils` lay 3 to 4 cm above the real mouth and nose base (mouth at nose height).

Dimensions: `heightM` 1.775, `widthM` 1.035, `depthM` 0.45 described the old body (bideltoid only 0.45 m with the armband, hands and shoes too long).

2D drawings (all 12): raster-traced outlines with stair steps (elevation 1:50 `torso` 270 points, `thigh-skin-l` 396 points) plus 26 `fit-*` patch shapes at 1:50 alone; front and plan a patchwork of slices (`foot-r-0..4`, `toe-r-0..2`, knee ellipses); different ids and tones for the same part in different views (`thigh-l` was shorts in the elevation and skin in the front).

Solids (old 1:50, 74 / 82 / 33): box torso with shoulder ball ellipsoids, tube limbs with cap ellipsoids, blade hands, slipper shoes, face of loose ellipsoids; skin running whole under every garment; plume a disc; structure = body lofts and blobs, at 1:500 a copy of the body.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[thigh-l].z` / `[thigh-r].z` | absent (0) | 0.09 / -0.09 |
| `rig.bones[torso].lengthM` | 0.4757 | 0.51 |
| `rig.bones[torso].mass.comX` | 0.2172 | 0.2329 (same de Leva ratio 0.4566) |
| `rig.bones[head].x` / `.lengthM` | 0.4757 / 0.2792 | 0.51 / 0.2449 (vertex stays at 1.82 m) |
| `rig.bones[head].mass.comX` | 0.1396 | 0.1224 (same ratio 0.5) |
| `anchors[mouth]` x, y | 0.1179, -0.0982 | 0.052, -0.1 (on the lips) |
| `anchors[nostrils]` x, y | 0.148, -0.1033 | 0.081, -0.112 (under the nose tip) |
| `anchors[crown]` x | 0.2876 | 0.2533 (same 8 mm above the vertex) |
| `dimensions.heightM` / `widthM` / `depthM` | 1.775 / 1.035 / 0.45 | 1.764 / 1.007 / 0.475 (default pose: pose height, stride, bideltoid breadth with sleeves) |
| manifest row `jogger` `heightM` / `widthM` | 1.775 / 1.035 | 1.764 / 1.007 |
| `representations[*].geometry.{elevation,front,plan}` | 437 shapes | redrawn, see below |

Poses and clips are angle-based and needed no change; default pose, `stretch-quad-r`, `stretch-hamstring` and `run-cycle` at 0.3 s were rendered after the change.

2D redraw (ruling E1): every filled shape is the orthographic outline of the body solids of its bone, Douglas-Peucker at 0.8 / 1.5 / 3 / 6 mm, 4 decimals. Points per scale (elevation / front / plan): 1:50 556 / about 490 / about 580 in 57 / 45 / 39 shapes; 1:100 about 262 / 265 / 320; 1:200 about 167 / 168 / 190; 1:500 about 118 / 117 / 140. One id, tone, region and material per part in all three views (`shirt`, `shorts`, `shorts-leg-*`, `thigh-*`, `shank-*`, `sock-*`, `shoe-*`, `shoe-toe-*`, `shoulder-*`, `sleeve-*`, `upper-arm-*`, `forearm-*`, `hand-*`, `armband`, `neck`, `head`, `hair`, `headband`, `ear-*`, `eye-*`); all `fit-*` patches and slice shapes are gone. Elevation: bone-local outlines (arm bones as `extruded w` solids, because the arms abduct in `stretch-stand`). Front: trunk, head, feet and hands as prisms of their lateral outline swept through their depth, limbs as revolved profiles. Plan: limbs revolved, feet and hands prisms, trunk and head flat outlines over a prism core. Detail strokes redrawn at the new landmarks: hems (shirt, sleeves, shorts), sock tops, and at 1:50 brow, mouth, nostril, jaw line, hair strands, sole lines, laces, shorts stripes, front nose, mouth and brows. Dropped because the model has no such part: shirt stripe and number, chest and waist fold strokes, finger lines (fingers are drawn forms now).

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 64 / 96 / 98.** Body: shirt as one garment loft (hem, waist, chest, shoulder slope, collar rib), shoulder caps and sleeves on the upper arms, neck, head with jaw and chin, nose ridge, ears, eyes, hair cap, head-band; shorts with gluteal masses and two legs; thighs, knee caps, shanks with the calf to the back, socks; shoes with separate midsole, upper and rocker toe; upper arms, forearms turning from elbow to wrist, cupped palms with four two-segment fingers and thumb; armband with pouch. Thermal: every region, nested shells for shirt, sleeves, shorts (2) and shoes (3) at the real layer thicknesses, face and bare neck as their own solids so the scalp is not counted as face, outlet patches under the nose, on the lips and on the crown. Structure: skull and facial block, cervical and thoraco-lumbar spine with curvature, 8 rib hoops, sternum, clavicles, scapulae with acromion, sacrum, hip bones, pubic rami, femur (head, neck, trochanter, bowed shaft, condyles), patella, tibia, fibula, talus, calcaneus, metatarsus, toe phalanges, humerus, crossed radius and ulna, carpus, metacarpus, finger and thumb bones; ball joints at hip and shoulder, hinge pins at knee, elbow, ankle, toes, discs at L5/S1 and C7/T1; masses of pelvis, trunk, thighs. No support solids: `structure.supports` is empty (inertial model).
- **1:100 — 29 / 34 / 57.** New, longer sections; nose, ears, eyes, knee caps, pouch left out; cap and sleeve one garment, one shoe form per bone, mitten hand with thumb. Skeleton with merged parts, 4 rib hoops.
- **1:200 — 22 / 28 / 33.** Head with neck, hair, shirt, shorts, shorts legs, thighs, shanks with sock, shoes, toes, sleeves, arms, mitten hands. Skeleton: skull with neck, spine, rib barrel, shoulder girdle bar, pelvis basin, one bone per limb segment, ten joints.
- **1:500 — 20 / 28 / 22.** Fewest masses; thermal splits sleeve and thigh at the garment edges. Structure is a stick skeleton of thickened bones (the feature rule asks 75 mm), not the body.

Final check line: `FAIL: 7 measurement(s) over 2 rule line(s)` — all seven are `areaDeviation` of `volumes-thermal-coverage` at 1:50 and 1:100. Silhouette IoU elevation / front / plan: 1:50 0.996 / 0.977 / 0.951, 1:100 0.994 / 0.976 / 0.954, 1:200 0.995 / 0.974 / 0.959, 1:500 0.999 / 0.985 / 0.967.

## 5. What I saw in the final renders

- `r3-sheet` 1:50 body, default pose. Front: head, neck and sloping shoulder line read as a person, eyes, nose and ears present, arms carried bent with cupped hands, shorts over tapering thighs, knee cap on the stance leg. Right and left side: forward lean, calf bulge behind a straight shin, heel-to-toe shoe with sole, forearm tapering to an upright wrist, thumb on top. Back: gluteal masses under the shorts, armband on the left arm. Top: shoulders, head with hair, forward thigh and trailing shank. Three-quarter: reads as a runner at first sight.
- 1:50 body in `stretch-quad-r`: knee stays closed at 142 degrees, hand reaches the held foot, no gaps at hip, shoulder or elbow.
- 1:50 structure (six views): recognisable skeleton, rib hoops tilted, pelvis with wings, femora running inward from the trochanters. Seen before the last edit to joints and hip bones; the changed joint solids (pins, balls, discs) were then seen only in the small panels of the 1:100, 1:200 sheets.
- 1:50 thermal cut (six views): shells nested, face red under the green hair, no shell outside the body.
- `volumes-render` sheets 1:50 (before the last refinements), 1:100, 1:200, 1:500: overlays almost entirely green; plan shows a thin red or blue rim at the trailing shoe and the back of the shirt.
- `r3-2d` 1:50 default pose (seen before the last front-view change): clean line drawings in all three views. `r3-2d` 1:50 `stretch-hamstring`: elevation right; front and plan show the folded trunk as a box.
- Rendered but not opened: `r3-sheet` 1:100 structure at `run-cycle` 0.3 s, the 1:200 and 1:500 three-quarter body views, `r3-2d` 1:200, and the `r3-2d` 1:50 sheet after the front limbs became revolved and the hair was split.

## 6. Still imperfect

- **Thermal coverage is red at 1:50 and 1:100.** Measured outer area against declared: shorts 0.406 against 0.182 m², bare legs 0.600 against 0.412, sleeves and armband 0.154 against 0.059, shoes 0.194 against 0.135 (1:50 only). The declared values are skin shares of the segments, not garment surfaces; C18 forbids me to change them and I did not shrink garments to fit. Filed in `author-requests.md` ("r3 humans/jogger") for the area reviewers.
- Grade as I see it: body 1:50 B+ to A-, not A. The shoulder caps still read slightly puffed in the front view, the shirt hem meets the shorts in a ragged intersection line, the shoes are slim, and the shirt has no folds beyond its loose hem.
- Lofts are superellipses, so pectorals, scapulae and separate calf heads are not modelled; the limbs are symmetric left to right within one section.
- 2D plan of trunk and head is a flat outline: exact in every running pose, but in `stretch-hamstring` (trunk folded 80 degrees) plan and front show the trunk as a box.
- `representations[*].decisions` texts still describe the old v1 drawing; not rewritten.
- Structure solids use material `skin` (the asset has no bone material; adding one needs thermal properties I have no source for).
- At 1:500 the feature rule forces members of 44 to 76 mm; the toe member is as thick as the toe cap.

## Round 5 handcraft (2026-11-21)

Scripts (ticket folder): `r5-runners-kit.ts` (spline lofts through hand-stated key sections, curved-path tubes, section frames for 2D), `r5-jogger-body.ts` (every key section of the body at 1:50, 1:100, 1:200, 1:500), `r5-jogger-figure.ts` (thermal, skeleton, decisions; writes `volumes/humans/jogger.json`), `r5-jogger-draw2d.ts` + `r5-jogger-draw2d.py` (2D redraw and splice), `r5-jogger-roots.py` (rig), `r5-runners-candidate.py`, `r5-jogger-run.sh` (candidate build and validation), `r5-runners-render.sh`, `r5-runners-crop.sh`, `r5-runners-print-probe.ts` (which pose/view carries a print finding).

State: spliced. `r3-validate.ts humans/jogger`: `VALID ... schema 0, validateAsset 0` (print clean at every scale). `volumes-check.ts humans/jogger --product`: every rule green except `volumes-thermal-coverage` 1:50 and 1:100 (the same four body-share areas as round 3, pending in `area-review.md`, C18/C27 forbid the author to change them).

### Reference (lean male runner, 1.808 m barefoot, 1.82 m shod)

| quantity | reference | source |
|---|---|---|
| hip / knee / ankle joint height | 0.53 / 0.285 / 0.039 H | Winter 2009, DIN 33402-2 |
| shoulder joint / elbow / wrist height | 0.818 / 0.630 / 0.485 H | Winter 2009 |
| upper arm / forearm / hand length | 0.186 / 0.146-0.154 / 0.108 H | Winter; de Leva 1996 |
| head height (chin-vertex) | H / 7.5 = 0.241 m | brief, ANSUR II |
| biacromial / bideltoid breadth | 0.23 H = 0.416 m / 0.47-0.49 m | ANSUR II |
| chest, waist, hip breadth | 0.32 / 0.28 / 0.34 m | ANSUR II, lean build |
| thigh mid / above knee, calf, ankle girth | 0.50 / 0.38 / 0.37 / 0.22 m | ANSUR II |
| upper arm / forearm / wrist girth | 0.30 / 0.27 / 0.17 m | ANSUR II |
| hand breadth / thickness | 0.086 / 0.028 m | ANSUR II |
| running shoe EU 44 | 0.30 x 0.105 m, heel stack 30 mm, toe spring 10-20 mm | manufacturer size tables |

### Model against reference (1:50, rest pose, barefoot heights)

| quantity | model | reference | deviation |
|---|---|---|---|
| hip joint | 0.950 m (0.525 H) | 0.53 H | -0.9 % |
| knee joint | 0.505 m (0.279 H) | 0.285 H | -2.0 % |
| ankle joint | 0.071 m (0.039 H) | 0.039 H | 0 % |
| shoulder joint | 1.498 m (0.829 H) | 0.818 H | +1.3 % |
| elbow | 1.162 m (0.643 H) (round 4: 0.651 H, +3.3 %) | 0.630 H | +2.0 % |
| wrist | 0.883 m (0.488 H) | 0.485 H | +0.7 % |
| upper arm / forearm / hand | 0.336 / 0.279 / 0.195 m (round 4: 0.321 / 0.279 / 0.186) | 0.336 / 0.264-0.278 / 0.195 | 0 / +0.4 (de Leva) / 0 % |
| head chin-vertex | 0.237 m | 0.241 m | -1.7 % |
| bideltoid (with shirt) | 0.482 m | 0.47-0.49 m | in range |
| trapezius slope acromion-collar | 24 degrees | 20-25 degrees | in range |
| thigh mid / above knee girth | 0.51 / 0.37 m | 0.50 / 0.38 m | +2 / -2.6 % |
| calf / ankle girth | 0.378 / 0.218 m | 0.37 / 0.22 m | +2.2 / -0.9 % |
| upper arm / forearm / wrist girth | 0.302 / 0.278 / 0.168 m | 0.30 / 0.27 / 0.17 m | +0.7 / +3.0 / -1.2 % |
| hand breadth / thickness | 0.082 / 0.027 m | 0.086 / 0.028 m | -4.7 / -3.6 % (closed hand, palm narrows when the fingers curl) |
| shoe length / width / heel stack | 0.309 / 0.106 m / 31 mm | 0.30 / 0.105 m / 30 mm | +3.0 / +1 / +3 % |

### Grades (body / thermal / structure)

| scale | before (round 4) | after |
|---|---|---|
| 1:50 | B / B+ / A- (pad shoulders, box shirt shelf, tube limbs with crease facets, blade hands, lumpy seat balls, slipper shoes, ragged shirt-shorts intersection) | A- / A- / A- |
| 1:100 | B / B+ / B+ | A- / A- / B+ |
| 1:200 | B / B / B | B+ / B+ / B+ |
| 1:500 | C (pencil-tipped hair, skin thigh reading as briefs) / B / B | B+ / B+ / B |
| 2D | B+ (pad shoulders in front) | A- (shoulder slope drawn as prism of its outline at every scale) |

### Changes

- Root: `upper-arm-*.lengthM` 0.3206 -> 0.336 (mass `comX` 0.185 -> 0.1939), `forearm-*.x` 0.3206 -> 0.336, `hand-*.lengthM` 0.1861 -> 0.195 (`comX` 0.0931 -> 0.0975). Poses and clips are angle based; all opened poses stay closed (stretch-quad-r: the hand still holds the shoe).
- Body rebuilt at all four scales with spline lofts through hand-stated anatomical key sections: shirt with flared hem over the shorts (ends at the hip instead of the crotch), waist, ribcage, chest, trapezius slope to the collar; deltoid caps that continue the slope; shorts with integrated gluteal mass (seat ellipsoids gone); thigh with quadriceps and low vastus medialis; calf set back with the medial head lower; wrist and ankle girths to ANSUR; shoe with flared midsole, heel counter, collar, instep and toe spring; nose as a bridge-to-tip loft; curled running hand (palm, grouped curled fingers along a path, thumb on the index finger) at 1:50, curled mitten plus thumb at 1:100, mitten at 1:200; 1:500 split into shorts leg and thigh so the shorts read, blunt hair dome.
- Thermal follows the new body (sleeve/shoulder split at the sleeve seam re-cut from the 1:100 and 1:500 sleeve rows); 1:500 no longer splits the thigh.
- Structure: humerus restated for the 0.336 m upper arm at every scale, finger phalanges and 1:100 finger row curled with the hand.
- 2D: all 12 views redrawn from the new bodies (ruling E1); shoulder caps drawn as prisms of their front outline instead of revolved blocks; hems, sock tops, sole and toe lines, laces, nostril and front nose moved to the new forms; at 1:500 the shorts legs lie under the trunk shorts in the elevation and the 1:500 shorts reach 2 cm lower so their band stays printable in `stretch-hamstring` (front view).
- Decisions rewritten (en/de) for the four bodies and the thermal splits.

Silhouette IoU elevation / front / plan: 1:50 0.995 / 0.975 / 0.958, 1:100 0.994 / 0.976 / 0.963, 1:200 0.996 / 0.976 / 0.966, 1:500 0.992 / 0.975 / 0.982.

### Looked at (opened)

`r3-sheet` 1:50 body (full sheet, enlarged front-top crop, three-quarter close view and hand crop), 1:50 structure (right side crop), 1:50 thermal cut, 1:100 and 1:500 body; `r3-2d` 1:50 (and front crop) and 1:500; poses at 1:50: initial-contact-l (default), loading-l, midstance-l, propulsion-l, toe-off-l, flight-l, stretch-stand, stretch-quad-r, stretch-hamstring and `run-cycle` t 0.2 s. Rendered but not opened: 1:200 body, structure and thermal of 1:100-1:500, initial-contact-r, `r3-2d` 1:100 and 1:200. The -r poses mirror the opened -l poses and were not opened one by one.

### Remaining issues

- Thermal coverage red at 1:50/1:100: arm-clothing 0.174 vs 0.059 m², leg-clothing 0.424 vs 0.182, leg-skin 0.605 vs 0.412, shoes 0.199 vs 0.135 (1:50). Declared values are skin shares, not garment surfaces; listed in `area-review.md`, not forced (C18, C27).
- In `stretch-hamstring` the shirt hem stands off the back of the shorts as a short ledge (rigid trunk solid over a folded pelvis).
- One straight trunk bone (curvature lives in the structure list and the lofts); fingers grouped at 1:50 as the brief allows; no cloth folds.

The thirteen projected jogger contours at 1:50 triggered an empty polygon union. The repaired slab stitching retains separate tread corners, and the independent polygon-clipping reference measures 0.008379514862238601 m². All fourteen polygon union tests and all twelve nominal and band-end jogger print samples passed.
