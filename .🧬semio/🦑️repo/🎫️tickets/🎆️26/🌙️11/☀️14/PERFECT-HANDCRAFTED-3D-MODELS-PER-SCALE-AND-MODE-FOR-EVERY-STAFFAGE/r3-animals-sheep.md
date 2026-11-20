# Round 3 — `animals/sheep` (2026-11-19)

Scripts (all in the ticket folder): `r3-sheep-rig.py` (rig, anchors, poses), `r3-sheep-io.py` (member writer), `r3-sheep-kit.ts`, `r3-sheep-50.ts`, `r3-sheep-coarse.ts` (hand tables of every solid), `r3-sheep-2d.ts` + `r3-sheep-2d.py` (2D redraw), `r3-sheep-run.sh` (whole pipeline), `r3-sheep-look.sh`, `r3-sheep-pose.ts`, `r3-sheep-bbox.ts`, `r3-sheep-frames.ts`. Input kept for re-runs: `🗑️generated/r3-sheep/asset-before.json` (the asset before this round, read by `r3-sheep-rig.py`).

## 1. Reference (70 kg wool-breed ewe, standing, full fleece)

Sources: own knowledge of sheep anatomy after Ellenberger/Dittrich/Baum "Handbook of Animal Anatomy for Artists", Goldfinger "Animal Anatomy for Artists", Nickel/Schummer/Seiferle "Lehrbuch der Anatomie der Haustiere" (bone lengths and joint heights of sheep), breed descriptions of Merinolandschaf and Texel ewes (withers 0.68 to 0.76 m, trunk length 0.76 to 0.84 m at 70 to 80 kg), the asset's own summary (0.74 m withers in fleece, 60 mm staple). No web search was run; the values are textbook orders of magnitude, not measurements of one animal.

| landmark / size | reference | model (world, x ahead of the hip joint, y up) |
|---|---|---|
| withers, skin / fleece top | 0.70 / 0.74 to 0.76 m | spinous tips 0.69, fleece top 0.75 |
| trunk length, point of shoulder to pin bone | 0.76 to 0.84 m | 0.80 (pin bone x -0.09, shoulder point 0.71) |
| chest depth | 45 % of withers, about 0.31 m | skin 0.70 to 0.39; fleece 0.757 to 0.36 |
| fleece barrel width | belly 0.33 m + 2 x 0.055 m | 0.443 |
| hip joint height | 0.58 to 0.60 m | 0.590 |
| femur / stifle height | 0.18 to 0.20 m / 0.42 to 0.45 m | 0.193 / 0.425 |
| tibia / hock joint height | 0.21 to 0.24 m / 0.25 to 0.28 m | 0.215 / 0.262 |
| tarsus + metatarsal / fetlock height | 0.17 to 0.18 m / 0.08 to 0.09 m | 0.177 / 0.085 |
| pastern and claw, fetlock to toe tip | about 0.105 m, dorsal wall 50 to 55 degrees | 0.110 hind, 0.107 fore, 50.5 degrees |
| scapula, inclination | 0.17 to 0.19 m, 55 to 60 degrees | 0.179, 60 degrees |
| shoulder joint height | 0.52 to 0.55 m | 0.530 |
| humerus / elbow height | 0.14 to 0.16 m / 0.40 to 0.42 m | 0.150 / 0.403 |
| radius / carpus height | 0.16 to 0.18 m / 0.21 to 0.23 m | 0.182 / 0.222 |
| metacarpal | 0.125 to 0.145 m | 0.137 |
| cannon girth | 8 to 9 cm | fore 29 x 27 mm, hind 31 x 26 mm |
| lumbar / thoracic / cervical spine | 0.23 to 0.26 / 0.32 to 0.35 / 0.30 to 0.33 m | 0.240 / 0.336 / 0.310 |
| skull length, depth, width | 0.23 to 0.26, 0.14 to 0.15, 0.11 to 0.125 m | 0.235 (+ lip), 0.145, 0.124 |
| ear | 0.10 to 0.12 x 0.05 to 0.06 m, carried horizontally | 0.105 x 0.054 |
| claw | 55 x 20 x 33 mm, two per foot, track 0.18 to 0.21 m | 55 x 20 x 33, fore track 0.18, hind 0.208 |
| docked wool tail | 0.15 to 0.20 m | 0.19 |

