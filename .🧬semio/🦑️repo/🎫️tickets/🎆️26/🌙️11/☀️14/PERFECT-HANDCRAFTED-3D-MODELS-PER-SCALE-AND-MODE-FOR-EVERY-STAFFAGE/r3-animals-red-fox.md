# Round 3 — `animals/red-fox` (2026-11-19)

State: spliced. `volumes-check.ts animals/red-fox --product`: PASS (34 rule lines ok, 0 fail). `r3-validate.ts animals/red-fox --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0). No earlier round-3 work existed for this asset; everything below was done in this run.

My own grade by eye: body 1:50 B+, 1:100 B+, 1:200 B; structure B+ / B / B-; thermal A- (it is the coat surface, nothing more); 2D A- elevation, B+ front and plan. It is a fox at first sight from every direction. It is not "perfect"; section 6 says where.

Scripts (all in the ticket folder), in the order they run:

1. `r3-red-fox-rig.py [--candidate|--apply]` — rig, anchors, poses, clip key overrides, dimensions (reads `🗑️generated/r3-red-fox/asset-before.json`, the asset before this round, kept as input so the script can be re-run).
2. `r3-red-fox-50.ts --write`, `r3-red-fox-coarse.ts 100 --write`, `r3-red-fox-coarse.ts 200 --write` — every section of every solid by hand table, all three lists, decisions.
3. `r3-red-fox-2d.ts` then `r3-red-fox-2d.py [--apply]` — ruling E1 redraw and dimensions.
4. `volumes-check.ts`, `volumes-insert.py`, `r3-validate.ts`.

Helpers: `r3-red-fox-kit.ts` (placing hand rows in bone frames), `r3-red-fox-io.py` (replaces whole members of the catalog file, everything else byte-identical), `r3-red-fox-frames.ts`, `r3-red-fox-bbox.ts`.

## 1. Reference (adult red fox, *Vulpes vulpes*, 6 kg, autumn coat)

Sources: my own knowledge of the species accounts (Larivière and Pasitschniak-Arts 1996, Mammalian Species 537; Macdonald, "Running with the Fox"; Lloyd, "The Red Fox": head and body 0.58 to 0.90 m, tail 0.32 to 0.49 m, shoulder 0.35 to 0.50 m, hind foot 0.125 to 0.185 m, ear 0.077 to 0.125 m, skull 0.13 to 0.155 m, zygomatic breadth 0.07 to 0.08 m), of canid anatomy (Ellenberger/Dittrich/Baum, "An Atlas of Animal Anatomy for Artists"; Goldfinger, "Animal Anatomy for Artists"; Miller's "Anatomy of the Dog" for the vertebral formula C7 T13 L7 S3 and the rib count 13 with 9 sternal) and of fox limb bone lengths from osteometric literature. No web search was run. Values are textbook figures for the species scaled to a 6 kg animal, about ±1 cm; they are not measurements of one specimen.

| landmark / size | reference | model (world, x ahead of the hip joint, y up) |
|---|---|---|
| shoulder height with coat | 0.35 to 0.50 m, about 0.40 to 0.44 at 6 kg | withers top 0.440 |
| head and body, nose to tail root | 0.62 to 0.72 m at 6 kg | 0.69 straight (nose 0.658, tail root -0.030) |
| tail with hair | 0.35 to 0.45 m | bones 0.400, with hair tip 0.435 |
| length standing, nose to brush tip | about 1.05 to 1.15 m | 1.100 |
| height to the ear tips, alert | about 0.58 to 0.62 m | 0.587 |
| skull length / with nose pad | 0.140 to 0.150 / 0.155 | head bone 0.155 |
| zygomatic breadth; head with cheek ruff | 0.075 to 0.080; about 0.10 to 0.11 | skull bone 0.073; coat 0.096 skull, 0.110 across the ruffs |
| ear length x breadth | 0.085 to 0.10 x 0.05 | 0.092 x 0.053 |
| cervical / thoracic / lumbar spine | 0.14 to 0.15 / 0.185 to 0.195 / 0.165 to 0.18 | 0.145 / 0.190 / 0.175 |
| hip joint height | about 0.37 to 0.38 | 0.375 |
| femur / stifle height | 0.125 to 0.135 / 0.25 to 0.26 | 0.130 / 0.257 |
| tibia / hock joint height | 0.14 to 0.15 / 0.12 to 0.13 | 0.141 / 0.123 |
| hock joint to metatarsophalangeal joint | 0.085 to 0.09 | 0.088 |
| hind foot, point of hock to toe tip | 0.15 to 0.17 | 0.168 |
| scapula, inclination | 0.09 to 0.10, 55 to 60 degrees | 0.095, 57 degrees |
| shoulder joint height | about 0.32 to 0.33 | 0.328 |
| humerus / elbow height | 0.118 to 0.125 / 0.21 to 0.22 | 0.120 / 0.217 |
| radius / carpus height | 0.115 to 0.12 / 0.095 to 0.10 | 0.118 / 0.099 |
| carpus to metacarpophalangeal joint | 0.06 to 0.07 | 0.066 |
| digits, joint to toe tip | 0.05 (fore), 0.055 (hind) | 0.050 / 0.055 |
| chest depth with coat; chest width with coat | 0.20 to 0.22; 0.14 to 0.15 | 0.216; 0.148 |
| cannon with coat | 0.021 to 0.025 | fore 0.023 x 0.024, hind 0.021 x 0.021 |
| paw length x width | 0.05 to 0.06 x 0.035 to 0.04 | 0.055 x 0.037 hind, 0.050 x 0.040 fore |
| brush diameter | 0.11 to 0.13 | 0.118 |
| ribs | 13 pairs, 9 sternal, about 5 mm | 13 pairs, 9 sternal, 5 mm |

## 2. Audit of the state before this round

- Rig: the hind foot was 0.205 m from the point of hock to the toe (metatarsus bone 0.110, paw 0.070; real 0.15 to 0.17), which put the hock at 0.160 m and the hip at 0.407 m. Fore: metacarpus 0.080 (real 0.066), paw 0.060 (0.050), humerus 0.110 (0.120). Trunk: lumbar 0.219 and thorax 0.200 (real 0.175 and 0.190), so the body was about 5 cm long. The spine chain was one horizontal line at y 0.432 from the loin to the base of the neck: the thoracic spine did not descend to C7/T1 between the shoulder blades, and the scapula hung from that line. Jaw bone 0.085 from a hinge 0.05 m ahead of the occiput (real mandible 0.105 from a joint 0.028 m ahead); head carried 32 degrees below the horizontal in the alert pose; brush nearly horizontal while the pose text says "hanging low".
- `dimensions`: 0.626 x 1.173 x 0.16 m followed that rig. The summary said "0.5 m at the shoulder", which neither the old rig (0.45) nor a 6 kg fox has.
- Poses: `curled-sleeping` stood on folded legs with the belly about 7 cm above the ground.
- 2D: elevation was a jointed-puppet outline; front and plan were extruded prisms (the fox head-on was a stack of rectangles with rectangular ears, the plan a row of rectangles). The 1:200 plan had a `nose` body shape that no other view of that scale has.
- Solids: 26 body solids at 1:50: box-like trunk in two segments, plank tail, wedge head, legs as tapered boxes, paws as wedges; structure = one capsule per bone; thermal plume patch.

## 3. Root fixes in the asset file (old to new)

- `rig.bones`, lengths in m: lumbar 0.2191 to 0.1750, thorax 0.2000 to 0.1897, neck 0.1400 to 0.1450, head 0.1600 to 0.1550, jaw 0.0850 to 0.1059, tail-3 0.1200 to 0.1100, metatarsus 0.1100 to 0.0878, hind paw 0.0700 to 0.0553, scapula 0.1100 to 0.0954, humerus 0.1100 to 0.1202, forearm 0.1200 to 0.1180, metacarpus 0.0800 to 0.0659, fore paw 0.0600 to 0.0502; pelvis, thigh, shank, ears, tail-1, tail-2 unchanged within a millimetre. Rest angles and origins follow the landmarks of section 1 (table `J` in `r3-red-fox-rig.py`): hip joint 0.375 high, thoracic spine descending 14 degrees to C7/T1 at (0.410, 0.362), scapula from the dorsal border at (0.372, 0.408), jaw hinge at head (0.028, -0.012), head 14 degrees below the horizontal, brush hanging at 25 to 12 degrees below the horizontal. `mass.comX/comY` moved with the bones (mid bone for limbs, hand-placed for the trunk); masses unchanged.
- `anchors`: paw contacts at the new toe tips; `nose` and `mouth` on the muzzle surface; `back-plume` on the coat above the thoracolumbar joint (it floated 7 cm above the spine line).
- `poses`, all 15 re-solved: toes keep their old offset from the rest position and the paws and cannons their old rotation (two-bone IK for thigh/shank and humerus/forearm; where the shorter leg could not reach, the cannon and paw give way); neck, head and brush keep their old world carriage in every pose but the default one; root heights lowered by the 0.0317 m the hip came down. `standing-alert`: far hind paw 0.08 m and far fore paw 0.055 m ahead (were 0.115 and 0.12). `curled-sleeping` authored anew as lying: trunk pitched 12 degrees, legs folded flat by stated world angles, hip 0.078 m high, belly on the ground; neck, head and brush wrap as before. `pounce-flight`: the stretched legs end 6 to 13 mm short of the old toe points (airborne, no contact).
- `clips`: the three `head` key overrides of `mousing-pounce` shifted by the 6 degrees the head rest angle changed, so the head cock keeps its old world angle. Nothing else in the clips changed.
- `dimensions`: height 0.626 to 0.587, width (length) 1.173 to 1.100, depth 0.16 to 0.152, against the reference rows of section 1. The same height and length in the fox row of `🗂️catalog/🔣️.json`.
- `summary` (en, de): "1.15 m ... 0.5 m at the shoulder" to "1.10 m ... 0.44 m at the shoulder".
- `materials`: `bone` added (the skeleton had no material of its own).
- `representations` (ruling E1): all 202 shapes of the three authored scales redrawn as the orthographic outline of the body solids assigned to them, same ids, bones, roles, order, tones, regions, hatches; outlines simplified to a quarter of the scale's feature. Because the fox bends spine, neck, head and brush sideways when curled, the validator refuses flat shapes on those bones (`lateral-ambiguous`), which is why front and plan used to be prisms. Each view now states its own true outline as a solid hint that draws exactly that outline in the default pose: elevation = profile (extruded over the measured lateral span on the bones that leave the plane), front = meridian profile (`revolved` without mirror about the bone's own axis), plan = a 1 mm slab in the bone's (u, w) plane (`extruded`, sweep `v`); upright limb segments and the ears are a tapered prism over their side profile in the plan. `mouth-line` redrawn by hand on the new lips. The 1:200 plan `nose` shape was removed.
- Not changed: `thermal` data and every `areaM2`, `structure` data (no supports are declared), masses, clip timing and contacts, the 1:500 unavailability.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 51: trunk in two lofts (loin, chest), thigh, shoulder and upper arm masses, neck with ruff, skull with stop and long muzzle, pale mandible, cheek ruffs, nose pad, eyes, cupped leaf ears, brush in three links with the pale tip; per leg gaskin or forearm, point of hock, cannon, paw with pad mass and four toes | 55: one reference per body solid by region (layer `fur`), two nostrils, lips, back strip | 106: skull, zygomatic arches, mandible halves, cervical S, thoracic spine with the spinous fin of the withers, 13 ribs per side at 5 mm (slender), costal arches, sternum, lumbar spine with transverse and spinous processes, sacrum, ilium, ischium, pubis, caudal vertebrae, ear cartilages, scapula, humerus, radius-ulna with olecranon, metacarpals, digits, femur, patella, tibia, calcaneus, metatarsals, digits, 29 joints, 2 masses |
| 1:100 | 25: own tables; shoulder blade and upper arm merged into one bent mass, head as one wedge without eyes and nose, hock merged into the cannon, paws without toes | 28 | 76: 7 ribs per side, one pelvic girdle, one member per long bone |
| 1:200 | 21: thighs and shoulders merged into the trunk lofts, gaskin and forearm reach into the trunk, cannons, paws and ears at the 30 mm floor | 24 | 69: bars at the floor, 4 ribs per side |
| 1:500 | unavailable (as its representation) | | |

`decisions` are bilingual sentences per list and scale. Every thermal region of the fox declares one fur layer, so there are no nested shells; all six regions are `areaBasis: exchange`, so no area was compared or changed.

Final check line: `PASS`. Silhouette IoU elevation / front / plan: 0.990 / 0.990 / 0.990 (1:50), 0.984 / 0.978 / 0.983 (1:100), 0.961 / 0.950 / 0.975 (1:200). These follow from the redraw and prove nothing about quality.

## 5. What I saw (PNG opened with the Read tool)

- 1:50 body, six-view sheet with the rig, default pose (final state): right and left side read as a fox at once: level back, tucked flank, deep narrow chest, long neck with ruff, flat skull with a long pointed muzzle, large erect ears, slim dark legs with the hock point and small paws, bushy brush hanging low with the pale tip. Front: narrow chest between straight legs, wedge face, ears as triangles. Back: brush over rounded quarters. Top: waist behind the ribs, hips with thighs. The rig lies inside the forms in every panel.
- 1:50 body close-ups (three-quarter front, three-quarter rear, front, right side) during the work: toes read on every paw; seams between solids are visible as lines at the trunk joint, at the shoulder and thigh masses and at the brush joints.
- 1:50 `trot-cycle` t = 0.1 s (six views): diagonal pair on the ground, the other pair lifted and folded, limbs closed at every joint, brush carried out behind.
- 1:50 `mousing-crouch` (right side): forequarters lowered, head pointing down, hindquarters high, all four paws on the ground; the near foreleg folds in a tight Z.
- 1:50 `curled-sleeping` (six views, then right side after the last paw change was rendered but not opened again): the fox lies on the ground, spine curved in plan, head turned back onto the haunch, brush wrapped forward; the forelegs lie stretched out.
- 1:50 structure sheet (six views): a recognisable canid skeleton with ribcage, scapula plates, pelvis, limb bones inside the limbs, joints no larger than the bone ends.
- 1:50 thermal sheet (six views): six regions follow the body exactly; the back strip lies flush on the coat.
- 1:100 and 1:200 body sheets (six views): fox at a glance at both scales. This was 1:100 before its tables were smoothed; 1:200 is faceted, with stout legs and ears like short tubes.
- 1:200 structure sheet: bar skeleton inside the ghost; the sternum is a thick stub under four thin ribs.
- 2D through `r3-2d.ts`: 1:50 default pose (clean fox in elevation, true head-on front with triangular ears, true plan), 1:50 `curled-sleeping` (a lying fox in all three views), 1:200 (a small fox symbol in all three views).

## 6. Still imperfect, plainly

- Not opened after the last edits: the 1:100 body sheet after smoothing, the 1:100 structure and thermal sheets, the 1:200 thermal sheet, the 1:100 2D sheet (all rendered), `pounce-flight` and `pounce-land` (not rendered), the walk of the whole trot clip beyond one frame, and the single-list renders of `volumes-render.ts` from four directions (I used the six-view `r3-sheet.ts` instead).
- Body: sections are symmetric superellipses, so thigh, shoulder and upper arm are overlapping convex masses with visible seam lines, not one skin; the trunk joint and the brush joints show as lines. No white bib, no claws, no dewclaws, no whiskers; the ear is a closed leaf without a concha; eyes are small ellipsoids. The cheek ruff reads as a flap from the side.
- The coat is a smooth volume; fur is not modelled.
- 1:200: legs are 33 mm thick against a real 22 mm and ears are 33 mm thick wedges (size floor); the model is faceted.
- Structure: bones thinner than the size floor (caudal vertebrae, digits, zygomatic arches) are drawn at the floor; digits and metapodials are one bundle each, not four rays; ribs are single curves without separate costal cartilages; radius and ulna are one member. At 1:200 limb bars are as thick as the legs.
- Thermal: the single declared fur layer makes each region the coat surface; there is no skin core under it in a cutaway.
- 2D: front and plan outlines are exact in the default pose; in other poses each is a flat cut-out carried by its bone, so a strongly turned part (the curled head) is an approximation. Far-side legs keep the lighter 2D tone; in 3D both sides have the same tone. The `decisions` texts of the three representations still describe the old drawing ("23 × 12.5 mm", "brush as three extruded masses"); they were not rewritten.
- `curled-sleeping`: the forelegs lie stretched forward rather than tucked, and the rig cannot roll the body onto its side.
- `mousing-crouch` and the pounce poses were re-solved from the old ones, not re-authored; the near foreleg of the crouch folds very tightly.
- Reference values are from memory (±1 cm), not from a measured specimen or a web lookup.
- Not run: `repo` test targets and the model export.