## 2. Audit of the state before this round

- Rig: fetlocks at 0.048 m (real 0.085), hock at 0.197 m (real 0.26), carpus at 0.188 m (0.22), stifle 0.407 (0.425); hoof bones 0.060 m (0.107 to 0.110); forearm 0.200 (0.182); femur 0.210 (0.193); jaw bone 0.12 (0.20). The spine chain was one horizontal line at y 0.691 (the top line of the back, not the vertebral axis), lumbar 0.293 and thorax 0.280 (real 0.24 and 0.34), neck base on that line 0.12 m too high, neck only 14 degrees above the horizontal.
- Default pose: the far hind hoof floated 39 mm above the ground, far legs staggered 0.17 and 0.20 m.
- 2D: fleece drawn as a box with a flat end, hooves as 60 mm wedges about three times the real claw, head a short wedge; front and plan constructed, not projections.
- Solids: 123 body solids, of which 92 "lock" ellipsoids over a barrel; thermal plume disc; structure of capsules with joint balls larger than the members, body material for bones.
- `dimensions` (1.27 x 0.823 x 0.44 m, 70 kg): consistent with the reference; not changed.

## 3. Root fixes in the asset file

- `rig.bones` (old to new length in m): pelvis 0.1249 to 0.1235, lumbar 0.2931 to 0.2401, thorax 0.28 to 0.3359, neck 0.31 to 0.3095, head 0.22 to 0.235, jaw 0.12 to 0.2052, ears 0.09 to 0.105, tail 0.10/0.09 to 0.1003/0.0886, thigh 0.21 to 0.1929, shank 0.22 to 0.2149, metatarsus 0.15 to 0.1773, hind hoof 0.062 to 0.1101, scapula 0.17 to 0.1792, humerus 0.17 to 0.1501, forearm 0.20 to 0.1816, metacarpus 0.14 to 0.137, fore hoof 0.06 to 0.107. All origins and rest angles follow the landmarks of section 1 (table `J` in `r3-sheep-rig.py`); the spine chain is now the vertebral axis (lumbosacral joint 0.105/0.655, thoracolumbar 0.345/0.660, C7/T1 0.670/0.575, poll 0.907/0.774). Centres of mass moved with the bones (`comX` at mid bone for limbs, hand-placed for the trunk). Joint limits widened: neck minimum -85 to -100, head maximum 55 to 80 (needed to graze).
- `anchors`: hoof contacts at the new toe tips, belly and brisket contacts on the new fleece underside, nose and mouth on the muzzle, `back-plume` on the fleece surface (it floated above the back).
- `poses`: all 15 re-solved. Standing: far legs 0.09 and 0.08 m ahead, all four hooves on the ground. Twelve walk keys: every toe keeps its old offset from its rest position and the old hoof, cannon and scapula rotations, thigh/shank and humerus/forearm by two-bone IK (no unreachable target). Grazing and lying were authored anew (trunk pitched 4 degrees, muzzle 40 mm above the ground; legs folded by stated world angles, hip 0.228 m high).
- `materials`: `bone` added (skeleton solids used the fleece material).
- `representations`: all 217 closed shapes of the four scales and three views redrawn as the orthographic outline of the solids assigned to them (ruling E1), same ids, bones, roles, order, tones and hatches; ellipse, capsule, path and extruded shapes became polygons. Outlines are simplified to a fifth of the scale's feature. `mouth-line` redrawn by hand.
- Not changed: `dimensions`, every `areaM2`, masses, clips, thermal and structural data.

## 4. Models

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 46: two trunk lofts, ham and shoulder, neck, skull, mandible, topknot, eyes, ears, tail in two parts; per leg gaskin or forearm, point of hock, cannon with fetlock, two dewclaws, pastern, two claws | 50: one reference per body solid by region, two nostrils, lips, plume patch | 113: skull, mandible, cervical / thoracic / lumbar spine with processes, sacrum, ilium, ischium, pubis, 26 ribs, costal arches, sternum, scapulae, long bones, olecranon, patella, calcaneus, digits and pedal bones, caudal vertebrae, ear cartilages, 28 joints, 2 masses |
| 1:100 | 23 | 27 | 62 (six rib hoops per side) |
| 1:200 | 19 | 26 (pastern and hoof split) | 51 (three rib hoops per side) |
| 1:500 | 12 | 21 (ears, tail, hooves as own small solids) | 30 (one bar per bone) |

Every coarser scale has its own hand tables in `r3-sheep-coarse.ts`. `decisions` are bilingual sentences per list.

Final check (`volumes-check.ts animals/sheep --product`): 38 rule lines ok, 3 fail, all the same cause: `areaDeviation:hooves` 1.45 at 1:50, 0.78 at 1:100, and the 1:200 total 0.70. The declared 0.016 m² is less than half of the real horn surface; request with measurements in `author-requests.md` ("r3 animals/sheep"). Silhouette IoU elevation / front / plan: 0.997 / 0.995 / 0.998 (1:50), 0.994 / 0.992 / 0.996, 0.987 / 0.983 / 0.994, 0.985 / 0.978 / 0.985. `r3-validate.ts animals/sheep`: VALID, schema 0, validateAsset 0. Spliced with `volumes-insert.py`.

## 5. What I saw

Opened: 1:50 body from the front, side, front and rear three-quarter; the `volumes-render.ts` sheets of all four scales; the 1:50 structure from the side; the 1:50 thermal cutaway; the `r3-sheet.ts` six-view body sheet at 1:50 with the rig; grazing, lying and walk key 3 at 1:50.

- 1:50 body: reads as a ewe in full fleece from every direction: round barrel on thin legs, stifle forward, hock back with its point, vertical cannons, sloping pasterns, cloven hooves, wedge head with horizontal ears. The rig lies inside the forms in the six-view sheet.
- Structure 1:50 (side): a recognisable skeleton with ribcage, scapula blade, pelvis and limb bones; joints are no larger than the bone ends.
- Poses: grazing puts the muzzle in front of the fore hooves, lying folds the legs under the fleece, the walk key has three hooves on the ground and one lifted.
- 1:100 and 1:200 keep the form; 1:500 is a sturdy symbol with legs at the 82 mm floor.

## 6. Still imperfect

- The three hoof-area check lines fail (see above); the asset is not green.
- The fleece is a smooth volume; a trial with staple lobes looked like barnacles and was removed. The top line and belly line are nearly straight.
- Thermal is the body surface per region (each region has one declared layer); there is no skin core under the fleece shell in the cutaway.
- Not looked at after the last small edits: structure and thermal from a second direction at 1:50 and 1:100, body alone from four directions at 1:100 (seen only in its sheet), the six-view sheets of the coarser scales and of structure/thermal (rendered under `🗑️generated/r3-sheet/animals/`, not opened), the 2D drawings through `r3-2d.ts`, and the walk clip beyond one key.
- The far legs' front and plan shapes rely on the product's carry of a rotated bone; the fit is exact in the default pose only.
- The text of `representations[*].decisions` still quotes the old drawing sizes (for example "Cannons 0.034 m", "head is 4.6 mm long"); it was not rewritten.
- The ham shows as a flap under the belly when a hind leg swings far back.
- 1:500 legs are 82 mm thick against a real 30 mm cannon.
